use std::io::{self, Write};
use std::env;
use std::fs;

//imports stack.rs to use it's public functionalities
mod stackvalues;
mod stack;
use crate::stack::*;

fn print_commands(){
    println!("?s to see stack\n?q to quit");
    println!("dup ( x -- x x )");
    println!("swap ( x y -- y x )");
    println!("pop ( x -- ) ");
    println!("push ( x ++ )");
    println!("print: (x--)");
    println!("read: (--x)");
    println!("+ ( x y -- x_plus_y )");
    println!("- ( x y -- x_minus_y )");
    println!("* ( x y -- mul )");
    println!("/ ( x y -- fdivision )");
    println!("div ( x y -- integer_division )");
    println!("< ( x y -- bool)");
    println!("> ( x y -- bool)");
    println!("== ( x y -- bool )");
    println!("True - literal");
    println!("False - literal");
    println!("&& ( x y -- bool )");
    println!("|| ( x y -- bool )");
    println!("not ( x -- bool )");
    println!("head ( list -- item )");
    println!("tail ( list -- tail )");
    println!("empty ( list -- bool )");
    println!("length ( list -- len )");
    println!("cons: ( item list -- list )");
    println!("append ( list1 list2 -- list3 ) ");
    println!("map quotation ( list -- newlist )");
    println!("each quotation ( list -- )");
    println!("foldl quotation ( list initial_accumulator -- final_accumulator )");
}

//interactive/terminal/gchi like mode
fn interactive_mode(stack: &mut Stack) {
    //is a loop where the user types in input and gets output right away until they quit
    loop {
        //to signify that we expect input
        print!("> ");
        //gets the input
        io::stdout().flush().unwrap();

        let mut input = String::new();
        //reads the inputted line
        io::stdin().read_line(&mut input).unwrap();
        //trims everything around so we get the input and nothing outisde of it
        let input = input.trim();

        //implemented an QA functionality with some functions to explain more about usage, see current stack or quit
        if input.starts_with("?") {
            match &input[1..] {
                "h" => print_commands(),
                "s" => stack.show(),
                "q" => break,
                _ => println!("Unknown command. Type ?h for help."),
            }
        } else {
            //else use the parse function to recognize the input
            match stack.parse(input) {
                Ok(()) => (),
                Err(e) => eprintln!("Error: {}", e),
            }
            println!("Stack: {:?}", stack.data);
        }
    }
}

//where the code is executed from a file
fn execute_from_file(stack: &mut Stack, path: &str) -> Result<(), &'static str> {
    let content = fs::read_to_string(path).map_err(|_| "Failed to read the file")?;
    //splits the file by lines
    let lines = content.split('\n');

    //goes trough each line
    for line in lines {
        //trims the  line
        let trimmed_line = line.trim();
        //if the line is not empty use the parse function to recognize it
        if !trimmed_line.is_empty() {
            match stack.parse(trimmed_line) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}

fn main() {

    let v = vec![1,2];
    let x: Option<&i32> = v.get(3);
    match x {
        Some(value) => println!("{value}"),
        None => println!("No value at that place"),
    }

    //creates the stack we will operate on
    let mut stack = Stack::new();
    //number of arguments when starting th program
    let args: Vec<String> = env::args().collect();

    //if the arguments are more than 1 it is filemode because then we also have to declare a file
    if args.len() > 1 {
        let file_path = &args[1];
        match execute_from_file(&mut stack, file_path) {
            Ok(()) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        interactive_mode(&mut stack);
    }
}

#[cfg(test)]
mod test;