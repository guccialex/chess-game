

use rapier3d::geometry::Shape;
use rapier3d::na::Isometry3;


//the struct to return to the frontend to render the state of the game
pub struct VisibleGameBoardObject{
    
    pub isometry: Isometry3<f32>,
    
    pub id: u16,
    
    pub shape: Box<dyn Shape>,
    
    pub color: (f32,f32,f32),
    
    pub texturelocation: Option<String>,

    //the amount the piece is rotated if this is a piece
    pub rotation: Option<f32>,
}
