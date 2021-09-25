use crate::token::*;
use anyhow::*;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenKind::And);
        keywords.insert("class", TokenKind::Class);
        keywords.insert("else", TokenKind::Else);
        keywords.insert("false", TokenKind::False);
        keywords.insert("for", TokenKind::For);
        keywords.insert("fun", TokenKind::Fun);
        keywords.insert("if", TokenKind::If);
        keywords.insert("nil", TokenKind::Nil);
        keywords.insert("or", TokenKind::Or);
        keywords.insert("print", TokenKind::Print);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("super", TokenKind::Super);
        keywords.insert("this", TokenKind::This);
        keywords.insert("true", TokenKind::True);
        keywords.insert("var", TokenKind::Var);
        keywords.insert("while", TokenKind::While);
        keywords
    };
}

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut failed = false;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(t) => {
                    if let Some(token) = t {
                        tokens.push(self.create_token(token));
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    failed = true;
                }
            }
        }

        if failed {
            bail!("Failed to tokenise input");
        }

        tokens.push(Token::new(TokenKind::Eof, "".into(), self.line));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<TokenKind>> {
        let c = self.advance();
        Ok(match c {
            '(' => Some(TokenKind::LeftParen),
            ')' => Some(TokenKind::RightParen),
            '{' => Some(TokenKind::LeftBrace),
            '}' => Some(TokenKind::RightBrace),
            ',' => Some(TokenKind::Comma),
            '.' => Some(TokenKind::Dot),
            '-' => Some(TokenKind::Minus),
            '+' => Some(TokenKind::Plus),
            ';' => Some(TokenKind::Semicolon),
            '*' => Some(TokenKind::Star),
            '!' => Some(if self.match_char('=') {
                TokenKind::BangEqual
            } else {
                TokenKind::Bang
            }),
            '=' => Some(if self.match_char('=') {
                TokenKind::EqualEqual
            } else {
                TokenKind::Equal
            }),
            '<' => Some(if self.match_char('=') {
                TokenKind::LessEqual
            } else {
                TokenKind::Less
            }),
            '>' => Some(if self.match_char('=') {
                TokenKind::GreaterEqual
            } else {
                TokenKind::Greater
            }),
            '/' => {
                if self.match_char('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenKind::Slash)
                }
            }
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => {
                while !self.is_at_end() && self.peek() != '"' {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    bail!("Unterminated string at line {}", self.line);
                }

                self.advance();
                Some(TokenKind::String(
                    self.source[self.start + 1..self.current - 1].into(),
                ))
            }
            _ => {
                if c.is_numeric() {
                    while self.peek().is_numeric() {
                        self.advance();
                    }

                    // Look for a decimal
                    if self.peek() == '.' && self.peek_next().is_numeric() {
                        // Consume the "."
                        self.advance();

                        while self.peek().is_numeric() {
                            self.advance();
                        }
                    }

                    Some(TokenKind::Number(
                        (&self.source[self.start..self.current]).parse()?,
                    ))
                } else if c.is_alphabetic() || c == '_' {
                    while self.peek().is_alphanumeric() || self.peek() == '_' {
                        self.advance();
                    }
                    let text = &self.source[self.start..self.current];
                    let kind = KEYWORDS.get(&text);
                    match kind {
                        None => Some(TokenKind::Identifier(text.into())),
                        Some(k) => Some(k.clone()),
                    }
                } else {
                    bail!("Unexpected character at line {}: {}", self.line, c);
                }
            }
        })
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        result
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn create_token(&self, r#type: TokenKind) -> Token {
        let text = &self.source[self.start..self.current];
        Token::new(r#type, text.to_owned(), self.line)
    }
}
