


use serde::{Serialize, Deserialize};

use board::FullAction;
use board::Piece;

#[derive(Serialize, Deserialize, Clone)]
pub enum GameInput{

    Draw,

    FullAction(Piece, FullAction),    
}