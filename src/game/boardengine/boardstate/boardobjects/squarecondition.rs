use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


//what condition has to be met on this boardsquare?
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SquareCondition{
    
    OpponentRequired,
    NoneFriendlyRequired,
    EmptyRequired,
}

