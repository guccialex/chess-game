use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RelativeSquare{
    
    relativepos: (i8,i8),    
}

impl RelativeSquare{
    
    pub fn new( relativepos: (i8,i8) ) -> Option<RelativeSquare>{
        
        if relativepos.0 >= -7 && relativepos.0 <= 7{
            
            if relativepos.1 >= -7 && relativepos.1 <= 7{
                
                //return the board square id
                return Some(  RelativeSquare{ relativepos: relativepos }   );
            };
        };
        return None;
    }
    
    pub fn new_from_perspective( relativepos: (i8,i8), perspectiverotation: f32 ) -> Option<RelativeSquare>{
        
        let rotpos = orthogonal_rotation::ortho_rotate_i8_point_at_point(relativepos , (0,0), perspectiverotation);
        
        return RelativeSquare::new( rotpos );
    }
    
    pub fn get_relative_pos(&self) -> (i8,i8){
        
        return self.relativepos;
    }
    
    
    //maybe pass in the offset this piece is from the square its on?
    pub fn to_relative_float(&self) -> (f32,f32){
        
        (self.relativepos.0 as f32, self.relativepos.1 as f32)
    }

}



#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct BoardSquarePosID{
    
    pos: (i8,i8),
    
}

impl BoardSquarePosID{
    
    pub fn get_range_on_self(&self) -> ((f32,f32),(f32,f32),(f32,f32)){
        
        let pos = self.to_physical_pos();
        let xrange = (pos.0 - 0.5, pos.0 + 0.5);
        let yrange = (-10.0, 10.0);
        let zrange = (pos.1 - 0.5, pos.1 + 0.5);
            
        return (xrange, yrange, zrange);
    }
    

    pub fn is_white(&self) -> bool{

        ((self.pos.0 + self.pos.1) % 2) == 0

    }

    pub fn new( pos: (i8,i8) ) -> Option<BoardSquarePosID>{
        
        //if its in range, return those integers, otherwise return none
        if pos.0 >= 0 && pos.0 <= 7{
            
            if pos.1 >= 0 && pos.1 <= 7{
                
                //return the board square id
                return Some(  BoardSquarePosID{ pos: pos }   );
            };
        };
        
        return None;
    }
    
    pub fn new_from_perspective( pos: (i8,i8), perspectiverotation: f32) -> Option<BoardSquarePosID>{
        
        let rotpos = orthogonal_rotation::ortho_rotate_i8_point_at_bot_left_of_point(
            pos, (4,4), perspectiverotation
        );

        //panic!("the points {:?}, to {:?}, rotamount{:?}", pos, rotpos, perspectiverotation);

        return BoardSquarePosID::new( rotpos );
    }
    
    pub fn from_physical_pos( fpos: (f32,f32) ) -> Option<BoardSquarePosID>{
        
        //add 4 to the center of it
        let newxpos = fpos.0 + 4.0;
        let newzpos = fpos.1 + 4.0;
        
        
        //round down, then convert to an integer
        let intxpos = newxpos.floor() as i8;
        let intzpos = newzpos.floor() as i8;
        
        
        BoardSquarePosID::new( (intxpos, intzpos) )
    }
    
    pub fn to_physical_pos(&self) -> (f32,f32){
        
        let mut xpos = self.pos.0 as f32;
        let mut zpos = self.pos.1 as f32;
        
        //subtract 3.5
        xpos = xpos - 3.5;
        zpos = zpos - 3.5;
        
        (xpos, zpos)
    }
    
    pub fn get_row(&self) -> i8{
        
        return self.pos.0;
    }
    
    pub fn get_pos(&self) -> (i8,i8){
        
        return self.pos;
    }
    
    pub fn new_from_added_relative_pos(&self, relativepos: RelativeSquare )  -> Option< BoardSquarePosID >{
        
        let oldposid = self.get_pos();
        
        let newposid = ( oldposid.0 + relativepos.get_relative_pos().0 ,  oldposid.1 + relativepos.get_relative_pos().1 );
        
        return BoardSquarePosID::new(newposid);
    }
}