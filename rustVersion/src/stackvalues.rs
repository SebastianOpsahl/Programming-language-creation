use std::fmt::{self, Display, Formatter};

//has debug, clone and partialeq trait
#[derive(Debug, Clone, PartialEq)]
//different types the programming languages accepts
pub enum StackValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
    Vec(Vec<StackValue>),
    Symbol(String),
    Quotation(Vec<StackValue>),
}

//implemitation of the StackValue enum
impl StackValue {
    //matches inputted type
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (StackValue::Int(_), StackValue::Int(_))
            | (StackValue::Float(_), StackValue::Float(_))
            | (StackValue::Bool(_), StackValue::Bool(_))
            | (StackValue::String(_), StackValue::String(_))
            | (StackValue::Vec(_), StackValue::Vec(_))
            | (StackValue::Symbol(_), StackValue::Symbol(_)) => true,
            | (StackValue::Quotation(_), StackValue::Quotation(_)) => true,
            _ => false,
        }
    }

    //method to reatrieve head of list
    pub fn head(&self) -> Result<StackValue, &'static str> {
        match self {
            //uses vec.first() 
            StackValue::Vec(vec) => {
                if let Some(first) = vec.first() {
                    Ok(first.clone())
                } else {
                    Err("The list is empty")
                }
            }
            _ => Err("The value is not a list"),
        }
    }

    //method to get tail of list
    pub fn tail(&self) -> Result<StackValue, &'static str> {
        match self {
            StackValue::Vec(vec) => {
                if !vec.is_empty() {
                    //everything but the first element 0
                    Ok(StackValue::Vec(vec[1..].to_vec()))
                } else {
                    Err("The list is empty")
                }
            }
            _ => Err("The value is not a list"),
        }
    }

    //checks if a list is empty or not
    pub fn is_empty(&self) -> Result<StackValue, &'static str> {
        match self {
            StackValue::Vec(vec) => Ok(StackValue::Bool(vec.is_empty())),
            _ => Err("The value is not a list"),
        }
    }

    //length of inputted values
    pub fn length(&self) -> Result<StackValue, &'static str> {
        match self {
            StackValue::Vec(vec) => Ok(StackValue::Int(vec.len() as i32)),
            _ => Err("The value is not a list"),
        }
    }
}

//determines how the StackValue should be displayed
impl Display for StackValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            StackValue::Int(value) => write!(f, "{}", value),
            StackValue::Float(value) => write!(f, "{}", value),
            StackValue::Bool(value) => write!(f, "{}", value),
            StackValue::String(value) => write!(f, "{}", value),
            StackValue::Vec(value) => write!(f, "{:?}", value),
            StackValue::Symbol(value) => write!(f, "{}", value),
            //has to deal with different types
            StackValue::Quotation(quot) => {
                let quot_str = quot
                    .iter()
                    .map(|stack_value| format!("{}", stack_value))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "{{{}}}", quot_str)
            },
        }
    }
}