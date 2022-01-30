use crate::parser::syntax_file::ast::{Annotation, Constructor, Sort, SyntaxFileAst};
use crate::source_file::{SourceFile, SourceFileIterator};
use thiserror::Error;
use crate::parser::syntax_file::character_class::CharacterClass;
use crate::parser::syntax_file::parser::ParseError::{DuplicateStartingRule, Expected, InvalidAnnotation, NoStartingRule, UnexpectedCharacter, UnexpectedEndOfFile};
use lazy_static::lazy_static;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("found duplicate starting rule definition found")]
    DuplicateStartingRule,

    #[error("syntax definition contains no starting rule")]
    NoStartingRule,

    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char),

    #[error("invalid annotation: {0}")]
    InvalidAnnotation(String),

    #[error("unexpected character, expected: {0}")]
    Expected(String),

    #[error("end of file (though more input was expected)")]
    UnexpectedEndOfFile,

    #[error("invalid character range (left side of range might have been higher than the right side)")]
    InvalidCharacterRange
}

type ParseResult<T> = Result<T, ParseError>;

lazy_static! {
    static ref SYNTAX_FILE_LAYOUT: CharacterClass = CharacterClass::all_in_vec(vec![' ', '\n', '\t', '\r']);
}

/// Parse a source file into a syntax file ast.
pub fn parse(f: &SourceFile) -> ParseResult<SyntaxFileAst> {
    let mut iterator = f.iter();
    parse_file(&mut iterator)
}

pub enum SortOrMeta {
    Sort(Sort),
    StartRule(String),
    Layout(CharacterClass),
}

fn parse_file(i: &mut SourceFileIterator) -> ParseResult<SyntaxFileAst> {
    let mut sorts = Vec::new();
    let mut layout = CharacterClass::Nothing;
    let mut starting_rule = None;

    while let Some(_) = i.peek() {
        match parse_sort_or_meta(i)? {
            SortOrMeta::Sort(c) => sorts.push(c),
            SortOrMeta::StartRule(_) if starting_rule.is_some() => {
                return Err(DuplicateStartingRule)
            }
            SortOrMeta::StartRule(c) => {
                starting_rule = Some(c)
            }
            SortOrMeta::Layout(c) => {
                layout = layout.combine(c)
            }
        }
    }

    Ok(SyntaxFileAst {
        sorts,
        starting_rule: starting_rule.ok_or(NoStartingRule)?,
        layout,
    })
}

fn parse_sort_or_meta(i: &mut SourceFileIterator) -> ParseResult<SortOrMeta> {
    if i.accept_str("layout") {
        Ok(SortOrMeta::Layout(parse_character_class(i)?))
    } else if i.accept_str("start") {
        Ok(SortOrMeta::StartRule(parse_identifier(i)?))
    } else {
        let name = parse_identifier(i)?;
        let mut constructors = vec![parse_constructor(i)?];

        while i.accept('|') {
            constructors.push(parse_constructor(i)?);
        }

        let annotations = parse_annotations(i)?;

        Ok(SortOrMeta::Sort(Sort {
            name,
            constructors,
            annotations
        }))
    }
}

fn parse_annotation(i: &mut SourceFileIterator) -> ParseResult<Option<Annotation>> {
    i.skip_layout(SYNTAX_FILE_LAYOUT.clone());

    if i.accept_str("no-pretty-print") {
        Ok(Some(Annotation::NoPrettyPrint))
    } else if i.accept_str("no-layout") {
        Ok(Some(Annotation::NoLayout))
    } else if i.peek() == Some(&'}') {
        Ok(None)
    } else {
        let chars: CharacterClass = SYNTAX_FILE_LAYOUT.clone().combine(CharacterClass::all_in_vec(vec!['}', ',']));
        Err(InvalidAnnotation(i.accept_to_next(chars)))
    }
}

fn parse_annotations(i: &mut SourceFileIterator) -> ParseResult<Vec<Annotation>> {
    i.skip_layout(SYNTAX_FILE_LAYOUT.clone());

    if i.accept('{') {
        let mut annotations = vec![];

        if let Some(a) = parse_annotation(i)? {
            annotations.push(a);

            while i.accept_skip_layout(",", SYNTAX_FILE_LAYOUT.clone()) {
                if let Some(a) = parse_annotation(i)? {
                    annotations.push(a);
                }
            }
        }

        if !i.accept("}") {
            return Err(Expected("closing brace (})".to_string()))
        }

        Ok(annotations)
    } else {
        Ok(vec![])
    }
}

fn parse_constructor(i: &mut SourceFileIterator) -> ParseResult<Constructor> {
    todo!()
}

fn parse_identifier(i: &mut SourceFileIterator) -> ParseResult<String> {
    i.skip_layout(SYNTAX_FILE_LAYOUT.clone());
    let mut res = String::new();
    if let Some(c) = i.accept_option(CharacterClass::from('a'..='z')
        .combine(CharacterClass::from('A'..='Z'))
        .combine("_$".into())
    ) {
        res.push(c);

        while let Some(c) = i.accept_option(CharacterClass::from('a'..='z')
            .combine(CharacterClass::from('A'..='Z'))
            .combine(CharacterClass::from('0'..='9'))
            .combine("_$".into())
        ){
            res.push(c)
        }
    }

    Ok(res)
}

fn parse_character_class(i: &mut SourceFileIterator) -> ParseResult<CharacterClass> {
    i.skip_layout(SYNTAX_FILE_LAYOUT.clone());

    if i.accept('[') {
        let mut res = CharacterClass::Nothing;
        let mut invert = false;

        if i.accept("^") {
            invert = true;
        }

        while let Some(&c) = i.peek() {
            if c == ']' {
                break;
            }
            i.advance();
            if i.peek() == Some(&'-') {
                let lower = c;
                i.advance();
                if let Some(upper) = i.next() {
                    if lower as u32 > upper as u32 {
                        return Err(ParseError::InvalidCharacterRange)
                    }

                    res = res.combine((lower..=upper).into());

                    continue
                }

                return Err(UnexpectedEndOfFile)
            } else {
                res = res.combine(c.into())
            }
        }

        if !i.accept(']') {
            Err(Expected("closing ] for character class".to_string()))
        } else if invert {
            Ok(res.invert())
        } else {
            Ok(res)
        }

    } else {
        Err(Expected("[ for character class".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_test {
        ($name: ident test that $input: literal parses) => {
            parse_test!($name test that $input parses with parse_file)
        };

        ($name: ident test that $input: literal parses with $parse_func: ident) => {
            #[test]
            fn $name () {
                let sf = SourceFile::new_for_test($input);
                let mut sfi = sf.iter();
                let res = $parse_func(&mut sfi);
                assert!(res.is_ok(), "{:?}", res);
            }
        };
        ($name: ident test that $input: literal fails to parse with $parse_func: ident) => {
            #[test]
            fn $name () {
                let sf = SourceFile::new_for_test($input);
                let mut sfi = sf.iter();
                let res = $parse_func(&mut sfi);
                assert!(res.is_err(), "{:?}", res);
            }
        };
    }
    parse_test!(empty_annotation test that "{}" parses with parse_annotations);
    parse_test!(single_annotation test that "{no-layout}" parses with parse_annotations);
    parse_test!(multiple_annotation test that "{no-layout, no-layout}" parses with parse_annotations);
    parse_test!(trailing_comma_annotation test that "{no-layout, }" parses with parse_annotations);
    parse_test!(double_trailing_comma_annotation test that "{no-layout,, }" fails to parse with parse_annotations);
    parse_test!(leading_comma_annotation test that "{,no-layout}" fails to parse with parse_annotations);
    parse_test!(layout_annotation test that "   {  no-layout  ,  no-layout  , }  " parses with parse_annotations);


    parse_test!(simple_cc test that "[]" parses with parse_character_class);
    parse_test!(simple_inversion_cc test that "[^]" parses with parse_character_class);
    parse_test!(with_chars_cc test that "[abc]" parses with parse_character_class);
    parse_test!(with_range_cc test that "[a-z]" parses with parse_character_class);
    parse_test!(with_ranges_cc test that "[a-z0-9]" parses with parse_character_class);
    parse_test!(no_end_range_cc test that "[a-]" fails to parse with parse_character_class);
    parse_test!(inverted_range_cc test that "[z-a]" fails to parse with parse_character_class);

    macro_rules! character_class_test {
        ($name: ident cc $input: literal contains $($c:literal)*) => {
            #[test]
            fn $name () {
                let sf = SourceFile::new_for_test($input);
                let mut sfi = sf.iter();
                let res = parse_character_class(&mut sfi);
                assert!(res.is_ok(), "{:?}", res);
                let res = res.unwrap();

                $(
                    for c in $c.chars() {
                        assert!(res.contains(c));
                    }
                )*
            }
        };
        ($name: ident cc $input: literal excludes $($c:literal)*) => {
            #[test]
            fn $name () {
                let sf = SourceFile::new_for_test($input);
                let mut sfi = sf.iter();
                let res = parse_character_class(&mut sfi);
                assert!(res.is_ok(), "{:?}", res);
                let res = res.unwrap();

                $(
                    for c in $c.chars() {
                        assert!(!res.contains(c), "{:?}", res);
                    }
                )*
            }
        };
    }

    character_class_test!(simple cc "[a-z]" contains "abcdefghijklmnopqrstuvwxyz");
    character_class_test!(simple_exclude cc "[a-z]" excludes "0123456789ABCDEFHIJKLMNOPQRSTUVWXYZ:)");
    character_class_test!(multiple_groups cc "[a-z0-9]" contains "abcdefghijklmnopqrstuvwxyz0123456789");
    character_class_test!(inverted_cc cc "[^a-z0-9]" excludes "abcdefghijklmnopqrstuvwxyz0123456789");
    character_class_test!(inverted_cc_2 cc "[^a-z0-9]" contains "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    character_class_test!(just_some_chars cc "[abc]" contains "abc");
    character_class_test!(just_some_chars_2 cc "[abc]" excludes "xyz");
}


