#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::parser::*;
use crate::env::*;
use crate::error::*;

const VALID_MINIMUM_ARGS: usize = 3;
const VALID_MINIMUM_COND_ARGS: usize = 4;

type EvalResult = Result<Object, String>;

fn eval_binop(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    if list.len() < VALID_MINIMUM_ARGS {
        return Err(format!("Invalid number of arguments: {}", list.len()));
    }
    let op = list[0].clone();
    let arg1 = eval_obj(&list[1].clone(), env)?;
    let arg2 = eval_obj(&list[2].clone(), env)?;
    let arg1_val = check_arg_is_number(&arg1)?;
    let arg2_val = check_arg_is_number(&arg2)?;

    match op {
        Object::Symbol(s) => {
            match s.as_str() {
                "+" => Ok(Object::Number(arg1_val + arg2_val)),
                "-" => Ok(Object::Number(arg1_val - arg2_val)),
                "*" => Ok(Object::Number(arg1_val * arg2_val)),
                "/" => Ok(Object::Number(arg1_val / arg2_val)),
                "owo" => Ok(Object::Bool(arg1_val == arg2_val)),
                "uwu" => Ok(Object::Bool(arg1_val != arg2_val)),
                "Owo" => Ok(Object::Bool(arg1_val >= arg2_val)),
                "owO" => Ok(Object::Bool(arg1_val <= arg2_val)),
                "O_o" => Ok(Object::Bool(arg1_val > arg2_val)),
                "o_O" => Ok(Object::Bool(arg1_val < arg2_val)),
                _ => Err(format!("Invalid operator: {s}")),
            }
        }
        _ => Err(format!("Invalid operator: {op}")),
    }
}

fn check_arg_is_number(arg: &Object) -> Result<i64, String> {
    match arg {
        Object::Number(n) => Ok(*n),
        _ => Err(format!("Invalid argument. argument must be an Number: {arg}")),
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    if list.len() != VALID_MINIMUM_ARGS {
        return Err(format!("Invalid number of arguments: {}", list.len()));
    }

    let symbol = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid argument. argument must be an Symbol: {}", list[1])),
    };

    let value = eval_obj(&list[2], env)?;
    env.borrow_mut().set(&symbol, value);

    Ok(Object::Void)
}

fn eval_cond(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    if list.len() != VALID_MINIMUM_COND_ARGS {
        return Err(format!("Invalid number of arguments for condition statement : {}", list.len()));
    }

    let obj = eval_obj(&list[1], env)?;
    let cond = match obj {
        Object::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean. got: {obj}")),
    };

    if cond {
        return eval_obj(&list[2], env)
    }

    eval_obj(&list[3], env)
}

fn eval_func_definition(list: &[Object]) -> EvalResult {
    let params = match &list[1] {
        Object::List(l) => {
            let mut params = Vec::new();
            for p in l {
                match p {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid argument. argument must be an Symbol: {p}")),
                }
            }
            params
        }
        _ => return Err("Invalid Lambda".to_string()),
    };

    let body = match &list[2] {
        Object::List(l) => l.clone(),
        _ => return Err("Invalid Lambda".to_string()),
    };

    Ok(Object::Lambda(params, body))
}

fn eval_func_call(
    s: &str,
    list: &[Object],
    env: &mut Rc<RefCell<Environment>>,
) -> EvalResult {
    let lambda = env.borrow_mut().get(s);
    if lambda.is_none() {
        return Err(format!("Undefined function: {s}"));
    }

    let func = lambda.unwrap();
    match func {
        Object::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Environment::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }
            eval_obj(&Object::List(body), &mut new_env)
        }
        _ => return Err(format!("Invalid function: {s}")),
    }
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    let val = env.borrow_mut().get(s);
    if val.is_none() {
        return Err(format!("Undefined symbol: {s}"));
    }

    Ok(val.unwrap())
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    let head = &list[0].clone();
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "owo" | "uwu" | "Owo" | "owO" | "O_o" | "o_O" => {
                eval_binop(list, env)
            }
            "define" => eval_define(list, env),
            "if" => eval_cond(list, env),
            "lambda" => eval_func_definition(list),
            _ => eval_func_call(s, list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => (),
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    match obj {
        Object::Lambda(_p, _b) => Ok(Object::Void),
        Object::List(l) => eval_list(l, env),
        Object::Number(n) => Ok(Object::Number(*n)),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Void => Ok(Object::Void),
    }
}

pub fn eval(program: &str, env: &mut Rc<RefCell<Environment>>) -> EvalResult {
    let parsed = parse(program);

    if parsed.is_err() {
        return Err(format!("Parse error: {}", parsed.err().unwrap()));
    }

    eval_obj(&parsed.unwrap(), env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let result = eval("(+ 1 2)", &mut env);
        assert_eq!(result.unwrap(), Object::Number(3));
    }

    #[test]
    fn test_sub() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let result = eval("(- 1 2)", &mut env);
        assert_eq!(result.unwrap(), Object::Number(-1));
    }

    #[test]
    fn test_mul() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let result = eval("(* 1 2)", &mut env);
        assert_eq!(result.unwrap(), Object::Number(2));
    }

    #[test]
    fn test_div() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let result = eval("(/ 1 2)", &mut env);
        assert_eq!(result.unwrap(), Object::Number(0));
    }

    #[test]
    fn test_is_equal() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let result_false = eval("(owo 1 2)", &mut env);
        assert_eq!(result_false.unwrap(), Object::Bool(false));

        let result_true = eval("(owo 1 1)", &mut env);
        assert_eq!(result_true.unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_not_equal() {
        let mut env = Rc::new(RefCell::new(Environment::new()));

        let result = eval("(uwu 1 2)", &mut env);
        assert_eq!(result.unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_is_less_equal() {
        let mut env = Rc::new(RefCell::new(Environment::new()));

        let result_true_equal = eval("(owO 2 2)", &mut env);
        assert_eq!(result_true_equal.unwrap(), Object::Bool(true));

        let result_true_greater = eval("(owO 2 3)", &mut env);
        assert_eq!(result_true_greater.unwrap(), Object::Bool(true));

        let result_false = eval("(owO 2 1)", &mut env);
        assert_eq!(result_false.unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_is_less() {
        let mut env = Rc::new(RefCell::new(Environment::new()));

        let result_true = eval("(o_O 1 2)", &mut env);
        assert_eq!(result_true.unwrap(), Object::Bool(true));

        let result_false = eval("(o_O 10 2)", &mut env);
        assert_eq!(result_false.unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_is_greater_equal() {
        let mut env = Rc::new(RefCell::new(Environment::new()));

        let result_true_equal = eval("(Owo 10 10)", &mut env);
        assert_eq!(result_true_equal.unwrap(), Object::Bool(true));

        let result_true_greater = eval("(Owo 10 5)", &mut env);
        assert_eq!(result_true_greater.unwrap(), Object::Bool(true));

        let result_false = eval("(Owo 1 2)", &mut env);
        assert_eq!(result_false.unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_is_greater() {
        let mut env = Rc::new(RefCell::new(Environment::new()));

        let result_true = eval("(O_o 10 5)", &mut env);
        assert_eq!(result_true.unwrap(), Object::Bool(true));

        let result_false = eval("(O_o 1 2)", &mut env);
        assert_eq!(result_false.unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_factorial() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let program = "
            (
                (define fact (lambda (n) (if (o_O n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Number(120)]));
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Environment::new()));
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Number((314 * 10 * 10) as i64)])
        );
    }
}