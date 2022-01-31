use crate::sources::span::Span;

/// A parse pair is a way of representing an AST, without actually using any datatypes that depend on the language definition.
/// This represents a parse pair for a sort. It stores which constructor was chosen.
#[derive(Debug, Clone)]
pub struct ParsePairSort<'src> {
    pub sort: &'src str,
    pub constructor_name: &'src str,
    pub constructor_value: ParsePairExpression<'src>,
}

impl<'src> ParsePairSort<'src> {
    /// What span does this parse pair occupy?
    pub fn span(&self) -> Span<'src> {
        self.constructor_value.span()
    }
}

/// A parse pair is a way of representing an AST, without actually using any datatypes that depend on the language definition.
/// This represents a parse pair for a constructor. Each constructor generates one of the variants of this enum.
#[derive(Debug, Clone)]
pub enum ParsePairExpression<'src> {
    /// This is generated when another sort is mentioned in the definition of this sort.
    /// That sort is parsed and the result is stored here.
    Sort(Span<'src>, Box<ParsePairSort<'src>>),
    /// This is generated when a list of constructors is executed. This can be generated by Sequence or Repeat.
    List(Span<'src>, Vec<ParsePairExpression<'src>>),
    /// This is generated when a Choice was made. The first argument is which choice was made, the second the parsed constructor.
    Choice(Span<'src>, usize, Box<ParsePairExpression<'src>>),
    /// This is generated when no useful information needed to be recorded here, but still a placeholder is needed to keep track of the span.
    /// Generated by Positive and Negative, as the actual values that were parsed in Positive and Negative are irrelevant.
    /// Generated by CharacterClass and Literal, as they don't generate values.
    Empty(Span<'src>),
}

impl<'src> ParsePairExpression<'src> {
    /// What span does this parse pair occupy?
    pub fn span(&self) -> Span<'src> {
        match self {
            ParsePairExpression::Sort(span, _) => span,
            ParsePairExpression::List(span, _) => span,
            ParsePairExpression::Choice(span, _, _) => span,
            ParsePairExpression::Empty(span) => span,
        }
        .clone()
    }
}