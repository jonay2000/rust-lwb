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
impl<M: AstInfo> FromPairs<M> for Program<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "program");
        let info = generator.generate(&pair);
        Self(
            info,
            if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { SortOrMeta :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "program") ; }) . collect ()
            } else {
                unreachable!(
                    "expected different parse pair expression in pair to ast conversion of {}",
                    "program"
                );
            },
        )
    }
}
impl<M: AstInfo> FromPairs<M> for SortOrMeta<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "sort-or-meta");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "meta" => {
                Self::Meta(
                    info,
                    if let ParsePairExpression::Sort(_, ref s) = pair.constructor_value {
                        Meta::from_pairs(s, generator)
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort-or-meta");
                    },
                )
            }
            "sort" => {
                Self::Sort(
                    info,
                    if let ParsePairExpression::Sort(_, ref s) = pair.constructor_value {
                        Sort::from_pairs(s, generator)
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort-or-meta");
                    },
                )
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Meta<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "meta");
        let info = generator.generate(&pair);
        if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
            Self(
                info,
                if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                    Identifier::from_pairs(s, generator)
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "meta"
                    );
                },
            )
        } else {
            unreachable!(
                "expected different parse pair expression in pair to ast conversion of {}",
                "meta"
            );
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Sort<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "sort");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "sort-documented" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::SortDocumented(
                        info,
                        if let ParsePairExpression::List(_, ref l) = l[0usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { DocComment :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[1usize] {
                            Box::new(Sort::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "sort"
                    );
                }
            }
            "sort" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Sort(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Identifier::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                        if let ParsePairExpression::List(_, ref l) = l[2usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Constructor :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "sort"
                    );
                }
            }
            "sort-single" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::SortSingle(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Identifier::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                        if let ParsePairExpression::List(_, ref l) = l[2usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Expression :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                        if let ParsePairExpression::List(_, ref l) = l[4usize] {
                            l . first () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Annotation :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort") ; })
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "sort");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "sort"
                    );
                }
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Identifier<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "identifier");
        let info = generator.generate(&pair);
        return Self(info, pair.constructor_value.span().as_str().to_string());
    }
}
impl<M: AstInfo> FromPairs<M> for DocComment<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "doc-comment");
        let info = generator.generate(&pair);
        return Self(info, pair.constructor_value.span().as_str().to_string());
    }
}
impl<M: AstInfo> FromPairs<M> for Constructor<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "constructor");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "constructor-documented" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::ConstructorDocumented(
                        info,
                        if let ParsePairExpression::List(_, ref l) = l[0usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { DocComment :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[1usize] {
                            Box::new(Constructor::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "constructor"
                    );
                }
            }
            "constructor" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Constructor(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[1usize] {
                            Identifier::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor");
                        },
                        if let ParsePairExpression::List(_, ref l) = l[3usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Expression :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor");
                        },
                        if let ParsePairExpression::List(_, ref l) = l[5usize] {
                            l . first () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Annotation :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor") ; })
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "constructor");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "constructor"
                    );
                }
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Expression<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "expression");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "star" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Star(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "plus" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Plus(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "maybe" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Maybe(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "repeat-exact" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::RepeatExact(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "repeat-range" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::RepeatRange(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[4usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "repeat-lower" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::RepeatLower(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "literal" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Literal(
                        info,
                        if let ParsePairExpression::List(_, ref l) = l[1usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { StringChar :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "single-quote-literal" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::SingleQuoteLiteral(
                        info,
                        if let ParsePairExpression::List(_, ref l) = l[1usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { StringChar :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "delimited" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Delimited(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[4usize] {
                            Box::new(Expression::from_pairs(s, generator))
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[6usize] {
                            DelimitedBound::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                        if let ParsePairExpression::List(_, ref l) = l[7usize] {
                            l.first().is_some()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            "sort" => {
                Self::Sort(
                    info,
                    if let ParsePairExpression::Sort(_, ref s) = pair.constructor_value {
                        Identifier::from_pairs(s, generator)
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                    },
                )
            }
            "class" => {
                Self::Class(
                    info,
                    if let ParsePairExpression::Sort(_, ref s) = pair.constructor_value {
                        CharacterClass::from_pairs(s, generator)
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                    },
                )
            }
            "paren" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Paren(
                        info,
                        if let ParsePairExpression::List(_, ref l) = l[1usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Box :: new (Expression :: from_pairs (s , generator)) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "expression");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "expression"
                    );
                }
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Annotation<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "annotation");
        let info = generator.generate(&pair);
        if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
            Self(
                info,
                if let ParsePairExpression::List(_, ref l) = l[1usize] {
                    l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { Identifier :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "annotation") ; }) . collect ()
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "annotation"
                    );
                },
            )
        } else {
            unreachable!(
                "expected different parse pair expression in pair to ast conversion of {}",
                "annotation"
            );
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Number<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "number");
        let info = generator.generate(&pair);
        return Self(info, pair.constructor_value.span().as_str().to_string());
    }
}
impl<M: AstInfo> FromPairs<M> for StringChar<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "string-char");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "escaped" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Escaped(
                        info,
                        if let ParsePairExpression::Empty(ref span) = l[1usize] {
                            span.as_str().to_string()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "string-char");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "string-char"
                    );
                }
            }
            "normal" => {
                Self::Normal(
                    info,
                    if let ParsePairExpression::Empty(ref span) = pair.constructor_value {
                        span.as_str().to_string()
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "string-char");
                    },
                )
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for DelimitedBound<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "delimited-bound");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "num-num" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::NumNum(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "delimited-bound");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "delimited-bound");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "delimited-bound"
                    );
                }
            }
            "num-inf" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::NumInf(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            Number::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "delimited-bound");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "delimited-bound"
                    );
                }
            }
            "num" => {
                Self::Num(
                    info,
                    if let ParsePairExpression::Sort(_, ref s) = pair.constructor_value {
                        Number::from_pairs(s, generator)
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "delimited-bound");
                    },
                )
            }
            "star" => Self::Star(info),
            "plus" => Self::Plus(info),
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for CharacterClass<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "character-class");
        let info = generator.generate(&pair);
        if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
            Self(
                info,
                if let ParsePairExpression::List(_, ref l) = l[1usize] {
                    l.first().is_some()
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "character-class"
                    );
                },
                if let ParsePairExpression::List(_, ref l) = l[2usize] {
                    l . iter () . map (| x | if let ParsePairExpression :: Sort (_ , ref s) = x { CharacterClassItem :: from_pairs (s , generator) } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "character-class") ; }) . collect ()
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "character-class"
                    );
                },
            )
        } else {
            unreachable!(
                "expected different parse pair expression in pair to ast conversion of {}",
                "character-class"
            );
        }
    }
}
impl<M: AstInfo> FromPairs<M> for CharacterClassItem<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "character-class-item");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "range" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Range(
                        info,
                        if let ParsePairExpression::Sort(_, ref s) = l[0usize] {
                            EscapeClosingBracket::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "character-class-item");
                        },
                        if let ParsePairExpression::Sort(_, ref s) = l[2usize] {
                            EscapeClosingBracket::from_pairs(s, generator)
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "character-class-item");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "character-class-item"
                    );
                }
            }
            "single-char" => {
                Self::SingleChar(
                    info,
                    if let ParsePairExpression::Sort(_, ref s) = pair.constructor_value {
                        EscapeClosingBracket::from_pairs(s, generator)
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "character-class-item");
                    },
                )
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for EscapeClosingBracket<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "escape-closing-bracket");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "escaped" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Escaped(
                        info,
                        if let ParsePairExpression::Empty(ref span) = l[1usize] {
                            span.as_str().to_string()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "escape-closing-bracket");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "escape-closing-bracket"
                    );
                }
            }
            "unescaped" => {
                Self::Unescaped(
                    info,
                    if let ParsePairExpression::Empty(ref span) = pair.constructor_value {
                        span.as_str().to_string()
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "escape-closing-bracket");
                    },
                )
            }
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Newline<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "newline");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "unix" => Self::Unix(info),
            "windows" => Self::Windows(info),
            a => unreachable!("{}", a),
        }
    }
}
impl<M: AstInfo> FromPairs<M> for Layout<M> {
    fn from_pairs<G: GenerateAstInfo<Result = M>>(pair: &ParsePairSort, generator: &mut G) -> Self {
        assert_eq!(pair.sort, "layout");
        let info = generator.generate(&pair);
        match pair.constructor_name {
            "simple" => {
                Self::Simple(
                    info,
                    if let ParsePairExpression::Empty(ref span) = pair.constructor_value {
                        span.as_str().to_string()
                    } else {
                        unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "layout");
                    },
                )
            }
            "comment" => {
                if let ParsePairExpression::List(_, ref l) = pair.constructor_value {
                    Self::Comment(
                        info,
                        if let ParsePairExpression::List(_, ref l) = l[1usize] {
                            l . iter () . map (| x | if let ParsePairExpression :: Empty (ref span) = x { span . as_str () . to_string () } else { unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "layout") ; }) . collect ()
                        } else {
                            unreachable ! ("expected different parse pair expression in pair to ast conversion of {}" , "layout");
                        },
                    )
                } else {
                    unreachable!(
                        "expected different parse pair expression in pair to ast conversion of {}",
                        "layout"
                    );
                }
            }
            a => unreachable!("{}", a),
        }
    }
}
