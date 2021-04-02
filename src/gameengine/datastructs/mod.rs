
use std::collections::HashMap;
use std::collections::HashSet;
//use ncollide3d::shape::ConvexHull;

use serde::{Serialize, Deserialize};

mod boardsquarestructs;

pub use boardsquarestructs::BoardSquarePosID;
pub use boardsquarestructs::RelativeSquare;



/*
fn board_rotation_and_distance_to_relative_pos(rotation: f32, distance: u8) -> Option<RelativeSquare>{
  

    let i8pos = orthogonal_rotation::ortho_rotate_i8_point_at_point( (0, distance as i8), (4,4), rotation);


    RelativeSquare::new(  )
}
*/


/*
fn perspective_to_objective_direction(playerdirection: &u8, curdirection: &u8) -> u8{

    let curdirection = *curdirection;
    
    let playerdirection = *playerdirection;
    
    let toreturn = (curdirection + playerdirection) % 8;
    
    toreturn
}
*/






#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum PieceAction{
    
    //direction and force
    flick(f32, f32),
    
    liftandmove( (i8,i8) ),
    
    //what direction, and how many steps
    slide( f32, u8 ),
    
    //what direction to capture in a checkers fashion
    checkerscapture(f32),


    //a slide that doesnt lift any squares
    capturelessslide( f32, u8 ),
    
    
    /*
    normal piece slide (slide x distance in y direction)
    pawn capture  (slide 1 square in x direction if there is a piece on that square)
    castle (if theres no squares 4 to the left to the 4th then an unmoved king/rook swap that piece with this one)
    */
}



impl PieceAction{


    pub fn lift_and_move_from_relative_square( relativepos: RelativeSquare ) -> PieceAction{

        PieceAction::liftandmove( relativepos.get_relative_pos() )
    }

    pub fn slide_from_perspective( directionrotation: f32, perspectiverotation: f32, distance: u8 ) -> PieceAction{

        let rotation = (directionrotation + perspectiverotation) % 1.0;

        PieceAction::slide( rotation, distance )
    }

    pub fn captureless_slide_from_perspective( directionrotation: f32, perspectiverotation: f32, distance: u8 ) -> PieceAction{

        let rotation = (directionrotation + perspectiverotation) % 1.0;

        PieceAction::capturelessslide( rotation, distance )
    }

    pub fn checkers_capture_from_perspective( directionrotation: f32, perspectiverotation: f32 ) -> PieceAction{

        let rotation = (directionrotation + perspectiverotation) % 1.0;

        PieceAction::checkerscapture(rotation)

    }


    //get if two actions are the same
    pub fn is_equal( &self, action: &PieceAction) -> bool{


        //TODO
        
        
        return true;
        
    }


    
    pub fn get_relative_position_action_takes_piece(&self) -> RelativeSquare{
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                return RelativeSquare::new(relativepos).unwrap() ;
            },
            PieceAction::slide( rotation, distance ) | PieceAction::capturelessslide( rotation, distance ) => {
                
                return RelativeSquare::new_from_perspective( (0, distance as i8), rotation ).unwrap();
            },
            PieceAction::checkerscapture(rotation) => {
                
                return RelativeSquare::new_from_perspective( (0,2), rotation).unwrap();
            },
            PieceAction::flick(_,_) => {
                panic!("i dont know the relative square a flick takes this piece");
            },
        }
    }
    
    //get the lift and move forces on the piece this action is applied to
    pub fn get_lift_and_move_forces(&self) -> Option<RelativeSquare>{
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                return Some( RelativeSquare::new(relativepos).unwrap()  );
            },
            PieceAction::slide( _,_ ) | PieceAction::capturelessslide(_,_)=> {
                
                return None;
            },
            PieceAction::checkerscapture(direction) => {
                
                return Some( RelativeSquare::new_from_perspective( (0,2), direction ).unwrap() );
            },
            PieceAction::flick(_,_) => {
                return None;
            },
        }
        
    }
    
    pub fn get_slide_forces(&self) -> Option<RelativeSquare>{
        
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                return None;
            },
            PieceAction::slide( rotation, distance ) | PieceAction::capturelessslide( rotation, distance) => {
                
                return RelativeSquare::new_from_perspective( (0, distance as i8), rotation );
            
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
    pub fn get_squares_dropped_relative(&self) -> Vec<( RelativeSquare, u32 )>{
        
        let mut toreturn = Vec::new();
        
        match *self{
            
            PieceAction::liftandmove( relativepos ) => {
                
                toreturn.push( (RelativeSquare::new(relativepos).unwrap() , 0) );
                
                return toreturn;
            },
            PieceAction::slide( rotation, distance) => {
                

                for x in 1..distance+1{

                    toreturn.push(
                        (RelativeSquare::new_from_perspective((0, distance as i8), rotation).unwrap(),
                        x as u32 * 5)
                    );
                }
                
                return toreturn;
            },
            PieceAction::checkerscapture(rotation) => {
                
                toreturn.push(
                    (RelativeSquare::new_from_perspective((0,2), rotation).unwrap(),
                    20)
                );
                
                return toreturn;
            },
            PieceAction::flick(_,_) | PieceAction::capturelessslide(_,_)=> {
                
                return toreturn;
            },
        }
    }
    
    
    //get the conditions for this action when it needs to capture to perform its action
    fn add_has_to_capture_conditions_for_action(&self, toaddto: &mut HashSet<( RelativeSquare, SquareCondition )>){
        
        //if this action drops squares
        if let Some(lastsquareandtick) = self.get_squares_dropped_relative().last(){

            //get the last square it drops
            let lastsquarerelativepos = lastsquareandtick.0.clone();

            //the final square dropped must have an opponents piece on it
            toaddto.insert( (lastsquarerelativepos, SquareCondition::OpponentRequired) );
        };
    }
    
    //the base conditions for this action
    //the squares it passes over must not have any friendly pieces
    //and the squares it passes over EXCEPT THE LAST ONE must not have any pieces / (no friendly or enemy pieces)
    fn add_base_conditions_for_action(&self, toaddto: &mut HashSet<( RelativeSquare, SquareCondition )>){
        

        match *self{
            
            PieceAction::flick(_,_) => {
                
            },
            PieceAction::liftandmove( relativepos ) => {
                
                toaddto.insert( (RelativeSquare::new(relativepos).unwrap(),  SquareCondition::NoneFriendlyRequired) );
                
            },
            PieceAction::checkerscapture( direction ) => {
                /*
                The square the checkers capture is going to must be empty
                and the squares that it drops must have an opponents piece on them
                */
                
                toaddto.insert( (self.get_relative_position_action_takes_piece(), SquareCondition::EmptyRequired) );
                
                for (relativesquarepos, _) in self.get_squares_dropped_relative(){
                    toaddto.insert( (relativesquarepos.clone(), SquareCondition::OpponentRequired) );
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
                        toaddto.insert( (x.0.clone(), SquareCondition::NoneFriendlyRequired) );
                    }
                    //if its not the last square being passed over
                    else{
                        toaddto.insert( (x.0.clone(), SquareCondition::EmptyRequired) );
                    }
                };

            },
            //all squares it passes over must be empty
            PieceAction::capturelessslide( rotation, distance ) =>{

                //get squares passed over relative
                for x in 1..=distance{

                    toaddto.insert( (
                        RelativeSquare::new_from_perspective( (0,x as i8), rotation ).unwrap(),
                        SquareCondition::EmptyRequired
                    ) );
                }
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
    fn is_action_valid(&self, ownerdirection: &f32, hasmoved: &bool, action: &PieceAction) -> bool{
        
        for validaction in &self.get_actions(ownerdirection, hasmoved){
            if validaction.is_equal(action){
                return true;
            }
        }        
        
        return false;
    }
    
    
    fn get_actions(&self, ownerdirection: &f32, hasmoved: &bool) -> Vec<PieceAction>{
        
        let mut toreturn: Vec<PieceAction> = Vec::new();
        
        match *self{
            
            Functionality::PawnAdvance =>{

                toreturn.push( PieceAction::slide_from_perspective(0.0, *ownerdirection, 1 ) );
                
                if hasmoved == &false{
                    toreturn.push( PieceAction::slide_from_perspective(0.0, *ownerdirection, 2 ) );
                }

                return toreturn;
            },
            Functionality::PawnCapture =>{
                
                //capturing diagonally
                toreturn.push( PieceAction::slide_from_perspective(0.125, *ownerdirection, 1 ) );

                toreturn.push( PieceAction::slide_from_perspective(0.875, *ownerdirection, 1 ) );

                return toreturn;
                
            },
            Functionality::Knight =>{
                
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (1,2), *ownerdirection ).unwrap()
                ));
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (2,1), *ownerdirection ).unwrap()
                ));
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (2,-1), *ownerdirection ).unwrap()
                ));
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (1,-2), *ownerdirection ).unwrap()
                ));


                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (-1,-2), *ownerdirection ).unwrap()
                ));
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (-2,-1), *ownerdirection ).unwrap()
                ));
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (-2,1), *ownerdirection ).unwrap()
                ));
                toreturn.push(
                    PieceAction::lift_and_move_from_relative_square(
                    RelativeSquare::new_from_perspective( (-1,2), *ownerdirection ).unwrap()
                ));
                
                
                return toreturn;
            },
            Functionality::Bishop =>{
                
                for dir in (1..8).step_by(2){
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            PieceAction::slide_from_perspective(rotation, *ownerdirection, dist)
                        );
                    }
                }
                
                return toreturn;
            },
            Functionality::Rook => {
                
                for dir in (0..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                    
                    for dist in 1..8{
                        toreturn.push(
                            PieceAction::slide_from_perspective(rotation, *ownerdirection, dist)
                        );
                    }
                }
                
                return toreturn;
            },
            Functionality::Queen => {
                
                for dir in 0..8{
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            PieceAction::slide_from_perspective(rotation, *ownerdirection, dist)
                        );
                    }
                }
                
                return toreturn;
            },
            Functionality::King => {
                


                for dir in 0..8{
                    let rotation = dir as f32 / 8.0;

                    toreturn.push(
                        PieceAction::slide_from_perspective(rotation, *ownerdirection, 1)
                    );
                }
                
                return toreturn;
            },
            Functionality::Checker => {
                
                for dir in (1..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                
                    toreturn.push(
                        PieceAction::slide_from_perspective(rotation, *ownerdirection, 1)
                    );

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
    fn get_conditions_for_action(&self, action: &PieceAction) -> HashSet< (RelativeSquare, SquareCondition)>{

        
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




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum PieceType{

    Nothing,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Checker
}

impl PieceType{

    pub fn value(&self) -> u8{

        match self{
            PieceType::Nothing => 0,
            PieceType::Pawn => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Rook => 4,
            PieceType::Queen => 5,
            PieceType::King => 1,
            PieceType::Checker => 2,
        }
    }

    fn default_functionalities(&self) -> HashSet<Functionality>{

        let mut toreturn = HashSet::new();


        match self{
            PieceType::Nothing => {
            },
            PieceType::Pawn => {
                toreturn.insert(Functionality::PawnAdvance);
                toreturn.insert(Functionality::PawnCapture);
            },
            PieceType::Knight => {
                toreturn.insert(Functionality::Knight);
            },
            PieceType::Bishop => {
                toreturn.insert(Functionality::Bishop);
            },
            PieceType::Rook => {
                toreturn.insert(Functionality::Rook);
            },
            PieceType::Queen => {
                toreturn.insert(Functionality::Queen);
            },
            PieceType::King => {
                toreturn.insert(Functionality::King);
            },
            PieceType::Checker => {
                toreturn.insert(Functionality::Checker);
            },
        }

        toreturn
    }

    //the name of the file that represents this objects image
    pub fn image_file(&self) -> String{

        match self{
            PieceType::Nothing => format!("none.png"),
            PieceType::Pawn => format!("pawn.png"),
            PieceType::Knight => format!("knight.png"),
            PieceType::Bishop => format!("bishop.png"),
            PieceType::Rook => format!("rook.png"),
            PieceType::Queen => format!("queen.png"),
            PieceType::King => format!("king.png"),
            PieceType::Checker => format!("checker.png"),
        }

    }

}





//a struct that has the information about the piece
//what type of piece it is
//what actions its allowed to perform
#[derive(Serialize, Deserialize, Clone)]
pub struct PieceData{
    
    //the name of the piece
    piecetype: PieceType,

    augmented: HashSet<Functionality>,
    
    value: u8,
    
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
            value: 0,
        }
    }

    pub fn is_this_piecetype(&self, piecetype: &PieceType) -> bool{

        return &self.piecetype == piecetype;
    }


    fn get_all_functionalities(&self) -> HashSet<Functionality>{
        self.piecetype.default_functionalities().into_iter().chain( self.augmented.clone() ).collect()
    }

    //give this piece the abilities of a knight
    pub fn augment_knight_abilities(&mut self){

        self.augmented.insert( Functionality::Knight );
    } 

    //remove any augmentations this piece might have, so it just has the default effects again
    pub fn remove_ability_augmentations(&mut self){

        self.augmented = HashSet::new();
    }


    pub fn get_value(&self) -> u8{  
        return self.value;
    }
    
    
    pub fn set_piecetype(&mut self, piecetype: PieceType){

        //panic!("setting epieceasstaaaaaaa{:?}", piecetype);
        
        self.value = piecetype.value();
        self.piecetype = piecetype;
    }
    
    
    pub fn moved_piece(&mut self){
        self.hasmoved = true;
    }
    
    pub fn get_image_location(&self) -> String{
        self.piecetype.image_file()
    }
    
    //get the piece actions that are listable
    pub fn get_numberable_piece_actions(&self, ownerdirection: &f32) -> Vec<PieceAction>{
        
        let mut toreturn = Vec::new();
        
        for functionality in &self.get_all_functionalities(){
            
            let allowedactions = functionality.get_actions(ownerdirection, &self.hasmoved);
            
            toreturn.extend(allowedactions);
        }
        
        toreturn
    }    
    
    
    //if this action is valid by the piecedata
    //return the conditions required for this action
    pub fn is_action_valid(&self, action: &PieceAction, ownerdirection: &f32) -> Option< HashSet< (RelativeSquare, SquareCondition)> >{
        
        for functionality in &self.get_all_functionalities(){
            
            if functionality.is_action_valid(ownerdirection, &self.hasmoved, action){
                
                return Some( functionality.get_conditions_for_action(action) );
            }
        }
        
        return None;
    }
    
    
}






