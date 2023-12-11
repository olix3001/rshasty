use std::{any::Any, rc::Rc};

use crate::util::HastyError;

/// Represents type of a token.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    UNDERSCORE, BANG, EQUAL, LESS, GREATER, AMPERSAND,
    PIPE, COLON,

    // Two-character tokens.
    BANG_EQUAL, EQUAL_EQUAL, GREATER_EQUAL, LESS_EQUAL,
    AND, OR, INCREMENT, DECREMENT,

    // Literals.
    IDENTIFIER, STRING, CHARACTER, INTEGER,
    FLOATING,

    // Keywords.
    FN, IF, ELSE, TRUE, FALSE, WHILE, FOR, RETURN, SELF,
    VAR, NIL, GUARD, PUB, IMPORT, FROM, AS, CONST, LET,

    EOF
}

/// Represents token with it's metadata.
#[derive(Debug, Clone)]
pub struct Token {
    /// Type of a token.
    pub token_type: TokenType,
    /// Source code associated with this token.
    pub lexeme: String,
    /// Line where this token appears.
    pub line: usize,
    /// Position at which token starts.
    pub start: usize,
    /// Additional data associated with this token
    pub data: Option<Rc<dyn Any>>,
}

impl Token {
    /// Creates new token.
    pub fn new(token_type: TokenType, lexeme: String, line: usize, start: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            start,
            data: None,
        }
    }

    /// Adds data to token.
    pub fn with_data(mut self, data: impl Any) -> Self {
        self.data = Some(Rc::new(data));
        self
    }
}

/// Possible errors that can occur during parsing.
#[derive(Debug)]
pub enum ScannerError {
    /// Found character that does not match any rule.
    UnexpectedCharacter,
    /// String was opened but not closed.
    UnterminatedString,
}

impl HastyError for ScannerError {
    fn as_hasty_error_string(&self) -> String {
        return "TODO".to_string();
    }

    fn get_error_description(&self) -> String {
        match self {
            ScannerError::UnexpectedCharacter { .. } => "Unexpected character.",
            ScannerError::UnterminatedString => "Unterminated string.",
        }.to_string()
    }
}

/// Type responsible for scanning source code and producing tokens.
pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    /// Create new scanner from source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    /// Check whether reader has reached the end of source.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Get next token.
    fn advance(&mut self) -> char {
        let current_char = self.source[self.current] as char;
        self.current += 1;
        current_char
    }

    /// Check next token without consuming it.
    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.source[self.current] as char
    }

    /// Peek next character.
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source[self.current + 1] as char
    }

    /// Add new token.
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        // This should never panic, as inserting non-utf8 characters into source code is not common.
        let lexeme = String::from_utf8(Vec::from(lexeme)).unwrap();
        self.tokens.push(Token::new(token_type, lexeme, self.line, self.start))
    }

    /// Add new token with data.
    fn add_token_with_data(&mut self, token_type: TokenType, data: impl Any) {
        let lexeme = &self.source[self.start..self.current];
        // This should never panic, as inserting non-utf8 characters into source code is not common.
        let lexeme = String::from_utf8(Vec::from(lexeme)).unwrap();
        self.tokens.push(Token::new(token_type, lexeme, self.line, self.start).with_data(data))
    }

    /// Scans source code to produce tokens.
    pub fn scan(mut self) -> Result<Vec<Token>, ScannerError> {
        while !self.is_at_end() {
            // Begin new lexeme.
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line, 0));
        Ok(self.tokens)
    }

    /// Tries to match character if possible, consuming it if matches.
    fn try_match(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source[self.current] as char != expected { return false }

        self.current += 1;
        return true;
    }

    /// Scan one token.
    fn scan_token(&mut self) -> Result<(), ScannerError> {
        // Macro for convenience
        macro_rules! try_match {
            ($char:literal => $a:ident | $b:ident) => {{
                let tt = if self.try_match($char) { TokenType::$a } else { TokenType::$b };
                self.add_token(tt);
            }};
        }

        let c = self.advance();
        match c {
            // Single-character only.
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            ':' => self.add_token(TokenType::COLON),

            // Single-character or two-character tokens.
            '!' => try_match!('=' => BANG_EQUAL | BANG),
            '=' => try_match!('=' => EQUAL_EQUAL | EQUAL),
            '<' => try_match!('=' => LESS_EQUAL | LESS),
            '>' => try_match!('=' => GREATER_EQUAL | GREATER),
            '+' => try_match!('+' => INCREMENT | PLUS),
            '-' => try_match!('-' => DECREMENT | MINUS),
            '&' => try_match!('&' => AND | AMPERSAND),
            '|' => try_match!('|' => OR | PIPE),

            // More complicated tokens.
            '/' => {
                if self.try_match('/') {
                    // Ignore comment.
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            '"' => { self.string()?; },
            '\'' => { self.character()?; },
            '0'..='9' => { self.number()?; },
            '_' => {
                if self.peek().is_whitespace() {
                    self.add_token(TokenType::UNDERSCORE);
                } else {
                    self.identifier()?;
                }
            },
            '$' | 'A'..='z' => { self.identifier()?; },

            // Useless characters.
            ' ' | '\r' | '\t' => { /* ignore */ },
            '\n' => self.line += 1,

            // Unexpected.
            _ => {
                self.err_unexpected_char()?
            }
        }

        Ok(())
    }

    /// Return unexpected char error
    fn err_unexpected_char(&self) -> Result<(), ScannerError> {
        Err(ScannerError::UnexpectedCharacter)
    }

    /// Produce token for string.
    fn string(&mut self) -> Result<(), ScannerError> {
        // TODO: Add support for escape characters in strings.
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 };
            self.advance();
        }

        if self.is_at_end() {
            Err(ScannerError::UnterminatedString)?;
        }

        // Match closing ".
        self.advance();

        // Produce token.
        let value = String::from_utf8(self.source[self.start + 1..self.current - 1].to_owned());
        self.add_token_with_data(TokenType::STRING, value.unwrap());
        Ok(())
    }

    /// Produce token for character literal.
    fn character(&mut self) -> Result<(), ScannerError> {
        unimplemented!()
    }

    /// Produce token for number literal.
    fn number(&mut self) -> Result<(), ScannerError> {
        while self.peek().is_digit(10) { self.advance(); }
        let mut is_floating = false;

        // Look for fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            is_floating = true;

            // Consume ".".
            self.advance();

            while self.peek().is_digit(10) { self.advance(); }
        }

        self.add_token(if is_floating { TokenType::FLOATING } else { TokenType::INTEGER });
        Ok(())
    }

    /// Produce token for identifiers
    fn identifier(&mut self) -> Result<(), ScannerError> {
        while self.peek().is_alphanumeric() { self.advance(); }

        let text = String::from_utf8(
            Vec::from(&self.source[self.start..self.current])
        ).unwrap();
        let text = text.as_str();

        // Match keywords.
        self.add_token(
            match text {
                "fn" => TokenType::FN,
                "if" => TokenType::IF,
                "else" => TokenType::ELSE,
                "true" => TokenType::TRUE,
                "false" => TokenType::FALSE,
                "while" => TokenType::WHILE,
                "for" => TokenType::FOR,
                "return" => TokenType::RETURN,
                "self" => TokenType::SELF,
                "var" => TokenType::VAR,
                "nil" => TokenType::NIL,
                "guard" => TokenType::GUARD,
                "pub" => TokenType::PUB,
                "import" => TokenType::IMPORT,
                "from" => TokenType::FROM,
                "as" => TokenType::AS,
                "let" => TokenType::LET,
                "const" => TokenType::CONST,
                _ => TokenType::IDENTIFIER
            }
        );

        Ok(())
    }
}