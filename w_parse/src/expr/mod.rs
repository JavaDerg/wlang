use crate::expr::call::{parse_call_wrapper, ExprCall};
use crate::expr::field::{parse_field_wrapper, ExprField};
use crate::expr::index::{parse_index_wrapper, ExprIndex};
use crate::expr::many::{ExprArray, ExprTuple, parse_array, parse_tuple};
use crate::expr::path::{parse_path, Path};
use crate::expr::unary::{parse_unary, ExprUnary, UnOp};
use crate::{parse_name, ErrorChain, Ident, ParResult, TokenSpan, Weak, parse_type};
use assert_matches::assert_matches;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{map, opt, verify};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use nom::{Err, InputTake};
use w_tokenize::{Kind, Number, Span, Token};
use crate::expr::block::ExprBlock;
use crate::expr::ctor::{ExprCtor, parse_ctor};

mod call;
mod field;
mod index;
mod many;
mod path;
mod unary;
mod ctor;
mod block;

#[macro_export]
macro_rules! tag {
    ($pt:pat, $spt:pat => $res:expr) => {
        crate::expr::tag(
            |tk| match &tk.kind {
                $pt => true,
                _ => false,
            },
            |tk| match tk {
                $spt => $res,
                _ => unreachable!(),
            },
        )
    };
}

pub enum Expr<'a> {
    Tuple(ExprTuple<'a>),
    Array(ExprArray<'a>),

    Path(Path<'a>),
    Ctor(ExprCtor<'a>),

    Block(ExprBlock<'a>),

    Number(Number<'a>),
    String(Span<'a>, String),
    Ident(Ident<'a>),

    Unary(ExprUnary<'a>),
    Field(ExprField<'a>),
    Call(ExprCall<'a>),
    Index(ExprIndex<'a>),
}


pub fn parse_expression(i: TokenSpan) -> ParResult<Expr> {
    parse_expr_pre_pass(i)
}

fn parse_expr_pre_pass(i: TokenSpan) -> ParResult<Expr> {
    let (i, unaries) = many0(parse_unary)(i)?;

    let (i, mut expr) = parse_expr_mid_pass(i)?;

    for op in unaries.into_iter().rev() {
        expr = Expr::Unary(ExprUnary {
            op,
            expr: Box::new(expr),
        });
    }
    Ok((i, expr))
}

fn parse_expr_mid_pass(i: TokenSpan) -> ParResult<Expr> {
    let (mut i, mut expr) = parse_expr_post_pass(i)?;

    loop {
        let (ni, (nexpr, cont)) = parse_succeeding(i, expr)?;
        i = ni;
        expr = nexpr;

        if !cont {
            break;
        }
    }

    Ok((i, expr))
}

fn parse_succeeding<'a>(i: TokenSpan<'a>, expr: Expr<'a>) -> ParResult<'a, (Expr<'a>, bool)> {
    let (i, succ) = opt(alt((
        parse_field_wrapper,
        parse_call_wrapper,
        parse_index_wrapper,
    )))(i)?;

    let ret = if let Some(mut succ) = succ {
        (succ(expr), true)
    } else {
        (expr, false)
    };

    Ok((i, ret))
}

fn parse_expr_post_pass(i: TokenSpan) -> ParResult<Expr> {
    alt((
        map(parse_ctor, Expr::Ctor),
        map(verify(parse_path, |pt| pt.path.len() >= 2), Expr::Path),
        map(parse_name, Expr::Ident),
        map(parse_tuple, Expr::Tuple),
        map(parse_array, Expr::Array),
        tag!(Kind::String(_), Token { kind: Kind::String(num), span } => Expr::String(span, num)),
        tag!(Kind::Number(_), Token { kind: Kind::Number(num), .. } => Expr::Number(num)),
    ))(i)
}

fn tag<'a, O: 'a>(
    parser: fn(&Token<'a>) -> bool,
    map: fn(Token<'a>) -> O,
) -> impl FnMut(TokenSpan<'a>) -> ParResult<'a, O> {
    move |i| {
        if i.is_empty() {
            return Err(Err::Error(ErrorChain::from_error_kind(i, ErrorKind::Eof)));
        }

        let (i, took) = TokenSpan::take_split(&i, 1);
        if !parser(&took[0]) {
            return Err(Err::Error(ErrorChain::from_error_kind(
                took.clone(),
                ErrorKind::Tag,
            )));
        }

        Ok((i, map(took[0].clone())))
    }
}

impl<'a> Expr<'a> {
    pub fn needs_termination(&self) -> bool {
        match self {
            Expr::Tuple(_)
            | Expr::Array(_)
            | Expr::Path(_)
            | Expr::Ctor(_)
            | Expr::Number(_)
            | Expr::String(_, _)
            | Expr::Ident(_)
            | Expr::Unary(_)
            | Expr::Field(_)
            | Expr::Call(_)
            | Expr::Index(_) => true,
            Expr::Block(_) => false,
        }
    }
}
