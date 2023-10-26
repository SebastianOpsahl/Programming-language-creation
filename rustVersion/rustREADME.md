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

- each quotation ( list -- ) takes a list an a code block, and executes the code block on each of the elements of the list, eg. [1,2,3] each {print} will print three lines with 1, 2, 3 respectively in each of the lines.

- foldl quotation ( list initial_accumulator -- final_accumulator ) folds the list from left to right.  E.g. [1,2,3] 0 foldl {+} will result in 6 on top of the stack.


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

### Test coverage
The total test coverage was 78.48% with 82.43% being the code regarding the stack (stack.rs) and 20.00% for the main function. Which is rather how to handle the IO and action of the user, and connecting it to the stack program.
![](gameExample.png){height"auto" width"auto"}

### Examples with each functionality
Disclaimer: some are used as a code block together and pusing the result on the stack, and some are used on the top element of the stack. Just because one is displayed here to do one or the other doesn't mean it can only do that, all functionalities can be used in a code block, and on it's one which will be performed on the element on the top of the stack.
```
> push 1
Stack: [Int(1)]
> dup
Stack: [Int(1), Int(1)]
> pop
Stack: [Int(1)]
> push 10 swap
Stack: [Int(10), Int(1)]
> read 
This is user inputted text
Stack: [Int(10), Int(1), String("This is user inputted text")]
> print
This is user inputted text
Stack: [Int(10), Int(1)]
```

```
> 10 10 +
Stack: [Int(20)]
> 19 -
Stack: [Int(1)]
> 10 *
Stack: [Int(10)]
> 1 /
Stack: [Float(10.0)]
> push 10
Stack: [Float(10.0), Int(10)]
> 10 div
Stack: [Float(10.0), Int(1)]
```

```
Stack: [Int(10), Int(1)]
> >
Stack: [Bool(true)]
> push 1 10
Stack: [Bool(true), Int(1), Int(10)]
> >
Stack: [Bool(true), Bool(false)]
> push 10 10
Stack: [Bool(true), Bool(false), Int(10), Int(10)]
> ==
Stack: [Bool(true), Bool(false), Bool(true)]
```

```
> 10 not
Stack: [Int(-10)]
```

```
> [1,2,3] head
Stack: [Int(1)]
> [1,2,3] tail
Stack: [Int(1), Vec([Int(2), Int(3)])]
> empty
Stack: [Int(1), Bool(false)]
> [1..10] length
Stack: [Int(1), Bool(false), Int(10)]
> push [1..20]
Stack: [Int(1), Bool(false), Int(10), Vec([Int(1), Int(2), Int(3), Int(4), Int(5), Int(6), Int(7), Int(8), Int(9), Int(10), Int(11), Int(12), Int(13), Int(14), Int(15), Int(16), Int(17), Int(18), Int(19), Int(20)])]
> pop
Stack: [Int(1), Bool(false), Int(10)]
> 1 [2,3] cons
Stack: [Int(1), Bool(false), Int(10), Vec([Int(1), Int(2), Int(3)])]
> [4,5,6] [1,2,3] append
Stack: [Int(1), Bool(false), Int(10), Vec([Int(1), Int(2), Int(3), Int(4), Int(5), Int(6)])]
```

```
> push [1..20]
Stack: [Vec([Int(1), Int(2), Int(3), Int(4), Int(5), Int(6)]), Vec([Int(1), Int(2), Int(3), Int(4), Int(5), Int(6), Int(7), Int(8), Int(9), Int(10), Int(11), Int(12), Int(13), Int(14), Int(15), Int(16), Int(17), Int(18), Int(19), Int(20)])]
```

```
> [1,2,3] map {10,:*}
Stack: [Vec([Int(10), Int(20), Int(30)])]
```

```
> [1,2,3] each {10,:*}
Stack: [Int(10), Int(20), Int(30)]
```

```
> [1,2,3] 0 foldl {:+}
Stack: [Int(6)]
```

```
> push {2,6,:*}
Stack: [Quotation([Int(2), Int(6), Symbol("*")])]
> exec
Stack: [Int(12)]
```

```
Stack: []
> ?h
?s to see stack
?q to quit
> ?s
Stack: []
> ?q
//program quit
```