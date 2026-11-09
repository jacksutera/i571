module Exprs where

{-
Contains predefined prj2 expression.  Some are used
as sub-expressions within other expressions.  Unless otherwise
noted, the expressions having suffixes "_d" or "_dd" (where
'd' denotes a decimal digit) are self-contained expressions
which should evaluate to that value.
-}

import Prj2LangTypes
import Prj3Sol

c0 = Coord { path = "test", lineN = 22, colN = 33 }

int_0 = Int_ { value = 0, coord = c0 }
int_1 = Int_ { value = 1, coord = c0 }

int_2 = Int_ { value = 2, coord = c0 }
int_4 = Int_ { value = 4, coord = c0 }

neg_1 = Int_ { value = -1, coord = c0 }

op2_6 = Op2 { op = "+", rand1 = int_2, rand2 = int_4, coord = c0 }
op2_24 = Op2 { op = "*", rand1 = op2_6, rand2 = int_4, coord = c0 }

if_24 = If { cond = int_1, then_ = op2_24, else_ = op2_6, coord=c0 }
if_6 = If { cond = int_0, then_ =op2_24, else_ = op2_6, coord=c0 }

id_a = Id { id_ = "a", coord = c0 }
id_b = Id { id_ = "b", coord = c0 }
id_n = Id { id_ = "n", coord = c0 }
id_z = Id { id_ = "z", coord = c0 }

add_a_b = Op2 { op = "+", rand1 = id_a, rand2 = id_b, coord = c0 }

inc_a = Op2 { op = "+", rand1 = id_a, rand2 = int_1, coord = c0 }

f_inc = Lambda { params = ["a"], body = inc_a, coord = c0 }

let_26 = Let {
    letKind = Parallel,
    defs = [ Def { id_ = "a", expr = int_2, coord = c0 },
             Def { id_ = "b", expr = if_24, coord = c0 }
           ],
    expr = add_a_b,
    coord = c0
  }

let_13 = Let {
    letKind = Sequential,
    defs = [ Def { id_ = "a", expr = if_6, coord = c0 },
             Def { id_ = "b", expr = inc_a, coord = c0 }
           ],
    expr = add_a_b,
    coord = c0
  }

let_9 = Let {
    letKind = Recursive,
    defs = [ Def { id_ = "b", expr = inc_a, coord = c0 },
             Def { id_ = "a", expr = int_4, coord = c0 }
           ],
    expr = add_a_b,
    coord = c0
  }

-- these two are not self-contained
lt_n_2 = Op2 { op = "<", rand1 = id_n, rand2 = int_2, coord = c0 }
sub_n_1 = Op2 { op = "+", rand1 = id_n, rand2 = neg_1, coord = c0 }

fact_app = App { name = "fact", args = [sub_n_1], coord = c0 }
fact_mul = Op2 { op = "*", rand1 = id_n, rand2 = fact_app, coord = c0 }
fact_if = If { cond = lt_n_2, then_ = int_1, else_ = fact_mul, coord = c0 }
fact_lambda = Lambda { params = ["n"], body = fact_if, coord = c0 }

fact_24 = Let {
    letKind = Recursive,
    defs = [ Def { id_ = "fact", expr = fact_lambda, coord = c0 } ],
    expr = App { name = "fact", args = [int_4], coord = c0 },
    coord = c0
  }
  
