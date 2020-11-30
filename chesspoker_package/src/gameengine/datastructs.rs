
use std::collections::HashMap;
use std::collections::HashSet;
use ncollide3d::shape::ConvexHull;









//used to determine how a piece can act
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllowedActions{    
    
    //what direction can it slide
    //what distance
    //does it have to capture an opponents piece to slide there
    //is it allowed to capture an opponents piece when sliding there
    slidedirection: HashSet<( u8, u8, bool, bool )>,
    
    
    
    //what relative positions can it move to
    //does it have to capture an opponents piece to move there
    //can it capture an opponents piece by moving there
    liftandmove: HashSet<( (i8, i8), bool, bool, )>,

    
}

//(direction, maxdistance, hastocapture, cancapture)

impl AllowedActions{
    
    pub fn get_allowed_slide_actions(&self, ownerdirection: u8) -> HashSet<( u8, u8, bool, bool )>{


        //rotate each allowed action in the slide by its owners direction
        let temp = self.slidedirection.clone();
        
        let mut toreturn = HashSet::new();

        for (direction, a, b, c) in temp.iter(){

            let newdirection = players_perspective_to_objective_perspective_slide(&ownerdirection, direction);

            toreturn.insert( (newdirection, *a, *b, *c)  );

        }

        toreturn


    }

    pub fn get_allowed_lift_and_move(&self, ownerdirection: u8) -> HashSet<( (i8, i8), bool, bool )>{
        
        //rotate each allowed action in the lift and move by its owners direction

        let temp = self.liftandmove.clone();
                        
        let mut toreturn = HashSet::new();


        for ( relativepos, a, b) in temp.iter(){

            if let Some(newrelativepos) = players_perspective_to_objective_perspective_lift(&ownerdirection, relativepos){

                toreturn.insert( (newrelativepos, *a, *b)  );

            }

        }



        toreturn

    }
    
    //get the allowed actions of a pawn that has not moved yet
    pub fn get_unmoved_pawn() -> AllowedActions{
        
        
        let mut slidedirection = HashSet::new();
        
        //moving forwards thing
        slidedirection.insert( (0, 2, false, false) );
        
        //capturing diagonally
        slidedirection.insert( (1, 1, true, true) );
        slidedirection.insert( (7, 1, true, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        
        
    }

    //get the allowed actions of a pawn that has been moved
    pub fn get_moved_pawn() -> AllowedActions{
        
        let mut slidedirection = HashSet::new();
        
        //moving forwards thing
        slidedirection.insert( (0, 1, false, false) );
        
        //capturing diagonally
        slidedirection.insert( (1, 1, true, true) );
        slidedirection.insert( (7, 1, true, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        
        
        
    }

    pub fn get_knight() -> AllowedActions{


        let mut slidedirection = HashSet::new();
        
        let mut liftandmove = HashSet::new();
        
        liftandmove.insert( ( (1,2), false, true   ) );
        liftandmove.insert( ( (2,1), false, true   ) );
        liftandmove.insert( ( (2,-1), false, true  ) );
        liftandmove.insert( ( (1,-2), false, true  ) );
        
        liftandmove.insert( ( (-1,-2), false, true ) );
        liftandmove.insert( ( (-2,-1), false, true ) );
        liftandmove.insert( ( (-2,1), false, true  ) );
        liftandmove.insert( ( (-1,2), false, true  ) );

        
        AllowedActions{
            
            liftandmove: liftandmove,
            
            slidedirection: slidedirection,
            
        }


    }

    pub fn get_bishop() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any diagonal direction
        slidedirection.insert( (1, 7, false, true) );
        slidedirection.insert( (3, 7, false, true) );
        slidedirection.insert( (5, 7, false, true) );
        slidedirection.insert( (7, 7, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }

    pub fn get_rook() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any diagonal direction
        slidedirection.insert( (0, 7, false, true) );
        slidedirection.insert( (2, 7, false, true) );
        slidedirection.insert( (4, 7, false, true) );
        slidedirection.insert( (6, 7, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }
    
    pub fn get_queen() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any orthogonal direction
        slidedirection.insert( (0, 7, false, true) );
        slidedirection.insert( (2, 7, false, true) );
        slidedirection.insert( (4, 7, false, true) );
        slidedirection.insert( (6, 7, false, true) );

        //move in any diagonal direction
        slidedirection.insert( (1, 7, false, true) );
        slidedirection.insert( (3, 7, false, true) );
        slidedirection.insert( (5, 7, false, true) );
        slidedirection.insert( (7, 7, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }

    pub fn get_king() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any orthogonal direction
        slidedirection.insert( (0, 1, false, true) );
        slidedirection.insert( (2, 1, false, true) );
        slidedirection.insert( (4, 1, false, true) );
        slidedirection.insert( (6, 1, false, true) );

        //move in any diagonal direction
        slidedirection.insert( (1, 1, false, true) );
        slidedirection.insert( (3, 1, false, true) );
        slidedirection.insert( (5, 1, false, true) );
        slidedirection.insert( (7, 1, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }
    
    
}







fn players_perspective_to_objective_perspective_slide(playerdirection: &u8, slidedirection: &u8) -> u8{

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

            return  Some( (resultingx, resultingy) )  ;

        }

    }


    //else return None
    return( None );
    



}









use serde::{Serialize, Deserialize};
use nalgebra::Vector3;







pub fn slide_id_to_direction_change_from_objective_perspective(slideid: u8) -> (i32,i32){
    
    
    
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
