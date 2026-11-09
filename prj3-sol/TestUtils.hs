module TestUtils (
  testHdr, onlyTests, runTests, SuiteStatus(..), ioTest, propTest)
where

import Test.QuickCheck

type SuiteFn = IO ()

testHdr name =
  let
    hdrLen = 60
    len = length name + 2
    rep1 = (hdrLen - len) `div` 2
    rep2 = hdrLen - len - rep1
  in
    replicate rep1 '*' ++ " " ++ name ++ " " ++ replicate rep2 '*'

data SuiteStatus =
  Run SuiteFn |
  Skip SuiteFn |
  Only SuiteFn

onlyTests [] = []
onlyTests ((Only t):tests) = t : onlyTests tests
onlyTests (_:tests) = onlyTests tests

runTests [] = []
runTests ((Run t):tests) = t : runTests tests
runTests (_:tests) = runTests tests

testLinePrefix = "*** "

ioTest descr actual expected = do
  putStr (testLinePrefix ++ descr ++ " ")
  quickCheckWith (stdArgs {maxSuccess = 1 }) $ actual == expected

propTest descr prop = do
    putStr (testLinePrefix ++ descr ++ " ")
    quickCheck prop
