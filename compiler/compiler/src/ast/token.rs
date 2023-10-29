use ascii::AsciiString;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    Eof,
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    Identifier(AsciiString),
}

#[derive(Debug)]
pub enum Literal {
    Null,
    Char(char),
    Boolean(bool),
    Number(Number),
    String(String),
}

#[derive(Debug)]
pub enum Keyword {
    In,
    If,
    Mut,
    Use,
    Else,
    Loop,
}

impl Keyword {
    pub const KEYWORDS: [&str; 6] = ["in", "if", "mut", "use", "else", "loop"];
}

impl From<AsciiString> for Keyword {
    fn from(keyword: AsciiString) -> Self {
        match keyword.to_string().as_str() {
            "in" => Self::In,
            "if" => Self::If,
            "mut" => Self::Mut,
            "use" => Self::Use,
            "else" => Self::Else,
            "loop" => Self::Loop,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum Number {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    F32(f32),
    F64(f64),
}

impl Number {
    pub const SUFFIXES: [&'static str; 16] = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u", "u8", "u16", "u32", "u64", "u128",
        "usize", "f", "f32", "f64",
    ];
}

#[derive(Debug)]
pub enum Operator {
    Exclamation,
    Quotation,
    Hash,
    Dollar,
    Percent,
    Ampersand,
    Apostrophe,
    ParenOpen,
    ParenClose,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Dot,
    Slash,
    Colon,
    Semicolon,
    LessThan,
    Assign,
    GreaterThan,
    Question,
    At,
    BracketOpen,
    BackSlash,
    BracketClose,
    Caret,
    Grave,
    CurlyBraceOpen,
    VerticalBar,
    CurlyBraceClose,
    Tilde,
    EqualTo,
    NotEqualTo,
    Increment,
    Decrement,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    Acceses,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '!' => Operator::Exclamation,
            '"' => Operator::Quotation,
            '#' => Operator::Hash,
            '$' => Operator::Dollar,
            '%' => Operator::Percent,
            '&' => Operator::Ampersand,
            '\'' => Operator::Apostrophe,
            '(' => Operator::ParenOpen,
            ')' => Operator::ParenClose,
            '*' => Operator::Asterisk,
            '+' => Operator::Plus,
            ',' => Operator::Comma,
            '-' => Operator::Minus,
            '.' => Operator::Dot,
            '/' => Operator::Slash,
            ':' => Operator::Colon,
            ';' => Operator::Semicolon,
            '<' => Operator::LessThan,
            '=' => Operator::Assign,
            '>' => Operator::GreaterThan,
            '?' => Operator::Question,
            '@' => Operator::At,
            '[' => Operator::BracketOpen,
            '\\' => Operator::BackSlash,
            ']' => Operator::BracketClose,
            '^' => Operator::Caret,
            '`' => Operator::Grave,
            '{' => Operator::CurlyBraceOpen,
            '|' => Operator::VerticalBar,
            '}' => Operator::CurlyBraceClose,
            '~' => Operator::Tilde,
            _ => unimplemented!(),
        }
    }
}
