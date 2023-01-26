use crate::{lexer::Token, error::ParserError};
use std::fmt;

use crate::lexer::tokenizer;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Number(i64),
    Symbol(String),
    Bool(bool),
    List(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Number(n) => write!(f, "Number({n})"),
            Object::Symbol(s) => write!(f, "StringLiteral({s})"),
            Object::Bool(b) => write!(f, "Bool({b})"),
            Object::List(l) => {
                write!(f, "(")?;
                for (i, o) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{o}")?;
                }
                write!(f, ")")
            },
            Object::Lambda(params, body) => {
                write!(f, "Lambda(")?;
                for param in params {
                    write!(f, "{param} ")?;
                }
                write!(f, ")")?;
                for expr in body {
                    write!(f, " {expr}")?;
                }
                Ok(())
            },
        }
    }
}

pub fn parse(program: &str) -> Result<Object, ParserError> {
    let result = tokenizer(program);
    if result.is_err() {
        return Err(ParserError::UnexpectedParserError);
    }

    let mut tokens = result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed = parsed(&mut tokens).unwrap();
    Ok(parsed)
}

fn parsed(tokens: &mut Vec<Token>) -> Result<Object, ParserError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParserError::UnexpectedParserError);
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token.is_none() {
            return Err(ParserError::UnexpectedParserError);
        }

        let t = token.unwrap();
        match t {
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub = parsed(tokens)?;
                list.push(sub);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
            Token::Number(n) => {
                list.push(Object::Number(n));
            }
            Token::Symbol(s) => {
                list.push(Object::Symbol(s));
            }
        }
    }

    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let program = "(+ 1 2)";
        let result = parse(program);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Number(1),
                Object::Number(2),
            ])
        );
    }
}
