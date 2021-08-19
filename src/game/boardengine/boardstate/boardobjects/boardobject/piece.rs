
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;



#[derive(Hash, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Piece{
    pub id: u16,
}


impl Piece{

    pub fn new(id: u16) -> Piece{

        Piece{
            id
        }
    }
}


