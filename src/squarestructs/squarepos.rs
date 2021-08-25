use serde::{Serialize, Deserialize};

use super::relativesquare::RelativeSquare;



#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct SquarePos{
    
    pos: (i8,i8),
    
}

impl SquarePos{

    /*
    //get all the square positions on a regular board
    pub fn get_all_default_square_pos() -> Vec<SquarePos>{

        let mut toreturn = Vec::new();

        for x in 0..8{
            for y in 0..8{
                toreturn.push(  SquarePos::new( (x,y) )   );
            }
        }

        toreturn
    }
    */
    
    pub fn new( pos: (i8,i8) ) -> SquarePos{

        return   SquarePos{ pos: pos }   ;
    }


    pub fn from_physical_pos( pos: (f32,f32,f32) ) -> Option<SquarePos>{
        
        //add 4 to the center of it
        let newxpos = pos.0 + 4.0;
        let newzpos = pos.2 + 4.0;
        
        //round down, then convert to an integer
        let intxpos = newxpos.floor() as i8;
        let intzpos = newzpos.floor() as i8;

        //if its above -2
        if pos.1 > -2.0{

            return Some(  SquarePos::new( (intxpos, intzpos) )   );
        }
        else{
            return None;
        }
        
    }
    

    //the default position of the square
    pub fn get_default_physical_pos(&self) -> (f32, f32, f32){

        let mut xpos = self.pos.0 as f32;
        let mut zpos = self.pos.1 as f32;
        
        //subtract 3.5
        xpos = xpos - 3.5;
        zpos = zpos - 3.5;
        
        (xpos, 0.0, zpos)
    }

    

    pub fn is_white(&self) -> bool{

        ((self.pos.0 + self.pos.1) % 2) == 0
    }

    
    pub fn new_from_perspective( pos: (i8,i8), perspectiverotation: f32) -> SquarePos{
        
        let rotpos = orthogonal_rotation::ortho_rotate_i8_point_at_bot_left_of_point(
            pos, (4,4), perspectiverotation
        );


        return SquarePos::new( rotpos );
    }
    
    
    /*
    pub fn get_row(&self) -> i8{
        return self.pos.0;
    }
    
    pub fn get_pos(&self) -> (i8,i8){
        return self.pos;
    }
    */

    pub fn is_backrow(&self) -> bool{

        if self.pos.1 == 8 || self.pos.1 == 7{

            return true;
        }
        else{

            return false;
        }

    }

    pub fn new_from_added_relative_pos(&self, relativepos: RelativeSquare )  ->  SquarePos {
        
        let oldposid = self.pos;
        
        let newposid = ( oldposid.0 + relativepos.get_relative_pos().0 ,  oldposid.1 + relativepos.get_relative_pos().1 );
        
        return SquarePos::new(newposid);
    }
}