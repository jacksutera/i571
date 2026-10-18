use crate::lexer::{Coord};
use crate::parser::{ExprNode, Value, LetKind};
use crate::env::{Env};

use std::fmt;
use std::error::Error;


/************************* Expression Evaluator ************************/

/// eval() Result possibilities: an EvalValue success or an
/// EvalError failure
type EvalResult = Result<EvalValue, EvalError>;

pub fn eval(ast: &ExprNode) -> EvalResult {
    //simply delegate to eval_in_env() with a new env.
    let env = &Env::<EvalValue>::new();
    eval_in_env(ast, &env)
}

fn eval_in_env(ast: &ExprNode, env: &Env<EvalValue>) -> EvalResult {
    match ast {
	ExprNode::Int { value, .. } => Ok(EvalValue::Value(*value)),
	ExprNode::Op2 { op, rand1, rand2, coord } => {
	    let f = op_fn(op, coord)?;
		let e1 = eval_in_env(rand1, env)?.val(coord)?;
		let e2 = eval_in_env(rand2, env)?.val(coord)?;
		Ok(EvalValue::Value(f(e1, e2)))
	},
	ExprNode::If { cond, then, else_, coord } => {
	    let c = eval_in_env(cond, env)?.val(coord)?;
		match c {
			0 => {
				eval_in_env(else_, env)
			}
			_ => {
				eval_in_env(then, env)
			}
		}
	},
	ExprNode::Lambda { params, body, .. } => Ok(EvalValue::Closure{params: params.clone(), body: *body.clone(), env: env.clone()}),
	ExprNode:: Let { let_kind, defs, expr, .. } => {
	    match let_kind {
			LetKind::Parallel => {
				par_let(ast, env, defs, expr)
			}
			LetKind::Sequential => {
				seq_let(ast, env, defs, expr)
			}
			LetKind::Recursive => {
				rec_let(ast, env, defs, expr)
			}
		}
	}
	ExprNode::Id { id, coord } => {
		let oc = env.get(id);
		if let Some(ev) = oc {
			Ok(ev)
		}
		else {
			panic!("cannot parse None value {:?}", ast)
		}
	},

	ExprNode::App { name, args, coord } => todo!(),

	// we panic below because not handling an AST variant is a bug
	_ => panic!("eval not defined for ast variant {:?}", ast),
    }
}

/*************************** Helpers for Eval **************************/

//TODO

fn par_let(ast: &ExprNode, env: &Env<EvalValue>, defs: &Vec<ExprNode>, expr: &Box<ExprNode>) -> EvalResult {
	let mut env1 = env.clone();
	for d in defs {
		if let ExprNode::Def { id, expr: def, coord } = d {
			let pb = eval_in_env(def, env)?;
			env1 = env1.push(id, pb);
		};
	}
	eval_in_env(expr, &env1)
}

fn seq_let(ast: &ExprNode, env: &Env<EvalValue>, defs: &Vec<ExprNode>, expr: &Box<ExprNode>) -> EvalResult {
	todo!()	
}

fn rec_let(ast: &ExprNode, env: &Env<EvalValue>, defs: &Vec<ExprNode>, expr: &Box<ExprNode>) -> EvalResult {
	todo!()
}

/**************************** Op2 Helper *******************************/

/// Avoid boilerplace for the different Op2 functions.  Simply call
/// op_fn() on the op and then apply the returned BinOpFn to
/// evaluate the recursively evaluted operands.

type BinOpFn = fn(Value, Value) -> Value;
struct OpFunc(&'static str,BinOpFn);

fn op_fn(op: &str, coord: &Coord) -> Result<BinOpFn, EvalError> {
    for op_fn in OP_FUNCS {
	if op == op_fn.0 {
	    return Ok(op_fn.1);
	}
    }
    Err(EvalError(format!("{}: unknown binary operation {}", op, coord)))
}


const OP_FUNCS: [OpFunc;7] = [
    OpFunc("+", |a, b| a+b),
    OpFunc("-", |a, b| a-b),
    OpFunc("*", |a, b| a*b),
    OpFunc("/", |a, b| a/b),
    OpFunc("%", |a, b| a%b),
    OpFunc("==", |a, b| if a == b { 1 } else { 0 }),
    OpFunc("<", |a, b| if a < b { 1 } else { 0 }),
];

/********************** Eval Results and Errors ************************/

/// The result of evaluating an expression
#[derive(Debug, Clone)]
pub enum EvalValue {
    Value(Value), //the expected value
    Closure { //captures a `lambda` and its referencing environment
	//owns its fields
	params: Vec<String>,
	body: ExprNode,
	env: Env<EvalValue>,
    },
    Undef,  //undefined value; use when implemented letrec
}

/// When an EvalValue is expected to be a Value, use .val() to
/// extract it without needing to pattern match. 
impl EvalValue {
    fn val(&self, coord: &Coord) -> Result<Value, EvalError> {
	if let Self::Value(value) = self {
	    Ok(*value)
	}
	else {
	    Err(EvalError(format!("{}: expected a Value, but got {:?}",
				  coord, self)))
	}
    }
}

/// Displaying EvalValue
impl fmt::Display for EvalValue {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Self::Value(val) => write!(f, "{}", val),
	    Self::Closure{..} => write!(f, "<closure>"),
	    Self::Undef => write!(f, "<undef>"),
	}
    }
}

/// Used to wrap an evaluation error message
#[derive(Debug, Clone)]
pub struct EvalError(String);

/// Displaying EvalError
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", self.0)
    }
}

impl Error for EvalError {}


