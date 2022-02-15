


use serde::{Serialize, Deserialize};

use board::FullAction;
use board::Piece;

#[derive(Serialize, Deserialize, Clone)]
pub enum GameInput{

    //draw from what pile
    Draw(u16),

    FullAction(Piece, FullAction),    
}