use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use super::relativesquare::RelativeSquare;
use super::typeofmovement::TypeOfMovement;
use super::squarecondition::SquareCondition;


//the definition of an action
#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct FullAction{

    


    destination: Option<(RelativeSquare, TypeOfMovement)>,

    squaresdropped: HashSet<( RelativeSquare, u32 )>,

    //the squares that are captured
    capturedsquares: HashSet< RelativeSquare >,

    conditions: HashSet<( RelativeSquare, SquareCondition )>,


    //flick force to apply to the piece
    force: Option<(f32,f32)>,
}


impl FullAction{

    pub fn new_checkers_capture(direction: &f32) -> FullAction{

        let opponentsposition = RelativeSquare::new_from_perspective( (0,1), *direction).unwrap();
        let destinationposition = RelativeSquare::new_from_perspective( (0,2), *direction ).unwrap();

        let mut squaresdropped = HashSet::new();
        squaresdropped.insert( (opponentsposition.clone(),  0)  );


        let mut conditions = HashSet::new();
        conditions.insert(  (opponentsposition.clone(), SquareCondition::OpponentRequired)  );
        conditions.insert(  (destinationposition.clone(), SquareCondition::EmptyRequired)  );

        let mut capturedsquares = HashSet::new();
        capturedsquares.insert(opponentsposition.clone());

        FullAction{
            destination: Some( (destinationposition, TypeOfMovement::Lift) ),

            squaresdropped: squaresdropped,

            capturedsquares: capturedsquares,

            conditions: conditions,

            force: None,
        }
    }

    pub fn new_cant_capture_slide(direction: &f32, distance: &u8) -> FullAction{

        let mut newslide = FullAction::new_slide(direction, distance);

        newslide.conditions.insert( (newslide.get_destination_square().unwrap(), SquareCondition::EmptyRequired)  );

        newslide
    }

    pub fn new_must_capture_slide( direction: &f32, distance: &u8) -> FullAction{

        let mut newslide = FullAction::new_slide(direction, distance);

        newslide.conditions.insert( (newslide.get_destination_square().unwrap(), SquareCondition::OpponentRequired)  );

        newslide
    }

    pub fn new_slide( direction: &f32, distance: &u8) -> FullAction{

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

        
        let mut capturedsquares = HashSet::new();
        capturedsquares.insert(destinationposition.clone().unwrap());

        FullAction{
            destination: Some( (destinationposition.unwrap(), TypeOfMovement::Slide) ),

            squaresdropped: squaresdropped,

            conditions: conditions,

            capturedsquares: capturedsquares,

            force: None,
        }
    }
    
    pub fn new_flick( direction:&f32, force: &f32 ) -> FullAction{

        FullAction{
            destination: None,

            squaresdropped: HashSet::new(),

            conditions: HashSet::new(),

            capturedsquares: HashSet::new(),

            force: Some( (*direction, *force) ),
        }
    }

    pub fn new_lift_and_move( destinationsquare: &RelativeSquare ) -> FullAction{

        let mut squaresdropped = HashSet::new();
        squaresdropped.insert( (destinationsquare.clone(), 0) );

        let mut conditions = HashSet::new();
        conditions.insert( (destinationsquare.clone(), SquareCondition::NoneFriendlyRequired) );

        
        let mut capturedsquares = HashSet::new();
        capturedsquares.insert(destinationsquare.clone());

        FullAction{
            destination: Some( (destinationsquare.clone(), TypeOfMovement::Lift) ),

            squaresdropped: squaresdropped,

            conditions: conditions,
            
            capturedsquares: capturedsquares,

            force: None,
        }
    }

    fn is_equal(&self, other:&FullAction) -> bool{

        return self == other;
    }

    //the squares it captures

    //get the squares dropped by this action and the tick it happens
    pub fn get_squares_dropped(&self) -> HashSet<( RelativeSquare, u32 )>{
        
        self.squaresdropped.clone()
    }

    pub fn get_squares_captured(&self) -> HashSet<RelativeSquare>{

        self.capturedsquares.clone()

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

    //given the state of the 
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



