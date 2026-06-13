//! Recursive-descent parser for Mendo.

use crate::lexer::{Token, TokenBuf, TokenKind};

const MAX_STMTS: usize = 24;
const MAX_EXPRS: usize = 32;

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy)]
pub enum Expr<'a> {
    Int(i64),
    Var(&'a str),
    Bin(BinOp, u8, u8),
}

#[derive(Debug, Clone, Copy)]
pub enum Stmt<'a> {
    PrintExpr(u8),
    PrintStr(&'a str),
    Let(&'a str, u8),
    Assign(&'a str, u8),
    If {
        cond: u8,
        then_start: u16,
        then_len: u16,
        else_start: u16,
        else_len: u16,
    },
    While {
        cond: u8,
        body_start: u16,
        body_len: u16,
    },
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    TooManyNodes,
}

pub struct Program<'a> {
    pub stmts: [Stmt<'a>; MAX_STMTS],
    pub stmt_count: usize,
    pub exprs: [Expr<'a>; MAX_EXPRS],
    pub expr_count: usize,
}

impl<'a> Program<'a> {
    pub const fn new() -> Self {
        Self {
            stmts: [Stmt::PrintExpr(0); MAX_STMTS],
            stmt_count: 0,
            exprs: [Expr::Int(0); MAX_EXPRS],
            expr_count: 0,
        }
    }
}

struct Parser<'a, 'tok> {
    tokens: &'tok TokenBuf<'a>,
    pos: usize,
    program: &'tok mut Program<'a>,
}

impl<'a, 'tok> Parser<'a, 'tok> {
    fn current(&self) -> Token<'a> {
        self.tokens.tokens[self.pos]
    }

    fn bump(&mut self) {
        if self.pos + 1 < self.tokens.len {
            self.pos += 1;
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.current().kind == kind {
            self.bump();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken)
        }
    }

    fn push_expr(&mut self, expr: Expr<'a>) -> Result<u8, ParseError> {
        if self.program.expr_count >= MAX_EXPRS {
            return Err(ParseError::TooManyNodes);
        }
        let id = self.program.expr_count as u8;
        self.program.exprs[id as usize] = expr;
        self.program.expr_count += 1;
        Ok(id)
    }

    fn push_stmt(&mut self, stmt: Stmt<'a>) -> Result<(), ParseError> {
        if self.program.stmt_count >= MAX_STMTS {
            return Err(ParseError::TooManyNodes);
        }
        self.program.stmts[self.program.stmt_count] = stmt;
        self.program.stmt_count += 1;
        Ok(())
    }

    fn parse_cmp(&mut self) -> Result<u8, ParseError> {
        let mut left = self.parse_add()?;
        loop {
            let op = match self.current().kind {
                TokenKind::Eq => BinOp::Eq,
                TokenKind::Ne => BinOp::Ne,
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Ge => BinOp::Ge,
                _ => break,
            };
            self.bump();
            let right = self.parse_add()?;
            left = self.push_expr(Expr::Bin(op, left, right))?;
        }
        Ok(left)
    }

    fn parse_add(&mut self) -> Result<u8, ParseError> {
        let mut left = self.parse_mul()?;
        loop {
            let op = match self.current().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.bump();
            let right = self.parse_mul()?;
            left = self.push_expr(Expr::Bin(op, left, right))?;
        }
        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<u8, ParseError> {
        let mut left = self.parse_primary()?;
        loop {
            let op = match self.current().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => break,
            };
            self.bump();
            let right = self.parse_primary()?;
            left = self.push_expr(Expr::Bin(op, left, right))?;
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<u8, ParseError> {
        match self.current().kind {
            TokenKind::Int => {
                let value = self.current().value;
                self.bump();
                self.push_expr(Expr::Int(value))
            }
            TokenKind::Ident => {
                let name = self.current().text;
                self.bump();
                self.push_expr(Expr::Var(name))
            }
            TokenKind::LParen => {
                self.bump();
                let expr = self.parse_cmp()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_block(&mut self) -> Result<(u16, u16), ParseError> {
        self.expect(TokenKind::LBrace)?;
        let start = self.program.stmt_count as u16;
        while self.current().kind != TokenKind::RBrace && self.current().kind != TokenKind::Eof {
            self.parse_stmt()?;
        }
        self.expect(TokenKind::RBrace)?;
        Ok((start, (self.program.stmt_count as u16).saturating_sub(start)))
    }

    fn parse_stmt(&mut self) -> Result<(), ParseError> {
        match self.current().kind {
            TokenKind::Print => {
                self.bump();
                if self.current().kind == TokenKind::String {
                    let text = self.current().text;
                    self.bump();
                    self.push_stmt(Stmt::PrintStr(text))?;
                } else {
                    let expr = self.parse_cmp()?;
                    self.push_stmt(Stmt::PrintExpr(expr))?;
                }
                self.expect(TokenKind::Semicolon)?;
            }
            TokenKind::Let => {
                self.bump();
                let name = self.current().text;
                self.expect(TokenKind::Ident)?;
                self.expect(TokenKind::Assign)?;
                let expr = self.parse_cmp()?;
                self.push_stmt(Stmt::Let(name, expr))?;
                self.expect(TokenKind::Semicolon)?;
            }
            TokenKind::If => {
                self.bump();
                let cond = self.parse_cmp()?;
                let (then_start, then_len) = self.parse_block()?;
                let (else_start, else_len) = if self.current().kind == TokenKind::Else {
                    self.bump();
                    self.parse_block()?
                } else {
                    (self.program.stmt_count as u16, 0)
                };
                self.push_stmt(Stmt::If {
                    cond,
                    then_start,
                    then_len,
                    else_start,
                    else_len,
                })?;
            }
            TokenKind::While => {
                self.bump();
                let cond = self.parse_cmp()?;
                let (body_start, body_len) = self.parse_block()?;
                self.push_stmt(Stmt::While {
                    cond,
                    body_start,
                    body_len,
                })?;
            }
            TokenKind::Ident => {
                let name = self.current().text;
                self.bump();
                self.expect(TokenKind::Assign)?;
                let expr = self.parse_cmp()?;
                self.push_stmt(Stmt::Assign(name, expr))?;
                self.expect(TokenKind::Semicolon)?;
            }
            _ => return Err(ParseError::UnexpectedToken),
        }
        Ok(())
    }

    fn parse_program(&mut self) -> Result<(), ParseError> {
        while self.current().kind != TokenKind::Eof {
            self.parse_stmt()?;
        }
        Ok(())
    }
}

pub fn parse<'a>(tokens: &TokenBuf<'a>, program: &mut Program<'a>) -> Result<(), ParseError> {
    program.stmt_count = 0;
    program.expr_count = 0;
    let mut parser = Parser {
        tokens,
        pos: 0,
        program,
    };
    parser.parse_program()
}
