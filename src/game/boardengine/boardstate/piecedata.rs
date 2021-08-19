
use std::collections::HashMap;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};


//use super::squarepos::SquarePos;
//use super::relativesquare::RelativeSquare;
use super::piecetype::PieceType;
use super::fullaction::FullAction;






//information about a piece
#[derive(Serialize, Deserialize, Clone)]
pub struct PieceData{
    
    //the name of the piece
    piecetype: PieceType,

    augmented: HashSet<PieceType>,
    
    //if the piece has moved (used for castling and moving pawns forward)
    hasmoved: bool,
}


//get the requirements to perform an action
//get the board squares that are dropped when the action is performeds


impl PieceData{

    pub fn new() -> PieceData{

        PieceData{
            piecetype: PieceType::Nothing,
            augmented: HashSet::new(),
            hasmoved: false,
        }
    }

    pub fn is_this_piecetype(&self, piecetype: &PieceType) -> bool{

        return &self.piecetype == piecetype;
    }

    //give this piece the abilities of a knight
    pub fn augment(&mut self, piecetype: &PieceType){

        self.augmented.insert(piecetype.clone());
    } 

    //remove any augmentations this piece might have, so it just has the default effects again
    pub fn remove_augmentations(&mut self){

        self.augmented = HashSet::new();
    }


    pub fn get_value(&self) -> u8{

        return self.piecetype.value();
    }
    
    
    pub fn set_piecetype(&mut self, piecetype: PieceType){
        
        self.piecetype = piecetype;
    }
    
    
    pub fn is_moved(&mut self){
        self.hasmoved = true;
    }
    
    pub fn get_image_location(&self) -> String{
        self.piecetype.image_file()
    }
    

    //get the piece actions
    pub fn get_allowed_actions(&self, ownerdirection: &f32) -> Vec<FullAction>{

        let mut alltypes = HashSet::new();

        alltypes.insert( self.piecetype.clone() );
        alltypes.extend( self.augmented.clone() );
        
        let mut toreturn = Vec::new();
        
        for piecetype in alltypes{
            
            let allowedactions = piecetype.get_actions(ownerdirection, &self.hasmoved);
            
            toreturn.extend(allowedactions);
        }
        
        toreturn
    }    
    
    
    //if this action is valid by the piecedata
    //return the conditions required for this action
    pub fn is_action_allowed(&self, action: &FullAction, ownerdirection: &f32) -> bool{
        
        for possibleaction in self.get_allowed_actions(ownerdirection){

            if &possibleaction == action{

                return true;
            }
        }

        return false;
    }

       
}



