use std::io::{self, BufRead};
use std::collections::HashMap;
pub struct Stack {
    pub data: Vec<StackValue>,
    symbols: HashMap<String, StackValue>,
}

use crate::stackvalues::*;

//implemitation of the stack
impl Stack {
    //creates a new Stack instance
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            symbols: HashMap::new(),
        }
    }

    //operations on the stack (functionality being done on top elements)
    fn binary_op<F>(&mut self, op: F) -> Result<(), &'static str>
    where
        F: Fn(StackValue, StackValue) -> Result<StackValue, &'static str>,
    {
        //must be atleast two
        if self.data.len() < 2 {
            return Err("Not enough elements in the stack");
        }

        let right = self.pop().unwrap();
        let left = self.pop().unwrap();

        match op(left, right) {
            Ok(result) => {
                self.push(result);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    //assignment of variable
    fn assignment(&mut self) -> Result<(), &'static str> {
        if let Some(rhs) = self.pop() {
            if let Some(lhs) = self.pop() {
                //must be a symbol before the actual value
                if let StackValue::Symbol(symbol) = lhs {
                    self.symbols.insert(symbol, rhs);
                    Ok(())
                } else {
                    Err("Left hand side of assignment must be a symbol")
                }
            } else {
                Err("Stack is empty")
            }
        } else {
            Err("Stack is empty")
        }
    }

    //inserts a symbol to a value
    fn assign(&mut self, symbol: &str, value: StackValue) -> Result<(), &'static str> {
        self.symbols.insert(symbol.to_string(), value);
        Ok(())
    }

    //pushes an element onto the stack
    pub fn push(&mut self, value: StackValue) {
        self.data.push(value);
    }

    //prints the stack
    pub fn show(&self) {
        println!("Stack: {:?}", self.data);
    }

    //pops an element from the stack
    pub fn pop(&mut self) -> Option<StackValue> {
        self.data.pop()
    }

    //duplicates the top element of the stack by cloing and pushing it
    fn dup(&mut self) -> Result<(), &'static str> {
        if let Some(top) = self.data.last() {
            self.data.push(top.clone());
            Ok(())
        } else {
            Err("Stack is empty")
        }
    }

    //swap the two top elements on the stack
    fn swap(&mut self) -> Result<(), &'static str> {
        let len = self.data.len();
        if len < 2 {
            return Err("Not enough elements in the stack");
        }
        self.data.swap(len - 1, len - 2);
        Ok(())
    }

    //pops and prints the top element from the stack 
    fn print(&mut self) -> Result<(), &'static str> {
        if let Some(top) = self.pop() {
            println!("{}", top);
            Ok(())
        } else {
            Err("Stack is empty")
        }
    }

    //read a line from standard input and push it onto the stack
    fn read(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        self.push(StackValue::String(line.trim_end().to_string()));
        Ok(())
    }

    //appends the item in front of the list so item + list = list
    fn cons(&mut self) -> Result<(), &'static str> {
        //have to be two values present
        if self.data.len() < 2 {
            return Err("Not enough elements in the stack");
        }

        let list = self.pop().unwrap();
        let item = self.pop().unwrap();

        match list {
            StackValue::Vec(mut vec) => {
                //inserts at index 0
                vec.insert(0, item);
                self.push(StackValue::Vec(vec));
                Ok(())
            }
            _ => Err("The value is not a list"),
        }
    }

    //concatenates two lists
    fn append(&mut self) -> Result<(), &'static str> {
        if self.data.len() < 2 {
            return Err("Not enough elements in the stack");
        }

        let list1 = self.pop().unwrap();
        let list2 = self.pop().unwrap();

        match (list1, list2) {
            (StackValue::Vec(mut vec1), StackValue::Vec(vec2)) => {
                //extends vec1 with vec2 and pushes vec1 to the new vec
                vec1.extend(vec2);
                self.push(StackValue::Vec(vec1));
                Ok(())
            }
            _ => Err("Both values must be lists"),
        }
    }

    //method to interpret individual values such that true is the bool true, things between "" is string etc.
    fn parse_single_value(&self, input: &str) -> Result<StackValue, &'static str> {
        if let Ok(int_value) = input.parse::<i32>() {
            Ok(StackValue::Int(int_value))
        } else if input.eq_ignore_ascii_case("true") {
            Ok(StackValue::Bool(true))
        } else if input.eq_ignore_ascii_case("false") {
            Ok(StackValue::Bool(false))
        } else if input.starts_with("'") && input.ends_with("'") {
            Ok(StackValue::Symbol(input[1..input.len() - 1].to_string()))
        } else if input.starts_with('"') && input.ends_with('"') {
            Ok(StackValue::String(input[1..input.len() - 1].to_string()))
        } else if input.len() == 1 {
            Ok(StackValue::Symbol(input.to_string()))
        } else {
            println!("Unrecognized input: {}", input);
            Err("Unrecognizable type")
        }
    }

    //matches values (not method calls like the parse method but other characters)
    fn parse_value(&self, input: &str) -> Result<StackValue, &'static str> {
        //enclosed by square brackets with .. in is list ranges
        if input.starts_with('[') && input.contains("..") && input.ends_with(']') {
            //splits one the sides of the .. and checks if there two 
            let range_parts: Vec<&str> = input[1..input.len() - 1].split("..").collect();
            if range_parts.len() != 2 {
                return Err("Invalid range list format");
            }
        
            let start_value_str = range_parts[0].trim();
            let end_value_str = range_parts[1].trim();
        
            //recursivle call to find out what each of them is
            let start_value = self.parse_value(start_value_str)?;
            let end_value = self.parse_value(end_value_str)?;
        
            //if int's creates a list from start to end
            let elements = match (start_value, end_value) {
                (StackValue::Int(start), StackValue::Int(end)) => {
                    (start..=end).map(StackValue::Int).collect::<Vec<_>>()
                }
                _ => return Err("Invalid range list format"),
            };
            Ok(StackValue::Vec(elements))
        } else if let Ok(int_value) = input.parse::<i32>() {
            Ok(StackValue::Int(int_value))
        //possibility of writing float with , and .
        } else if let Ok(float_value) = input.replace(',', ".").parse::<f64>() { 
            Ok(StackValue::Float(float_value))
        } else if input.eq_ignore_ascii_case("true") {
            Ok(StackValue::Bool(true))
        } else if input.eq_ignore_ascii_case("false") {
            Ok(StackValue::Bool(false))
        //suqare brackets without .. is normal list
        } else if input.starts_with('[') && input.ends_with(']') {
            let elements_str = &input[1..input.len() - 1];
            let elements_result: Result<Vec<StackValue>, &str> = elements_str
                //design choice of me to split them by commas as a find this more concise 
                .split(',')
                //trims and parses each value in the list
                .map(|s| self.parse_value(s.trim()))
                .collect();
        
            match elements_result {
                //have to be of same type
                Ok(elements) => {
                    if elements.windows(2).all(|w| w[0].same_type(&w[1])) {
                        Ok(StackValue::Vec(elements))
                    } else {
                        Err("Lists with mixed types are not allowed")
                    }
                }
                Err(e) => Err(e),
            }
        }  //symbol starts with : //quotation is between { }
        else if input.starts_with('{') && input.ends_with('}') {
            let quotation_str = input.trim_start_matches('{').trim_end_matches('}');
            //also split by commas to stick with the design choice 
            let tokens = quotation_str.split(',');
            self.parse_quotation(tokens)
        } else if input.starts_with('"') && input.ends_with('"') {
            Ok(StackValue::String(input[1..input.len() - 1].to_string()))
        } else {
            //if not a group parsing as in this method it is most likely a single value so call that method
            self.parse_single_value(input)
        }
    }

    //parses quotation
    fn parse_quotation<'a, I>(&self, tokens: I) -> Result<StackValue, &'static str>
    where
        I: Iterator<Item = &'a str>,
    {
        //divides the tokens given and parts them into StackValue types
        let tokens: Vec<String> = tokens.map(|s| s.to_string()).collect();

        let stack_values = tokens
            .iter()
            .map(|token| self.parse_value(token))
            .collect::<Result<Vec<StackValue>, _>>()?;

        Ok(StackValue::Quotation(stack_values))
    }

    //from string to int if possible, if number in string format
    fn parse_integer(self: &mut Stack) -> Result<(), &'static str> {
        if let Some(StackValue::String(s)) = self.data.pop() {
            match s.parse::<i32>() {
                Ok(i) => {
                    self.data.push(StackValue::Int(i));
                    Ok(())
                }
                Err(_) => Err("Failed to parse integer"),
            }
        } else {
            Err("Stack is empty or top value is not a Text")
        }
    }
    
    //from stirng to float if possible, if number with . or ,
    fn parse_float(self: &mut Stack) -> Result<(), &'static str> {
        if let Some(StackValue::String(s)) = self.data.pop() {
            match s.parse::<f64>() {
                Ok(f) => {
                    self.data.push(StackValue::Float(f));
                    Ok(())
                }
                Err(_) => Err("Failed to parse float"),
            }
        } else {
            Err("Stack is empty or top value is not a Text")
        }
    }
    
    //from string to words
    fn words(self: &mut Stack) -> Result<(), &'static str> {
        if let Some(StackValue::String(s)) = self.data.pop() {
            let tokens: Vec<StackValue> = s.split_whitespace().map(|word| StackValue::String(word.to_string())).collect();
            self.data.push(StackValue::Vec(tokens));
            Ok(())
        } else {
            Err("Stack is empty or top value is not a Text")
        }
    }    
    
    //exec method to execute a quotation from the stack
    fn exec(&mut self) -> Result<(), &'static str> {
        if let Some(StackValue::Quotation(quotation)) = self.pop() {
            for stack_value in quotation {
                match stack_value {
                    StackValue::Symbol(ref symbol) => {
                        //handles symbols by executing commands based on symbol string
                        self.parse(symbol)?;
                    }
                    //pushes everything else as their type
                    StackValue::Int(value) => self.push(StackValue::Int(value)),
                    StackValue::Float(value) => self.push(StackValue::Float(value)),
                    StackValue::Bool(value) => self.push(StackValue::Bool(value)),
                    StackValue::String(value) => self.push(StackValue::String(value)),
                    StackValue::Vec(value) => self.push(StackValue::Vec(value)),
                    _ => return Err("Invalid StackValue in Quotation"),
                }
            }
            Ok(())
        } else {
            Err("Expected a Vec on the stack")
        }
    }    

    //tokenizes input by taking &str and converts it to a vector of &str
    fn tokenize(input: &str) -> Vec<&str> {
        //initializes a vector to store tokens, bool to track if we're inside quotes and index of current token's start pos
        let mut tokens = Vec::new();
        let mut in_quotes = false; 
        let mut token_start = 0; 
    
        //iterates over the input string and its character indices
        for (i, c) in input.char_indices() {
            match c {
                //if the character is a double quote, toggle the in_quotes flag
                '"' => {
                    in_quotes = !in_quotes;
                    //if we're leaving the quoted section, add the token to the vector
                    if !in_quotes {
                        tokens.push(&input[token_start..i + 1]);
                        token_start = i + 1;
                    }
                }
                //if the character is a whitespace, and we're not inside quotes
                ' ' | '\t' | '\n' | '\r' => {
                    if !in_quotes {
                        //if we have a non-empty token, add it to the vector
                        if i > token_start {
                            tokens.push(&input[token_start..i]);
                        }
                        token_start = i + 1;
                    }
                }
                //ignore other characters
                _ => {}
            }
        }
    
        //if there's a token at the end of the input, add it to the vector
        if token_start < input.len() {
            tokens.push(&input[token_start..]);
        }
    
        tokens 
    }
    
    //function to parse the if blocks
    fn parse_if_blocks(&mut self, tokens: &[String], start_index: usize) -> Result<(String, String, usize), &'static str> {
        //variables to differentiate which block which is being evaluated 
        let mut true_block = Vec::new();
        let mut false_block = Vec::new();
        let mut block_counter = 1;
        let mut index = start_index;
        let mut current_block = &mut true_block;
    
        while index < tokens.len() {
            let token = &tokens[index];
            match token.as_str() {
                "if" => {
                    current_block.push(token.clone());
                    block_counter += 1;
                }
                //pushes that you are inside a block
                "{" => {
                    current_block.push(token.clone());
                }
                //-1 for block counter because it is now outside the block
                "}" => {
                    block_counter -= 1;
                    if block_counter == 0 {
                        index += 1;
                        break;
                    } else if block_counter == 1 {
                        current_block = &mut false_block;
                    } else {
                        current_block.push(token.clone());
                    }
                }
                //if not going out or coming in of block, push content to the block
                _ => {
                    current_block.push(token.clone());
                }
            }
            index += 1;
        }
    
        if block_counter != 0 {
            return Err("Missing closing '}' for 'if' block");
        }
    
        let true_block_str = true_block.join(" ");
        let false_block_str = false_block.join(" ");
        Ok((true_block_str, false_block_str, index))
    } 

    fn process_map(&mut self) -> Result<(), &'static str> {
        //checks if there is a quotation and list, because it needs it to evaluate it
        let code_block = match self.pop() {
            Some(StackValue::Quotation(block)) => block,
            _ => return Err("Expected a code block for 'map'"),
        };
    
        let list = match self.pop() {
            Some(StackValue::Vec(l)) => l,
            _ => return Err("Expected a list for 'map'"),
        };
    
        let mut new_list = Vec::new();
        //for each element in the least apply the quotation (code) on each element in the list
        for value in list {
            self.push(value);
            self.push(StackValue::Quotation(code_block.clone()));
            self.exec()?;
            new_list.push(self.pop().unwrap());
        }
    
        //push the new list
        self.push(StackValue::Vec(new_list));
        Ok(())
    }
     
    pub fn process_each(&mut self) -> Result<(), &'static str> {
        //checks for a quotation and a lsit if not returns error
        let code_block = match self.pop() {
            Some(StackValue::Quotation(block)) => block,
            _ => return Err("Expected a code block for 'each'"),
        };
    
        let list = match self.pop() {
            Some(StackValue::Vec(l)) => l,
            _ => return Err("Expected a list for 'each'"),
        };

        //does the quotation for each element but unlike map a new list isn't created
        for value in list {
            self.push(value);
            self.push(StackValue::Quotation(code_block.clone()));
            self.exec()?;
        }
        Ok(())
    }

    fn process_foldl(&mut self) -> Result<(), &'static str> {
        //checks for a quotation
        let code_block = match self.pop() {
            Some(StackValue::Quotation(block)) => block,
            _ => return Err("Expected a code block for 'foldl'"),
        };
        
        let mut start = self.pop().unwrap();

        //checks for start
        let list = match self.pop() {
            Some(StackValue::Vec(l)) => l,
            _ => return Err("Expected a list for 'map'"),
        };

        //for every value in the list
        //foldl works as we have a default start value which the user defines, we apply this to the first element
        //then set the start to the next element so it is like a domino doing the quotation on each other
        for value in list {
            self.push(start);
            self.push(value);
            self.push(StackValue::Quotation(code_block.clone()));
            self.exec()?;
            start = self.pop().unwrap();
        }

        self.push(start);
        Ok(())
    }    
        
    //processes an if sentence
    fn process_if(&mut self, tokens: &[String], index: &mut usize) -> Result<(), &'static str> {
        *index += 1;
        //parses each if block
        let (true_block, false_block, new_index) = self.parse_if_blocks(tokens, *index)?;
        *index = new_index;
    
        //the if chooses it's block based on the bool
        let condition = match self.pop() {
            Some(StackValue::Bool(value)) => value,
            _ => return Err("Expected a boolean value for the condition"),
        };
    
        let block_to_execute = if condition { &true_block } else { &false_block };
        let prev_data = self.data.clone();
        println!("Block to execute: {}", block_to_execute);
        self.parse(block_to_execute)?;
        self.data = prev_data;
    
        Ok(())
    }

    fn push_next(&mut self, temp_self: &mut Stack, tokens: Vec<&str>, index: &mut usize) {
        *index += 1;
        match temp_self.symbols.get(tokens[index.clone()]) {
            Some(value) => temp_self.push(value.clone()),
            //if it's not a symbol, try to parse it as a value and push it onto the stack
            None => {
                let value = temp_self.parse_value(tokens[index.clone()]).unwrap();
                temp_self.push(value);
            }
        }
    }

    //parse a string and execute commands accordingly
    pub fn parse(&mut self, input: &str) -> Result<(), &'static str> {
    //based on input turns them into tokens
    let tokens = Self::tokenize(input);
    let mut temp_self = Stack::new();

    //creates temporary stack so when an error occurs it doesnt do anything with the already parsed tokens
    std::mem::swap(&mut self.data, &mut temp_self.data);

    let mut index = 0;
    //goes trough every token and matches them with function calls 
    while index < tokens.len() {
        let token = tokens[index];
            match token {
                "dup" => temp_self.dup()?,
                "if" => {
                    let tokens_as_string = tokens.iter().map(|s| s.to_string()).collect::<Vec<String>>();
                    temp_self.process_if(&tokens_as_string, &mut index)?;
                }
                "swap" => temp_self.swap()?,
                ":=" => temp_self.assignment()?,
                "pop" => {
                    temp_self.pop();
                }            
                "parseInteger" => temp_self.parse_integer()?,
                "parseFloat" => temp_self.parse_float()?,
                "words" => temp_self.words()?,
                "print" => temp_self.print()?,
                "read" => temp_self.read().map_err(|_| "Failed to read from stdin")?,
                "exec" => temp_self.exec()?,
                "map" => {
                    self.push_next(&mut temp_self, tokens.clone(), &mut index);
                    temp_self.process_map()?
                },
                "each" => {
                    self.push_next(&mut temp_self, tokens.clone(), &mut index);
                    temp_self.process_each()?
                },
                "foldl" => {
                    self.push_next(&mut temp_self, tokens.clone(), &mut index);
                    temp_self.process_foldl()?;
                },
                //needs a value after push to work
                "push" => {
                    if index + 1 < tokens.len() {
                        let value = tokens[index + 1];
                        let stack_value = temp_self.parse_value(value)?;
                        temp_self.push(stack_value);
                        index += 1;
                    } else {
                        return Err("Missing value after 'push'");
                    }
                }
                //arithemtic operations ensures that the types are compatible
                "+" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => Ok(StackValue::Int(x + y)),
                    (StackValue::Float(x), StackValue::Float(y)) => Ok(StackValue::Float(x + y)),
                    _ => Err("Mismatched types for addition"),
                })?,
                "-" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => Ok(StackValue::Int(x - y)),
                    (StackValue::Float(x), StackValue::Float(y)) => Ok(StackValue::Float(x - y)),
                    _ => Err("Mismatched types for subtraction"),
                })?,
                "*" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => Ok(StackValue::Int(x * y)),
                    (StackValue::Float(x), StackValue::Float(y)) => Ok(StackValue::Float(x * y)),
                    _ => Err("Mismatched types for multiplication"),
                })?,
                "/" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => {
                        //doesn't allow division by zero because it isn't possible
                        if y == 0 {
                            Err("Division by zero")
                        } else {
                            Ok(StackValue::Float(x as f64 / y as f64))
                        }
                    }
                    (StackValue::Float(x), StackValue::Float(y)) => {
                        if y == 0.0 {
                            Err("Division by zero")
                        } else {
                            Ok(StackValue::Float(x / y))
                        }
                    }
                    _ => Err("Mismatched types for floating point division"),
                })?,
                "div" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => {
                        if y == 0 {
                            Err("Division by zero")
                        } else {
                            Ok(StackValue::Int(x / y))
                        }
                    }
                    _ => Err("Mismatched types for integer division"),
                })?,
                "<" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => Ok(StackValue::Bool(x < y)),
                    (StackValue::Float(x), StackValue::Float(y)) => Ok(StackValue::Bool(x < y)),
                    _ => Err("Mismatched types for '<' comparison"),
                })?,
                ">" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Int(x), StackValue::Int(y)) => Ok(StackValue::Bool(x > y)),
                    (StackValue::Float(x), StackValue::Float(y)) => Ok(StackValue::Bool(x > y)),
                    _ => Err("Mismatched types for '>' comparison"),
                })?,
                "&&" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Bool(x), StackValue::Bool(y)) => Ok(StackValue::Bool(x && y)),
                    _ => Err("Mismatched types for logical AND"),
                })?,
                "||" => temp_self.binary_op(|left, right| match (left, right) {
                    (StackValue::Bool(x), StackValue::Bool(y)) => Ok(StackValue::Bool(x || y)),
                    _ => Err("Mismatched types for logical OR"),
                })?,
                //needs two values to compare them
                "==" => {
                    let rhs = temp_self.pop().ok_or("No value on the stack")?;
                    let lhs = temp_self.pop().ok_or("No value on the stack")?;
                    let comparison_result = lhs == rhs;
                    temp_self.push(StackValue::Bool(comparison_result));
                }
                "not" => {
                    if let Some(value) = temp_self.pop() {
                        match value {
                            //only works for int float and bool
                            StackValue::Int(x) => temp_self.push(StackValue::Int(-x)),
                            StackValue::Float(x) => temp_self.push(StackValue::Float(-x)),
                            StackValue::Bool(x) => temp_self.push(StackValue::Bool(!x)),
                            _ => return Err("Invalid type for negation or logical NOT"),
                        }
                    } else {
                        return Err("Stack is empty");
                    }
                },
                "head" => {
                    if let Some(value) = temp_self.pop() {
                        let head = value.head()?;
                        temp_self.push(head);
                    } else {
                        return Err("Stack is empty");
                    }
                }
                "tail" => {
                    if let Some(value) = temp_self.pop() {
                        let tail = value.tail()?;
                        temp_self.push(tail);
                    } else {
                        return Err("Stack is empty");
                    }
                }
                "empty" => {
                    if let Some(value) = temp_self.pop() {
                        let is_empty = value.is_empty()?;
                        temp_self.push(is_empty);
                    } else {
                        return Err("Stack is empty");
                    }
                }
                "length" => {
                    if let Some(value) = temp_self.pop() {
                        let length = value.length()?;
                        temp_self.push(length);
                    } else {
                        return Err("Stack is empty");
                    }
                }
                "cons" => temp_self.cons()?,
                "append" => temp_self.append()?,
                _ => {
                    //possible assignment
                    if token.contains(" ") {
                        let parts: Vec<&str> = token.split(" ").collect();
                        //checks the syntax for assignment, three parts and a := in the middle
                        if parts.len() == 3 && parts[1] == ":=" {
                            //sets the first to be symbol (variable) and second to be value, [1] is the :=
                            let symbol = parts[0];
                            let value_str = parts[2];
        
                            //assings the symbol to the value
                            let value = temp_self.parse_value(value_str)?;
                            temp_self.assign(symbol, value)?;
                        } else {
                            return Err("Invalid assignment syntax");
                        }
                    //if no space it can either be a symbol or a value
                    } else { 
                        //checks if the token is a symbol by looking it up in the symbols hashmap, and push the associated value onto the stack if it is
                        match temp_self.symbols.get(token) {
                            Some(value) => temp_self.push(value.clone()),
                            //if it's not a symbol, try to parse it as a value and push it onto the stack
                            None => {
                                let value = temp_self.parse_value(token)?;
                                temp_self.push(value);
                            }
                        }
                    }
                }
            }
            //next token
            index += 1;
        }
        //if no error has occured, we can safely push it to the actual stack 
        std::mem::swap(&mut self.data, &mut temp_self.data);
    Ok(())
    }   

}