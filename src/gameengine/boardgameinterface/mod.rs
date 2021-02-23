
use std::collections::HashSet;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};


mod physicsengine;
use physicsengine::RapierPhysicsEngine;
use physicsengine::RapierMissionExtender;


use physicsengine::Mission;
use physicsengine::MissionType;



/*

methods:

add piece -> u16

remove piece (u16)


change piece shapes


slide piece

lift and drop piece



raise square

drop square


get raised squares

get dropped squares

get empty squares not on mission

*/






//the state of the board game
#[derive(Serialize, Deserialize)]
pub struct BoardGame{
    
    
    //the list of piece as IDs
    pieces: HashSet<u16>,
    
    //the list of board squares to their physical object ID
    boardsquares: HashMap<(u8,u8), u16>,
    
    
    
    physicsengine: RapierPhysicsEngine,
    
    
    missions: RapierMissionExtender,
    
}





impl BoardGame{
    
    
    pub fn new_empty_board() -> BoardGame{
        
        
        let mut boardgame = BoardGame{
            
            pieces: HashSet::new(),
            boardsquares: HashMap::new(),
            physicsengine: RapierPhysicsEngine::new(),
            
            missions: RapierMissionExtender::new(),
            
            
        };
        
        
        //create the 4 invisible walls bordering the game
        {
            
            let horizontalwalldimensions = (20.0 , 20.0 , 4.0);
            let verticalwalldimensions = (4.0 , 20.0 , 20.0 );
            
            
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_translation( &physicalid,  (0.0,0.0,-6.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape_cuboid(&physicalid, horizontalwalldimensions );
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_translation( &physicalid,  (0.0,0.0,6.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape_cuboid(&physicalid, horizontalwalldimensions );
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_translation( &physicalid,  (-6.0,0.0,0.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape_cuboid(&physicalid, verticalwalldimensions );
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_translation( &physicalid,  (6.0,0.0,0.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape_cuboid(&physicalid, verticalwalldimensions );
        }
        
        
        //create the boardsquares
        //set up the boardsquares
        {
            let boardxsize = 8;
            let boardysize = 8;
            
            
            //create the 64 squares of the board as objects
            for x in 0..boardxsize{
                
                for z in 0..boardysize{
                    
                    boardgame.create_boardsquare( (x,z) );
                    
                }
            }
        }
        
        
        boardgame
    }
    
    
    
    pub fn get_raised_squares(&self) -> Vec<u16>{
        self.missions.get_active_missions_of_type(MissionType::RaiseSquare)
    }
    
    pub fn get_dropped_squares(&self) -> Vec<u16>{

        self.missions.get_active_missions_of_type(MissionType::DropSquare)
    }
    
    
    
    pub fn remove_piece(&mut self, pieceid: &u16){
        
        self.missions.end_missions(pieceid, &mut self.physicsengine);
        
        self.pieces.remove(pieceid);
        
        self.physicsengine.remove_object(pieceid);
    }
    
    //make their physical properties that of a ball or a piece
    pub fn make_object_pool_ball_shape(&mut self, objectid: &u16) {
        
        if ! self.pieces.contains(objectid){
            panic!("What else could this object be other than a piece?");
        }
        
        
        //make it a ball
        self.physicsengine.set_shape_sphere(objectid, 0.7);
        
        //move it up, or itll sink through the floor when ccd is on
        self.physicsengine.apply_delta_position(objectid , (0.0, 1.0, 0.0));
        
        //elasticity and friction
        self.physicsengine.set_materials(objectid, 1.0, 1.0);
        
        //unlock all the axis of rotation
        //self.physicsengine.set_kinematic_axis_of_rotation_locked( objectid, (false,false,false) );
    }
    
    pub fn make_object_piece_shape(&mut self, objectid: &u16) {
        
        if ! self.pieces.contains(objectid){
            
            panic!("What else could this object be other than a piece?");
        }
        
        self.physicsengine.set_shape_cylinder(objectid, 0.5, 0.7 );        
        
        //elasticity and friction
        self.physicsengine.set_materials(objectid, 0.5, 0.5);
        
    }
    
    pub fn new_piece(&mut self, posid:(u8,u8) ) -> u16{
        
        let pos = BoardGame::boardsquare_posid_physical_pos(posid);        
        
        let pieceid = self.physicsengine.add_object();
        self.pieces.insert(pieceid);
        
        self.physicsengine.set_translation( &pieceid, ( pos.0 , 4.0 , pos.1 ) );
        
        self.make_object_piece_shape(&pieceid);
        
        return pieceid;
    }
    
    
    pub fn end_mission(&mut self, id: &u16){
        
        self.missions.end_missions(id, &mut self.physicsengine);
    }
    
    
    
    
    
    
    pub fn slide_piece(&mut self, pieceid: u16, mut relativepos: (f32,f32)){
        
        
        if self.get_board_square_piece_is_on(pieceid).is_none() {
            return ();
        }
        
        
        let startsquareid = self.get_board_square_piece_is_on(pieceid).unwrap();
        let pieceoffset = self.piece_on_square_offset(pieceid, startsquareid);
        
        //slide an additional distance that this piece is offset by so it slides
        //to the center of the new piece
        relativepos.0 = relativepos.0 - pieceoffset.0;
        relativepos.1 = relativepos.1 - pieceoffset.1;
        
        //slide to the center of a piece
        let slidemission = Mission::make_slide_mission( relativepos );
        
        self.missions.set_future_mission(25, pieceid, slidemission);        
        
    }
    
    
    //flick a piece in a direction (radians), with a force
    pub fn flick_piece(&mut self, objectid: u16, direction: f32, force: f32){
        
        //create a mission
        let flickmission = Mission::make_flick_mission( direction, force);
        
        self.missions.set_mission(objectid, flickmission );
    }
    
    //lift and move a piece to another position
    pub fn lift_and_move_piece_to(&mut self, pieceid: u16, mut relativepos: (f32,f32)){
        
        
        //get the board square this piece is on
        if let Some(boardsquare) = self.get_board_square_piece_is_on(pieceid){
            
            //get the difference between this piece and the center of the board square its on
            let offset = self.piece_on_square_offset(pieceid, boardsquare);
            
            //create the mission for the piece
            relativepos.0 = relativepos.0 - offset.0;
            relativepos.1 = relativepos.1 - offset.1;
            
            let liftandmovemission = Mission::make_lift_mission( relativepos );
            
            self.missions.set_mission(pieceid, liftandmovemission);
            
        }
        
    }
    
    
    
    
    
    
    
    pub fn set_long_boardsquare_drop(&mut self, length: u32, boardsquare: u16){
        
        let mut mission = Mission::make_lengthed_drop(length);
        self.mission_set_current_pos_as_default(&boardsquare, &mut mission);
        
        self.missions.set_mission(boardsquare, mission);
        
    }
    
    pub fn set_long_boardsquare_raise(&mut self, length: u32, boardsquare: u16){
        
        let mut mission = Mission::make_lengthed_raise(length);
        self.mission_set_current_pos_as_default(&boardsquare, &mut mission);
        
        self.missions.set_mission(boardsquare, mission);
        
    }
    
    pub fn set_future_boardsquare_drop(&mut self, ticks: u32, bsid: u16){
        
        let mut mission = Mission::make_drop_and_loop_around();
        
        self.mission_set_current_pos_as_default(&bsid, &mut mission);
        
        self.missions.set_future_mission(ticks, bsid, mission);
    }
    
    
    
    //set the current position of the object as the default position on the object
    fn mission_set_current_pos_as_default(&self, id: &u16,  mission: &mut Mission){
        
        let pos = self.physicsengine.get_translation(id);
        let rot = self.physicsengine.get_rotation(id);
        
        mission.set_default_isometry(pos, rot);
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn tick(&mut self){
        
        self.missions.tick_missions(&mut self.physicsengine);
        
        //tick the physics world
        self.physicsengine.tick();
    }
    
    
    
    
    //get a pieces offset on the square its on
    fn piece_on_square_offset(&self, pieceid: u16, square: u16) -> (f32,f32){
        
        let squareposid = self.boardsquare_id_to_posid(square).unwrap();
        
        //position of the board square
        let squarepos = BoardGame::boardsquare_posid_physical_pos( squareposid );
        
        //get the position of the piece
        let piecepos = self.physicsengine.get_translation(&pieceid);
        
        //get the pieces x and z position and subtract the position of the piece its on from it
        let xoffset = piecepos.0 - squarepos.0;
        let zoffset = piecepos.2 - squarepos.1;
        
        
        return (xoffset, zoffset);
    }
    
    
    fn create_boardsquare(&mut self, posid: (u8,u8)){
        
        let physicalid = self.physicsengine.add_object();                    
        self.boardsquares.insert( posid, physicalid );
        
        
        let ypos = 0.0;
        let (xpos, zpos) = BoardGame::boardsquare_posid_physical_pos(posid);
        
        self.physicsengine.set_translation( &physicalid, ( xpos , ypos ,zpos  ) );
        self.physicsengine.make_static(&physicalid);
        self.physicsengine.set_materials(&physicalid, 0.0, 0.0);
        self.physicsengine.set_shape_cuboid(&physicalid, (1.0, 1.0, 1.0) );
        
    }
    
    
    
    
    
    
    
    
    /*
    the functions I need conversion between
    
    bs float pos
    <->
    bs posid
    <->
    bs id
    
    what piece on top of this square
    what square under this piece
    */
    
    
    
    //get the id of the boardsquare by its position
    pub fn boardsquare_posid_to_id(&self, pos: (u8,u8) ) -> Option<u16>{
        
        if let Some(bsid) = self.boardsquares.get(&pos){
            
            return Some(*bsid);
        }
        else{
            
            return None;
        };
        
    }
    
    //get the position of the boardsquare by its id
    //needs to be public for "is this boardsquare white"
    pub fn boardsquare_id_to_posid(&self, id: u16) -> Option<(u8,u8)>{
        
        
        for (curpos, curid) in &self.boardsquares{
            
            if id == *curid{
                return Some(*curpos);
            }
            
        }
        
        return None;
    }
    
    
    
    
    //bs float pos -> bs posid
    fn boardsquare_physical_pos_to_posid( xpos: f32, zpos: f32 ) -> Option<(u8,u8)>{
        
        //add 4 to the center of it
        let newxpos = xpos + 4.0;
        let newzpos = zpos + 4.0;
        
        
        //round down, then convert to an integer
        let intxpos = newxpos.floor() as i32;
        let intzpos = newzpos.floor() as i32;
        
        
        //if its in range, return those integers, otherwise return none
        if intxpos >= 0 && intxpos <= 7{
            
            if intzpos >= 0 && intzpos <= 7{
                
                //return the board square id
                return Some(  (intxpos as u8, intzpos as u8)  )  ;
                
            };
        };
        
        
        return None;
    }
    
    //bs posid -> bs float pos
    fn boardsquare_posid_physical_pos( boardsquare:(u8,u8) ) -> (f32,f32) {
        
        let mut xpos = boardsquare.0 as f32;
        let mut zpos = boardsquare.1 as f32;
        
        //subtract 3.5
        xpos = xpos - 3.5;
        zpos = zpos - 3.5;
        
        (xpos, zpos)
    }
    
    //the id of the boardsquare this position is on
    fn boardsquare_physical_pos_to_id(&self, xpos: f32, zpos:f32) -> Option<u16>{
        
        if let Some(posid) = BoardGame::boardsquare_physical_pos_to_posid(xpos, zpos){
            
            if let Some(id) = self.boardsquare_posid_to_id(posid){
                
                return Some(id);
                
            }
            
        }
        
        return None;
    }
    
    
    
}







//external getters
impl BoardGame{
    
    
    pub fn is_object_on_mission(&self, id: &u16) -> bool{
        
        self.missions.is_object_on_mission(id)
    }
    
    
    
    pub fn get_piece_ids(&self) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        for curid in &self.pieces{
            
            toreturn.push(*curid);
            
        };
        
        toreturn
        
    }
    
    
    pub fn get_square_ids(&self) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        for (_, bsid) in self.boardsquares.clone(){
            
            toreturn.push(bsid);
        }
        
        toreturn
    }
    
    
    pub fn get_translation(&self, id: u16) -> (f32,f32,f32){
        
        self.physicsengine.get_translation(&id)
        
    }
    
    pub fn get_rotation(&self, id: u16) -> (f32,f32,f32){
        
        self.physicsengine.get_rotation(&id)
        
    }
    
    //is this board game object a square
    pub fn is_board_game_object_square(&self, objectid: u16) -> bool{
        
        for (_, bsid) in &self.boardsquares{
            
            if &objectid == bsid{
                
                return true;
            }
        }
        
        return false;
    }
    
    //is this board game object a piece
    pub fn is_board_game_object_piece(&self, objectid: u16) -> bool{
        
        if self.pieces.contains(&objectid){
            return true;
        }
        
        return false;
        
    }
    
    
    pub fn get_square_pos_piece_is_on(&self, pieceid:u16) -> Option<(u8, u8)>{
        
        if let Some(bsid) = self.get_board_square_piece_is_on(pieceid){
            
            if let Some(posid) = self.boardsquare_id_to_posid(bsid){
                
                return Some(posid);
            };
        };
        
        return None;
    }
    
    
    //get all the pieces that are on this board square
    pub fn get_pieces_on_board_square(&self, boardsquareid: u16) -> HashSet<u16>{
        
        let mut toreturn = HashSet::new();
        
        //for each piece
        for physicalid in self.pieces.iter(){
            
            //if that piece is on a board square
            if let Some( curpieceboardsquareid ) = self.get_board_square_piece_is_on(*physicalid){
                
                //if the board square this piece is on is the one thats requested
                //add it to the hashset being returned
                if boardsquareid == curpieceboardsquareid {
                    
                    toreturn.insert( *physicalid );
                }
            }
        }
        
        toreturn
        
    }
    
    
    //get the id of the board square that a certain piece is on
    pub fn get_board_square_piece_is_on(&self, pieceid: u16) -> Option<u16>{
        //get its position
        let (mut xpos, mut ypos, mut zpos) = self.physicsengine.get_translation(&pieceid);
        
        
        //if its yposition is below zero, its not considered "on" any particular board square
        if ypos < -2.0{
            return None;
        };
        
        
        self.boardsquare_physical_pos_to_id(xpos, zpos)
        
    }
    
    
    
}






//perfect data struct

/*

iterable

the things added are also added in order an accessable as a list

when you insert you also insert as


get by key (one or multiple value)

get by value (the key for this value)



now this isnt for when you dont know what data struct to use
its because sometimes you need all of those functionalities
*/





/*
//takes the shapeid and returns the convexhull of the shape
struct ShapeIDtoConvexHull{
}

impl ShapeIDtoConvexHull{
    
    fn horizontalwall() -> ConvexHull<f32>{
        
        use nalgebra::{Point3, RealField, Vector3};
        
        let mut points: Vec<Point3<f32>> = Vec::new();
        
        //these are really "half _ size"
        let xsize = 20.0 / 2.0;
        let ysize = 20.0 / 2.0;
        let zsize = 2.0 / 2.0;
        
        
        points.push(  Point3::new(  -xsize,  -ysize,  -zsize ) );
        points.push(  Point3::new(  -xsize,  -ysize,  zsize ) );
        points.push(  Point3::new(  -xsize,  ysize,  zsize ) );
        points.push(  Point3::new(  -xsize,  ysize,  -zsize ) );
        
        points.push(  Point3::new(  xsize,  ysize, -zsize ) );
        points.push(  Point3::new(  xsize,  ysize,  zsize ) );
        points.push(  Point3::new(  xsize,  -ysize,  zsize ) );
        points.push(  Point3::new(  xsize,  -ysize,  -zsize ) );
        
        
        
        
        let wallshape = ConvexHull::try_from_points(&points).unwrap();
        
        
        return(wallshape);
        
        
    }
    
    fn verticalwall() -> ConvexHull<f32>{
        
        
        use nalgebra::{Point3, RealField, Vector3};
        
        let mut points: Vec<Point3<f32>> = Vec::new();
        
        
        //these are really "half _ size" or i guess twice
        let xsize = 2.0 / 2.0;
        let ysize = 20.0 / 2.0;
        let zsize = 20.0 / 2.0;
        
        
        
        points.push(  Point3::new(  -xsize,  -ysize,  -zsize ) );
        points.push(  Point3::new(  -xsize,  -ysize,  zsize ) );
        points.push(  Point3::new(  -xsize,  ysize,  zsize ) );
        points.push(  Point3::new(  -xsize,  ysize,  -zsize ) );
        
        points.push(  Point3::new(  xsize,  ysize, -zsize ) );
        points.push(  Point3::new(  xsize,  ysize,  zsize ) );
        points.push(  Point3::new(  xsize,  -ysize,  zsize ) );
        points.push(  Point3::new(  xsize,  -ysize,  -zsize ) );
        
        
        
        
        let wallshape = ConvexHull::try_from_points(&points).unwrap();
        
        
        return(wallshape);
        
        
        
    }
    
    fn dischull() -> ConvexHull<f32>{
        
        use nalgebra::{Point3, RealField, Vector3};
        
        let mut points: Vec<Point3<f32>> = Vec::new();
        
        
        let circledetail = 10;
        
        let diameter = 0.7;
        
        let height = 0.5;
        
        //a flat cylinder with a diameter of like 0.7
        
        for circle in 0..circledetail{
            
            let fraction = (circle as f32) / (circledetail as f32);
            let x = (fraction * 3.14159 * 2.0).cos() * 0.5 * diameter;
            let y = (fraction * 3.14159 * 2.0).sin() * 0.5 * diameter;
            
            
            
            points.push( Point3::new( x, height/2.0, y ));            
            points.push( Point3::new( x, -height/2.0, y ));
            
        }
        
        
        
        let boardsquareshape = ConvexHull::try_from_points(points.as_slice() ).unwrap();
        
        return(boardsquareshape);
    }
    
    fn shapeidtoconvexhull(shapeID: &u32) -> ConvexHull<f32>{
        
        //0 is a 
        
        //1 is normal board square
        
        //2 is a piece shape
        
        //10 is sphere underneath (for capturing)
        //and then 11 - 14 are the shapes that are to the east, north, west, and south of the origin
        
        
        
        use nalgebra::{Point3, RealField, Vector3};
        
        
        if (shapeID == &1){
            
            
            
            
            
            let mut points: Vec<Point3<f32>> = Vec::new();
            //a cube with edge lengths of 1 centered on the origin
            points.push(  Point3::new(  -0.5,  -0.5,  -0.5 ) );
            points.push(  Point3::new(  -0.5,  -0.5,  0.5 ) );
            points.push(  Point3::new(  -0.5,  0.5,  0.5 ) );
            points.push(  Point3::new(  -0.5,  0.5,  -0.5 ) );
            
            points.push(  Point3::new(  0.5,  0.5, -0.5 ) );
            points.push(  Point3::new(  0.5,  0.5,  0.5 ) );
            points.push(  Point3::new(  0.5,  -0.5,  0.5 ) );
            points.push(  Point3::new(  0.5,  -0.5,  -0.5 ) );
            
            
            
            let boardsquareshape = ConvexHull::try_from_points(&points).unwrap();
            
            
            return(boardsquareshape)
        }
        else{
            
            return( ShapeIDtoConvexHull::dischull() );
            
            
            
        }
        
        
    }
    
}
*/

/*
//I HAVE NEEDED THIS TYPE OF DATA STRUCT TOO MANY TIMES SO NOW I NEED TO MAKE IT
struct DoubleHashMap<K, V>{
    
    
    //mapping from key to value
    keytovalue: HashMap<K, V>,
    
    //mapping from value to teh id of the key?
    //i dont need a second copy of the key... but no excessive harm... i guess
    valuetokey: HashMap<V, K>,
    
    
}

impl<K, V> DoubleHashMap<K, V>{
    
    
    
    
    
}
*/