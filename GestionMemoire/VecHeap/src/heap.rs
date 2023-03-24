use std::fmt::Display;

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
pub struct VecHeap {
    vec: Vec<Option<Value>>
}

type VecHeapAddress = usize; 

/*
usize is used to represent memory addresses and indices into arrays and other data structures. 
*/

impl VecHeap {

    fn accepts_address(&self, address:&VecHeapAddress) -> bool {
        /*address >= 0 && */ *address < self.vec.len()
    }

    fn can_allocate(&self, address: &VecHeapAddress) -> bool {
        self.accepts_address(address) && self.vec[*address] == None
    }

}

impl HeapTrait for VecHeap { 

/*  
methodes d'une classe VecHeap qui a pour champs un Vecteur qui contient une Option de type Value. Les adresses donc 
sont stockees dans un tableau avec un entier 
*/

    type HeapAddress = VecHeapAddress;

    fn new(size: usize) -> VecHeap { // create a new heap with the speified size as parameter 
        let mut vec = Vec::new();
        vec.resize(size, None);
        VecHeap{ vec }
    }

    fn allocate_box(&mut self, v: Value) -> Result<Self::HeapAddress, String> {
        for i in 0..self.vec.len() {
            if self.can_allocate(&i) { // address >0 and < length of the vector and the value is None at this address, so we can give this memory to a variable 
                self.vec[i] = Some(v);
                return Ok(i)
            }
        };
        Err("heap overflow".to_string())
    }

    fn lookup(&self, address: &Self::HeapAddress) -> Result<Value, String> { // return the value of the variable stored at a address passed as parameter 
        if ! self.accepts_address(address) { 
            return Err(format!("segmentation fault: invalid address {}", address))
        }
        if self.can_allocate(address) { 
            return Err(format!("dangling pointer error: address {} is not allocated", address))
        }
        let v = self.vec[*address].clone().unwrap();
        Ok(v)
    }

    fn update(&mut self, _address: &Self::HeapAddress, _v: Value) -> Result<(), String> {
        // si l'adresse n'est pas valide ou allouée, échoue avec le même message que pour le lookup
        // si l'adresse contient une valeur avec un type différent de celui de v, échoue avec un message d'erreur
        // sinon met à jour la cellule
        
        match self.lookup(_address){
            
            Ok(valeur) => { // type de retour soit Ok, soit un String qui affiche un erreur 
                
                if valeur.get_type() != _v.get_type() {
                    Err(format!("Type de valeur incompatible {} ", _v.get_type()))
                }
                else {
                    Ok(self.vec[*_address] = Some(_v))
                }
            },
            
            Err(_) => Err(format!("Can't make an update {} ", _v)),
        
        }
        
    }

    fn free(&mut self, _address: &Self::HeapAddress) -> Result<(), String> {
        // si la cellule n'est pas allouée échoue avec le message
        // Err(format!("double free or dangling pointer error: trying to free {} that is not allocated", address))
        // sinon désalloue la cellule, au sens de VecHeap (pas de Rust)
        
        match self.can_allocate(_address) {
            true => Err(format!("double free or dangling pointer error: trying to free {} that is not allocated", _address)), 
            false => Ok(self.vec[*_address] = None), // si la cellule est alloue, donc on peut le supprimer 
        }

    }

    /* 
    2 version

    fn free(&mut self, adress: &Self::HeapAdress) -> Result<(), String> {
        self.accepts_adress(adress)?; type bool
        if self.vec[*adress].is_none(){
            return Err("Adress is already empty".to_owned());
        }
        self.vec[*adress] = None;
        Ok(())
    }
    */

    fn drop(&mut self, _address: &Self::HeapAddress) -> Result<(), String> {
        // désallocation récursive de tout ce qui se trouve "dessous" l'adresse passée en paramètre
        // adresse est un index dans une liste, donc on eleve ce qui est > de l'adresse qu'on passe en parametre 
        
        if !self.accepts_address(_address){ // si l'addresse n'est pas dans l'intervalle qu'on souhaite 
            return Err(format!("segmentaion fault: {}", _address));
        }

        if self.can_allocate(_address){ // est None
            return Err(format!("dangling {}", _address));
        }
        
        for i in *_address+1..self.vec.len() {
            if let Err(e) = self.free(&i) {
                return Err(e);
            }
            
            /*  
            if let syntax - shorthand of a pattern matching expression.
            if the result is an 'Err' variant, the code inside the block will be executed
            otherwise we pass to the next iteration
            */
        
        }
        
        Ok(())
    
    }
        
}

impl Display for VecHeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.vec.iter().map(|x| match x {
            Some(v) => format!("{}", v),
            None => "Ø".to_string(),
        }).collect::<Vec<_>>().join(", "))
    }
}












