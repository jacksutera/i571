module Tests where

import Prj3Sol
import Prj2Tests


import Control.Monad (forM_)

import TestUtils

import Test.QuickCheck

-------------------------- testToSingletonLists -------------------------

testToSingletonLists = do
  putStrLn $ testHdr "toSingletonLists"
  ioTest "ints" (toSingletonLists [ 5, 7, 2 ]) [ [5], [7], [2] ]
  ioTest "chars" (toSingletonLists [ 'a', 'x', 'd' ]) [ "a", "x", "d" ]
  ioTest "empty" (toSingletonLists []) ([] :: [[Int]])

------------------------------- testListMap -----------------------------

testListMap = do
  putStrLn $ testHdr "listMap"
  ioTest "add" (listMap (+) 5 [1, 2, 3]) [6, 7, 8]
  ioTest "sub" (listMap (-) 5 [1, 2, 3]) [4, 3, 2]
  ioTest "mul" (listMap (*) 5 [1, 2, 3]) [5, 10, 15]
  ioTest "empty" (listMap (-) 5 []) []

-------------------------------- testAppend -----------------------------

propAppendIdentityLeft :: [Int] -> Bool
propAppendIdentityLeft xs = append [] xs == xs

propAppendIdentityRight :: [Int] -> Bool
propAppendIdentityRight xs = append xs [] == xs

propAppendAssoc :: [Int] -> [Int] -> [Int] -> Bool
propAppendAssoc xs ys zs = append (append xs ys) zs == append xs (append ys zs)

propAppendLength :: [Int] -> [Int] -> Bool
propAppendLength xs ys = length (append xs ys) == length xs + length ys

propAppendConcat :: [Int] -> [Int] -> Bool
propAppendConcat xs ys = append xs ys == xs ++ ys

testAppend = do
  putStrLn $ testHdr "append"
  propTest "left-identity-append" propAppendIdentityLeft
  propTest "right-identity-append" propAppendIdentityRight
  propTest "assoc-append" propAppendAssoc
  propTest "append-same-as-++" propAppendConcat
  
-------------------------------- testMember -----------------------------

testMember = do
  putStrLn $ testHdr "member"
  ioTest "ints-first" (member 5 [ 5, 7, 2 ]) True
  ioTest "ints-last" (member 2 [ 5, 7, 2 ]) True
  ioTest "ints-mid" (member 7 [ 5, 7, 2 ]) True
  ioTest "ints-fail"  (member 4 [ 5, 7, 2 ]) False
  ioTest "empty" (member 4 []) False

----------------------------- testselectNApart --------------------------

testSelectNApart = do
  putStrLn $ testHdr "selectNApart"
  ioTest "2-apart" (selectNApart 2 [0..10])  [0, 2, 4, 6, 8, 10]
  ioTest "2-apart chars" (selectNApart 2 ['a'..'z'])  "acegikmoqsuwy"
  ioTest "3-apart" (selectNApart 3 [0..20]) [0, 3, 6, 9, 12, 15, 18]
  ioTest "5-apart" (selectNApart 5 [0..21]) [0, 5, 10, 15, 20]
  ioTest "empty" (selectNApart 5 ([]::[Int])) []

------------------------------ sumListsRec ------------------------------

testSumListsRec = do
  putStrLn $ testHdr "sumListsRec"  
  ioTest "empty" (sumListsRec [] 3) 0
  ioTest "multi" 
         (sumListsRec [[1, 2, 3], [4, 5], [2, 3, 4], [1], [1, 2, 3, 4]] 3)
         15
  ioTest "none" 
         (sumListsRec [[1, 2, 3], [4, 5], [2, 3, 4], [1], [1, 2, 3, 4]] 5)
         0

---------------------------- sumListsPipe ------------------------------

testSumListsPipe = do
  putStrLn $ testHdr "sumListsPipe"  
  ioTest "empty" (sumListsPipe [] 3) 0
  ioTest "multi" 
         (sumListsPipe [[1, 2, 3], [4, 5], [2, 3, 4], [1], [1, 2, 3, 4]] 3)
         15
  ioTest "none" 
         (sumListsPipe [[1, 2, 3], [4, 5], [2, 3, 4], [1], [1, 2, 3, 4]] 5)
         0


-------------------------------- evalExpr -------------------------------

testEvalPrj2Expr = do
  putStrLn $ testHdr "evalPrj2Expr"
  forM_ prj2Tests $ \(name, Just expected, expr) -> do
    ioTest name (evalPrj2Expr expr) (Val expected)


--------------------------------- Tests ---------------------------------

-- Can mark test suites with following test statuses:
--   Only:  run only these tests and other tests marked Only.
--   Run:   run these tests when no tests are marked Only.
--   Skip:  skip these tests.
allTests = [
    (Run testToSingletonLists),
    (Run testListMap),
    (Run testAppend),
    (Run testMember),
    (Run testSelectNApart),
    (Run testSumListsRec),
    (Run testSumListsPipe),
    (Skip testEvalPrj2Expr)
  ]


main = do
  mapM_ id tests
  where
    only = onlyTests allTests
    tests = if (length only > 0) then only else runTests allTests
  
