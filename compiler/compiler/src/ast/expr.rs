use super::token::*;

pub enum Expr {
    Assign {
        ident: Token,
        value: Box<Self>,
    },
    Binary {
        left: Box<Self>,
        operator: Operator,
        right: Box<Self>,
    },
    Function {
        callee: Box<Self>,
        param: Operator,
        arguments: Vec<Self>,
    },
    Group {
        expresion: Box<Self>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Self>,
        operator: Operator,
        right: Box<Self>,
    },
    Unary {
        operator: Operator,
        expresion: Side<Box<Self>>,
    },
    Variable {
        name: Token,
    },
}

#[allow(dead_code)]
impl Expr {
    pub fn assign(ident: Token, value: Box<Self>) -> Self {
        Self::Assign { ident, value }
    }
    pub fn binary(left: Box<Self>, operator: Operator, right: Box<Self>) -> Self {
        Self::Binary {
            left,
            operator,
            right,
        }
    }
    pub fn function(callee: Box<Self>, param: Operator, arguments: Vec<Self>) -> Self {
        Self::Function {
            callee,
            param,
            arguments,
        }
    }
    pub fn group(expresion: Box<Self>) -> Self {
        Self::Group { expresion }
    }
    pub fn literal(value: Literal) -> Self {
        Self::Literal { value }
    }
    pub fn logical(left: Box<Self>, operator: Operator, right: Box<Self>) -> Self {
        Self::Logical {
            left,
            operator,
            right,
        }
    }
    pub fn unary(operator: Operator, expresion: Side<Box<Self>>) -> Self {
        Self::Unary {
            operator,
            expresion,
        }
    }
    pub fn variable(name: Token) -> Self {
        Self::Variable { name }
    }
}

pub enum Side<T> {
    Left(T),
    Right(T),
}
