


use serde::{Serialize, Deserialize};

use super::boardengine::FullAction;
use super::boardengine::Piece;

#[derive(Serialize, Deserialize, Clone)]
pub enum GameInput{

    Draw,

    FullAction(Piece, FullAction),    
}