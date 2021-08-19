use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum TypeOfMovement{
    Slide,
    Lift,
}
