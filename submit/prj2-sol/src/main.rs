#![allow(unused)]  //comment or remove this before submission

use std::env as sys_env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

mod lexer;
mod parser;
mod env;
mod list;
mod eval;
//mod prj2_error;


fn main() -> ExitCode  {
    let mut is_error = false;
    let args: Vec<String> = sys_env::args().collect();
    if args.len() != 2 {
	if let Some(name) = Path::new(&args[0]).file_name() {
	    eprintln!("usage: {} FILE_PATH", name.to_string_lossy());
	}
	is_error = true;
    }
    else {
	let path = String::from(&args[1]);
	let text = fs::read_to_string(&path)
	    .expect(&format!("cannot read {}", path));
	let toks = lexer::scan(&path, &text);
	let mut parser = parser::Parser::new(&toks);

	match parser.parse() {
	    Err(e) => { eprintln!("error: {}", e); is_error = true }
	    Ok(ast) => {
		match eval::eval(&ast) {
		    Ok(eval_val) => println!("{}", eval_val),
		    Err(e) => { eprintln!("{}", e); is_error = true }
		}
	    }
	}
    }
    if is_error { 
	ExitCode::from(1)
    }
    else {
	ExitCode::SUCCESS
    }
}
