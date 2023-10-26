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

### String parsing


### Arithmetic operations
- +: ( x y -- x_plus_y ) - addition

- -: ( x y -- x_minus_y ) - subtraction

- *: ( x y -- mul ) - multiplication

- /: ( x y -- fdivision ) - floating point division

- div: ( x y -- integer_division ) - integer division

- <: ( x y -- bool) checks if x < y, and puts true or false on the stack

- **>**: ( x y -- bool) checks if x > y, and puts true or false on the stack

- ==: ( x y -- bool ) checks if x == y and puts true or false on the stack

### Logical operations
- True: - literal

- False: - literal

- &&: ( x y -- bool ) - logical AND

- ||: ( x y -- bool ) - logical OR

- not: ( x -- bool ) - logical NOT. I've implemented it such that it also works like a negation on numbers, so, if you call with 6 the program will put -6 on top of the stack.


### Lists
Lists are delimited by square brackets and seperated by commas. I have implemented them so they have to be of same type.
```
push [1,2,3,4,5,6,7,8,9,10]
```

Operations possible for lists:
- head: ( list -- item ) takes a list and returns its head

- tail: ( list -- tail ) takes a list and returns the tail

- empty: ( list -- bool ) takes a list and returns true if the list is empty

- length: ( list -- len ) puts the length of a given list onto the stack

- cons: ( item list -- list ) appends the item in front of the list

- append: ( list1 list2 -- list3 ) concatenates both lists

- map quotation ( list -- newlist ) takes a list, and a block, and executes the block on each of the elements of the list, forming a new list that is put on the stack. E.g. [1,2,3] map {10,:* } will result in a list [10,20,30]


### Quotations 
Quotations are code blocks stored within {curly brackets}, these can consist of every possible type and is used to store an block of code for later execution.
Example of pushing a code block:
```
push {2,:*,2}
//The stack will now contain
Stack: [Quotation([Int(2), Symbol("*"), Int(2)])]
```
For example executing the following block {1,:+} will increment the top element on the stack by 1.


To execute the quotation call the "exec" function which executes the quotation laying on the stack, an example to multiple 2 by 6:
```
> push {2,6,:*}
Stack: [Quotation([Int(2), Int(6), Symbol("*")])]
> exec
Stack: [Int(12)]
```

### Variables (assignments to a symbol)
Assignment := takes two arguments, left hand side must be a symbol (aka variable), and right hand side can be any value different from a symbol, eg. number, bool, list or code_block. It assigns a value to a variable using ':='
For example ":a "This is a text := :a eval print"
Outputs: This is a text

Or you can push it then later call it. Then the interpreter will locate the variable value and use it.
For example ":a "This is a text := :a"
Then later use it "a print"
Outputs: This is a text

### Extra features
I took inspiration for some of haskell's list operations and decided to added the feauture of creating list ranges for fast list creation.
For example:
[1..10] = 1,2,3,4,5,6,7,8,9,10
This feature is as of now limited to int's as it's not a priority for the language right now.

Added an information feature. It is recognized with '?' following a char, so
?s = prints stack
?h = prints all helpful commands (those starting with ?)
?q = quits service

### Execution
It can either be used as a GHCI like service where you can type commands and the program will interpret and execute the commands and spit the result. This will be done by running the program without any file specified:
"./main.exe"

You can also input an entire file and the program will interpret it calculate it and finish when everything is executed. This is done by running the program with a file specified:
"./main.exe file_of_choice.txt"