
use std::collections::HashMap;
use std::collections::HashSet;
//use ncollide3d::shape::ConvexHull;

use serde::{Serialize, Deserialize};

mod boardsquarestructs;

pub use boardsquarestructs::BoardSquarePosID;
pub use boardsquarestructs::RelativeSquare;


//ive lost sight of ht egoal im working towards
//of what would make this "good code"
//I cant settle on a way to structure this that makes it clear its the best way, or even a good way
//so I think im just going to aim for doing it in the shortest amount of lines


fn dir_from_pers(objectdirection: f32, playerdirection: f32) -> f32{

    return objectdirection + playerdirection;
}




//what condition has to be met on this boardsquare?
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SquareCondition{
    
    OpponentRequired,
    NoneFriendlyRequired,
    EmptyRequired,
}


#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
enum TypeOfMovement{
    Slide,
    Lift,
}


#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct FullAction{

    destination: Option<(RelativeSquare, TypeOfMovement)>,

    squaresdropped: HashSet<( RelativeSquare, u32 )>,

    conditions: HashSet<( RelativeSquare, SquareCondition )>,


    //flick force to apply to the piece
    force: Option<(f32,f32)>,
}


impl FullAction{


    fn new_checkers_capture(direction: &f32) -> FullAction{

        let opponentsposition = RelativeSquare::new_from_perspective( (0,1), *direction).unwrap();
        let destinationposition = RelativeSquare::new_from_perspective( (0,2), *direction ).unwrap();

        let mut squaresdropped = HashSet::new();
        squaresdropped.insert( (opponentsposition.clone(),  0)  );


        let mut conditions = HashSet::new();
        conditions.insert(  (opponentsposition, SquareCondition::OpponentRequired)  );
        conditions.insert(  (destinationposition.clone(), SquareCondition::EmptyRequired)  );


        FullAction{
            destination: Some( (destinationposition, TypeOfMovement::Lift) ),

            squaresdropped: squaresdropped,

            conditions: conditions,

            force: None,
        }
    }


    fn new_cant_capture_slide(direction: &f32, distance: &u8) -> FullAction{

        let mut newslide = FullAction::new_slide(direction, distance);

        newslide.conditions.insert( (newslide.get_destination_square().unwrap(), SquareCondition::EmptyRequired)  );

        newslide
    }


    fn new_must_capture_slide( direction: &f32, distance: &u8) -> FullAction{

        let mut newslide = FullAction::new_slide(direction, distance);

        newslide.conditions.insert( (newslide.get_destination_square().unwrap(), SquareCondition::OpponentRequired)  );

        newslide
    }


    fn new_slide( direction: &f32, distance: &u8) -> FullAction{

        let mut squaresdropped = HashSet::new();
        let mut conditions = HashSet::new();
        let mut destinationposition = None;


        for curdistance in 1..=*distance{
            let cursquare = RelativeSquare::new_from_perspective( (0, curdistance as i8),  *direction).unwrap();
            
            squaresdropped.insert(  (cursquare.clone(), curdistance as u32 *5)  );

            if *distance == curdistance{
                destinationposition = Some(cursquare.clone());
                
                conditions.insert( (cursquare, SquareCondition::NoneFriendlyRequired) );
            }
            else{    

                conditions.insert( (cursquare, SquareCondition::EmptyRequired) );
            }
        }


        FullAction{
            destination: Some( (destinationposition.unwrap(), TypeOfMovement::Slide) ),

            squaresdropped: squaresdropped,

            conditions: conditions,

            force: None,
        }
    }

    fn new_flick( direction:&f32, force: &f32 ) -> FullAction{

        FullAction{
            destination: None,

            squaresdropped: HashSet::new(),

            conditions: HashSet::new(),

            force: Some( (*direction, *force) ),
        }
    }

    fn new_lift_and_move( destinationsquare: &RelativeSquare ) -> FullAction{

        let mut squaresdropped = HashSet::new();
        squaresdropped.insert( (destinationsquare.clone(), 0) );

        let mut conditions = HashSet::new();
        conditions.insert( (destinationsquare.clone(), SquareCondition::NoneFriendlyRequired) );

        FullAction{
            destination: Some( (destinationsquare.clone(), TypeOfMovement::Lift) ),

            squaresdropped: squaresdropped,

            conditions: conditions,

            force: None,
        }
    }


    fn is_equal(&self, other:&FullAction) -> bool{

        return self == other;
    }
    




    //get the squares dropped by this action and the tick it happens
    pub fn get_squares_dropped(&self) -> HashSet<( RelativeSquare, u32 )>{
        
        self.squaresdropped.clone()
    }

    pub fn get_flick_forces(&self) -> Option<(f32,f32)> {
        
        self.force
    }


    //get the square moved to
    pub fn get_destination_square(&self) -> Option<RelativeSquare>{
        
        if let Some( (destination, movementtype) ) = &self.destination{

            return Some(destination.clone());
        }

        return None;
    }

    pub fn get_conditions(&self) -> HashSet<( RelativeSquare, SquareCondition )>{

        self.conditions.clone()

    }


    pub fn is_lifted(&self) -> bool{

        if let Some( (_, movementtype) ) = &self.destination{

            if movementtype == &TypeOfMovement::Lift{

                return true;
            }
        }

        return false;
    }

}














#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
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

    fn value(&self) -> u8{

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

    fn get_actions(&self, ownerdirection: &f32, hasmoved: &bool) -> Vec<FullAction>{

        let mut toreturn: Vec<FullAction> = Vec::new();

        match *self{
            
            PieceType::Pawn =>{

                toreturn.push( 
                    FullAction::new_cant_capture_slide(&dir_from_pers(0.0, *ownerdirection), &1)
                );
                
                if hasmoved == &false{
                    toreturn.push( 
                        FullAction::new_cant_capture_slide(&dir_from_pers(0.0, *ownerdirection), &2)
                    );
                }
                
                toreturn.push( 
                    FullAction::new_must_capture_slide(&dir_from_pers(0.875, *ownerdirection), &1)
                );

                toreturn.push( 
                    FullAction::new_must_capture_slide(&dir_from_pers(0.125, *ownerdirection), &1)
                );

                return toreturn;
            },
            PieceType::Knight =>{
                
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (1,2), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (2,1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (2,-1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (1,-2), *ownerdirection ).unwrap()
                    )
                );


                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-1,-2), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-2,-1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-2,1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-1,2), *ownerdirection ).unwrap()
                    )
                );
                
                return toreturn;
            },
            PieceType::Bishop =>{
                
                for dir in (1..8).step_by(2){
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &dist)
                        );
                    }
                }
                
                return toreturn;
            },
            PieceType::Rook => {
                
                for dir in (0..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                    
                    for dist in 1..8{
                        
                        toreturn.push(
                            FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &dist)
                        );
                    }
                }
                
                return toreturn;
            },
            PieceType::Queen => {
                
                for dir in 0..8{
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &dist)
                        );
                    }
                }
                
                return toreturn;
            },
            PieceType::King => {
                for dir in 0..8{
                    let rotation = dir as f32 / 8.0;

                    toreturn.push(
                        FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &1)
                    );
                }
                
                return toreturn;
            },
            PieceType::Checker => {
                
                for dir in (1..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                
                    toreturn.push(

                        FullAction::new_checkers_capture( &dir_from_pers(rotation, *ownerdirection) )
                    );

                    toreturn.push(

                        FullAction::new_cant_capture_slide( &dir_from_pers(rotation, *ownerdirection), &1)
                    );
                }
                
                return toreturn;
            },
            PieceType::Nothing => return toreturn,  
        };
    }

    //the name of the file that represents this objects image
    fn image_file(&self) -> String{

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


    
    fn is_action_valid(&self, ownerdirection: &f32, hasmoved: &bool, action: &FullAction) -> bool{
        
        for validaction in &self.get_actions(ownerdirection, hasmoved){
            if validaction.is_equal(action){
                return true;
            }
        }
        
        return false;
    }
    


}





//a struct that has the information about the piece
//what type of piece it is
//what actions its allowed to perform
#[derive(Serialize, Deserialize, Clone)]
pub struct PieceData{
    
    //the name of the piece
    piecetype: PieceType,

    augmented: HashSet<PieceType>,
    
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


    //give this piece the abilities of a knight
    pub fn augment_knight_abilities(&mut self){

        self.augmented.insert( PieceType::Knight );
    } 

    //remove any augmentations this piece might have, so it just has the default effects again
    pub fn remove_ability_augmentations(&mut self){

        self.augmented = HashSet::new();
    }


    pub fn get_value(&self) -> u8{  
        return self.value;
    }
    
    
    pub fn set_piecetype(&mut self, piecetype: PieceType){
        
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
    pub fn get_numberable_actions(&self, ownerdirection: &f32) -> Vec<FullAction>{

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
    pub fn is_action_valid(&self, action: &FullAction, ownerdirection: &f32) -> bool{
        
        let mut alltypes = HashSet::new();

        alltypes.insert( self.piecetype.clone() );
        alltypes.extend( self.augmented.clone() );

        
        for piecetype in alltypes{

            if piecetype.is_action_valid( ownerdirection, &self.hasmoved, action   ){

                return true;
            };


        }

        return false;        
    }

       
}





/*
//theres piece data
all information about a certain piece
call to get all actions valid

//actions
what an action does, and what its conditions are

//type
The name of the piece
And the actions associated with each piece

*/