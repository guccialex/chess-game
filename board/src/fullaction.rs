use crate::squarestructs::RelativeSquare;
use crate::squarestructs::SquarePos;
use crate::boardobject::BoardObject;
use crate::boardobject::Piece;
use crate::boardobject::Square;
use serde::{Serialize, Deserialize};





//how much should full action be specified?
//and how much should it be modified by its piecedata
//i think i should define it so that the physical engine has a full idea about what to do with it
//but the piecedata has to use more data to determine what teh square conditions should be

//or like, only define enough that it can be differentiated by both
//or so like the piecetype can determine its actions?

//i THINK this is fine
//just slide, lift and move, and checkers capture


//the physical effects


#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum CaptureType{
    CantCapture,
    MustCapture,
    OptionallyCapture
}



#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum FullAction{

    //the relative square it moves to
    LiftAndMove( RelativeSquare ),


    //the direction, distance and capture type?
    Slide( f32, u8 , CaptureType ),
    
    //the captured square
    //the destination square
    CheckersCapture( RelativeSquare, RelativeSquare ),


    //a flick that drops a square
    //a flick that doesnt drop a certain square

    //direction, force
    Flick(f32, f32),
}


//get the square destination
impl FullAction{


    //turn self into a flick
    pub fn into_flick(&mut self){


        if let Some(rel) = self.destination(){

            *self = FullAction::Flick( rel.to_radians() , 50.0*rel.absolute_distance() );


        }
        else{


            *self = FullAction::Flick( 0.0 , 100.0 );
        }

    } 

    pub fn is_lift(&self) -> bool{

        match self{

            FullAction::CheckersCapture(_,_) => { return true },

            FullAction::LiftAndMove( _) => {return true},

            FullAction::Slide(_,_,_) => {return false},

            _ => {  panic!("isnt lift or slide") },

        }


    }


    pub fn rotate(&mut self, rotation: &f32){

        if let FullAction::LiftAndMove(pos ) = self{
            *pos = RelativeSquare::new_from_perspective( pos.get_relative_pos(), *rotation);
        }
        else if let FullAction::Slide( rot, _, _) = self{
            *rot = (*rot + rotation) % 1.0;
        }
        else if let FullAction::CheckersCapture( cap, dest) = self{
            *cap = RelativeSquare::new_from_perspective( cap.get_relative_pos(), *rotation);
            *dest = RelativeSquare::new_from_perspective( dest.get_relative_pos(), *rotation);
        }
        else{
            //dont panic, but a flick cant be rotated
            //panic!("cant rotate this");
        }
    }




    //the relative square this piece is taken to
    pub fn destination(&self) -> Option<RelativeSquare>{

        match self{
            FullAction::LiftAndMove( x ) => {return Some(x.clone())},

            FullAction::Slide( rot, dist , _) => {return Some( RelativeSquare::from_distance_and_rotation(*dist, *rot) )},
            
            FullAction::CheckersCapture( _, dest ) => {return Some(dest.clone())},

            FullAction::Flick(_, _) => {return None;},
        }
    }



    //the squares it passes over in order EXCEPT THE LAST ONE
    pub fn passes_over(&self) -> Vec<RelativeSquare>{

        let mut toreturn = Vec::new();

        //if its a slide
        if let FullAction::Slide( rot, dist, _ ) = self{

            //the squares it passes over, excluding the final one
            for x in 1..*dist{

                toreturn.push(  RelativeSquare::from_distance_and_rotation(x, *rot)  );
            }
        }

        toreturn
    }



    //the squares it captures
    pub fn captures(&self) -> Option<RelativeSquare>{

        match self{
            FullAction::LiftAndMove( _ ) => {
                return self.destination();
            },
            FullAction::Slide( _, _,  _ ) => {
                return self.destination() ;
            },
            FullAction::CheckersCapture( cap, _ ) => {
                return Some( cap.clone() );
            },
            FullAction::Flick(_,_) =>{
                return None;
            }
        }
    }

}

