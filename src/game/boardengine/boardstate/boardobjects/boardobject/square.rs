
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;



#[derive(Hash, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Square{
    pub id: u16,
}

impl Square{

    pub fn new(id: u16) -> Square{

        Square{
            id
        }
    }
}

