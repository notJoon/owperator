use std::error::Error;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserError {
    UnexpectedParserError,
    ParenIsNotMatched,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnexpectedParserError => write!(f, "Unexpected parser error"),
            ParserError::ParenIsNotMatched => write!(f, "Parentheses pair do not match"),
        }
    }
}

impl Error for ParserError {}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EvalError {
    InvalidArgumentHasDetected,
    InvalidOperatorHasDetected,
    InvalidConditionType,
    InvalidLambda,
    NotEnoughArgs,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::InvalidArgumentHasDetected => write!(f, "Invalid number of arguments"),
            EvalError::InvalidOperatorHasDetected => write!(f, "Invalid operator"),
            EvalError::InvalidConditionType => write!(f, "Condition must be a boolean"),
            EvalError::InvalidLambda => write!(f, "Invalid Lambda"),
            EvalError::NotEnoughArgs => write!(f, "Invalid number of arguments"),
        }
    }
}

impl Error for EvalError {}