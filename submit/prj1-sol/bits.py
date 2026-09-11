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
