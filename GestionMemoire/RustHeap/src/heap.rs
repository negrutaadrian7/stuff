use std::{fmt::Display, cell::RefCell, rc::Rc};

use super::*;


pub trait HeapTrait : Display {
    type HeapAddress : Display;
    fn new(size: usize) -> Self;
    fn allocate_box(&mut self, v: Value) -> Result<Self::HeapAddress, String>;
    fn lookup(&self, address: &Self::HeapAddress) -> Result<Value, String>;
    fn update(&mut self, address: &Self::HeapAddress, v: Value) -> Result<(), String>;
    fn free(&mut self, address: &Self::HeapAddress) -> Result<(), String>;
    fn drop(&mut self, address: &Self::HeapAddress) -> Result<(), String>;
}


#[derive(Debug)]
pub struct RustHeap { }

#[derive(Debug, PartialEq)]
pub struct RustHeapAddress(Rc<RefCell<Value>>);





impl Display for RustHeapAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@rustheap")
    }
}


impl Clone for RustHeapAddress {
    /*
    Creates a clone of a RustHeapAddress object. Using Rc::clone() method
    which creates a new reference-counted pointer to the same data as the
    original RustHeapAddress object
    */
    fn clone(&self) -> RustHeapAddress {
       RustHeapAddress(Rc::clone(&self.0))
    }
}



impl HeapTrait for RustHeap {
    type HeapAddress = RustHeapAddress;

    fn new(_size: usize) -> Self {
        RustHeap {}
    }

    fn allocate_box(&mut self, v: Value) -> Result<Self::HeapAddress, String> {
        let rc = Rc::new(RefCell::new(v));
        Ok(RustHeapAddress(rc))
    }

    fn lookup(&self, address: &Self::HeapAddress) -> Result<Value, String> {
        /* 
        Looks up for a value in the heap. Return the value at the address passed as argument or an error message 
        */
        match address.0.try_borrow() {
            Ok(value) => Ok(value.clone()),
            Err(_) => Err("Can't reach the address".to_string()),
        }        
    }


    fn update(&mut self, address: &Self::HeapAddress, v: Value) -> Result<(), String> {
        /*
        Update a value stored in a heap address. Takes 2 param.
        Attempts to borrow the address mutably.
        Succes => Sets the value 
        Error => Error message
        */
        match address.0.try_borrow_mut(){
            Ok(mut value) => {
                *value = v;
                Ok(())
            }
            Err(_) => Err(String::from("Error")),
        }
    }


    fn free(&mut self, address: &Self::HeapAddress) -> Result<(), String> {
        match address.0.try_borrow_mut() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Failed to free the address")),
        }
    }
    

    fn drop(&mut self, address: &Self::HeapAddress) -> Result<(), String> {
        match address.0.try_borrow_mut(){
            Ok(value) => {
                drop(value);
                Ok(())
            }
            Err(_) => {
                Err("Can't drop cause still having references".to_string())
            }
        }
    }
}



impl Display for RustHeap {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RUST HEAP")
    }
}