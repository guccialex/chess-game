
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

mod piece;
mod square;

pub use piece::Piece;
pub use square::Square;




#[derive(Hash, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum BoardObject{

    Piece(Piece),

    Square(Square),
}

impl BoardObject{

    pub fn as_piece(&self) -> Option<Piece>{

        if let BoardObject::Piece(piece) = self{

            return Some( piece.clone() );
        }

        return None;
    }

    pub fn as_square(&self) -> Option<Square>{

        if let BoardObject::Square(square) = self{

            return Some( square.clone() );
        }

        return None;
    }


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
