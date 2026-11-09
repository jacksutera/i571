{-# LANGUAGE OverloadedRecordDot #-}
{-# LANGUAGE NoFieldSelectors #-}

module Prj2LangTypes where

-- transliteration of Rust types, using Haskell conventions.
-- Use a trailing _ when identifier clashes with a Haskell reserved word
-- or Prelude function

data LetKind = Parallel | Sequential | Recursive deriving (Eq, Show)

data Coord
  = Coord { path :: String, lineN :: Int, colN :: Int }
  deriving (Eq, Show)

data Expr
  = Lambda { params :: [String], body :: Expr, coord :: Coord } 
  | If { cond :: Expr, then_ :: Expr, else_ :: Expr, coord :: Coord } 
  | Let { letKind :: LetKind, defs :: [Expr], expr :: Expr, coord :: Coord } 
  | Def { id_ :: String, expr :: Expr, coord :: Coord } 
  | App { name :: String, args :: [Expr], coord :: Coord } 
  | Op2 { op :: String, rand1 :: Expr, rand2 :: Expr, coord :: Coord } 
  | Id { id_ :: String, coord :: Coord } 
  | Int_ { value :: Int, coord :: Coord }
  deriving (Eq, Show)
