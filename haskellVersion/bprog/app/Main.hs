module Main (main) where

import System.IO (hFlush, stdout)
import Control.Monad (foldM)
import Data.List (isPrefixOf, isSuffixOf)
import System.Environment (getArgs)
import Text.Read (readMaybe)
import System.Directory (doesFileExist)

--the stack can have the following types
data StackValue = StackInt Int | StackFloat Float | StackString String | StackBool Bool | StackList [StackValue] | StackQuotation String

--how it is displayed when printing the stack
instance Show StackValue where
  show (StackInt i) = show i
  show (StackFloat f) = show f
  show (StackString s) = "\"" ++ s ++ "\""
  show (StackBool b) = show b
  show (StackList l) = show l
  show (StackQuotation q) = "{" ++ q ++ "}"

--the stack is a list of stack values stored in it
type Stack = [StackValue]

--x is the top element and xs is the rest of the stack so two identical elements on the top of the stack
dup :: Stack -> Stack
dup [] = []
dup (x:xs) = x:x:xs

--swaps top two elements
swap :: Stack -> Stack
swap [] = []
swap [x] = [x]
swap (x:y:xs) = y:x:xs

--don't matter the element remove it from the rest of the stack
pop :: Stack -> Stack
pop [] = []
pop (_:xs) = xs

--takes a value on top of stack 
push :: StackValue -> Stack -> Stack
push x xs = x:xs

parseInteger :: Stack -> Either String Stack
--if nothing is inputted after nothing is done
parseInteger [] = Right []
--if string 
parseInteger (StackString s : xs) =
  --dependent on if it goes well transforms the string to int or error
  case readMaybe s :: Maybe Int of
    Just n -> Right (StackInt n : xs)
    Nothing -> Left "Error: Unable to parse string as an integer."
--nothing was inputted just return the stack
parseInteger stack = Right stack

--same as above only with float instead
parseFloat :: Stack -> Either String Stack
parseFloat [] = Right []
parseFloat (StackString s : xs) =
  case readMaybe s :: Maybe Float of
    Just n -> Right (StackFloat n : xs)
    Nothing -> Left "Error: Unable to parse string as a float."
parseFloat stack = Right stack

--common declaration since they all have the same one
add, sub, mul, fdivision, integerDivision :: Stack -> Either String Stack

--in add, sub, mul and fdivision both values must be of same type
add (StackInt x : StackInt y : xs) = Right (StackInt (x + y) : xs)
add (StackFloat x : StackFloat y : xs) = Right (StackFloat (x + y) : xs)
add _ = Left "Error: Unable to perform addition."

sub (StackInt x : StackInt y : xs) = Right (StackInt (x - y) : xs)
sub (StackFloat x : StackFloat y : xs) = Right (StackFloat (x - y) : xs)
sub _ = Left "Error: Unable to perform subtraction."

mul (StackInt x : StackInt y : xs) = Right (StackInt (x * y) : xs)
mul (StackFloat x : StackFloat y : xs) = Right (StackFloat (x * y) : xs)
mul _ = Left "Error: Unable to perform multiplication."

fdivision (StackInt x : StackInt y : xs) = Right (StackFloat (fromIntegral x / fromIntegral y) : xs)
fdivision (StackFloat x : StackFloat y : xs) = Right (StackFloat (x / y) : xs)
fdivision _ = Left "Error: Unable to perform float division."

--only works on both integers
integerDivision (StackInt x : StackInt y : xs) = Right (StackInt (div x y) : xs)
integerDivision _ = Left "Error: Unable to perform integer division."

--the interpreter of the input
processCommand :: String -> Stack -> IO Stack
--based on string input it reacts on the functionality call which is inputted
processCommand "?" stack = do
  putStrLn "Available commands:"
  putStrLn "?s - prints stack"
  putStrLn "?h - prints all functions"
  putStrLn "?q - quit"
  return stack
processCommand "?s" stack = do
  putStrLn $ "Current stack: " ++ show stack
  return stack
processCommand "?h" stack = do
  putStrLn "Available commands:"
  putStrLn "?s - prints stack"
  putStrLn "?h - prints all functions"
  putStrLn "?q - quit"
  putStrLn "dup - duplicate the top element of the stack"
  putStrLn "swap - swap the top two elements of the stack"
  putStrLn "pop - remove the top element of the stack"
  putStrLn "print - print the top element of the stack"
  putStrLn "read - read a string from the user and push it onto the stack"
  putStrLn "parseInteger - parse a string as an integer and push it onto the stack"
  putStrLn "parseFloat - parse a string as a float and push it onto the stack"
  putStrLn "+ - pop two numbers, add them, and push the result"
  putStrLn "- - pop two numbers, subtract the second from the first, and push the result"
  putStrLn "* - pop two numbers, multiply them, and push the result"
  putStrLn "/ - pop two numbers, divide the first by the second, and push the result"
  putStrLn "div - pop two integers, divide the first by the second using integer division, and push the result"
  return stack
processCommand "dup" stack = return (dup stack)
processCommand "swap" stack = return (swap stack)
processCommand "pop" stack = return (pop stack)
processCommand "print" stack = printTop stack
processCommand "read" stack = readInput stack
processCommand "parseInteger" stack = case parseInteger stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand "parseFloat" stack = case parseFloat stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand "+" stack = case add stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand "-" stack = case sub stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand "*" stack = case mul stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand "/" stack = case fdivision stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand "div" stack = case integerDivision stack of
  Left err -> putStrLn err >> return stack
  Right newStack -> return newStack
processCommand cmd stack
  | "{" `isPrefixOf` cmd && "}" `isSuffixOf` cmd = return (push (StackQuotation (init (tail cmd))) stack)
  | "\"" `isPrefixOf` cmd && "\"" `isSuffixOf` cmd = return (push (StackString (init (tail cmd))) stack)
  | "push" `elem` words cmd = return (push (parseValue (last (words cmd))) stack)
  | cmd == "True" = return (push (StackBool True) stack)
  | cmd == "False" = return (push (StackBool False) stack)
  | otherwise = do
      let cmdWords = words cmd
      case cmdWords of
        ["push", value] -> return (push (parseValue value) stack)
        _ -> do
          putStrLn $ "Unrecognized command: " ++ cmd
          return stack

--parses different values based on their characteristics
parseValue :: String -> StackValue
parseValue s
  | all (`elem` ['0'..'9']) s = StackInt (read s)
  | all (`elem` (['0'..'9'] ++ ".")) s = StackFloat (read s)
  | "{" `isPrefixOf` s && "}" `isSuffixOf` s = StackQuotation (init (tail s))
  | "\"" `isPrefixOf` s && "\"" `isSuffixOf` s = StackString (init (tail s))
  | otherwise = StackString s

--prints top element
printTop :: Stack -> IO Stack
printTop [] = putStrLn "Error: Stack is empty." >> return []
printTop (x:xs) = print x >> return xs

--reads user input
readInput :: Stack -> IO Stack
readInput stack = do
  input <- getLine
  return (push (StackString input) stack)

--loop that constantly gets userinput until the user wants to quit
loop :: Stack -> IO ()
loop stack = do
  putStrLn $ "Current stack: " ++ show stack
  putStr "> "
  hFlush stdout
  command <- getLine
  if command == "?q"
    then putStrLn "Program quit"
    else do
      newStack <- processCommand command stack
      loop newStack

--handlers file input
executeFile :: FilePath -> IO ()
executeFile filePath = do
  fileExists <- doesFileExist filePath
  if fileExists then do
    contents <- readFile filePath
    let commands = lines contents
    finalStack <- foldM (flip processCommand) [] commands
    putStrLn $ "File executed successfully. Final stack: " ++ show finalStack
  else do
    putStrLn $ "Error executing file: " ++ filePath ++ " does not exist."
  
main :: IO ()
main = do
    args <- getArgs
    --dependent on inputted arguments, file mode or terminal mode
    case args of
        [filePath] -> executeFile filePath
        [] -> loop []
        _ -> putStrLn "Usage: stack_program [FILE]"