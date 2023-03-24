use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Unit,
    Box { heap_address:HeapAddress, 
          box_type: Type }
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
    Box(std::boxed::Box<Type>)
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