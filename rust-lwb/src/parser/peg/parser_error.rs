use crate::sources::character_class::CharacterClass;
use crate::sources::span::Span;
use itertools::Itertools;
use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// A parsing error represents a single error that occurred during parsing.
/// The parsing error occurs at a certain position in a file, represented by the span.
/// The parsing error consists of multiple `ParseErrorSub`, which each represent a single thing that went wrong at this position.
#[derive(Debug, Clone, Error)]
#[error("A parse error occured!")]
pub struct ParseError<'src> {
    pub span: Span<'src>,
    pub expected: Vec<ParseErrorSub>,
    pub left_rec: bool,
}

impl Diagnostic for ParseError<'_> {
    /// Diagnostic severity. This may be used by [ReportHandler]s to change the
    /// display format of this diagnostic.
    ///
    /// If `None`, reporters should treat this as [Severity::Error]
    fn severity(&self) -> Option<Severity> {
        Some(Severity::Error)
    }

    /// Source code to apply this Diagnostic's [Diagnostic::labels] to.
    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.span)
    }

    /// Labels to apply to this Diagnostic's [Diagnostic::source_code]
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let expect_str = self.expected.iter().map(|exp| exp.to_string()).join(", ");
        let mut labels = vec![];

        //Leftrec label
        if self.left_rec {
            labels.push(LabeledSpan::new_with_span(Some("Encountered left recursion here. This is a problem with the grammar, and may hide other errors.".to_string()), self.span.clone()));
        }

        //Expected label
        match self.expected.len() {
            0 => {}
            1 => labels.push(LabeledSpan::new_with_span(
                Some(format!("Expected {} here", expect_str)),
                self.span.clone(),
            )),
            _ => labels.push(LabeledSpan::new_with_span(
                Some(format!("Expected one of {} here", expect_str)),
                self.span.clone(),
            )),
        }

        Some(Box::new(labels.into_iter()))
    }
}

impl<'src> ParseError<'src> {
    pub fn expect_char_class(span: Span<'src>, val: CharacterClass) -> Self {
        ParseError {
            span,
            expected: vec![ParseErrorSub::ExpectCharClass(val)],
            left_rec: false,
        }
    }

    pub fn expect_string(span: Span<'src>, val: String) -> Self {
        ParseError {
            span,
            expected: vec![ParseErrorSub::ExpectString(val)],
            left_rec: false,
        }
    }

    pub fn not_entire_input(span: Span<'src>) -> Self {
        ParseError {
            span,
            expected: vec![ParseErrorSub::NotEntireInput()],
            left_rec: false,
        }
    }

    pub fn left_recursion(span: Span<'src>) -> Self {
        ParseError {
            span,
            expected: vec![],
            left_rec: true,
        }
    }
}

/// Represents a single thing that went wrong at this position.
#[derive(Debug, Clone)]
pub enum ParseErrorSub {
    /// Expect a character from a certain char class to be there, but it was not.
    ExpectCharClass(CharacterClass),

    /// Expect a certain string (keyword) to be there, but it was not.
    ExpectString(String),

    /// This happens when not the entire input was parsed, but also no errors occurred during parsing.
    NotEntireInput(),
}

impl Display for ParseErrorSub {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorSub::ExpectCharClass(cc) => {
                write!(f, "{}", cc)
            }
            ParseErrorSub::ExpectString(s) => {
                write!(f, "\'{}\'", s)
            }
            ParseErrorSub::NotEntireInput() => {
                write!(f, "more input")
            }
        }
    }
}

impl<'src> ParseError<'src> {
    /// Combine multiple parse errors. When one has precedence over
    /// another, the highest precedence error is kept and the other
    /// is discarded.
    ///
    /// Highest precedence is defined as furthest starting position for now. This might be changed later.
    pub fn combine(mut self, mut other: ParseError<'src>) -> ParseError<'src> {
        assert_eq!(self.span.source.name(), other.span.source.name());

        //Compare the starting positions of the span
        match self.span.position.cmp(&other.span.position) {
            Ordering::Less => other,
            Ordering::Greater => self,
            Ordering::Equal => {
                //The span is extended such that the longest one is kept.
                self.span.length = self.span.length.max(other.span.length);
                //Merge the expected tokens
                self.expected.append(&mut other.expected);
                //Left recursion
                self.left_rec |= other.left_rec;

                self
            }
        }
    }

    /// A helper that combines optional parse errors, and returns an optional parse error if either exists.
    /// If both exist, use `ParseError::combine` to combine the errors.
    pub fn combine_option_parse_error(
        a: Option<ParseError<'src>>,
        b: Option<ParseError<'src>>,
    ) -> Option<ParseError<'src>> {
        match (a, b) {
            (None, None) => None,
            (None, Some(e)) => Some(e),
            (Some(e), None) => Some(e),
            (Some(e1), Some(e2)) => Some(e1.combine(e2)),
        }
    }
}
