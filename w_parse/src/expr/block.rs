use crate::expr::{parse_expression, Expr};
use crate::{tag, ParResult, TokenSpan, Weak};
use assert_matches::assert_matches;
use either::Either;
use nom::branch::alt;
use nom::combinator::{cond, eof, map, opt};
use nom::sequence::pair;
use nom::{Offset, Parser, Slice};
use std::rc::Rc;
use w_tokenize::{Kind, Span, Token};

#[derive(Debug, Clone)]
pub struct Statement<'a> {
    pub expr: Expr<'a>,
    pub sim: Option<Token<'a>>,
}

#[derive(Debug, Clone)]
pub struct ExprBlock<'a> {
    pub span: Span<'a>,
    pub kind: BlockKind<'a>,
}

#[derive(Debug, Clone)]
pub enum BlockKind<'a> {
    Many {
        stmts: Vec<Statement<'a>>,
        returning: Option<Box<Expr<'a>>>,
    },
    Inline(Box<Expr<'a>>),
}

pub fn parse_block(i: TokenSpan) -> ParResult<ExprBlock> {
    alt((parse_block_many, parse_block_inline))(i)
}

fn parse_block_many(i: TokenSpan) -> ParResult<ExprBlock> {
    let (oi, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let span = block.span;
    let mut i = assert_matches!(block.kind, Kind::Block(vals) => TokenSpan::new(oi.file, vals));

    let mut acc = vec![];
    let last;

    loop {
        let (ni, expr) = parse_expression(i)?;
        let (ni, sim) = alt((
            map(
                pair(
                    alt((
                        map(Weak(Kind::Semicolon), Some),
                        cond(expr.needs_termination(), Weak(Kind::Semicolon)),
                    )),
                    opt(eof),
                ),
                Either::Left,
            ),
            map(eof, Either::Right),
        ))(ni)?;

        match sim {
            Either::Left((sim, Some(_))) => {
                acc.push(Statement { expr, sim });
                last = None;
                break;
            }
            Either::Left((sim, None)) => acc.push(Statement { expr, sim }),
            // EOF
            Either::Right(_) => {
                last = Some(expr);
                break;
            }
        }

        i = ni;
    }

    Ok((
        oi,
        ExprBlock {
            span,
            kind: BlockKind::Many {
                stmts: acc,
                returning: last.map(Box::new),
            },
        },
    ))
}

fn parse_block_inline(oi: TokenSpan) -> ParResult<ExprBlock> {
    let (i, _arrow) = tag!(Kind::InlineBlk)(oi.clone())?;
    let (i, expr) = parse_expression(i)?;

    let offset = oi.offset(&i);
    let span = TokenSpan::slice(&oi, ..offset);

    Ok((
        i,
        ExprBlock {
            span: (&span).into(),
            kind: BlockKind::Inline(Box::new(expr)),
        },
    ))
}
