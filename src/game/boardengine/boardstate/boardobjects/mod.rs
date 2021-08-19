


/*
board objects 

create piece
create boardsquare
send piece or boardsquare on mission
get the boardsquare the pieces are on
get the square and the condition of the square
get the pieces



the physical position of each object

DOESNT have any piece data
*/

mod boardphysics;
use boardphysics::BoardPhysics;

mod boardobject;
use boardobject::BoardObject;
use boardobject::Piece;
use boardobject::Square;

use std::collections::HashSet;
use std::collections::HashMap;

mod relativesquare;
use relativesquare::RelativeSquare;
mod squarepos;
use squarepos::SquarePos;

pub struct BoardState{

    physics: BoardPhysics,
    
    pieces: HashSet<Piece>,

    squares: HashMap<Square, SquarePos>,

}

impl BoardState{

    pub fn create_piece(   ){


    }

    pub fn create_boardsquare(   ){


    }


    pub fn send_on_mission(   ){


    }



}