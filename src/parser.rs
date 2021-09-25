use crate::ast::*;
use crate::token::*;
use crate::token::TokenKind::*;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_types(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_types(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_types(vec![Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_types(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_types(Bang, Minus) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_types(False) { return Expr::Literal(Token::new()) }
    }

    fn match_types(&mut self, kinds: Vec<TokenKind>) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
