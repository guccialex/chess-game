
use std::collections::HashMap;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::PieceType;
use crate::FullAction;

mod squarecondition;


pub use squarecondition::SquareCondition;

use crate::RelativeSquare;
use crate::SquarePos;

/*
use super::PhysicalAction;
use super::RelativeSquare;



use super::PieceType;
use super::SquareCondition;
use super::FullAction;
*/



//information about a piece
#[derive(Serialize, Deserialize, Clone)]
pub struct PieceData{
    
    owner: u8,

    direction: f32,
    
    piecetype: PieceType,

    augmented: HashSet<PieceType>,
    
    //if the piece has moved (used for castling and moving pawns forward)
    hasmoved: bool,
}


//get the requirements to perform an action
//get the board squares that are dropped when the action is performeds


impl PieceData{

    pub fn new(owner: &u8, direction: &f32) -> PieceData{

        let owner = *owner;
        let direction = *direction;

        PieceData{
            owner,
            direction,

            piecetype: PieceType::Nothing,
            augmented: HashSet::new(),
            hasmoved: false,
        }
    }

    pub fn get_owner(&self) -> u8{

        return self.owner;
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
    
    
    pub fn set_piecetype(&mut self, piecetype: &PieceType){
        self.piecetype = piecetype.clone();
    }
    
    
    pub fn has_moved(&mut self){
        self.hasmoved = true;
    }
    
    pub fn get_image_location(&self) -> String{

        if self.owner == 1{
            return  self.piecetype.image_file();
        }
        else{

            return "b_".to_string() + &self.piecetype.image_file();
        }
    }


    pub fn opponent(&self) -> u8{

        if self.owner == 1{
            return 2;
        }
        else if self.owner ==2{
            return 1;
        }

        panic!("no owner?");
    }


    //get the allowed actions and conditions
    pub fn get_action_conditions(&self, square: &SquarePos, fullaction: &FullAction) -> Vec<(SquarePos, SquareCondition)>{


        let mut toreturn = Vec::new();


        

        match fullaction{

            FullAction::CheckersCapture( _, _ ) => {
                        
                let cap = fullaction.captures().unwrap();
                let dest = fullaction.destination().unwrap();

                toreturn.push( (square.new_from_added_relative_pos(cap.clone()), SquareCondition::NeedsPlayerPiece( self.opponent() )) );
                toreturn.push( (square.new_from_added_relative_pos(dest.clone()), SquareCondition::NeedsEmpty ) );

            },
            FullAction::LiftAndMove( _ ) => {

                let cap = fullaction.captures().unwrap();
                let dest = fullaction.destination().unwrap();

                toreturn.push( (square.new_from_added_relative_pos(dest.clone()), SquareCondition::NeedsNoPlayerPiece(self.owner) ) );

            },
            FullAction::Slide( _, _, capturetype) => {
                
                let cap = fullaction.captures().unwrap();
                let dest = fullaction.destination().unwrap();

                //for every squre it passes over except the last one, change it to that
                for relative in fullaction.passes_over(){

                    toreturn.push( (square.new_from_added_relative_pos(relative), SquareCondition::NeedsEmpty ) );
                }


                toreturn.push( (square.new_from_added_relative_pos(dest.clone()), SquareCondition::NeedsNoPlayerPiece(self.owner) ) )  ;


                if &CaptureType::MustCapture == capturetype{

                    toreturn.push( (square.new_from_added_relative_pos(dest.clone()), SquareCondition::NeedsPlayerPiece(self.opponent() ) ) )  ;
                }

                else if &CaptureType::CantCapture == capturetype{

                    toreturn.push( (square.new_from_added_relative_pos(dest.clone()), SquareCondition::NeedsEmpty ) )  ;

                }

            },
            FullAction::Flick(_, _) =>{
                //dont do anything for flick, dont have conditions
            }
        }

        
        return toreturn;
    }




    

    //get the piece actions
    pub fn get_allowed_actions(&self) -> Vec<FullAction>{

        let mut alltypes = HashSet::new();

        alltypes.insert( self.piecetype.clone() );
        alltypes.extend( self.augmented.clone() );
        
        let mut toreturn = Vec::new();
        
        for piecetype in alltypes{
            
            let allowedactions = piecetype.get_actions(&self.direction, &self.hasmoved);
            toreturn.extend(allowedactions);

        }
        
        toreturn
    }
    
    
    //if this action is valid by the piecedata
    //return the conditions required for this action
    pub fn is_action_allowed(&self, action: &FullAction) -> bool{
        
        for possibleaction in self.get_allowed_actions(){

            if &possibleaction == action{

                return true;
            }
        }

        return false;
    }

       
}








use crate::fullaction::CaptureType;


impl PieceType{

    //get the actions and the conditions
    pub fn get_actions(&self, ownerdirection: &f32, hasmoved: &bool) -> Vec<FullAction>{

        let mut toreturn: Vec<FullAction> = Vec::new();

        //rotate all the actions by the amount of the player?

        match *self{
            
            PieceType::Pawn =>{

                toreturn.push( 
                    FullAction::Slide( 0.0, 1 , CaptureType::CantCapture )
                );
                
                if hasmoved == &false{
                    toreturn.push( 
                        FullAction::Slide( 0.0, 2, CaptureType::CantCapture )
                    );
                };
                
                toreturn.push( 
                    FullAction::Slide( 0.125, 1, CaptureType::MustCapture )
                );

                toreturn.push( 
                    FullAction::Slide( 0.875, 1, CaptureType::MustCapture )
                );

            },
            PieceType::Knight =>{
                
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (1,2) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (2,1) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (2,-1) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (1,-2) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (-1,-2) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (-2,-1) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (-2,1) ) )   );
                toreturn.push(  FullAction::LiftAndMove( RelativeSquare::new( (-1,2) ) )   );
                
            },
            PieceType::Bishop =>{
                
                for dir in (1..8).step_by(2){
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                         
                        toreturn.push(
                            FullAction::Slide( rotation, dist , CaptureType::OptionallyCapture)
                        );
                    }
                }  
            },
            PieceType::Rook => {
                
                for dir in (0..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                    
                    for dist in 1..8{
                        toreturn.push(
                            FullAction::Slide( rotation, dist , CaptureType::OptionallyCapture)
                        );
                    }
                }
            },
            PieceType::Queen => {
                
                for dir in 0..8{
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            FullAction::Slide( rotation, dist , CaptureType::OptionallyCapture)
                        );
                    }
                }
                
            },
            PieceType::King => {
                for dir in 0..8{
                    let rotation = dir as f32 / 8.0;

                    toreturn.push(
                        FullAction::Slide( rotation, 1 , CaptureType::OptionallyCapture)
                    );
                }
            },
            PieceType::Checker => {
                
                for dir in (1..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                    let cap = RelativeSquare::new_from_perspective( (0, 1), rotation);
                    let dest = RelativeSquare::new_from_perspective( (0,2), rotation);
                    toreturn.push(
                        FullAction::CheckersCapture( cap.clone(), dest  )
                    );
                    toreturn.push(
                        FullAction::Slide( rotation, 1, CaptureType::CantCapture )
                    );
                }
            },
            PieceType::Nothing => {},  
        };

        //rotate each action by the players rotation
        for action in toreturn.iter_mut(){
            action.rotate( ownerdirection );
        }

        return toreturn;
    }
}