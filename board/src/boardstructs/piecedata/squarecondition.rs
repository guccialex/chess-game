use serde::{Serialize, Deserialize};


use crate::FullAction;


//the conditions that must be met, for each relative square

//what condition has to be met on this boardsquare?
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SquareCondition{
    
    //piece owned by player x required
    NeedsPlayerPiece(u8),

    //no piece owned by player x required
    NeedsNoPlayerPiece(u8),


    NeedsEmpty,

    //just needs it to exist
    NeedsNothing,
}


