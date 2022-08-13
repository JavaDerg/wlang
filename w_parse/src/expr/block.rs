use std::rc::Rc;
use assert_matches::assert_matches;
use either::Either;
use nom::branch::alt;
use nom::combinator::{eof, map};
use nom::Parser;
use w_tokenize::{Kind, Span, Token};
use crate::{ParResult, TokenSpan, Weak};
use crate::expr::{Expr, parse_expression};


pub struct Statement<'a> {
    pub expr: Expr<'a>,
    pub sim: Token<'a>,
}

pub struct ExprBlock<'a> {
    pub span: Span<'a>,
    pub stmts: Vec<Statement<'a>>,
    pub returning: Option<Box<Expr<'a>>>,
}

pub fn parse_block(i: TokenSpan) -> ParResult<ExprBlock> {
    let (oi, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let span = block.span;
    let mut i = assert_matches!(block.kind, Kind::Block(vals) => TokenSpan::new(oi.file, vals));

    let mut acc = vec![];
    let mut last = None;
    
    loop {
        let (ni, expr) = parse_expression(i)?;
        let (ni, sim) = alt((
            map(Weak(Kind::Semicolon), Either::Left),
            map(eof, Either::Right),
        ))(ni)?;

        match sim {
            Either::Left(sim) => acc.push(Statement { expr, sim }),
            // EOF
            Either::Right(_) => {
                last = Some(expr);
                break;
            }
        }
        
        i = ni;
    }

    Ok((oi, ExprBlock {
        span,
        stmts: acc,
        returning: last.map(Box::new),
    }))
}
