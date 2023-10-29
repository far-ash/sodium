use super::Result;
use crate::{ast::token::Number, position::Position};
use color_print::cformat;
use std::fmt::Debug;

pub enum Error {
    IllegalCharacter(char),
    InvalidSuffix(String),
    FileErr(String),
    UnknownIdent(String),
    NumOverFlow(String, String),
    UnclosedComment(Position),
    InvalidEscapeSequence(char),
}

impl Error {
    pub fn throw<T>(self, pos: &Position) -> Result<T> {
        eprintln!("{}", cformat!("<b>--></b> <dim>{pos:?}</>"));
        Err(self)
    }

    fn format(&self) -> String {
        let formated = cformat!(
            "<r,s>!</> - {name}: {message}\n<c>hint</>: {hint}",
            name = self.name(),
            message = self.msg(),
            hint = self.hint()
        );
        formated
    }

    fn name(&self) -> String {
        String::from(match self {
            Error::IllegalCharacter(_) => "illegal character",
            Error::InvalidSuffix(_) => "invalid suffix",
            Error::FileErr(_) => "file error",
            Error::UnknownIdent(_) => "unknown identifier",
            Error::NumOverFlow(_, num_type) => return format!("{num_type} number overflow"),
            Error::UnclosedComment(_) => "unclosed comment",
            Error::InvalidEscapeSequence(_) => "invalid escape sequence",
        })
    }

    fn msg(&self) -> String {
        match self {
            Error::IllegalCharacter(ch) => cformat!("unknown character found <W>'{ch}'</>"),
            Error::InvalidSuffix(sfx) => format!("\"{sfx}\" is an invalid suffix"),
            Error::FileErr(msg) => String::from(msg),
            Error::UnknownIdent(ident) => cformat!("unknown identifer <g>\"{ident}\"</>"),
            Error::NumOverFlow(num, num_type) => {
                format!("{num} is beyond the max value of {num_type}")
            }
            Error::UnclosedComment(_) => {
                cformat!("comment started with <g>\"/*\"</> but never ended with <g>\"*/\"<g>")
            }
            Error::InvalidEscapeSequence(es) => {
                cformat!("<g>'{es}'</> is not recognized as <u>escape sequence</>")
            }
        }
    }

    fn hint(&self) -> String {
        match self {
            Error::IllegalCharacter(ch) => cformat!("remove <g!>'{ch}'</> from the source"),
            Error::InvalidSuffix(sfx) => {
                let mut hint = String::from("did you meant to use ");
                let range = match sfx.as_bytes()[0] as char {
                    'i' => 0..=5,
                    'u' => 6..=12,
                    'f' => 13..=15,
                    _ => 0..=13,
                };
                for i in range {
                    hint.push_str(&format!("{}, ", Number::SUFFIXES[i]));
                }
                hint.pop();
                hint.pop();

                hint
            }
            Error::FileErr(file) => format!("make sure that <g>\"{file}\"</> exists"),
            Error::UnknownIdent(ident) => {
                cformat!("remove or declare <g>\"{ident}\"</> in source")
            }
            Error::NumOverFlow(_, num_type) => format!("{num_type}"),
            Error::UnclosedComment(pos) => cformat!("add <g>\"*/\"</> to <dim>{pos:?}</>",),
            Error::InvalidEscapeSequence(es) => {
                let c = es.escape_default().nth(0).unwrap();
                cformat!("remove <g>'{c}'</> & replace it with <g>'\\'</>",)
            }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}
