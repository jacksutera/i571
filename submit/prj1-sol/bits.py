import re
import sys
from collections import namedtuple
import json

'''
program
  : expr*
  ;
expr
  : shift_ex
  ;
shift_ex
  : or_ex ( ( '<<' | '>>' ) or_ex )?
  ;
or_ex
  : xor_ex ( ( '|' | '&' ) xor_ex )*
  ;
xor_ex
  : unary ( '^' xor_ex)?
  ;
unary
  : '~' unary
  | prim
  ;
prim
  : int
  | '(' expr ')'
  ;
int
  : digit ( '_'? digit )*
  | ( '0x' | '0X' ) hex ( '_'? hex )*
  ;
digit
  : \d
  ;
hex
  : digit
  | [a-f]
  | [A-F]
  ;
'''

### PARSER ###

#MODELED OFF OF PYTHON EXAMPLE
def parse(text):
    toks = tokenize(text)
    toksIndex = 0
    tok = toks[toksIndex]
    toksIndex += 1

    def program(asts):
        if peek('EOF'):
            return asts
        else:
            e = expr()
            asts.append(e)
            return program(asts)

    def expr():
        return shift_ex()

    def shift_ex():
        o = or_ex()
        if peek('<<') or peek('>>'):
            op = tok.kind
            consume(op)
            o1 = or_ex()
            o = {'op': op, 'operand1': o, 'operand2': o1}
        return o

    def or_ex():
        x = xor_ex()
        while peek('|') or peek('&'):
            op = tok.kind
            consume(op)
            x1 = xor_ex()
            x = {'op': op, 'operand1': x, 'operand2': x1}
        return x

    def xor_ex():
        u = unary()
        if peek('^'):
            consume('^')
            x = xor_ex()
            return {'op': '^', 'operand1': u, 'operand2': x}
        return u

    def unary():
        if peek('~'):
            consume('~')
            u = unary()
            return {'op': '~', 'operand1': u}
        else:
            return prim()

    def prim():
        if peek('INT'):
            v = parse_int(tok.lexeme)
            consume('INT')
            return v
        else:
            consume('(')
            e = expr()
            consume(')')
            return e

    def parse_int(lexeme):
        s = lexeme.replace('_', '')
        if s[:2] in ('0x', '0X'):
            return int(s, 16)
        return int(s, 10)

    def peek(kind):
        nonlocal tok
        return tok.kind == kind

    def consume(kind):
        nonlocal tok, toks, toksIndex
        if (peek(kind)):
            tok = toks[toksIndex]
            toksIndex+=1
        else:
            error(kind, text)

    def error(kind, test):
        nonlocal tok
        pos = tok.pos
        if pos >= len(text) or text[pos] =='\n': pos -= 1
        lineBegin = text.rfind('\n', 0, pos)
        if lineBegin < 0: lineBegin=0
        lineEnd = text.find('\n', pos)
        if lineEnd < 0: lineEnd = len(text)
        line = text[lineBegin:lineEnd]
        print(f"error: expecting '{kind}' but got '{tok.kind}'", file=sys.stderr)
        sys.exit(1)

    
    asts = [];
    program(asts)
    if tok.kind != 'EOF': error('EOF', text)
    return asts



### LEXER ###

#MODELED OFF OF PYTHON EXAMPLE
#skip whitespace
SKIP_RE = re.compile(r'(\s|//[^\n]*)+')
#integer recognizer. inlcuding hex integers. including internal underscores
INT_RE = re.compile(r'0[xX][0-9a-fA-F](_?[0-9a-fA-F])*|\d(_?\d)*')

#Token structure
Token = namedtuple('Token', 'kind lexeme pos')

def tokenize(text, pos=0):
    toks = []
    while pos < len(text):
        m = SKIP_RE.match(text, pos)
        if m:
            pos += len(m.group())
        if pos >= len(text): break
        #Check for integer
        if m := INT_RE.match(text, pos):
            tok = Token('INT', m.group(), pos)
        #next check for two-character shift operators
        elif text[pos:pos+2] in ('<<', '>>'):
            s = text[pos:pos+2]
            tok = Token(s, s, pos)
        #otherwise just another operator
        else:
            tok = Token(text[pos], text[pos], pos)
        pos += len(tok.lexeme)
        toks.append(tok)
    toks.append(Token('EOF', '<EOF>', pos))
    return toks

### Main ###
def main():
    text = sys.stdin.read()
    #print(tokenize(text))
    asts = parse(text)
    print(json.dumps(asts, separators=(',', ':')))

if __name__ == "__main__":
    main()
