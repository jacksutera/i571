use lazy_regex::{lazy_regex, Lazy, Regex};

use std::fmt;

struct KindRegex(&'static str, Lazy<Regex>);


static KIND_REGEXS: [KindRegex; 13] = [
    KindRegex("_IGNORE", lazy_regex!(r"^[ \t]+")),
    KindRegex("_IGNORE", lazy_regex!(r"^#.*")),
    KindRegex("_NL", lazy_regex!(r"^\n+")),
    KindRegex("let*", lazy_regex!(r"^let\*")),
    KindRegex("ID", lazy_regex!(r"^[a-zA-Z_]\w*")),
    KindRegex("INT", lazy_regex!(r"^\d+")),
    KindRegex("&&", lazy_regex!(r"^&&")),
    KindRegex("||", lazy_regex!(r"^\|\|")),
    KindRegex("<=", lazy_regex!(r"^<=")),
    KindRegex(">=", lazy_regex!(r"^>=")),
    KindRegex("==", lazy_regex!(r"^==")),
    KindRegex("!=", lazy_regex!(r"^!=")),
    KindRegex("_SINGLE_CHAR", lazy_regex!(r"^.")),    
];

static RESERVED: &[&str] = &[
    "if", "then", "else", "let", "letrec",  "in", "lambda",
];

#[derive(Debug, Clone)]
pub struct Coord {
    path: String,
    line_n: usize,
    col_n: usize,
}

impl Coord {
    // pub fn path(&self) -> &str { &self.path }
    // pub fn line_n(&self) -> usize { self.line_n }
    // pub fn col_n(&self) -> usize { self.line_n }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}:{}:{}", self.path, self.line_n, self.col_n)
    }
}

#[derive(Debug)]
pub struct Token {
    kind: String,
    lexeme: String,
    coord: Coord,
}

impl Token {
    pub fn kind(&self) -> &str { &self.kind }
    pub fn lexeme(&self) -> &str { &self.lexeme }
    pub fn coord(&self) -> Coord {
	Coord {
	    path: self.coord.path.to_string(),
	    line_n: self.coord.line_n,
	    col_n: self.coord.col_n,
	}
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}(\"{}\")@{}", self.kind, self.lexeme, self.coord)
    }
}

pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn tokens(&self) -> &Vec<Token> { &self.0 }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let s = self.0.iter()
	    .map(|t| t.to_string())
	    .collect::<Vec<_>>().join("\n");
	write!(f, "{}", s)
    }
}

    
pub fn scan(path: &str, text: &str) -> Tokens {

    let len = text.len();
    let mut text_index = 0;
    let mut toks = vec![];
    let mut line_n = 1;
    let mut col_n = 0;
    while text_index < len {
	for kr in &KIND_REGEXS {
	    if let Some(m) = kr.1.find(&text[text_index..]) {
		assert!(m.start() == 0);
		let lexeme = m.as_str();
		let mut kind = kr.0;
		if kind == "ID" {
		    if let Some(i) =
			RESERVED.iter().position(|&r| r == lexeme) {
			    kind = RESERVED[i];
			}
		};
		if kind == "_SINGLE_CHAR" { kind = lexeme; }
		if kind != "_NL" && kind != "_IGNORE" {
		    let coord = Coord { path: path.to_string(), line_n, col_n };
		    let tok = Token {
			kind: kind.to_string(),
			lexeme: lexeme.to_string(),
			coord,
		    };
		    toks.push(tok);
		}
		if kind == "_NL" {
		    line_n += m.len();
		    col_n = 0;
		}
		else {
		    col_n += m.len();
		}
		text_index += lexeme.len();
		break;
	    }
	}
    }
    toks.push(Token {
	kind: "EOF".to_string(),
	lexeme: "<EOF>".to_string(),
	coord: Coord { path: path.to_string(), line_n, col_n },
    });
    Tokens(toks)
}
