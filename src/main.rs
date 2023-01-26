use std::{cell::RefCell, rc::Rc};

use linefeed::{Interface, ReadResult};

use parser::Object;

mod env;
mod eval;
mod lexer;
mod parser;
mod error;

type ReplResult = Result<(), Box<dyn std::error::Error>>;

const PROMPT: &str = ">>> ";

fn main() -> ReplResult {
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = Rc::new(RefCell::new(env::Environment::new()));

    reader.set_prompt(format!("{PROMPT}").as_ref()).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input == "exit" {
            break;
        }

        let val = eval::eval(input.as_ref(), &mut env)?;
        match val {
            Object::Void => {},
            Object::Number(n) => println!("{n}"),
            Object::Bool(b) => println!("{b}"),
            Object::Symbol(s) => println!("{s}"),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{param} ");
                }
                println!(")");
                for expr in body {
                    println!(" {expr}");
                }
            }
            _ => println!("{val}"),
        }
    }
    println!("Exit..");
    Ok(())
}
