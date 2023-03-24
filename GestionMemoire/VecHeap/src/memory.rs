use super::*;

#[derive(Debug)]
pub struct Memory {
    pub stack: NameSpaceStack, // Vec<NameSpace>, regular variables stored on the stack
    pub heap: Heap, // Vec<Option<Value>>, otherwise we search the variable on the heap 
}

impl Memory {

    pub fn new(heap_size: usize) -> Self {
        Self {
            stack: NameSpaceStack::root(),
            heap: Heap::new(heap_size)
        }
    } 
}