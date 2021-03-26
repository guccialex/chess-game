
use std::collections::HashMap;
use std::collections::HashSet;
//use ncollide3d::shape::ConvexHull;

use serde::{Serialize, Deserialize};

//use super::BoardSquarePosID;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
pub enum PieceAction{
    
    flick(f32, f32),
    
    liftandmove( (i8,i8) ),
    
    //what direction, and how many steps
    slide( u8, u8 ),
    
    //what direction to capture in a checkers fashion
    checkerscapture(u8),


    //a slide that doesnt lift any squares
    capturelessslide(u8, u8),
    
    
    /*
    normal piece slide (slide x distance in y direction)
    pawn capture  (slide 1 square in x direction if there is a piece on that square)
    castle (if theres no squares 4 to the left to the 4th then an unmoved king/rook swap that piece with this one)
    
    */
}



impl PieceAction{
    
    pub fn get_relative_position_action_takes_piece(&self) -> (i8, i8){
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                return relativepos;
            },
            PieceAction::slide( direction, distance ) | PieceAction::capturelessslide(direction, distance) => {
                
                let (xstep, zstep) = direction_to_step_from_objective_perspective(direction);
                
                let relativepos = (xstep * distance as i8, zstep * distance as i8);
                
                return relativepos;
            },
            PieceAction::checkerscapture(direction) => {
                
                let (xstep, zstep) = direction_to_step_from_objective_perspective(direction);
                
                let relativepos = (xstep * 2, zstep * 2);
                
                return relativepos;
            },
            PieceAction::flick(_,_) => {
                panic!("i dont know the relative square a flick takes this piece");
            },
        }
    }
    
    //get the lift and move forces on the piece this action is applied to
    pub fn get_lift_and_move_forces(&self) -> Option<(i8,i8)>{
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                return Some(relativepos);
            },
            PieceAction::slide( _,_ ) | PieceAction::capturelessslide(_,_)=> {
                
                return None;
            },
            PieceAction::checkerscapture(direction) => {
                
                let (xstep, zstep) = direction_to_step_from_objective_perspective(direction);
                
                let relativepos = (xstep * 2, zstep * 2);
                
                return Some(relativepos);
            },
            PieceAction::flick(_,_) => {
                return None;
            },
        }
        
    }
    
    pub fn get_slide_forces(&self) -> Option<(i8,i8)>{
        
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                return None;
            },
            PieceAction::slide( direction, distance ) | PieceAction::capturelessslide(direction, distance)=> {
                
                let (xstep, zstep) = direction_to_step_from_objective_perspective(direction);
                
                let relativepos = (xstep * distance as i8, zstep * distance as i8);
                
                return Some( relativepos );
            },
            PieceAction::checkerscapture(direction) => {
                
                return None;
            },
            PieceAction::flick(_,_) => {
                return None;
            },
        }
        
        
    }
    
    pub fn get_flick_forces(&self) -> Option<(f32,f32)> {
        
        
        match *self{
            
            PieceAction::liftandmove( _ ) => {
                
                return None;
            },
            PieceAction::slide( _, _ ) | PieceAction::capturelessslide( _, _ ) => {
                
                return None;    
            },
            PieceAction::checkerscapture(_) => {
                return None;
            },
            PieceAction::flick(dir,force) => {
                return Some( (dir,force) );
            },
        };
        
    }
    
    //get the squares dropped by this action and the tick it happens
    pub fn get_squares_dropped_relative(&self) -> Vec<( (i8,i8), u32 )>{
        
        let mut toreturn = Vec::new();
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                toreturn.push( (relativepos, 0) );
                
                return toreturn;
            },
            PieceAction::slide( direction, distance ) => {
                
                let (xstep, zstep) = direction_to_step_from_objective_perspective(direction);
                
                for x in 1..distance+1{
                    
                    let relativepos = (xstep * x as i8, zstep * x as i8);
                    
                    toreturn.push( (relativepos, x as u32 * 5) );
                }
                
                return toreturn;
            },
            PieceAction::checkerscapture(direction) => {
                
                let step = direction_to_step_from_objective_perspective(direction);
                
                let relativepos = step;
                
                toreturn.push( (relativepos, 0) );
                
                return toreturn;
            },
            PieceAction::flick(_,_) | PieceAction::capturelessslide(_,_)=> {
                
                return toreturn;
            },
        }
    }
    
    
    
    //get the conditions for this action when it needs to capture to perform its action
    fn add_has_to_capture_conditions_for_action(&self, toaddto: &mut HashSet<( (i8,i8), SquareCondition )>){
        
        //if this action drops squares
        if let Some(lastsquareandtick) = self.get_squares_dropped_relative().last(){

            //get the last square it drops
            let lastsquarerelativepos = lastsquareandtick.0;

            //the final square dropped must have an opponents piece on it
            toaddto.insert( (lastsquarerelativepos, SquareCondition::OpponentRequired) );
        };
    }
    
    //the base conditions for this action
    //the squares it passes over must not have any friendly pieces
    //and the squares it passes over EXCEPT THE LAST ONE must not have any pieces / (no friendly or enemy pieces)
    fn add_base_conditions_for_action(&self, toaddto: &mut HashSet<( (i8,i8), SquareCondition )>){
        

        match *self{
            
            PieceAction::flick(_,_) => {
                
            },
            PieceAction::liftandmove( relativepos ) => {
                
                toaddto.insert( (relativepos,  SquareCondition::NoneFriendlyRequired) );
                
            },
            PieceAction::checkerscapture( direction ) => {
                /*
                The square the checkers capture is going to must be empty
                and the squares that it drops must have an opponents piece on them
                */
                
                toaddto.insert( (self.get_relative_position_action_takes_piece(), SquareCondition::EmptyRequired) );
                
                for (relativesquarepos, _) in self.get_squares_dropped_relative(){
                    toaddto.insert( (relativesquarepos, SquareCondition::OpponentRequired) );
                    toaddto.insert( (relativesquarepos, SquareCondition::NoneFriendlyRequired) );
                }
                
            },
            PieceAction::slide( _dir, _dist) =>{
                /*
                For a slide action, all boardsquares besides the last one passed over must be empty
                And the last one must not have any of my own peices on it
                */
                
                let droppedrelative = self.get_squares_dropped_relative();
                                
                for x in &droppedrelative{

                    //if this is the last square being passed over
                    if x == droppedrelative.last().unwrap(){
                        toaddto.insert( (x.0, SquareCondition::NoneFriendlyRequired) );
                    }
                    //if its not the last square being passed over
                    else{
                        toaddto.insert( (x.0, SquareCondition::EmptyRequired) );
                    }
                };

            },
            //all squares it passes over must be empty
            PieceAction::capturelessslide(dir, dist) =>{

                //get squares passed over relative



                
                let (xstep, zstep) = direction_to_step_from_objective_perspective(dir);
                
                for x in 1..=dist{

                    let relativepos = (xstep * x as i8, zstep * x as i8);
                
                    toaddto.insert( (relativepos, SquareCondition::EmptyRequired) );
                }
                

                /*
                let droppedrelative = self.get_squares_dropped_relative();
                                
                for x in &droppedrelative{

                        toaddto.insert( (x.0, SquareCondition::EmptyRequired) );
                };
                */
            }
            
            
        };
        
        
    }
    
    
    
}




//what condition has to be met on this boardsquare?
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum SquareCondition{
    
    OpponentRequired,
    NoneFriendlyRequired,
    EmptyRequired,
}

//get the square conditions for a 






#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
enum Functionality{
    
    //the functionality for a pawn to move 1 or 2 forwards if unmoved
    PawnAdvance,
    //the functionality for a pawn to capture the pieces diagonally in front of it
    PawnCapture,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Checker,
    Flickable,
    
}

impl Functionality{
    
    //TODO: add case for flick
    fn is_action_valid(&self, ownerdirection: &u8, hasmoved: &bool, action: &PieceAction) -> bool{
        
        for validaction in &self.get_actions(ownerdirection, hasmoved){
            if validaction == action{
                return true;
            }
        }        
        
        return false;
    }
    
    
    fn get_actions(&self, ownerdirection: &u8, hasmoved: &bool) -> Vec<PieceAction>{
        
        let mut toreturn: Vec<PieceAction> = Vec::new();
        
        match *self{
            
            Functionality::PawnAdvance =>{

                toreturn.push( PieceAction::capturelessslide( perspective_to_objective_slide(ownerdirection, &0), 1) );
                
                if hasmoved == &false{
                    toreturn.push( PieceAction::capturelessslide( perspective_to_objective_slide(ownerdirection, &0), 2) );
                }

                return toreturn;
            },
            Functionality::PawnCapture =>{
                
                //capturing diagonally
                toreturn.push( PieceAction::slide( perspective_to_objective_slide(ownerdirection, &1) , 1) );
                toreturn.push( PieceAction::slide( perspective_to_objective_slide(ownerdirection, &7) , 1) );

                return toreturn;
                
            },
            Functionality::Knight =>{
                
                toreturn.push( PieceAction::liftandmove( (1,2)  ) );
                toreturn.push( PieceAction::liftandmove( (2,1)  ) );
                toreturn.push( PieceAction::liftandmove( (2,-1) ) );
                toreturn.push( PieceAction::liftandmove( (1,-2) ) );
                
                
                toreturn.push( PieceAction::liftandmove( (-1,-2) ) );
                toreturn.push( PieceAction::liftandmove( (-2,-1) ) );
                toreturn.push( PieceAction::liftandmove( (-2,1)  ) );
                toreturn.push( PieceAction::liftandmove( (-1,2)  ) );
                
                
                return toreturn;
            },
            Functionality::Bishop =>{
                
                for dir in (1..8).step_by(2){
                    
                    for dist in 1..8{
                        toreturn.push(  PieceAction::slide(dir, dist)  );
                    }
                }
                
                return toreturn;
            },
            Functionality::Rook => {
                
                for dir in (0..8).step_by(2){
                    
                    for dist in 1..8{
                        toreturn.push(  PieceAction::slide(dir, dist)  );
                    }
                }
                
                return toreturn;
            },
            Functionality::Queen => {
                
                for dir in 0..8{
                    
                    for dist in 1..8{
                        toreturn.push(  PieceAction::slide(dir, dist)  );
                    }
                }
                
                return toreturn;
            },
            Functionality::King => {
                
                for dir in 0..8{
                    
                    toreturn.push(  PieceAction::slide(dir, 1)  );
                }
                
                return toreturn;
            },
            Functionality::Checker => {
                
                for dir in (1..8).step_by(2){
                    toreturn.push(  PieceAction::checkerscapture(dir)  );
                }

                for dir in (1..8).step_by(2){
                    toreturn.push(  PieceAction::capturelessslide(dir, 1)  );
                }

                
                return toreturn;
            },
            Functionality::Flickable => {
                
                return toreturn;
            },
            
            
        };
    }
    
    
    //get the conditions of the action

    //assume the action is valid
    fn get_conditions_for_action(&self, action: &PieceAction) -> HashSet< ((i8,i8), SquareCondition)>{

        
        let mut toreturn = HashSet::new();
        
        //if this action is valid
        match *self{
            
            Functionality::PawnCapture =>{
                action.add_base_conditions_for_action(&mut toreturn);
                action.add_has_to_capture_conditions_for_action(&mut toreturn);

                return toreturn;
            },
            _ =>{
                action.add_base_conditions_for_action(&mut toreturn);
                    
                return toreturn;
            },
            
        };
        
        
    }
    
}






//a struct that has the information about the piece
//what type of piece it is
//what actions its allowed to perform
#[derive(Serialize, Deserialize, Clone)]
pub struct PieceData{
    
    //the name of the piece
    typename: String,
    
    functionalities: HashSet<Functionality>,
    
    value: u8,
    
    //if the piece has moved (used for castling and moving pawns forward)
    hasmoved: bool,
    
}


//get the requirements to perform an action
//get the board squares that are dropped when the action is performeds


impl PieceData{
    
    pub fn new() -> PieceData{    
        PieceData{
            typename: "none".to_string(),
            functionalities: HashSet::new(),
            hasmoved: false,
            value: 1,
        }
    }


    //give this piece the abilities of a knight
    pub fn augment_knight_abilities(&mut self){


        self.functionalities.insert( Functionality::Knight );
        
    } 

    //remove any augmentations this piece might have, so it just has the default effects again
    pub fn remove_ability_augmentations(&mut self){

        if self.typename != "knight"{
            self.functionalities.remove( &Functionality::Knight );
        }

    }

    
    
    pub fn get_value(&self) -> u8{        
        return self.value;
    }
    
    
    pub fn set_checker(&mut self){
        
        self.typename = "checker".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::Checker );
        
        self.value = 2;
    }
    
    pub fn set_king(&mut self){
        
        self.typename = "king".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::King );
        
        self.value = 0;
    }
    
    //set selfs actions to be those of a pawn
    pub fn set_pawn(&mut self){
        
        self.typename = "pawn".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::PawnCapture );
        self.functionalities.insert( Functionality::PawnAdvance );
        
        self.value = 1;
    }
    
    pub fn set_knight(&mut self){
        
        self.typename = "knight".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::Knight );
        
        self.value = 2;
    }
    
    pub fn set_bishop(&mut self){
        
        self.typename = "bishop".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::Bishop );
        
        self.value = 3;
    }
    
    pub fn set_rook(&mut self){
        
        self.typename = "rook".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::Rook );
        
        self.value = 5;
    }
    
    pub fn set_queen(&mut self){
        
        self.typename = "queen".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::Queen );
        
        self.value = 8;
    }
    
    
    //get rid of its allowed actions and make it flickable
    pub fn set_pool_ball(&mut self){
        
        self.typename = "poolball".to_string();
        
        self.functionalities = HashSet::new();
        self.functionalities.insert( Functionality::Flickable );
        
        self.value = 2;
    }
    
    
    //turn this piece back into an appropriately valued chess piece
    pub fn set_chess_piece(&mut self){
        
        if self.value == 0{
            
            self.set_king();
        }
        else if self.value <= 1{
            
            self.set_pawn();
        }
        else if self.value <= 2{
            //could be a bishop of a knight
            self.set_knight();
        }
        else if self.value <= 3{
            
            self.set_bishop();
        }
        else if self.value <= 5{
            
            self.set_rook();
        }
        else{
            self.set_queen();
        }
        
    }
    
    
    pub fn moved_piece(&mut self){
        self.hasmoved = true;
    }
    
    pub fn get_type_name(&self) -> String{
        self.typename.clone()
    }
    
    
    
    //get the piece actions that are listable
    pub fn get_numberable_piece_actions(&self, ownerdirection: &u8) -> Vec<PieceAction>{
        
        let mut toreturn = Vec::new();
        
        for functionality in &self.functionalities{
            
            let allowedactions = functionality.get_actions(ownerdirection, &self.hasmoved);
            
            toreturn.extend(allowedactions);
        }
        
        toreturn
    }    
    
    
    //if this action is valid by the piecedata
    //return the conditions required for this action
    pub fn is_action_valid(&self, action: &PieceAction, ownerdirection: &u8) -> Option< HashSet< ((i8,i8), SquareCondition)> >{
        
        
        for functionality in &self.functionalities{
            
            if functionality.is_action_valid(ownerdirection, &self.hasmoved, action){
                
                return Some( functionality.get_conditions_for_action(action) );
            }
        }
        
        return None;
    }
    
    
}







fn perspective_to_objective_slide(playerdirection: &u8, slidedirection: &u8) -> u8{
    
    let slidedirection = *slidedirection as i32;
    
    let playerdirection = *playerdirection as i32;
    
    //add slide direction and player direction to get the new direction
    //and mod by 8 so it loops around if its too large
    let toreturn = (slidedirection + playerdirection) % 8;
    
    toreturn as u8
    
}


//if the object cant be rotated and still represented as an i8, return none
fn players_perspective_to_objective_perspective_lift(playerdirection: &u8, relativepos: &(i8,i8)) -> Option<(i8,i8)>{
    
    let angleasradians = *playerdirection as f32;
    let angleasradians = angleasradians / 8.0 ;
    let angleasradians = angleasradians * 2.0 * 3.14159;
    
    let relativeposx = relativepos.0 as f32;
    let relativeposy = relativepos.1 as f32;
    
    
    let roundedcosangle = angleasradians.cos().round();
    let roundedsinangle = angleasradians.sin().round();
    
    
    let newxfloat = (relativeposx * roundedcosangle - relativeposy * roundedsinangle) as i32;
    let newyfloat = (relativeposx * roundedsinangle + relativeposy * roundedcosangle) as i32;
    
    
    //if the new coordinates can be converted into an i8
    //which now that im thinking, should always be the case
    //but im not sure, and its already set up this way
    
    use std::convert::TryFrom;
    
    if let Some(resultingx) = i8::try_from(newxfloat).ok(){
        
        if let Some(resultingy) = i8::try_from(newyfloat).ok(){
            
            return  Some( (resultingx, resultingy) ) ;
        }
    }
    
    //else return None
    return None ;
}





fn direction_to_step_from_objective_perspective(slideid: u8) -> (i8,i8){
    
    
    let mut toreturn = (0,0);
    
    //the 8 semi-cardinal directions
    //and the value for how much the piece moves going in each one        
    if slideid == 0{
        toreturn = (0,1);
    }
    else if slideid == 1{
        toreturn= (1,1) ;
    }
    else if slideid == 2{
        toreturn= (1,0 );
    }
    else if slideid == 3{
        toreturn =(1,-1 );
    }
    else if slideid == 4{
        toreturn = (0,-1 );
    }
    else if slideid == 5{
        toreturn = (-1,-1);
    }
    else if slideid == 6{
        toreturn = (-1,0 );
    }
    else if slideid == 7{
        toreturn = (-1,1);
    }
    
    
    toreturn
}
