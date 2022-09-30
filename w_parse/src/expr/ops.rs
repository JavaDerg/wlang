use crate::expr::{parse_expr_pre_pass, Expr};
use crate::{ErrorChain, ParResult, TokenSpan};
use assert_matches::assert_matches;
use nom::combinator::opt;
use nom::error::{ErrorKind, ParseError};
use nom::Err;
use nom::InputTake;
use w_tokenize::{Kind, Span};

#[derive(Debug, Clone)]
pub struct ExprBinary {
    pub op: BiOp,
    pub op_span: Span,

    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Copy, Clone)]
pub enum BiOp {
    // Math operands
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Bitwise operands
    And,
    Or,
    Xor,
    Shl,
    Shr,

    // Comparison operands
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,

    // Logical operands
    AndL,
    OrL,
}

pub fn parse_binary_ops(mut i: TokenSpan) -> ParResult<ExprBinary> {
    let mut exprs = vec![];
    let mut ops = vec![];

    loop {
        let (ni, expr) = parse_expr_pre_pass(i, true)?;
        let (ni, op) = opt(parse_bi_op)(ni)?;
        i = ni;

        exprs.push(expr);

        if let Some(op) = op {
            ops.push(op);
        } else {
            break;
        }
    }

    debug_assert_eq!(exprs.len() - 1, ops.len());

    if exprs.len() == 1 {
        return Err(Err::Error(ErrorChain::from_error_kind(i, ErrorKind::Count)));
    }

    eval_order(&mut exprs, &mut ops);

    Ok((
        i,
        assert_matches!(exprs.pop().unwrap(), Expr::Binary(bin) => bin),
    ))
}

fn eval_order(exprs: &mut Vec<Expr>, ops: &mut Vec<(Span, BiOp)>) {
    while !ops.is_empty() {
        let lowest = ops.iter().map(|op| op.1.priority()).min().unwrap();

        while let Some(idx) = ops
            .iter()
            .enumerate()
            .find_map(|(id, op)| (op.1.priority() == lowest).then_some(id))
        {
            let (op_span, op) = ops.remove(idx);
            let left = exprs.remove(idx);
            let right = exprs.remove(idx);

            exprs.insert(
                idx,
                Expr::Binary(ExprBinary {
                    op,
                    op_span,
                    left: Box::new(left),
                    right: Box::new(right),
                }),
            );
        }
    }

    debug_assert_eq!(exprs.len(), 1);
}

pub fn parse_bi_op(i: TokenSpan) -> ParResult<(Span, BiOp)> {
    if i.is_empty() {
        return Err(Err::Error(ErrorChain::from_error_kind(i, ErrorKind::Eof)));
    }

    let (i, took) = TokenSpan::take_split(&i, 1);
    let kind = match took[0].kind {
        Kind::Add => BiOp::Add,
        Kind::Sub => BiOp::Sub,
        Kind::Mul => BiOp::Mul,
        Kind::Div => BiOp::Div,
        Kind::Mod => BiOp::Mod,
        Kind::And => BiOp::And,
        Kind::Or => BiOp::Or,
        Kind::Xor => BiOp::Xor,
        Kind::Shl => BiOp::Shl,
        Kind::Shr => BiOp::Shr,
        Kind::Eq => BiOp::Eq,
        Kind::Neq => BiOp::Neq,
        Kind::Lt => BiOp::Lt,
        Kind::Le => BiOp::Le,
        Kind::Gt => BiOp::Gt,
        Kind::Ge => BiOp::Ge,
        Kind::AndL => BiOp::AndL,
        Kind::OrL => BiOp::OrL,
        _ => {
            return Err(Err::Error(ErrorChain::from_error_kind(
                took.clone(),
                ErrorKind::Tag,
            )))
        }
    };

    Ok((i, (took[0].span.clone(), kind)))
}

impl BiOp {
    pub fn priority(&self) -> u32 {
        match self {
            BiOp::Add => 1,
            BiOp::Sub => 1,
            BiOp::Mul => 0,
            BiOp::Div => 0,
            BiOp::Mod => 0,

            BiOp::And => 2,
            BiOp::Or => 2,
            BiOp::Xor => 2,
            BiOp::Shl => 3,
            BiOp::Shr => 3,

            BiOp::Eq => 4,
            BiOp::Neq => 4,
            BiOp::Lt => 4,
            BiOp::Gt => 4,
            BiOp::Le => 4,
            BiOp::Ge => 4,

            BiOp::AndL => 6,
            BiOp::OrL => 5,
        }
    }
}
