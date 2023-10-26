use crate::stack::*;
use crate::stackvalues::*;

//testing-----------
#[cfg(test)]
mod tests {
    use super::*;

    //helper function to execute code and get the top value of the stack
    fn execute_and_get_top(stack: &mut Stack, code: &str) -> Result<StackValue, &'static str> {
        stack.parse(code)?;
        stack.pop().ok_or("Empty stack")
    }

    //helper function to execute code and get the top two values of the stack
    fn execute_and_get_top_two(stack: &mut Stack, code: &str) -> Result<(StackValue, StackValue), &'static str> {
        stack.parse(code)?;
        
        let top = stack.pop().ok_or("Empty stack")?;
        let second_top = stack.pop().ok_or("Only one element in the stack")?;
        stack.push(second_top.clone());
        stack.push(top.clone());
        Ok((top, second_top))
    }

    //testing that the literals work
    #[test]
    fn literals() {
        let mut stack = Stack::new();
        assert_eq!(execute_and_get_top(&mut stack, "3"), Ok(StackValue::Int(3)));
        assert_eq!(execute_and_get_top(&mut stack, "1.0"), Ok(StackValue::Float(1.0)));
        assert_eq!(execute_and_get_top(&mut stack, "False"), Ok(StackValue::Bool(false)));
        assert_eq!(execute_and_get_top(&mut stack, "True"), Ok(StackValue::Bool(true)));
    }

    //test arithetic functions
    #[test]
    fn simple_arithmetic() {
        let mut stack = Stack::new();
        assert_eq!(execute_and_get_top(&mut stack, "1 1 +"), Ok(StackValue::Int(2)));
        assert_eq!(execute_and_get_top(&mut stack, "10 20 *"), Ok(StackValue::Int(200)));
        assert_eq!(execute_and_get_top(&mut stack, "20 2 div"), Ok(StackValue::Int(10)));
        assert_eq!(execute_and_get_top(&mut stack, "20 2 /"), Ok(StackValue::Float(10.0)));
        assert_eq!(execute_and_get_top(&mut stack, "20 4 -"), Ok(StackValue::Int(16)));
    }

    //test comparsion operations
    #[test]
    fn comparison_operations() {
        let mut stack = Stack::new();
        assert_eq!(execute_and_get_top(&mut stack, "1 1 =="), Ok(StackValue::Bool(true)));
        assert_eq!(execute_and_get_top(&mut stack, "1 2 =="), Ok(StackValue::Bool(false)));
        assert_eq!(execute_and_get_top(&mut stack, "1 2 <"), Ok(StackValue::Bool(true)));
        assert_eq!(execute_and_get_top(&mut stack, "1 2 >"), Ok(StackValue::Bool(false)));
        assert_eq!(execute_and_get_top(&mut stack, "True True =="), Ok(StackValue::Bool(true)));
    }

    //tests logical operations
    #[test]
    fn logical_operations() {
        let mut stack = Stack::new();
        assert_eq!(execute_and_get_top(&mut stack, "True True &&"), Ok(StackValue::Bool(true)));
        assert_eq!(execute_and_get_top(&mut stack, "True False &&"), Ok(StackValue::Bool(false)));
        assert_eq!(execute_and_get_top(&mut stack, "True False ||"), Ok(StackValue::Bool(true)));
        assert_eq!(execute_and_get_top(&mut stack, "False False ||"), Ok(StackValue::Bool(false)));
        let mut stack = Stack::new();
        assert_eq!(execute_and_get_top(&mut stack, "1 not"), Ok(StackValue::Int(-1)));
        assert_eq!(execute_and_get_top(&mut stack, "-2.5 not"), Ok(StackValue::Float(2.5)));
        assert_eq!(execute_and_get_top(&mut stack, "True not"), Ok(StackValue::Bool(false)));
        assert_eq!(execute_and_get_top(&mut stack, "False not"), Ok(StackValue::Bool(true)));
    }

    //test lists operations
    #[test]
    fn list_operations() {
        let mut stack = Stack::new();
    
        assert_eq!(
            execute_and_get_top(&mut stack, "1 [2,3] cons"),
            Ok(StackValue::Vec(vec![
                StackValue::Int(1),
                StackValue::Int(2),
                StackValue::Int(3)
            ]))
        );
    
        assert_eq!(
            execute_and_get_top(&mut stack, "[3,4] [1,2] append"),
            Ok(StackValue::Vec(vec![
                StackValue::Int(1),
                StackValue::Int(2),
                StackValue::Int(3),
                StackValue::Int(4)
            ]))
        );
    
        assert_eq!(
            execute_and_get_top(&mut stack, "[1,2,3] head"),
            Ok(StackValue::Int(1))
        );
    
        assert_eq!(
            execute_and_get_top(&mut stack, "[1,2,3] tail"),
            Ok(StackValue::Vec(vec![StackValue::Int(2), StackValue::Int(3)]))
        );
    
        assert_eq!(
            execute_and_get_top(&mut stack, "[1,2,3] length"),
            Ok(StackValue::Int(3))
        );
    
        assert_eq!(
            execute_and_get_top(&mut stack, "[1,2,3] empty"),
            Ok(StackValue::Bool(false))
        );
    }

    //tests stack operations
    #[test]
    fn stack_operations() {
        let mut stack = Stack::new();
    
        assert_eq!(
            execute_and_get_top(&mut stack, "push 42"),
            Ok(StackValue::Int(42))
        );

        stack.push(StackValue::String("10".to_string()));
        assert_eq!( 
            execute_and_get_top(&mut stack, "parseInteger"),
            Ok(StackValue::Int(10))
        );
    
        assert_eq!(
            execute_and_get_top(&mut stack, "push 3.14"),
            Ok(StackValue::Float(3.14))
        );

        stack.push(StackValue::Int(10));
        stack.push(StackValue::Int(100));
        assert_eq! (
            execute_and_get_top(&mut stack, "swap"),
            Ok(StackValue::Int(10))  
        );

        stack.push(StackValue::Int(6));
        let (top, second_top) = execute_and_get_top_two(&mut stack, "dup").unwrap();
        assert_eq!(top, StackValue::Int(6));
        assert_eq!(second_top, StackValue::Int(6));


        stack.push(StackValue::String("3.14".to_string()));
        assert_eq!(
            execute_and_get_top(&mut stack, "parseFloat"),
            Ok(StackValue::Float(3.14))
        );
    
        stack.push(StackValue::String("one two three".to_string()));
        assert_eq!(
            execute_and_get_top(&mut stack, "words"),
            Ok(StackValue::Vec(vec![
                StackValue::String("one".to_string()),
                StackValue::String("two".to_string()),
                StackValue::String("three".to_string()),
            ]))
        );
    }

    #[test]
    fn test_process_map() {
        let mut stack = Stack::new();

        // Test case: Map with a list of integers and 10 times multiplication
        assert_eq!(
            execute_and_get_top(&mut stack, "10 10 10 [1,2,3] map {*}"),
            Ok(StackValue::Vec(vec![
                StackValue::Int(10),
                StackValue::Int(20),
                StackValue::Int(30)
            ]))
        );
    }

    #[test]
    fn test_process_each() {
        let mut stack = Stack::new();
    
        // Test case: Each with a list of integers and 10 times multiplication
        stack.push(StackValue::Int(10));
        stack.push(StackValue::Vec(vec![
            StackValue::Int(1),
            StackValue::Int(2),
            StackValue::Int(3),
        ]));
        stack.push(StackValue::Quotation(vec![
            StackValue::Symbol(String::from("*"))
        ]));
        stack.process_each().unwrap();
        assert_eq!(stack.pop(), Some(StackValue::Int(60)));
    }

    #[test]
    fn test_process_foldl() {
        let mut stack = Stack::new();

        // Test case: Foldl with a list of integers and addition starting from 0
        assert_eq!(
            execute_and_get_top(&mut stack, "[1,2,3] 0 foldl {+}"),
            Ok(StackValue::Int(6))
        );
    }

}