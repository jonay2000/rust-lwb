#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
// |==========================================================|
// |      WARNING: THIS FILE IS AUTOMATICALLY GENERATED.      |
// |      CHANGES TO IT WILL BE DELETED WHEN REGENERATED.     |
// | IN GENERAL, THIS FILE SHOULD NOT BE MODIFIED IN ANY WAY. |
// |==========================================================|
// Generated at 17/04/2022 23:31:58 +02:00 - 17/04/2022 21:31:58 UTC
use super::prelude::*;

impl<M: AstInfo> AstNode<M> for Identifier<M> {
    fn ast_info(&self) -> &M {
        let meta = &self.0;
        {
            meta
        }

    }

    fn constructor(&self) -> &'static str {
        let meta = &self.0;
        {
            "identifier"
        }

    }

    fn sort(&self) -> &'static str {
        "identifier"
    }
}

impl<M: AstInfo> AstNode<M> for Int<M> {
    fn ast_info(&self) -> &M {
        let meta = &self.0;
        {
            meta
        }

    }

    fn constructor(&self) -> &'static str {
        let meta = &self.0;
        {
            "int"
        }

    }

    fn sort(&self) -> &'static str {
        "int"
    }
}

impl<M: AstInfo> AstNode<M> for Bool<M> {
    fn ast_info(&self) -> &M {
        match self {
        Self::True(meta, ..) => {
            meta
        }

        Self::False(meta, ..) => {
            meta
        }

        }
    }

    fn constructor(&self) -> &'static str {
        match self {
        Self::True(meta, ..) => {
            "true"
        }

        Self::False(meta, ..) => {
            "false"
        }

        }
    }

    fn sort(&self) -> &'static str {
        "bool"
    }
}

impl<M: AstInfo> AstNode<M> for Expression<M> {
    fn ast_info(&self) -> &M {
        match self {
        Self::Add(meta, ..) => {
            meta
        }

        Self::Sub(meta, ..) => {
            meta
        }

        Self::Eq(meta, ..) => {
            meta
        }

        Self::Index(meta, ..) => {
            meta
        }

        Self::List(meta, ..) => {
            meta
        }

        Self::Bool(meta, ..) => {
            meta
        }

        Self::Int(meta, ..) => {
            meta
        }

        Self::Identifier(meta, ..) => {
            meta
        }

        Self::Paren(meta, ..) => {
            meta
        }

        }
    }

    fn constructor(&self) -> &'static str {
        match self {
        Self::Add(meta, ..) => {
            "add"
        }

        Self::Sub(meta, ..) => {
            "sub"
        }

        Self::Eq(meta, ..) => {
            "eq"
        }

        Self::Index(meta, ..) => {
            "index"
        }

        Self::List(meta, ..) => {
            "list"
        }

        Self::Bool(meta, ..) => {
            "bool"
        }

        Self::Int(meta, ..) => {
            "int"
        }

        Self::Identifier(meta, ..) => {
            "identifier"
        }

        Self::Paren(meta, ..) => {
            "paren"
        }

        }
    }

    fn sort(&self) -> &'static str {
        "expression"
    }
}

impl<M: AstInfo> AstNode<M> for Statement<M> {
    fn ast_info(&self) -> &M {
        match self {
        Self::If(meta, ..) => {
            meta
        }

        Self::Expression(meta, ..) => {
            meta
        }

        Self::Assignment(meta, ..) => {
            meta
        }

        }
    }

    fn constructor(&self) -> &'static str {
        match self {
        Self::If(meta, ..) => {
            "if"
        }

        Self::Expression(meta, ..) => {
            "expression"
        }

        Self::Assignment(meta, ..) => {
            "assignment"
        }

        }
    }

    fn sort(&self) -> &'static str {
        "statement"
    }
}

impl<M: AstInfo> AstNode<M> for Program<M> {
    fn ast_info(&self) -> &M {
        let meta = &self.0;
        {
            meta
        }

    }

    fn constructor(&self) -> &'static str {
        let meta = &self.0;
        {
            "program"
        }

    }

    fn sort(&self) -> &'static str {
        "program"
    }
}

impl<M: AstInfo> AstNode<M> for Layout<M> {
    fn ast_info(&self) -> &M {
        let meta = &self.0;
        {
            meta
        }

    }

    fn constructor(&self) -> &'static str {
        let meta = &self.0;
        {
            "layout"
        }

    }

    fn sort(&self) -> &'static str {
        "layout"
    }
}