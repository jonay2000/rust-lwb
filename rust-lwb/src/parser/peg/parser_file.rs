use crate::codegen_prelude::ParsePairSort;
use crate::parser::bootstrap::ast::SyntaxFileAst;
use crate::parser::peg::parse_error::{Expect, PEGParseError};
use crate::parser::peg::parser::{ParserContext, ParserState};
use crate::parser::peg::parser_sort::parse_sort;
use crate::sources::source_file::SourceFile;
use crate::sources::span::Span;
use std::collections::{HashMap, VecDeque};

/// Parses a file, given the syntax to parse it with, and the file.
/// When successful, it returns a `ParsePairSort`.
/// When unsuccessful, it returns a `ParseError`.
pub fn parse_file<'src>(
    syntax: &'src SyntaxFileAst,
    file: &'src SourceFile,
) -> Result<ParsePairSort<'src>, PEGParseError> {
    //Create a new parser state
    let mut state = ParserContext {
        file,
        rules: HashMap::new(),
        layout: syntax.layout.clone(),
    };
    syntax.sorts.iter().for_each(|rule| {
        state.rules.insert(&rule.name, rule);
    });

    let mut cache = ParserState {
        cache: HashMap::new(),
        cache_stack: VecDeque::new(),
        best_error: None,
        trace: VecDeque::new(),
        no_layout_nest_count: 0usize,
        no_errors_nest_count: 0usize,
        allow_layout: true,
    };

    //Parse the starting sort
    let starting_sort = state
        .rules
        .get(&syntax.starting_sort[..])
        .expect("Starting sort exists");
    let mut res = parse_sort(&state, &mut cache, starting_sort, file.iter());
    if !res.ok {
        return Err(cache.best_error.unwrap());
    }

    //If there is no input left, return Ok.
    res.pos.skip_layout(&state.layout);
    if res.pos.peek().is_none() {
        Ok(res.result)
    } else {
        //If any occurred during the parsing, return it. Otherwise, return a generic NotEntireInput error.
        //I'm not entirely sure this logic always returns relevant errors. Maybe we should inform the user the parse was actually fine, but didn't parse enough?
        match cache.best_error {
            Some(err) => Err(err),
            None => {
                let curpos = res.pos.position();
                while res.pos.next().is_some() {}
                let endpos = res.pos.position();
                Err(PEGParseError::expect(
                    Span::from_end(file, curpos, endpos),
                    Expect::NotEntireInput(),
                ))
            }
        }
    }
}
