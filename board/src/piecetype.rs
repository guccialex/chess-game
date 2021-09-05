use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use crate::FullAction;
use crate::RelativeSquare;



fn dir_from_pers(objectdirection: f32, playerdirection: f32) -> f32{

    return objectdirection + playerdirection;
}





#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceType{

    Nothing,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Checker
}

impl PieceType{

    pub fn value(&self) -> u8{

        match self{
            PieceType::Nothing => 0,
            PieceType::Pawn => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Rook => 4,
            PieceType::Queen => 5,
            PieceType::King => 12,
            PieceType::Checker => 2,
        }
    }

    pub fn get_random() -> PieceType{

        let mut piecetypes = HashSet::new();

        piecetypes.insert( PieceType::Bishop  );
        piecetypes.insert( PieceType::Checker  );
        piecetypes.insert( PieceType::Queen  );
        piecetypes.insert( PieceType::Knight  );
        piecetypes.insert( PieceType::Pawn  );
        piecetypes.insert( PieceType::Rook  );

        piecetypes.iter().next().unwrap().clone()
    }


    //the name of the file that represents this objects image
    pub fn image_file(&self) -> String{

        match self{
            PieceType::Nothing => format!("none.png"),
            PieceType::Pawn => format!("pawn.png"),
            PieceType::Knight => format!("knight.png"),
            PieceType::Bishop => format!("bishop.png"),
            PieceType::Rook => format!("rook.png"),
            PieceType::Queen => format!("queen.png"),
            PieceType::King => format!("king.png"),
            PieceType::Checker => format!("checker.png"),
        }
    }
}

