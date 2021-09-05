use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RelativeSquare{
    
    relativepos: (i8,i8),    
}

impl RelativeSquare{
    
    pub fn new( relativepos: (i8,i8) ) -> RelativeSquare{
        
        return  RelativeSquare{ relativepos: relativepos }   ;
    }


    pub fn to_radians(&self) -> f32{

        let x = self.relativepos.0 as f32;
        let y = self.relativepos.1 as f32;

        y.atan2(x)
    }

    pub fn absolute_distance(&self) -> f32{

        let x = self.relativepos.0 as f32;
        let y = self.relativepos.1 as f32;

        return (x * x + y * y).sqrt();

    }


    pub fn from_distance_and_rotation(distance: u8, rotation: f32) -> RelativeSquare{

        let pos = orthogonal_rotation::ortho_rotate_i8_point_at_point( (0, distance as i8) , (0,0), rotation);
        
        return RelativeSquare::new( pos );
    }
    
    pub fn new_from_perspective( relativepos: (i8,i8), perspectiverotation: f32 ) -> RelativeSquare{
        
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

