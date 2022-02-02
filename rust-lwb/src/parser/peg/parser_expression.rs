#![allow(clippy::result_unit_err)]

use crate::codegen_prelude::{ParsePairExpression, ParsePairSort};
use crate::parser::bootstrap::ast::{Expression, Sort};
use crate::parser::peg::parse_error::{Expect, PEGParseError};
use crate::parser::peg::parse_result::ParseResult;
use crate::parser::peg::parser::{ParserCache, ParserState};
use crate::parser::peg::parser_sort::parse_sort;
use crate::sources::source_file::SourceFileIterator;
use crate::sources::span::Span;

/// Given an expression and the current position, attempts to parse this constructor.
pub fn parse_expression<'src>(
    state: &ParserState<'src>,
    cache: &mut ParserCache<'src>,
    constructor: &'src Expression,
    mut pos: SourceFileIterator<'src>,
) -> ParseResult<'src, ParsePairExpression<'src>> {
    match constructor {
        //To parse a sort, call parse_sort recursively.
        Expression::Sort(sort_name) => {
            let sort: &'src Sort = state
                .rules
                .get(&sort_name[..])
                .expect("Name is guaranteed to exist");
            let res = parse_sort(state, cache, sort, pos);
            res.map(|s| ParsePairExpression::Sort(s.span(), Box::new(s)))
        }
        //To parse a literal, use accept_str to check if it parses.
        Expression::Literal(lit) => {
            while cache.allow_layout && !pos.clone().accept_str(lit) && pos.accept(&state.layout) {}
            let span = Span::from_length(state.file, pos.position(), lit.len());
            if pos.accept_str(lit) {
                if cache.no_layout_nest_count > 0 {
                    cache.allow_layout = false;
                }
                ParseResult::new_ok(ParsePairExpression::Empty(span), pos)
            } else {
                cache.add_error(PEGParseError::expect(
                    span.clone(),
                    Expect::ExpectString(lit.clone()),
                ));
                ParseResult::new_err(ParsePairExpression::Error(span), pos)
            }
        }
        //To parse a character class, check if the character is accepted, and make an ok/error based on that.
        Expression::CharacterClass(characters) => {
            while cache.allow_layout && !pos.clone().accept(characters) && pos.accept(&state.layout)
            {
            }
            let span = Span::from_length(state.file, pos.position(), 1);
            if pos.accept(characters) {
                if cache.no_layout_nest_count > 0 {
                    cache.allow_layout = false;
                }
                ParseResult::new_ok(ParsePairExpression::Empty(span), pos)
            } else {
                cache.add_error(PEGParseError::expect(
                    span.clone(),
                    Expect::ExpectCharClass(characters.clone()),
                ));
                ParseResult::new_err(ParsePairExpression::Error(span), pos)
            }
        }
        //To parse a sequence, parse each constructor in the sequence.
        //The results are added to `results`, and the best error and position are updated each time.
        //Finally, construct a `ParsePairConstructor::List` with the results.
        Expression::Sequence(constructors) => {
            let mut results = vec![];
            let start_pos = pos.position();

            //Parse all subconstructors in sequence
            for subconstructor in constructors {
                let res = parse_expression(state, cache, subconstructor, pos);
                pos = res.pos;
                results.push(res.result);
                if !res.ok {
                    let span = Span::from_end(state.file, start_pos, pos.position());
                    return ParseResult::new_err(ParsePairExpression::List(span, results), pos);
                }
            }

            //Construct result
            let span = Span::from_end(state.file, start_pos, pos.position());
            ParseResult::new_ok(ParsePairExpression::List(span, results), pos)
        }
        //To parse a sequence, first parse the minimum amount that is needed.
        //Then keep trying to parse the constructor until the maximum is reached.
        //The results are added to `results`, and the best error and position are updated each time.
        //Finally, construct a `ParsePairConstructor::List` with the results.
        Expression::Repeat { c, min, max } => {
            let mut results = vec![];
            let start_pos = pos.position();
            let mut last_pos = pos.position();

            //Parse at most maximum times
            for i in 0..max.unwrap_or(u64::MAX) {
                let res = parse_expression(state, cache, c.as_ref(), pos);
                pos = res.pos;
                results.push(res.result);
                if !res.ok {
                    let span = Span::from_end(state.file, start_pos, pos.position());
                    //If we have not yet reached the minimum, we error.
                    //Otherwise, we break and ok after the loop body.
                    if i < *min {
                        return ParseResult::new_err(ParsePairExpression::List(span, results), pos);
                    } else {
                        break;
                    }

                }
                //If the position hasn't changed, then we're in an infinite loop
                if last_pos == pos.position() {
                    let span = Span::from_length(state.file, pos.position(), 0);
                    cache.add_error(PEGParseError::fail_loop(span.clone()));
                    return ParseResult::new_err(ParsePairExpression::List(span, results), pos);
                }
                last_pos = pos.position();
            }

            //Construct result
            let span = Span::from_end(state.file, start_pos, pos.position());
            ParseResult::new_ok(ParsePairExpression::List(span, results), pos)
        }
        //To parse a choice, try each constructor, keeping track of the best error that occurred while doing so.
        //If none of the constructors succeed, we will return this error.
        Expression::Choice(constructors) => {
            let mut results = vec![];
            assert!(constructors.len() > 0);
            for (i, subconstructor) in constructors.iter().enumerate() {
                let res = parse_expression(state, cache, subconstructor, pos.clone());
                if res.ok {
                    return ParseResult::new_ok(ParsePairExpression::Choice(res.result.span(), i, Box::new(res.result)), res.pos);
                } else {
                    results.push(res);
                }
            }
            let (i, res) = results.into_iter().enumerate().max_by_key(|(_, r)| r.pos.position()).unwrap();
            ParseResult::new_err(ParsePairExpression::Choice(res.result.span(), i, Box::new(res.result)), res.pos)
        }

        Expression::Negative(_) => {
            todo!()
        }
        Expression::Positive(_) => {
            todo!()
        }
    }
}
