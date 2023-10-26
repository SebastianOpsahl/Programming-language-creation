## Overview
The program is a simple concatenative, stack-based, programming language interpreter. The interpreter is called bprog. bprog will accept instruction from standard input and execute them following the semantics of the language that we defined below.

The stack-based language evaluates expressions in postfix notation. In postfix notation, operators follow their operands, and calculations are performed using a stack to store intermediate results. To calculate the result of an expression, you push the operands onto the stack and then apply the operator. 

### The stack
Since the language is a stack-based one, all arguments to functions are stored and taken from the stack, and all results of functions are put on top of the stack. The instructions can take arbitrary number of arguments from the stack, and produce and arbitrary number of values that are put on the stack. To describe the instructions we use a notation called "stack effects", that looks like that ( left -- right ) where left are the variables that are taken from the stack (first popped value on the right-hand side), and right depicts what is put onto the stack (from left to right, meaning, the right-most element will be on top of the stack). E.g. to describe addition + we write ( x y -- z) where x and y are popped from the stack, and z is put back, on top. The stack as the following operations:
- dup ( x -- x x ) duplicates the top element on the stack

- swap ( x y -- y x ) swaps the two top elements on the stack

All literals like Integers, Floats, Bools, Strings and Lists are simply pushed onto the stack

- pop ( x -- ) removes the top element from the stack

- push ( x ++ ) adds and element to the top of the stack 

### IO
- print: (x--) takes the top element from the stack and prints it to the standard output.

- read ( -- x ) reads a line from standard input and puts it into the stack as string.

### Arithmetic operations
- +: ( x y -- x_plus_y ) - addition

- -: ( x y -- x_minus_y ) - subtraction

- *: ( x y -- mul ) - multiplication

- /: ( x y -- fdivision ) - floating point division

- div: ( x y -- integer_division ) - integer division


### Extra features
You can input "?" to get informations about usage, "?" will output the different possibilities for helpful feedback:
Added an information feature. It is recognized with '?' following a char, so
?s = prints stack
?h = prints all possible functioanilites the user can use
?q = quits service

### Execution
It can either be used as a GHCI like service where you can type commands and the program will interpret and execute the commands and spit the result. This will be done by running the program without any file specified:
"stack run"

You can also input an entire file and the program will interpret it calculate it and finish when everything is executed. This is done by running the program with a file specified:
stack exec bprog-exe -- fileName

For example:
stack exec bprog-exe -- ./example.txt
