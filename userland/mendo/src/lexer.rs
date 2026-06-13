//! Tokenizer for Mendo source.

const MAX_TOKENS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Print,
    Let,
    If,
    Else,
    While,
    Ident,
    Int,
    String,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub value: i64,
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar,
    UnterminatedString,
    TooManyTokens,
}

pub struct TokenBuf<'a> {
    pub tokens: [Token<'a>; MAX_TOKENS],
    pub len: usize,
}

impl<'a> TokenBuf<'a> {
    pub const fn new() -> Self {
        Self {
            tokens: [Token {
                kind: TokenKind::Eof,
                text: "",
                value: 0,
            }; MAX_TOKENS],
            len: 0,
        }
    }

    fn push(&mut self, token: Token<'a>) -> Result<(), LexError> {
        if self.len >= MAX_TOKENS {
            return Err(LexError::TooManyTokens);
        }
        self.tokens[self.len] = token;
        self.len += 1;
        Ok(())
    }
}

fn keyword(text: &str) -> Option<TokenKind> {
    match text {
        "print" => Some(TokenKind::Print),
        "let" => Some(TokenKind::Let),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "while" => Some(TokenKind::While),
        _ => None,
    }
}

fn is_ident_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_ident_continue(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn skip_ws(input: &[u8], mut i: usize) -> usize {
    while i < input.len() && (input[i] as char).is_ascii_whitespace() {
        i += 1;
    }
    i
}

pub fn tokenize<'a>(source: &'a str, out: &mut TokenBuf<'a>) -> Result<(), LexError> {
    out.len = 0;
    let input = source.as_bytes();
    let mut i = 0usize;

    while i < input.len() {
        i = skip_ws(input, i);
        if i >= input.len() {
            break;
        }
        let start = i;
        let c = input[i];

        if c.is_ascii_digit() {
            let mut value: i64 = 0;
            while i < input.len() && input[i].is_ascii_digit() {
                value = value
                    .saturating_mul(10)
                    .saturating_add((input[i] - b'0') as i64);
                i += 1;
            }
            let text = &source[start..i];
            out.push(Token {
                kind: TokenKind::Int,
                text,
                value,
            })?;
            continue;
        }

        if c == b'"' {
            i += 1;
            let str_start = i;
            while i < input.len() && input[i] != b'"' {
                i += 1;
            }
            if i >= input.len() {
                return Err(LexError::UnterminatedString);
            }
            let text = &source[str_start..i];
            i += 1;
            out.push(Token {
                kind: TokenKind::String,
                text,
                value: 0,
            })?;
            continue;
        }

        if is_ident_start(c) {
            i += 1;
            while i < input.len() && is_ident_continue(input[i]) {
                i += 1;
            }
            let text = &source[start..i];
            let kind = keyword(text).unwrap_or(TokenKind::Ident);
            out.push(Token {
                kind,
                text,
                value: 0,
            })?;
            continue;
        }

        let (kind, advance) = match c {
            b'(' => (TokenKind::LParen, 1),
            b')' => (TokenKind::RParen, 1),
            b'{' => (TokenKind::LBrace, 1),
            b'}' => (TokenKind::RBrace, 1),
            b';' => (TokenKind::Semicolon, 1),
            b'+' => (TokenKind::Plus, 1),
            b'-' => (TokenKind::Minus, 1),
            b'*' => (TokenKind::Star, 1),
            b'/' => (TokenKind::Slash, 1),
            b'=' if i + 1 < input.len() && input[i + 1] == b'=' => (TokenKind::Eq, 2),
            b'=' => (TokenKind::Assign, 1),
            b'!' if i + 1 < input.len() && input[i + 1] == b'=' => (TokenKind::Ne, 2),
            b'<' if i + 1 < input.len() && input[i + 1] == b'=' => (TokenKind::Le, 2),
            b'<' => (TokenKind::Lt, 1),
            b'>' if i + 1 < input.len() && input[i + 1] == b'=' => (TokenKind::Ge, 2),
            b'>' => (TokenKind::Gt, 1),
            _ => return Err(LexError::UnexpectedChar),
        };
        i += advance;
        out.push(Token {
            kind,
            text: &source[start..i],
            value: 0,
        })?;
    }

    out.push(Token {
        kind: TokenKind::Eof,
        text: "",
        value: 0,
    })?;
    Ok(())
}
