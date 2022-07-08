#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
// |==========================================================|
// |      WARNING: THIS FILE IS AUTOMATICALLY GENERATED.      |
// |      CHANGES TO IT WILL BE DELETED WHEN REGENERATED.     |
// | IN GENERAL, THIS FILE SHOULD NOT BE MODIFIED IN ANY WAY. |
// |==========================================================|
use super::prelude::*;
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Program<M: AstInfo>(pub M, pub Vec<SortOrMeta<M>>);
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum SortOrMeta<M: AstInfo> {
    Meta(M, Meta<M>),
    Sort(M, Sort<M>),
}
#[doc = "Other top-level constructs that are not sorts"]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Meta<M: AstInfo>(pub M, pub Identifier<M>);
#[doc = "A sort is a group of constructors. See [`constructor`] for more details."]
#[doc = ""]
#[doc = "There is one special sort, called `layout`. It can be used to denote"]
#[doc = "that a grammar should for example ignore certain whitespace or comments."]
#[doc = "Between every part of an expression, the parser will attempt to parse the"]
#[doc = "layout sort 0 or more times, and throw away whatever it parses."]
#[doc = ""]
#[doc = "Sorts can have doc-comments using triple slashes."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum Sort<M: AstInfo> {
    SortDocumented(M, Vec<DocComment<M>>, Box<Sort<M>>),
    Sort(M, Identifier<M>, Vec<Constructor<M>>),
    #[doc = "When a sort has only one constructor, it has simpler syntax."]
    SortSingle(M, Identifier<M>, Vec<Expression<M>>, Option<Annotation<M>>),
}
#[doc = "An identifier is any name of a constructor or sort, and is used in various places."]
#[doc = "Identifiers always start with a letter (capital or not) or an underscore, and can be"]
#[doc = "followed by letters or numbers. This is very similar to how variables in most major"]
#[doc = "programming languages work."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Identifier<M: AstInfo>(pub M, pub std::string::String);
#[doc = "A documentation comment (doc comment) is always associated with a sort"]
#[doc = "or constructor. It documents what it does. Doc comments will be interpreted"]
#[doc = "and will be put on the generated types during codegen."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct DocComment<M: AstInfo>(pub M, pub std::string::String);
#[doc = "A [`sort`] consists of constructors. A sort will try each of the constructors"]
#[doc = "from top to bottom, and use the first one that succesfully parses the input string."]
#[doc = ""]
#[doc = "A constructor consists of a name, followed by an [`expression`]"]
#[doc = ""]
#[doc = "Constructors can have doc-comments using triple slashes."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum Constructor<M: AstInfo> {
    ConstructorDocumented(M, Vec<DocComment<M>>, Box<Constructor<M>>),
    Constructor(M, Identifier<M>, Vec<Expression<M>>, Option<Annotation<M>>),
}
#[doc = "With expressions, you can give the syntax rules of a single constructor."]
#[doc = "Expressions can be nested and combined."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum Expression<M: AstInfo> {
    #[doc = "Repeat some expression zero or more times"]
    #[doc = "Equivalent to `<expression> {0,}`"]
    Star(M, Box<Expression<M>>),
    #[doc = "Repeat some expression one or more times"]
    #[doc = "Equivalent to `<expression> {1,}`"]
    Plus(M, Box<Expression<M>>),
    #[doc = "Optionally have some expression."]
    #[doc = "Equivalent to `<expression> {0,1}`"]
    Maybe(M, Box<Expression<M>>),
    #[doc = "Exact repetition. The expression is repeated an exact number of times. Equivalent"]
    #[doc = "to ranged repetition with an equal lower and upper bound."]
    RepeatExact(M, Box<Expression<M>>, Number<M>),
    #[doc = "Ranged repetition. The expression may be repeated any number of times, within the range."]
    #[doc = "Both bounds are inclusive."]
    RepeatRange(M, Box<Expression<M>>, Number<M>, Number<M>),
    #[doc = "Ranged repetition, without upper bound (or an infinite maximum)"]
    RepeatLower(M, Box<Expression<M>>, Number<M>),
    #[doc = "Matches a piece of text exactly. Layout is parsed within a literal."]
    Literal(M, Vec<StringChar<M>>),
    #[doc = "Also a literal, see [`literal`]"]
    SingleQuoteLiteral(M, Vec<StringChar<M>>),
    #[doc = "Delimited expressions. Says that some expression should be repeatedly parsed,"]
    #[doc = "but between two parses, a delimiter should be parsed too. For example, comma seperated expressions."]
    #[doc = "The final trailing keyword enables a trailing separator after the sequence. If not present, no trailing"]
    #[doc = "separator is allowed."]
    Delimited(
        M,
        Box<Expression<M>>,
        Box<Expression<M>>,
        DelimitedBound<M>,
        bool,
    ),
    #[doc = "Reference another sort within this expression. That sort should be parsed in this position in the expression."]
    Sort(M, Identifier<M>),
    #[doc = "A [`character class`](character-class) (range of characters) should be parsed here."]
    Class(M, CharacterClass<M>),
    #[doc = "You can use parentheses to group parts of expressions."]
    Paren(M, Vec<Box<Expression<M>>>),
}
#[doc = "Annotations are tags that modify a specific sort or more often constructor."]
#[doc = "TODO: Document each"]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Annotation<M: AstInfo>(pub M, pub Vec<Identifier<M>>);
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Number<M: AstInfo>(pub M, pub std::string::String);
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum StringChar<M: AstInfo> {
    Escaped(M, std::string::String),
    Normal(M, std::string::String),
}
#[doc = "A delimited expression can be repeated just like normal repetition expressions."]
#[doc = "To denote this, you can use a delimitation bound."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum DelimitedBound<M: AstInfo> {
    #[doc = "Within a range or possible repetitions."]
    NumNum(M, Number<M>, Number<M>),
    #[doc = "At least some number of repetitions, but no upper bound."]
    NumInf(M, Number<M>),
    #[doc = "Exactly this number of repetitions."]
    Num(M, Number<M>),
    Star(M),
    #[doc = "One or more repetitions."]
    Plus(M),
}
#[doc = "A character class represent a selection of terminal characters. This is similar to"]
#[doc = "Regex character classes. Character classes can be inverted by starting them with a `^`."]
#[doc = "For example, `[^\\n]` means it matches any character that is not a newline."]
#[doc = ""]
#[doc = "Character classes can contain a range of characters. Either by listing each individual character, or using"]
#[doc = "a dash (`-`). For example, `[a-z]` means any character in the range a through z (inclusive!),"]
#[doc = "and `[abc]` means an a, b or c"]
#[doc = ""]
#[doc = "Note that to use a closing square bracket within a character class, you need to escape it."]
#[doc = ""]
#[doc = "`[^\\]]` means any character that isn't a square bracket."]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct CharacterClass<M: AstInfo>(pub M, pub bool, pub Vec<CharacterClassItem<M>>);
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum CharacterClassItem<M: AstInfo> {
    Range(M, EscapeClosingBracket<M>, EscapeClosingBracket<M>),
    SingleChar(M, EscapeClosingBracket<M>),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum EscapeClosingBracket<M: AstInfo> {
    Escaped(M, std::string::String),
    Unescaped(M, std::string::String),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum Newline<M: AstInfo> {
    Unix(M),
    Windows(M),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum Layout<M: AstInfo> {
    Simple(M, std::string::String),
    Comment(M, Vec<std::string::String>),
}
pub type AST_ROOT<M> = Program<M>;
