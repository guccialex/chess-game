
use serde::{Deserialize, Serialize};



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




#[derive(Hash, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum BoardObject{

    Piece(Piece),

    Square(Square),
}

impl BoardObject{

    pub fn id(&self) -> u16{

        match self{

            BoardObject::Piece(piece) =>{
                return piece.id;
            },
            BoardObject::Square(square) =>{
                return square.id;
            }
        }
    }

}





