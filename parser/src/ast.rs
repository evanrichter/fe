use std::convert::TryFrom;

use serde::{
    Deserialize,
    Serialize,
};

use crate::tokenizer::types::{
    Offset,
    Position,
    TokenInfo,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SourceSpan {
    pub start_pos: Position,
    pub start_off: Offset,
    pub end_pos: Position,
    pub end_off: Offset,
}

impl From<&TokenInfo<'_>> for SourceSpan {
    fn from(token_info: &TokenInfo) -> Self {
        Self {
            start_pos: token_info.start_pos,
            start_off: token_info.start_off,
            end_pos: token_info.end_pos,
            end_off: token_info.end_off,
        }
    }
}

impl From<(&TokenInfo<'_>, &TokenInfo<'_>)> for SourceSpan {
    fn from(tokens: (&TokenInfo, &TokenInfo)) -> Self {
        let (start, end) = tokens;

        Self {
            start_pos: start.start_pos,
            start_off: start.start_off,
            end_pos: end.end_pos,
            end_off: end.end_off,
        }
    }
}

impl From<(&SourceSpan, &SourceSpan)> for SourceSpan {
    fn from(spans: (&SourceSpan, &SourceSpan)) -> Self {
        let (start, end) = spans;

        Self {
            start_pos: start.start_pos,
            start_off: start.start_off,
            end_pos: end.end_pos,
            end_off: end.end_off,
        }
    }
}

impl From<(&TokenInfo<'_>, &SourceSpan)> for SourceSpan {
    fn from(input: (&TokenInfo, &SourceSpan)) -> Self {
        let (start, end) = input;

        Self {
            start_pos: start.start_pos,
            start_off: start.start_off,
            end_pos: end.end_pos,
            end_off: end.end_off,
        }
    }
}

impl From<(&SourceSpan, &TokenInfo<'_>)> for SourceSpan {
    fn from(input: (&SourceSpan, &TokenInfo)) -> Self {
        let (start, end) = input;

        Self {
            start_pos: start.start_pos,
            start_off: start.start_off,
            end_pos: end.end_pos,
            end_off: end.end_off,
        }
    }
}

pub trait GetSourceSpan {
    fn get_source_span(&self) -> &SourceSpan;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Module<'a> {
    #[serde(borrow)]
    pub body: Vec<ModuleStmt<'a>>,
    pub source_span: SourceSpan,
}

impl<'a> GetSourceSpan for Module<'a> {
    fn get_source_span(&self) -> &SourceSpan {
        &self.source_span
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ModuleStmt<'a> {
    EventDef {
        name: &'a str,
        fields: Vec<EventField<'a>>,
        source_span: SourceSpan,
    },
}

impl<'a> GetSourceSpan for ModuleStmt<'a> {
    fn get_source_span(&self) -> &SourceSpan {
        match self {
            Self::EventDef { source_span, .. } => source_span,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TypeDesc<'a> {
    pub base: &'a str,
    pub dimensions: Vec<u32>,
    pub annotations: Vec<&'a str>,
    pub source_span: SourceSpan,
}

impl<'a> From<&'a TokenInfo<'a>> for TypeDesc<'a> {
    fn from(token_info: &'a TokenInfo<'a>) -> Self {
        Self {
            base: token_info.string,
            dimensions: vec![],
            annotations: vec![],
            source_span: token_info.into(),
        }
    }
}

impl<'a> GetSourceSpan for TypeDesc<'a> {
    fn get_source_span(&self) -> &SourceSpan {
        &self.source_span
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventField<'a> {
    pub name: &'a str,
    pub typ: TypeDesc<'a>,
    pub source_span: SourceSpan,
}

impl<'a> GetSourceSpan for EventField<'a> {
    fn get_source_span(&self) -> &SourceSpan {
        &self.source_span
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
}

impl TryFrom<&str> for Operator {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mult),
            "/" => Ok(Self::Div),
            "%" => Ok(Self::Mod),
            "**" => Ok(Self::Pow),
            "<<" => Ok(Self::LShift),
            ">>" => Ok(Self::RShift),
            "|" => Ok(Self::BitOr),
            "^" => Ok(Self::BitXor),
            "&" => Ok(Self::BitAnd),
            _ => Err("unrecognized binary operator string"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum UnaryOp {
    Invert,
    Not,
    UAdd,
    USub,
}

impl TryFrom<&str> for UnaryOp {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "~" => Ok(Self::Invert),
            "not" => Ok(Self::Not),
            "+" => Ok(Self::UAdd),
            "-" => Ok(Self::USub),
            _ => Err("unrecognized unary operator string"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ConstExpr<'a> {
    BinOp {
        left: Box<ConstExpr<'a>>,
        op: Operator,
        right: Box<ConstExpr<'a>>,
        source_span: SourceSpan,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<ConstExpr<'a>>,
        source_span: SourceSpan,
    },
    Name {
        name: &'a str,
        source_span: SourceSpan,
    },
    Num {
        num: &'a str,
        source_span: SourceSpan,
    },
}

impl<'a> GetSourceSpan for ConstExpr<'a> {
    fn get_source_span(&self) -> &SourceSpan {
        match self {
            Self::BinOp { source_span, .. } => source_span,
            Self::UnaryOp { source_span, .. } => source_span,
            Self::Name { source_span, .. } => source_span,
            Self::Num { source_span, .. } => source_span,
        }
    }
}
