use crate::expr::call::{parse_call_wrapper, ExprCall};
use crate::expr::field::{parse_field_wrapper, ExprField};
use crate::expr::index::{parse_index_wrapper, ExprIndex};
use crate::expr::many::{parse_array, parse_tuple, ExprArray, ExprTuple};
use crate::expr::path::{parse_path, Path};
use crate::expr::unary::{parse_unary, ExprUnary};
use crate::{parse_name, ErrorChain, Ident, ParResult, TokenSpan};

use nom::branch::alt;

use crate::expr::block::ExprBlock;
use crate::expr::branch::{parse_branch, ExprBranch};
use crate::expr::ctor::{parse_ctor, ExprCtor};
use crate::expr::loops::{parse_while, ExprWhile};
use crate::expr::ops::{parse_binary_ops, ExprBinary};
use nom::combinator::{cond, map, map_opt, opt, verify};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use nom::{Err, InputTake};
use w_tokenize::{Number, Span, Token};

pub use many::parse_many0;

pub mod block;
pub mod branch;
pub mod call;
pub mod ctor;
pub mod field;
pub mod index;
pub mod loops;
pub mod many;
pub mod ops;
pub mod path;
pub mod unary;

#[macro_export]
macro_rules! tag {
    ($pt:pat) => {{
        use w_tokenize::Kind;
        crate::expr::tag(
            |tk| match &tk.kind {
                $pt => true,
                _ => false,
            },
            |tk| tk.span,
        )
    }};
    ($pt:pat, $spt:pat => $res:expr) => {{
        use w_tokenize::{Kind, Token};
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
    }};
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Tuple(ExprTuple<'a>),
    Array(ExprArray<'a>),

    Path(Path<'a>),
    Ctor(ExprCtor<'a>),

    Block(ExprBlock<'a>),
    Binary(ExprBinary<'a>),

    Branch(ExprBranch<'a>),
    While(ExprWhile<'a>),

    Number(Number<'a>),
    String(Span<'a>, String),
    Ident(Ident<'a>),

    Unary(ExprUnary<'a>),
    Field(ExprField<'a>),
    Call(ExprCall<'a>),
    Index(ExprIndex<'a>),
}

pub fn parse_expression(i: TokenSpan) -> ParResult<Expr> {
    parse_expr_pre_pass(i, false)
}

pub fn parse_expr_pre_pass(i: TokenSpan, deep: bool) -> ParResult<Expr> {
    let (i, unaries) = many0(parse_unary)(i)?;

    let (i, mut expr) = parse_expr_mid_pass(i, deep)?;

    for op in unaries.into_iter().rev() {
        expr = Expr::Unary(ExprUnary {
            op,
            expr: Box::new(expr),
        });
    }
    Ok((i, expr))
}

fn parse_expr_mid_pass(i: TokenSpan, deep: bool) -> ParResult<Expr> {
    let (mut i, mut expr) = parse_expr_post_pass(i, deep)?;

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

    let ret = if let Some(succ) = succ {
        (succ(expr), true)
    } else {
        (expr, false)
    };

    Ok((i, ret))
}

fn parse_expr_post_pass(i: TokenSpan, deep: bool) -> ParResult<Expr> {
    alt((
        map_opt(cond(!deep, map(parse_binary_ops, Expr::Binary)), |x| x),
        map(parse_ctor, Expr::Ctor),
        map(verify(parse_path, |pt| pt.path.len() >= 2), Expr::Path),
        map(parse_name, Expr::Ident),
        map(parse_tuple, Expr::Tuple),
        map(parse_array, Expr::Array),
        map(parse_branch, Expr::Branch),
        map(parse_while, Expr::While),
        tag!(Kind::String(_), Token { kind: Kind::String(num), span } => Expr::String(span, num)),
        tag!(Kind::Number(_), Token { kind: Kind::Number(num), .. } => Expr::Number(num)),
    ))(i)
}

pub fn tag<'a, O: 'a>(
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
            | Expr::Index(_)
            | Expr::Binary(_) => true,
            Expr::Block(_) | Expr::Branch(_) => false,
            Expr::While(ExprWhile { body, .. }) => body.needs_termination(),
        }
    }
}
