use std::thread::current;
use crate::parser::ast::ASTNode;
use crate::scanner::{Token, TokenType};
use crate::util::{HastyError, unified_error};

mod ast;

/// Struct for parsing tokens into AST.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub enum ParserErrorTy {
    ExpectedToken {
        token: TokenType
    }
}

#[derive(Debug)]
pub struct ParserError {
    ty: ParserErrorTy,
    token: Token,
}

impl ParserError {
    pub fn new(ty: ParserErrorTy, token: Token) -> Self {
        Self {
            ty,
            token,
        }
    }
}

impl HastyError for ParserError {
    fn as_hasty_error_string(&self) -> String {
        // TODO: Put line str here
        unified_error(
            "PARSER", &self.get_error_description(),
            self.token.line, self.token.start,
            &self.token.lexeme, ""
        )
    }

    fn get_error_description(&self) -> String {
        match &self.ty {
            ParserErrorTy::ExpectedToken { token } => format!("Expected {:?}.", token)
        }.to_string()
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    // Utility functions
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        // Should never be called if would panic.
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        // Should never be called if would panic.
        self.tokens.get(self.current - 1).unwrap()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }
    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().token_type == ty
    }

    fn try_match(&mut self, ty: TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            return true;
        }
        return false;
    }

    fn match_any(&mut self, types: &Vec<TokenType>) -> bool {
        for ty in types.iter() {
            if self.check(ty.clone()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn expect(&mut self, ty: TokenType, error: ParserError) -> Result<(), ParserError> {
        if self.try_match(ty) { Ok(()) } else { Err(error) }
    }

    fn parser_error(&self, et: ParserErrorTy) -> Result<(), ParserError> {
        Err(ParserError::new(et, self.peek().clone()))
    }

    // Basics.
    pub fn parse(mut self) -> Result<ASTNode, ParserError> {
        self.expression()
    }

    // Parsing functions.
    fn expression(&mut self) -> Result<ASTNode, ParserError> {
        self.logic_or()
    }

    /// logic_or -> logic_and ( "||" logic_and )*;
    fn logic_or(&mut self) -> Result<ASTNode, ParserError> {
        let mut expr: ASTNode = self.logic_and()?;

        while self.match_any(&vec![TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.logic_and()?;
            expr = ASTNode::Logical {
                left: expr.boxed(),
                operator: operator.clone(),
                right: right.boxed(),
            };
        }

        Ok(expr)
    }

    /// logic_and -> equality ( "&&" equality )*;
    fn logic_and(&mut self) -> Result<ASTNode, ParserError> {
        let mut expr: ASTNode = self.equality()?;

        while self.match_any(&vec![TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = ASTNode::Logical {
                left: expr.boxed(),
                operator: operator.clone(),
                right: right.boxed(),
            };
        }

        Ok(expr)
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison)*;
    fn equality(&mut self) -> Result<ASTNode, ParserError> {
        let mut expr: ASTNode = self.comparison()?;

        while self.match_any(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = ASTNode::Binary {
                left: expr.boxed(),
                operator: operator.clone(),
                right: right.boxed(),
            };
        }

        Ok(expr)
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*;
    fn comparison(&mut self) -> Result<ASTNode, ParserError> {
        let mut expr = self.term()?;

        while self.match_any(&vec![
            TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = ASTNode::Binary {
                left: expr.boxed(),
                operator: operator.clone(),
                right: right.boxed(),
            };
        }

        Ok(expr)
    }

    /// factor -> unary ( ( "/" | "*" ) unary )*;
    fn term(&mut self) -> Result<ASTNode, ParserError> {
        let mut expr = self.factor()?;

        while self.match_any(&vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = ASTNode::Binary {
                left: expr.boxed(),
                operator: operator.clone(),
                right: right.boxed(),
            };
        }

        Ok(expr)
    }

    /// factor -> unary ( ( "/" | "*" ) unary )*;
    fn factor(&mut self) -> Result<ASTNode, ParserError> {
        let mut expr = self.unary()?;

        while self.match_any(&vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = ASTNode::Binary {
                left: expr.boxed(),
                operator: operator.clone(),
                right: right.boxed(),
            };
        }

        Ok(expr)
    }

    /// unary -> ( "!" | "-" ) unary | primary;
    fn unary(&mut self) -> Result<ASTNode, ParserError> {
        if self.match_any(&vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(ASTNode::Unary {
                operator: operator.clone(),
                right: right.boxed(),
            });
        }

        return self.primary();
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")";
    fn primary(&mut self) -> Result<ASTNode, ParserError> {
        if self.match_any(&vec![TokenType::TRUE, TokenType::FALSE, TokenType::NIL]) {
            return Ok(ASTNode::Literal {
                value: self.previous().clone(),
            });
        }

        if self.match_any(&vec![TokenType::INTEGER, TokenType::FLOATING, TokenType::STRING]) {
           return Ok(ASTNode::Literal {
               value: self.previous().clone(),
           });
        }

        if self.try_match(TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            self.expect(
                TokenType::RIGHT_PAREN,
                self.parser_error(ParserErrorTy::ExpectedToken { token: TokenType::RIGHT_PAREN }).unwrap_err()
            )?;
            return Ok(ASTNode::Grouping {
                expr: expr.boxed(),
            });
        }

        todo!()
    }
}