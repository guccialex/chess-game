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

