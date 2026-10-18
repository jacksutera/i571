use crate::lexer::{Coord, Token, Tokens};
use std::error::Error;
use std::fmt;

/***************************** AST Definitions *************************/

// AST set up to own all its data

//.1
pub type Value = i64;

#[derive(Debug, Clone)]
pub enum LetKind {
    Parallel,
    Sequential,
    Recursive,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ExprNode {
    Lambda {
        params: Vec<String>, //formal parameter names
        body: Box<ExprNode>,
        coord: Coord,
    },
    If {
        cond: Box<ExprNode>,
        then: Box<ExprNode>,
        else_: Box<ExprNode>,
        coord: Coord,
    },
    Let {
        let_kind: LetKind,
        defs: Vec<ExprNode>,
        expr: Box<ExprNode>,
        coord: Coord,
    },
    Def {
        id: String,
        expr: Box<ExprNode>,
        coord: Coord,
    },
    App {
        //function application
        name: String,        //name of function
        args: Vec<ExprNode>, //actual arguments
        coord: Coord,
    },
    Op2 {
        op: String, //only +, -, *, /, %, <, ==
        rand1: Box<ExprNode>,
        rand2: Box<ExprNode>,
        coord: Coord,
    },
    Id {
        id: String,
        coord: Coord,
    },
    Int {
        value: Value,
        coord: Coord,
    },
    Dummy, //only used for convenience during development
}
//.2

//use for placeholder during development
const DUMMY_NODE: ExprNode = ExprNode::Dummy;

impl fmt::Display for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn let_keyword(let_kind: &LetKind) -> &'static str {
            match let_kind {
                LetKind::Parallel => "let",
                LetKind::Sequential => "let*",
                LetKind::Recursive => "letrec",
            }
        }

        fn out(expr: &ExprNode, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
            const INDENT_INC: usize = 2;
            let indent1 = indent + INDENT_INC;
            match expr {
                ExprNode::Lambda { params, body, .. } => {
                    write!(f, "{:indent$}lambda( ", "", indent = indent)?;
                    for param in params {
                        write!(f, "{} ", param)?;
                    }
                    write!(f, ")\n")?;
                    out(body, f, indent)
                }
                ExprNode::Let {
                    let_kind,
                    defs,
                    expr,
                    ..
                } => {
                    write!(
                        f,
                        "{:indent$}{}\n",
                        "",
                        let_keyword(let_kind),
                        indent = indent
                    )?;
                    for def in defs {
                        if let ExprNode::Def { id, expr, .. } = def {
                            write!(f, "{:indent$}{} =\n", "", id, indent = indent1)?;
                            out(expr, f, indent1)?;
                        }
                    }
                    write!(f, "{:indent$}in\n", "", indent = indent)?;
                    out(expr, f, indent1)
                }
                ExprNode::If {
                    cond, then, else_, ..
                } => {
                    write!(f, "{:indent$}if\n", "", indent = indent)?;
                    out(cond, f, indent1)?;
                    out(then, f, indent1)?;
                    out(else_, f, indent1)
                }
                ExprNode::Op2 {
                    op, rand1, rand2, ..
                } => {
                    write!(f, "{:indent$}{}\n", "", op, indent = indent)?;
                    out(rand1, f, indent1)?;
                    out(rand2, f, indent1)
                }
                ExprNode::App { name, args, .. } => {
                    write!(f, "{:indent$}{}\n", "", name, indent = indent)?;
                    for arg in args {
                        out(arg, f, indent1)?;
                    }
                    Ok(())
                }
                ExprNode::Int { value, .. } => {
                    write!(f, "{:indent$}Int({})\n", "", value, indent = indent)
                }
                ExprNode::Id { id, .. } => {
                    write!(f, "{:indent$}Id(\"{}\")\n", "", id, indent = indent)
                }
                _ => write!(f, "{:indent$}UNIMPLEMENTED\n", "", indent = indent),
            }
        }
        out(self, f, 0)
    }
}

/********************** Recursive-Descent Parser ***********************/

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}

//type ParseResult = Result<ExprNode, ParseError>;
type ParseResult = Result<ExprNode, ParseError>;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    tok: &'a Token,
    next_index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens) -> Parser<'a> {
        let tokens_vec = tokens.tokens();
        let tok = tokens_vec.get(0).unwrap();
        Self {
            tokens: tokens_vec,
            tok,
            next_index: 1,
        }
    }

    fn peek(&self, kind: &str) -> bool {
        self.tok.kind() == kind
    }

    fn consume(&mut self, kind: &str) -> Result<(), ParseError> {
        if self.peek(kind) {
            self.tok = self.tokens.get(self.next_index).unwrap();
            self.next_index += 1;
            Ok(())
        } else {
            let msg = format!(
                "{}: expected \"{}\" at \"{}\"",
                self.tok.coord(),
                kind,
                self.tok.lexeme()
            );
            Err(ParseError(msg))
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.expr()
    }

    fn expr(&mut self) -> ParseResult {
        let tok = self.tok;
        match tok.kind() {
            "lambda" => self.lambda_expr(),
            "let" => self.let_expr(LetKind::Parallel),
            "let*" => self.let_expr(LetKind::Sequential),
            "letrec" => self.let_expr(LetKind::Recursive),
            "if" => self.if_expr(),
            _ => self.logical_expr(),
        }
    }

    fn lambda_expr(&mut self) -> ParseResult {
        let tok = self.tok;
        let coord = tok.coord();
        self.consume("lambda")?;
        self.consume("(")?;
        let mut params = Vec::new();
        if self.peek("ID") {
            params.push(self.tok.lexeme().to_string());
            self.consume("ID")?;
            while self.peek(",") {
                self.consume(",")?;
                params.push(self.tok.lexeme().to_string());
                self.consume("ID")?;
            }
        }
        self.consume(")")?;
        let body = Box::new(self.expr()?);
        let node = ExprNode::Lambda {
            params,
            body,
            coord,
        };
        Ok(node)
    }

    fn let_expr(&mut self, let_kind: LetKind) -> ParseResult {
        let tok = self.tok;
        self.consume(tok.kind())?; //past let, let* or letrec
        let mut defs = Vec::new();
        defs.push(self.def()?);
        while self.peek("ID") {
            defs.push(self.def()?);
        }
        self.consume("in")?;
        let expr = Box::new(self.expr()?);
        let node = ExprNode::Let {
            let_kind,
            defs,
            expr,
            coord: tok.coord(),
        };
        Ok(node)
    }

    fn def(&mut self) -> ParseResult {
        let id = self.tok.lexeme().to_string();
        self.consume("ID")?;
        let coord = self.tok.coord();
        self.consume("=")?;
        let expr = Box::new(self.expr()?);
        let node = ExprNode::Def { id, expr, coord };
        Ok(node)
    }

    fn if_expr(&mut self) -> ParseResult {
        let tok = self.tok;
        assert!(tok.kind() == "if");
        self.consume("if")?;
        let cond = Box::new(self.expr()?);
        self.consume("then")?;
        let then = Box::new(self.expr()?);
        self.consume("else")?;
        let else_ = Box::new(self.expr()?);
        let node = ExprNode::If {
            cond,
            then,
            else_,
            coord: tok.coord(),
        };
        Ok(node)
    }

    /* Reduce logical operators to if:
        A && B => if A if B 1 else 0 else 0
        A || B => if A 1 else if B 1 else 0
        !A => if A 0 else 1
    */

    fn logical_expr(&mut self) -> ParseResult {
        let mut e = self.not_expr()?;
        while self.peek("&&") || self.peek("||") {
            let tok = self.tok;
            self.consume(tok.kind())?;
            let e1 = self.not_expr()?;
            if tok.kind() == "&&" {
                e = Self::and_node(e, e1, tok);
            } else {
                e = Self::or_node(e, e1, tok);
            }
        }
        Ok(e)
    }

    fn not_expr(&mut self) -> ParseResult {
        if self.peek("!") {
            let tok = self.tok;
            self.consume("!")?;
            let r = self.rel_expr()?;
            let node = ExprNode::If {
                cond: Box::new(r),
                then: Box::new(ExprNode::Int {
                    value: 0,
                    coord: tok.coord(),
                }),
                else_: Box::new(ExprNode::Int {
                    value: 1,
                    coord: tok.coord(),
                }),
                coord: tok.coord(),
            };
            Ok(node)
        } else {
            self.rel_expr()
        }
    }

    /* reduce rel-ops to simply < and ==
       A <= B => if A < B 1 else if A == B 1 else 0
       A > B => B < A
       A >= B => B <= A
       A != B => if A == B 0 else 1
    */

    fn rel_expr(&mut self) -> ParseResult {
        let mut e = self.arith_expr()?;
        let is_rel_op = self.peek("<")
            || self.peek("<=")
            || self.peek(">")
            || self.peek(">=")
            || self.peek("==")
            || self.peek("!=");
        if is_rel_op {
            let tok = self.tok;
            self.consume(tok.kind())?;
            let e1 = self.arith_expr()?;
            match tok.kind() {
                "<" => e = Self::lt_node(e, e1, tok),
                "<=" => e = Self::le_node(e, e1, tok),
                ">" => e = Self::lt_node(e1, e, tok),
                ">=" => e = Self::le_node(e1, e, tok),
                "==" => e = Self::eq_node(e, e1, tok),
                "!=" => e = Self::ne_node(e, e1, tok),
                &_ => panic!("unhandled rel-op {}", tok.kind()),
            }
        }
        Ok(e)
    }

    fn arith_expr(&mut self) -> ParseResult {
        let mut t = self.arith_term()?;
        while self.peek("+") || self.peek("-") {
            let tok = self.tok;
            self.consume(tok.kind())?;
            let t1 = self.arith_term()?;
            t = ExprNode::Op2 {
                op: tok.kind().to_string(),
                rand1: Box::new(t),
                rand2: Box::new(t1),
                coord: tok.coord(),
            };
        }
        Ok(t)
    }

    fn arith_term(&mut self) -> ParseResult {
        let mut f = self.arith_factor()?;
        while self.peek("*") || self.peek("/") || self.peek("%") {
            let tok = self.tok;
            self.consume(tok.kind())?;
            let f1 = self.arith_factor()?;
            f = ExprNode::Op2 {
                op: tok.kind().to_string(),
                rand1: Box::new(f),
                rand2: Box::new(f1),
                coord: tok.coord(),
            };
        }
        Ok(f)
    }

    fn arith_factor(&mut self) -> ParseResult {
        let tok = self.tok;
        if tok.kind() == "-" {
            //let coord = &self.tok.coord();
            self.consume("-")?;
            let node = ExprNode::Op2 {
                op: tok.kind().to_string(),
                rand1: Box::new(ExprNode::Int {
                    value: 0,
                    coord: tok.coord(),
                }),
                rand2: Box::new(self.arith_factor()?),
                coord: tok.coord(),
            };
            Ok(node)
        } else if tok.kind() == "INT" {
            let value: Value = tok.lexeme().parse().unwrap();
            let node = ExprNode::Int {
                value,
                coord: tok.coord(),
            };
            self.consume("INT")?;
            Ok(node)
        } else if tok.kind() == "ID" {
            let id = tok.lexeme().to_string();
            self.consume("ID")?;
            if self.peek("(") {
                //function application
                let mut args = Vec::new();
                self.consume("(")?;
                if !self.peek(")") {
                    let arg = self.expr()?;
                    args.push(arg);
                    while self.peek(",") {
                        self.consume(",")?;
                        let arg = self.expr()?;
                        args.push(arg);
                    }
                }
                self.consume(")")?;
                let node = ExprNode::App {
                    name: id,
                    args,
                    coord: tok.coord(),
                };
                Ok(node)
            } else {
                let node = ExprNode::Id {
                    id,
                    coord: tok.coord(),
                };
                Ok(node)
            }
        } else if tok.kind() == "(" {
            self.consume("(")?;
            let node = self.expr()?;
            self.consume(")")?;
            Ok(node)
        } else {
            Ok(DUMMY_NODE)
        }
    }
}

/************************ Parser Node Utilities ************************/

impl<'a> Parser<'a> {
    fn and_node(e1: ExprNode, e2: ExprNode, tok: &Token) -> ExprNode {
        let e2_if = ExprNode::If {
            cond: Box::new(e2),
            then: Box::new(ExprNode::Int {
                value: 1,
                coord: tok.coord(),
            }),
            else_: Box::new(ExprNode::Int {
                value: 0,
                coord: tok.coord(),
            }),
            coord: tok.coord(),
        };
        ExprNode::If {
            cond: Box::new(e1),
            then: Box::new(e2_if),
            else_: Box::new(ExprNode::Int {
                value: 0,
                coord: tok.coord(),
            }),
            coord: tok.coord(),
        }
    }

    fn or_node(e1: ExprNode, e2: ExprNode, tok: &Token) -> ExprNode {
        let e2_if = ExprNode::If {
            cond: Box::new(e2),
            then: Box::new(ExprNode::Int {
                value: 1,
                coord: tok.coord(),
            }),
            else_: Box::new(ExprNode::Int {
                value: 0,
                coord: tok.coord(),
            }),
            coord: tok.coord(),
        };
        ExprNode::If {
            cond: Box::new(e1),
            then: Box::new(ExprNode::Int {
                value: 1,
                coord: tok.coord(),
            }),
            else_: Box::new(e2_if),
            coord: tok.coord(),
        }
    }

    fn eq_node(e1: ExprNode, e2: ExprNode, tok: &Token) -> ExprNode {
        ExprNode::Op2 {
            op: "==".to_string(),
            rand1: Box::new(e1),
            rand2: Box::new(e2),
            coord: tok.coord(),
        }
    }

    fn ne_node(e1: ExprNode, e2: ExprNode, tok: &Token) -> ExprNode {
        let eq_node = ExprNode::Op2 {
            op: "==".to_string(),
            rand1: Box::new(e1),
            rand2: Box::new(e2),
            coord: tok.coord(),
        };
        ExprNode::If {
            cond: Box::new(eq_node),
            then: Box::new(ExprNode::Int {
                value: 0,
                coord: tok.coord(),
            }),
            else_: Box::new(ExprNode::Int {
                value: 1,
                coord: tok.coord(),
            }),
            coord: tok.coord(),
        }
    }

    fn lt_node(e1: ExprNode, e2: ExprNode, tok: &Token) -> ExprNode {
        ExprNode::Op2 {
            op: "<".to_string(),
            rand1: Box::new(e1),
            rand2: Box::new(e2),
            coord: tok.coord(),
        }
    }

    fn le_node(e1: ExprNode, e2: ExprNode, tok: &Token) -> ExprNode {
        let lt_node = ExprNode::Op2 {
            op: "<".to_string(),
            rand1: Box::new(e1.clone()),
            rand2: Box::new(e2.clone()),
            coord: tok.coord(),
        };
        let eq_node = ExprNode::Op2 {
            op: "==".to_string(),
            rand1: Box::new(e1),
            rand2: Box::new(e2),
            coord: tok.coord(),
        };
        let if_eq = ExprNode::If {
            cond: Box::new(eq_node),
            then: Box::new(ExprNode::Int {
                value: 1,
                coord: tok.coord(),
            }),
            else_: Box::new(ExprNode::Int {
                value: 0,
                coord: tok.coord(),
            }),
            coord: tok.coord(),
        };
        ExprNode::If {
            cond: Box::new(lt_node),
            then: Box::new(ExprNode::Int {
                value: 1,
                coord: tok.coord(),
            }),
            else_: Box::new(if_eq),
            coord: tok.coord(),
        }
    }
}
