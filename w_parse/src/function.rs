use std::mem::swap;
use crate::types::parse_function_type;
use crate::{parse_identifier, parse_name, ErrorChain, Identifier, ParResult, TokenSpan, Weak, Error};
use assert_matches::assert_matches;
use nom::branch::alt;
use nom::combinator::{all_consuming, cond, map, map_opt, opt, verify};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{pair, preceded, terminated};
use nom::{Err, Parser};
use std::rc::Rc;
use w_tokenize::{Kind, Span};

pub fn parse_function(oi: TokenSpan) -> ParResult {
    let (i, _name) = parse_name(oi.clone())?;
    let (i, _generics) = many0(parse_identifier)(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;
    let (_i, _func_head) = parse_function_type(i)?;

    todo!()
}

pub enum Expression<'a> {
    Scoped(CodeBlock<'a>),
    Assignment {
        mutable: Option<Identifier<'a>>,
        target: Vec<Identifier<'a>>,
        value: Box<Expression<'a>>,
    },
    Reassignment {
        target: Vec<Identifier<'a>>,
        value: Box<Expression<'a>>,
    },
    OpReassignment {
        target: Vec<Identifier<'a>>,
        op: MathOp<'a>,
        value: Box<Expression<'a>>,
    },
    BinaryOp {
        lhs: Box<Expression<'a>>,
        op: BinaryOp<'a>,
        rhs: Box<Expression<'a>>,
    },
    Tuple(Vec<Expression<'a>>),
    Array(Vec<Expression<'a>>),
    Accessor {
        accessor: Vec<Identifier<'a>>,
        preceding: Option<Box<Expression<'a>>>
    },
    Call {
        target: Box<Expression<'a>>,
        args: Vec<Expression<'a>>,
    },
    Index {
        target: Box<Expression<'a>>,
        args: Vec<Expression<'a>>,
    },
    Deref {
        span: Span<'a>,
        target: Box<Expression<'a>>,
    },
    Terminated {
        span: Span<'a>,
        value: Box<Expression<'a>>,
    },
}

pub struct CodeBlock<'a>(Vec<Expression<'a>>);

pub struct MathOp<'a> {
    pub span: Span<'a>,
    pub kind: MathOpKind,
}

pub enum MathOpKind {
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
}

pub struct BinaryOp<'a> {
    pub span: Span<'a>,
    pub kind: BinaryOpKind,
}

pub enum BinaryOpKind {
    Math(MathOpKind),
    Comparison(ComparisonOpKind),
    Logic(LogicOpKind),
}

pub enum ComparisonOpKind {
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
}

pub enum LogicOpKind {
    And,
    Or,
}

pub fn parse_expression(i: TokenSpan) -> ParResult<Expression> {
    parse_expression_inner(true)(i)
}

fn parse_expression_inner(binaries: bool) -> impl FnMut(TokenSpan) -> ParResult<Expression> {
    move |i| {
        let (i, derefs) = many0(map(Weak(Kind::Mul), |tk| tk.span))(i)?;
        let (i, expr) = alt((
            map(parse_code_block, Expression::Scoped),
            parse_assignment,
            parse_reassignment,
            parse_op_reassign,
            map(parse_accessor, |idents| Expression::Accessor {
                accessor: idents,
                preceding: None
            }),
            map(parse_tuple(parse_expression), Expression::Tuple),
            map(parse_array(parse_expression), Expression::Tuple),
            map_opt(cond(binaries, parse_binaries), |x| x)
            // TODO,
        ))(i)?;

        let (i, mut expr) = parse_trailing(i, expr)?;

        let access = expr.left_most();
        // Tuple has no particular meaning here, we just use it as it only requires a single vec
        // and empty vecs should not allocate any ram
        let mut tmp = Expression::Tuple(Vec::new());
        swap(access, &mut tmp);

        for deref in derefs {
            tmp = Expression::Deref {
                span: deref,
                target: Box::new(tmp),
            };
        }

        swap(access, &mut tmp);
        let _ = tmp;

        let (i, terminated) = opt(Weak(Kind::Semicolon))(i)?;
        if let Some(terminated) = terminated {
            expr = Expression::Terminated {
                span: terminated.span,
                value: Box::new(expr),
            };
        }

        Ok((i, expr))
    }
}

pub fn parse_trailing<'a>(i: TokenSpan<'a>, mut expr: Expression<'a>) -> ParResult<'a, Expression<'a>> {
    let (i, tail) = opt(alt((
        map(parse_tuple(parse_expression), Expression::Tuple),
        map(parse_array(parse_expression), Expression::Array),
        map(preceded(Weak(Kind::Dot), parse_accessor), |accessor| Expression::Accessor {
            accessor,
            preceding: None,
        }),
    )))(i)?;

    if tail.is_none() {
        return Ok((i, expr));
    }

    let access = expr.right_most();
    // Tuple has no particular meaning here, we just use it as it only requires a single vec
    // and empty vecs should not allocate any ram
    let mut tmp = Expression::Tuple(Vec::new());
    swap(access, &mut tmp);

    match tail.unwrap() {
        Expression::Tuple(args) =>
            tmp = Expression::Call { target: Box::new(tmp), args },
        Expression::Array(args) =>
            tmp = Expression::Index { target: Box::new(tmp), args },
        Expression::Accessor { accessor, .. } =>
            tmp = Expression::Accessor { accessor, preceding: Some(Box::new(tmp)) },
        _ => unreachable!(),
    }
    swap(access, &mut tmp);

    // there can be multiple trailing expressions, we just need to keep track of the last one
    parse_trailing(i, expr)
}

pub fn parse_binaries(i: TokenSpan) -> ParResult<Expression> {
    let mut exp_acc = vec![];
    let mut op_acc = vec![];

    let mut i = i;
    loop {
        let (ni, exop) = opt(pair(parse_expression_inner(false), parse_binary_op))(i)?;
        i = ni;

        if let Some((exp, op)) = exop {
            exp_acc.push(exp);
            op_acc.push(op);
        } else {
            break;
        }
    }

    if exp_acc.is_empty() {
        return Err(Err::Error(ErrorChain::from(Error::new(i, "Expected expressions seperated by binary operations"))));
    }

    let (i, last) = parse_expression_inner(false)(i)?;
    exp_acc.push(last);

    while exp_acc.len() > 1 {
        // unwrap is safe as op_acc len always exp_acc len - 1
        let op_prio = op_acc.iter().map(|op| op.kind.priority()).min().unwrap();
        for i in 0..op_acc.len() {
            if op_acc[i].kind.priority() != op_prio {
                continue;
            }

            let op = op_acc.remove(i);
            let lhs = exp_acc.remove(i);
            let rhs = exp_acc.remove(i);

            exp_acc.insert(i, Expression::BinaryOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });
        }
    }

    assert_eq!(exp_acc.len(), 1);
    assert_eq!(op_acc.len(), 0);

    Ok((i, exp_acc.pop().unwrap()))
}

pub fn parse_code_block(i: TokenSpan) -> ParResult<CodeBlock> {
    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);

    all_consuming(parse_code_block_inner)(block_span).map(|(_, r)| (i, r))
}

pub fn parse_code_block_inner(_i: TokenSpan) -> ParResult<CodeBlock> {
    todo!()
}

pub fn parse_assignment(i: TokenSpan) -> ParResult<Expression> {
    let (i, mutable) = opt(parse_specific("mut"))(i)?;
    let (i, target) = alt((map(parse_name, |name| vec![name]), parse_tuple(parse_name)))(i)?;

    let (i, _) = Weak(Kind::Define).parse(i)?;

    let (i, value) = parse_expression(i)?;

    Ok((i, Expression::Assignment {
        mutable,
        target,
        value: Box::new(value),
    }))
}

pub fn parse_reassignment(i: TokenSpan) -> ParResult<Expression> {
    let (i, target) = alt((map(parse_name, |name| vec![name]), parse_tuple(parse_name)))(i)?;

    let (i, _) = Weak(Kind::Set).parse(i)?;

    let (i, value) = parse_expression(i)?;

    Ok((i, Expression::Reassignment {
        target,
        value: Box::new(value),
    }))
}

pub fn parse_op_reassign(i: TokenSpan) -> ParResult<Expression> {
    let (i, target) = alt((map(parse_name, |name| vec![name]), parse_tuple(parse_name)))(i)?;

    let (i, op_tk) = alt((
        Weak(Kind::AddAssign),
        Weak(Kind::SubAssign),
        Weak(Kind::MulAssign),
        Weak(Kind::DivAssign),
        Weak(Kind::ModAssign),

        Weak(Kind::AndAssign),
        Weak(Kind::OrAssign),
        Weak(Kind::XorAssign),
        Weak(Kind::ShlAssign),
        Weak(Kind::ShrAssign),
    ))(i)?;

    let (i, value) = parse_expression(i)?;

    Ok((
        i,
        Expression::OpReassignment {
            target,
            op: MathOp {
                span: op_tk.span,
                kind: op_tk.kind.try_into().unwrap(),
            },
            value: Box::new(value)
        }
    ))
}

pub fn parse_tuple<'a, F, O: 'a>(parser: F) -> impl FnMut(TokenSpan<'a>) -> ParResult<Vec<O>>
    where
        F: Parser<TokenSpan<'a>, O, ErrorChain<'a>> + Clone,
{
    move |i| {
        let (i, block) = Weak(Kind::Tuple(Rc::from([]))).parse(i)?;
        let block_tokens = assert_matches!(block.kind, Kind::Tuple(tk) => tk);

        let block_span = TokenSpan::new(i.file, block_tokens);

        all_consuming(terminated(
            separated_list0(Weak(Kind::Comma), parser.clone()),
            opt(Weak(Kind::Comma)),
        ))(block_span)
            .map(|(_, o)| (i, o))
    }
}

pub fn parse_array<'a, F, O: 'a>(parser: F) -> impl FnMut(TokenSpan<'a>) -> ParResult<Vec<O>>
where
    F: Parser<TokenSpan<'a>, O, ErrorChain<'a>> + Clone,
{
    move |i| {
        let (i, block) = Weak(Kind::Array(Rc::from([]))).parse(i)?;
        let block_tokens = assert_matches!(block.kind, Kind::Tuple(tk) => tk);

        let block_span = TokenSpan::new(i.file, block_tokens);

        all_consuming(terminated(
            separated_list0(Weak(Kind::Comma), parser.clone()),
            opt(Weak(Kind::Comma)),
        ))(block_span)
        .map(|(_, o)| (i, o))
    }
}

pub fn parse_accessor(i: TokenSpan) -> ParResult<Vec<Identifier>> {
    separated_list1(Weak(Kind::Dot), parse_name)(i)
}

pub fn parse_specific(
    specific: &str,
) -> impl FnMut(TokenSpan) -> ParResult<Identifier> + '_ {
    move |i| verify(parse_identifier, |ident| *ident.0 == specific)(i)
}

pub fn parse_binary_op(i: TokenSpan) -> ParResult<BinaryOp> {
    let (i, tk) = alt((
        Weak(Kind::Add),
        Weak(Kind::Sub),
        Weak(Kind::Mul),
        Weak(Kind::Div),
        Weak(Kind::Mod),

        Weak(Kind::And),
        Weak(Kind::Or),
        Weak(Kind::Xor),
        Weak(Kind::Shl),
        Weak(Kind::Shr),

        Weak(Kind::Eq),
        Weak(Kind::Neq),
        Weak(Kind::Lt),
        Weak(Kind::Gt),
        Weak(Kind::Le),
        Weak(Kind::Ge),
    ))(i)?;

    Ok((i, BinaryOp {
        span: tk.span,
        kind: tk.kind.try_into().unwrap(),
    }))
}

impl<'a> TryFrom<Kind<'a>> for MathOpKind {
    type Error = ();

    fn try_from(value: Kind<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Kind::Add | Kind::AddAssign => MathOpKind::Add,
            Kind::Sub | Kind::SubAssign => MathOpKind::Sub,
            Kind::Mul | Kind::MulAssign => MathOpKind::Mul,
            Kind::Div | Kind::DivAssign => MathOpKind::Div,
            Kind::Mod | Kind::ModAssign => MathOpKind::Mod,

            Kind::And | Kind::AndAssign => MathOpKind::And,
            Kind::Or | Kind::OrAssign => MathOpKind::Or,
            Kind::Xor | Kind::XorAssign => MathOpKind::Xor,
            Kind::Shl | Kind::ShlAssign => MathOpKind::Shl,
            Kind::Shr | Kind::ShrAssign => MathOpKind::Shr,

            _ => return Err(()),
        })
    }
}

impl<'a> TryFrom<Kind<'a>> for BinaryOpKind {
    type Error = ();

    fn try_from(value: Kind<'a>) -> Result<Self, Self::Error> {
        if let Ok(op) = MathOpKind::try_from(value.clone()) {
            return Ok(BinaryOpKind::Math(op));
        }
        Ok(match value {
            Kind::Eq => BinaryOpKind::Comparison(ComparisonOpKind::Eq),
            Kind::Neq => BinaryOpKind::Comparison(ComparisonOpKind::Neq),
            Kind::Lt => BinaryOpKind::Comparison(ComparisonOpKind::Lt),
            Kind::Le => BinaryOpKind::Comparison(ComparisonOpKind::Le),
            Kind::Gt => BinaryOpKind::Comparison(ComparisonOpKind::Gt),
            Kind::Ge => BinaryOpKind::Comparison(ComparisonOpKind::Ge),
            Kind::AndL => BinaryOpKind::Logic(LogicOpKind::And),
            Kind::OrL => BinaryOpKind::Logic(LogicOpKind::Or),
            _ => return Err(()),
        })
    }
}

impl BinaryOpKind {
    pub fn priority(&self) -> u32 {
        match self {
            BinaryOpKind::Math(op) => match op {
                MathOpKind::Add => 1,
                MathOpKind::Sub => 1,
                MathOpKind::Mul => 0,
                MathOpKind::Div => 0,
                MathOpKind::Mod => 0,

                MathOpKind::And => 2,
                MathOpKind::Or => 2,
                MathOpKind::Xor => 2,
                MathOpKind::Shl => 3,
                MathOpKind::Shr => 3,
            }
            BinaryOpKind::Comparison(cmp) => match cmp {
                ComparisonOpKind::Eq => 4,
                ComparisonOpKind::Neq => 4,
                ComparisonOpKind::Lt => 4,
                ComparisonOpKind::Gt => 4,
                ComparisonOpKind::Le => 4,
                ComparisonOpKind::Ge => 4,
            }
            BinaryOpKind::Logic(logic) => match logic {
                LogicOpKind::And => 6,
                LogicOpKind::Or => 5,
            },
        }
    }
}

impl<'a> Expression<'a> {
    pub fn left_most(&mut self) -> &mut Self {
        match self {
            Expression::BinaryOp { lhs, .. } => lhs.left_most(),
            _ => self,
        }
    }

    pub fn right_most(&mut self) -> &mut Self {
        match self {
            Expression::BinaryOp { rhs, .. } => rhs.right_most(),
            Expression::Assignment { value, .. } => value.right_most(),
            Expression::Reassignment { value, .. } => value.right_most(),
            Expression::OpReassignment { value, .. } => value.right_most(),
            _ => self,
        }
    }
}
