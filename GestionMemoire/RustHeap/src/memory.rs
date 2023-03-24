use super::*;

#[derive(Debug)]
pub struct Memory {
    pub stack: NameSpaceStack,
    pub heap: Heap,
}

impl Memory {

    pub fn new(heap_size: usize) -> Self {
        Self {
            stack: NameSpaceStack::root(),
            heap: Heap::new(heap_size)
        }
    } 
}