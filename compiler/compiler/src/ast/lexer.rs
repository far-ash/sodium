use super::token::*;
use crate::{
    error::*,
    position::{Next, Position},
};
use ascii::{AsciiChar, AsciiString};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    pos: Position,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a str, source: &'a str) -> Self {
        Self {
            pos: Position::new(file),
            chars: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.token()?;
            if let Token::Eof = token {
                tokens.push(Token::Eof);
                break;
            } else {
                tokens.push(token);
            }
        }
        Ok(tokens)
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            if c == '\n' {
                self.pos.next(Next::Line)
            } else {
                self.pos.next(Next::Column)
            }
            c
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn number_str(&mut self) -> String {
        let mut number = String::new();
        let mut isint = true;
        while let Some(ch) = self.peek() {
            if ch == '.' || ch == '_' || ch.is_ascii_digit() {
                if ch == '.' {
                    if !isint {
                        break;
                    }
                    isint = false;
                } else if ch == '_' {
                    continue;
                }
                number.push(ch);
                self.next();
            } else {
                break;
            }
        }
        number += &String::from(self.ident_str());

        number
    }

    fn numeric(&mut self) -> Result<Number> {
        let value = self.number_str();
        let mut suffix = String::new();
        if let Some(i) = value.find(|c: char| c.is_ascii_alphabetic()) {
            suffix = value.clone().drain(i..).collect::<String>();
            if !suffix.starts_with(|c| matches!(c, 'i' | 'u' | 'f')) {
                return Error::UnknownIdent(suffix).throw(&self.pos);
            }
        }
        let number = value.trim_end_matches(&suffix).trim();

        let float_overflow = |_| {
            Error::NumOverFlow(
                String::from(number),
                String::from(if suffix.is_empty() { "f64" } else { &suffix }),
            )
        };

        if number.contains('.') {
            return Ok(match &*suffix {
                "f32" => Number::F32(number.parse().map_err(float_overflow)?),
                "" | "f" | "f64" => Number::F64(number.parse().map_err(float_overflow)?),
                _ => return Error::InvalidSuffix(suffix).throw(&self.pos),
            });
        }
        let int_overflow = |_| {
            Error::NumOverFlow(
                String::from(number),
                String::from(if suffix.is_empty() { "i32" } else { &suffix }),
            )
        };

        Ok(match &*suffix {
            "i8" => Number::I8(number.parse().map_err(int_overflow)?),
            "i16" => Number::I16(number.parse().unwrap()),
            "" | "i32" => Number::I32(number.parse().map_err(int_overflow)?),
            "i64" => Number::I64(number.parse().map_err(int_overflow)?),
            "i128" => Number::I128(number.parse().map_err(int_overflow)?),
            "isize" => Number::ISize(number.parse().map_err(int_overflow)?),
            "u8" => Number::U8(number.parse().map_err(int_overflow)?),
            "u16" => Number::U16(number.parse().map_err(int_overflow)?),
            "u" | "u32" => Number::U32(number.parse().map_err(int_overflow)?),
            "u64" => Number::U64(number.parse().map_err(int_overflow)?),
            "u128" => Number::U128(number.parse().map_err(int_overflow)?),
            "usize" => Number::USize(number.parse().map_err(int_overflow)?),
            "f32" => Number::F32(number.parse().map_err(float_overflow)?),
            "f" | "f64" => Number::F64(number.parse().map_err(float_overflow)?),
            _ => return Error::InvalidSuffix(suffix).throw(&self.pos),
        })
    }

    fn ident_str(&mut self) -> AsciiString {
        let mut ident = AsciiString::new();
        while let Some(ch) = self.peek() {
            if ch == '_' || ch.is_ascii_alphanumeric() {
                ident.push(AsciiChar::new(ch));
                self.next();
            } else {
                break;
            }
        }
        ident
    }

    fn identifier(&mut self) -> Token {
        let ident = self.ident_str();
        let is_keyword = Keyword::KEYWORDS.iter().any(|&kw| kw == ident);
        if is_keyword {
            Token::Keyword(Keyword::from(ident))
        } else if ident == "true" || ident == "false" {
            Token::Literal(Literal::Boolean(ident == "true"))
        } else {
            Token::Identifier(ident)
        }
    }

    fn chars(&mut self) -> Result<Token> {
        self.next();
        let c;

        if let Some(ch) = self.next() {
            match ch {
                '\\' => {
                    // Handle escape sequences
                    if let Some(escape) = self.next() {
                        match escape {
                            'n' => c = '\n',
                            'r' => c = '\r',
                            't' => c = '\t',
                            '\\' => c = '\\',
                            '\'' => c = '\'',
                            _ => {
                                // Invalid escape sequence
                                return Error::InvalidEscapeSequence(escape).throw(&self.pos);
                            }
                        }
                    } else {
                        todo!()
                        // Unexpected end of input after backslash
                        //return Error::UnexpectedEndOfFile().throw(&self.pos);
                    }
                }
                '\'' => {
                    todo!()
                    // Empty character literal
                    // return Error::EmptyCharacterLiteral().throw(&start_pos);
                }
                _ => {
                    // Regular character
                    if let Some(next) = self.next() {
                        if next == '\'' {
                            // End of the character literal
                            c = ch;
                        } else {
                            todo!();
                            // Invalid character literal (more than one character)
                            // return Error::InvalidCharacterLiteral().throw(&self.pos);
                        }
                    } else {
                        todo!();
                        // Unexpected end of input after the character
                        //return Error::UnexpectedEndOfFile().throw(&self.pos);
                    }
                }
            }
        } else {
            // Unexpected end of input
            todo!();
            // return Error::UnexpectedEndOfFile().throw(&start_pos);
        }

        Ok(Token::Literal(Literal::Char(c)))
    }

    fn string(&mut self) -> Result<Token> {
        // let start_pos = self.pos.clone();

        // Consume the opening double quote
        self.next();
        let mut value = String::from(match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        });

        while let Some(ch) = self.next() {
            match ch {
                '"' => {
                    // End of the string literal
                    return Ok(Token::Literal(Literal::String(value)));
                }
                '\\' => {
                    // Handle escape sequences
                    if let Some(escaped_char) = self.next() {
                        match escaped_char {
                            'n' => value.push('\n'),
                            'r' => value.push('\r'),
                            't' => value.push('\t'),
                            '\\' => value.push('\\'),
                            '"' => value.push('"'),
                            _ => {
                                todo!();
                                // Invalid escape sequence
                                // return Error::InvalidEscapeSequence(escaped_char)
                                //    .throw(&self.pos);
                            }
                        }
                    } else {
                        // Unexpected end of input after backslash
                        todo!();
                        // return Error::UnexpectedEndOfFile().throw(&self.pos);
                    }
                }
                '\n' => {
                    todo!();
                    // Unterminated string literal
                    //return Error::UnterminatedStringLiteral(start_pos).throw(&self.pos);
                }
                _ => {
                    // Regular character, add to the string
                    value.push(ch);
                }
            }
        }

        // Unexpected end of input
        todo!();
        // Error::UnexpectedEndOfFile().throw(&start_pos)
    }

    fn skip_comments(&mut self) -> Result<()> {
        let mut comment: Option<Position> = None;

        loop {
            while self.peek().is_some_and(|c| matches!(c, ' ' | '\t' | '\n')) {
                self.next();
            }

            if let Some(c) = self.peek() {
                if c == '/' {
                    self.next();
                    match self.peek() {
                        Some('/') => {
                            // Line comment, skip until the end of the line
                            while let Some(ch) = self.next() {
                                if ch == '\n' {
                                    break;
                                }
                            }
                        }
                        Some('*') => {
                            // Block comment, skip until the end of the comment
                            let mut depth = 1;
                            self.next(); // Consume the '*'
                            if comment.is_none() {
                                // Set the start position of the multi-line comment
                                comment = Some(self.pos.clone());
                            }
                            while depth > 0 {
                                match (self.next(), self.peek()) {
                                    (Some('/'), Some('*')) => {
                                        self.next(); // Consume the '/'
                                        self.next(); // Consume the '*'
                                        depth += 1;
                                    }
                                    (Some('*'), Some('/')) => {
                                        self.next(); // Consume the '*'
                                        self.next(); // Consume the '/'
                                        depth -= 1;
                                    }
                                    (Some(_), _) => {}
                                    _ => {
                                        // Handle unclosed comment error here
                                        if let Some(mut pos) = comment {
                                            for _ in 0..3 {
                                                pos.next(Next::Column);
                                            }
                                            return Error::UnclosedComment(pos.clone())
                                                .throw(&self.pos);
                                        }
                                        // If we don't have a start position, just return
                                        return Ok(());
                                    }
                                }
                            }
                            // Reset the start position as the multi-line comment is now closed
                            comment = None;
                        }
                        _ => break,
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn token(&mut self) -> Result<Token> {
        // Skip comments and whitespace
        self.skip_comments()?;

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        Ok(match c {
            // Handle operators
            '`' | '!'..='/' | ':'..='@' | '['..='^' | '{'..='~' => match self.peek() {
                Some(ch) => {
                    if c == '?' {
                        Token::Literal(Literal::Null)
                    } else if c == '\'' {
                        self.chars().unwrap()
                    } else if c == '"' {
                        self.string().unwrap()
                    } else if c == ':' && ch == ':' {
                        self.next();
                        Token::Operator(Operator::Acceses)
                    } else if matches!(c, '+' | '-') && matches!(ch, '+' | '-') {
                        self.next();
                        Token::Operator(if c == '+' {
                            Operator::Increment
                        } else {
                            Operator::Decrement
                        })
                    } else if matches!(c, '!' | '=' | '<' | '>') && ch == '=' {
                        self.next();
                        Token::Operator(match c {
                            '!' => Operator::NotEqualTo,
                            '=' => Operator::EqualTo,
                            '<' => Operator::LessThanOrEqualTo,
                            '>' => Operator::GreaterThanOrEqualTo,
                            _ => unimplemented!(),
                        })
                    } else {
                        Token::Operator(Operator::from(c))
                    }
                }
                None => return Ok(Token::Eof),
            },
            // Handle numbers
            '0'..='9' => Token::Literal(Literal::Number(self.numeric()?)),
            // Handle identifiers and keywords
            '_' | 'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => {
                return Error::IllegalCharacter(c).throw(&self.pos);
            }
        })
    }
}
