use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Unit,
    Box { heap_address:HeapAddress, // des adresses dans le tas
          box_type: Type } // necessary because Rust is a statically-typed language, means that the compiler needs to know the type of data that a variable or exprssion represents at compile time.
    /*  
    Two properties: heap_adress and box_type. 
    The heap_adress property is used to store a HeapAddress, which is an adress in the heap memory.
    The box_type property is used to store a Type, which can be any type of data.

    */
}

impl fmt::Display for Value {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Unit => write!(f, "()"),
            Value::Box{ heap_address, .. } => write!(f, "@{}", heap_address)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Unit,
    Box(std::boxed::Box<Type>) // Type of smart pointer in Rust that allocates memory on the heap for a given type. 
                               // Provides ownership of the data it holds, meaning that the data can only be accessed through the Box pointer. 
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "int"),
            Type::Unit => write!(f, "unit"),
            Type::Box(typ) => write!(f, "Box<{}>", typ),
        }
    }
}

impl Value {

    pub fn get_type(&self) -> Type {
        match self {
            Value::Integer(_) => Type::Integer,
            Value::Unit => Type::Unit,
            Value::Box { box_type, .. }  => Type::Box(Box::new(box_type.clone()))
        }
    }

}