



//imposes rules onto the physics engine so it acts like a board game
//turns a physics engine into a board game with rules


/*
what do I want the board games public interface to be?


new_empty_board ()

new_piece (on board square, piece type)

is_boardsquare_empty ()


flick_piece
slide_piece
lift_and_drop_piece

tick




is that sufficient?



the main game:

requests a board game with specific settings


the game engine:

maps all the pieces and board squares to an ID so they can be referenced consistently

keeps track of who owns each piece




the board game:


creates piece of type return ID
*/



mod physicsengine;

use physicsengine::PhysicsEngine;

use std::collections::HashSet;
use std::collections::HashMap;

use ncollide3d::shape::ConvexHull;


use serde::{Serialize, Deserialize};
use nalgebra::Vector3;


pub struct BoardGame{
    
    
    //the list of piece as IDs
    pieces: HashSet<u16>,
    
    //the list of board squares to their physical object ID
    boardsquares: HashMap<(u8,u8), u16>,
    
    
    //the physical engine
    physicsengine: PhysicsEngine,
    
    
    //the ID of the object to the mission the object is on
    idtomission: HashMap< u16, Mission>,
    
    
    //the missions that are yet to occur
    futuremissions: Vec<(u32, u16, Mission)>,


    physicalidtoshapeid: HashMap< u16, u32>,


    
    
}



impl BoardGame{
    
    
    pub fn new_empty_board() -> BoardGame{
        
        
        let mut boardgame = BoardGame{
            
            pieces: HashSet::new(),
            boardsquares: HashMap::new(),
            physicsengine: PhysicsEngine::new(),
            idtomission: HashMap::new(),
            futuremissions: Vec::new(),
            physicalidtoshapeid: HashMap::new(),
            
        };
        
        
        //create the 4 invisible walls bordering the game
        {
            
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_position( &physicalid,  (0.0,0.0,-5.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::horizontalwall() );
            
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_position( &physicalid,  (0.0,0.0,5.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::horizontalwall() );
            
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_position( &physicalid,  (-5.0,0.0,0.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::verticalwall() );
            
            
            let physicalid = boardgame.physicsengine.add_object();
            boardgame.physicsengine.set_position( &physicalid,  (5.0,0.0,0.0) );
            boardgame.physicsengine.make_static(&physicalid);
            boardgame.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::verticalwall() );
            
        }
        
        
        //create the boardsquares
        //set up the boardsquares
        {
            let boardxsize = 8;
            let boardysize = 8;
            let halfboardxsize = 4.0;
            let halfboardysize = 4.0;
            
            
            //create the 64 squares of the board as objects
            for x in 0..boardxsize{
                
                for z in 0..boardysize{
                    
                    let physicalid = boardgame.physicsengine.add_object();
                    
                    
                    let ypos = 0.0;
                    let (xpos, zpos) = convert_id_pos_to_physical_pos( (x, z) ); 
                    
                    boardgame.physicsengine.set_position( &physicalid, ( xpos , ypos ,zpos  ) );
                    boardgame.physicsengine.toggle_gravity( &physicalid, false);
                    boardgame.physicsengine.make_static(&physicalid);
                    
                    
                    
                    let boardsquareshape = ShapeIDtoConvexHull::shapeidtoconvexhull( &(1 as u32) );
                    
                    
                    boardgame.physicsengine.set_shape(&physicalid, boardsquareshape);
                    
                    
                    boardgame.boardsquares.insert( (x,z), physicalid );
                }
                
            }
            
        }
        
        
        
        
        boardgame
        
    }
    
    
    
    pub fn new_piece(&mut self, pos:(u8,u8) ) -> u16{
        
        let pos = convert_id_pos_to_physical_pos( pos );
        let shape = ShapeIDtoConvexHull::shapeidtoconvexhull(&2);
        
        let pieceid = self.physicsengine.add_object();
        self.pieces.insert(pieceid);
        
        
        self.physicsengine.set_position( &pieceid, ( pos.0 , 6.0 , pos.1 ) );
        self.physicsengine.toggle_gravity( &pieceid, true);
        self.physicsengine.set_shape(&pieceid, shape );
        
        
        return  pieceid;
        
    }
    
    
    
    
    
    pub fn slide_piece(&mut self, pieceid: u16, slidestepchange: (i32,i32), slidedistance: u8){
        
        
        //get the board square this piece is on
        if let Some(boardsquare) = self.get_board_square_piece_is_on(pieceid){
            
            
            //make the slide mission want to go to the middle of the square its going to
            {
                let mut relativepos = ((slidestepchange.0 * slidedistance as i32) as f32, (slidestepchange.1 * slidedistance as i32) as f32);
                
                let pieceoffset = self.piece_on_square_offset(pieceid).unwrap();
                
                relativepos.0 = relativepos.0 - pieceoffset.0;
                relativepos.1 = relativepos.1 - pieceoffset.1;
                
                //slide to the center of a piece
                let slidemission = Mission::make_slide_mission( relativepos);
                
                
                //put that mission into the lists of future missions
                self.futuremissions.push( (0, pieceid, slidemission) );
            }
            
            
            //make the missions that drop the pieces that its passing over
            {
                
                let mut curboardsquare = boardsquare;
                let mut stepnumber = 0;            
                //how long into the future to drop the piece
                let mut curdroptick = 0;
                
                
                while let Some(boardsquareid) = self.get_id_of_boardsquare( curboardsquare ){
                    
                    self.set_future_drop_and_raise(curdroptick, boardsquareid);
                    
                    //this should be a function that handles the "out of range" case
                    curboardsquare = ((curboardsquare.0 as i32 + slidestepchange.0) as u8 , (curboardsquare.1 as i32 + slidestepchange.1) as u8);
                    
                    
                    stepnumber += 1;
                    
                    curdroptick += (10.0 * ((slidestepchange.0 as f32).powf(2.0) + (slidestepchange.1 as f32).powf(2.0)).sqrt()) as u32;
                    
                    if stepnumber as u8 > slidedistance{
                        break;
                    }
                    
                    
                }
                
                
            }
            
            
        };
        
        
    }
    
    fn set_future_drop_and_raise(&mut self, ticks: u32, id: u16){
        
        let liftanddropmission = Mission::make_drop_and_raise();
        
        self.futuremissions.push(  (ticks, id , liftanddropmission)  );        
        
    }
    
    pub fn set_long_boardsquare_drop(&mut self, length: u32, boardsquarepos: (u8,u8)){
        
        if let Some(bsid) = self.get_id_of_boardsquare(boardsquarepos){
            
            let longliftanddropmission = Mission::make_lengthed_drop_and_raise(length);
            
            self.futuremissions.push(  (0, bsid , longliftanddropmission)  );    
            
        }
        
        
    }
    
    //flick a piece in a direction (radians), with a force
    pub fn flick_piece(&mut self, objectid: u16, direction: f32, force: f32){
        
        //create a mission
        let flickmission = Mission::make_flick_mission( direction, force);
        
        
        self.futuremissions.push( (0, objectid, flickmission) );
        
    }
    
    //lift and move a piece to another position
    pub fn lift_and_move_piece_to(&mut self, pieceid: u16, mut relativepos: (f32,f32)){
        
        
        //get the difference between this piece and the center of the board square its on
        if let Some(offset) = self.piece_on_square_offset(pieceid){
            
            //create the mission for the piece
            {
                relativepos.0 = relativepos.0 - offset.0;
                relativepos.1 = relativepos.1 - offset.1;
                
                let liftandmovemission = Mission::make_lift_mission( relativepos );
                self.futuremissions.push( (0, pieceid, liftandmovemission) );
            }
            
            
            //create the mission for the piece its landing on
            {
                
                let piecexpos = self.physicsengine.get_translation(&pieceid).0 + relativepos.0;
                let piecezpos = self.physicsengine.get_translation(&pieceid).2 + relativepos.1;
                
                if let Some(bspos) = convert_physical_pos_to_id_pos(piecexpos, piecezpos){
                    
                    if let Some(bsid) = self.get_id_of_boardsquare(bspos){
                        
                        self.set_future_drop_and_raise(0, bsid);
                        
                    }
                }
                
                
            }
            
            
        }
        
    }
    
    
    pub fn tick(&mut self){
        
        
        //the future missions
        {
            
            //tick the missions down and start it if the tick is 0
            for thing in self.futuremissions.iter_mut(){
                
                let (tick, objectid, mission) = thing;
                
                //tick it
                *tick = *tick - 1;
                
                //if its time to start the mission, just start it by putting it in the list of missions 
                if *tick <= 0{
                    BoardGame::associated_set_mission( &mut self.idtomission, *objectid, mission.clone());
                }
                
            };
            
            
            //remove the future mission if the tick is 0
            self.futuremissions.retain(|(tick, objectid, mission)|{            
                
                //if the tick is 0 or less
                if tick <= &0{
                    //remove it
                    return(false);
                }
                else{
                    //keep it
                    return(true);
                }
                
            });
            
            
            
            
        }
        
        
        
        
        //the ids of the missions that are expired
        let mut finishedmissions: Vec<u16> = Vec::new();
        
        //for each mission
        for (physicalid, mission) in self.idtomission.iter_mut(){
            
            
            
            //if there is an impulse
            if mission.is_current_impulse(){
                
                let currentimpulsevector = mission.get_current_impulse();
                
                //make the vector into a force
                let currentimpulse = nphysics3d::algebra::Force3::new( currentimpulsevector, Vector3::new(0.0,0.0,0.0) ); 
                
                self.physicsengine.apply_delta_impulse(physicalid, currentimpulse);
                
            }
            
            
            if mission.is_current_position_change(){          
                
                let currentposchangevector = mission.get_current_position_change();
                
                self.physicsengine.apply_delta_position(physicalid, currentposchangevector);
                
                //and set this object to not experience the force of gravity for the next tick
                self.physicsengine.turn_gravity_off_for_tick(physicalid);
                
                //and set it to be static for a tick
                self.physicsengine.make_static_for_tick(physicalid);
                
            }
            
            
            
            //then tick the mission
            //end and remove it if it needs to be ended and removed
            //and remove the sensor that the piece had on that mission
            mission.tick();
            
            if mission.is_finished() {
                finishedmissions.push(*physicalid);
            }
            
            
        }
        
        //remove each finished mission
        for finishedmissionpieceid in finishedmissions{
            self.idtomission.remove( &finishedmissionpieceid);
        }
        
        
        
        //tick the physics world
        self.physicsengine.tick();
        
        
    }
    
    
    pub fn get_translation(&self, id: u16) -> (f32,f32,f32){
        
        self.physicsengine.get_translation(&id)
        
    }
    
    pub fn get_rotation(&self, id: u16) -> (f32,f32,f32){
        
        self.physicsengine.get_rotation(&id)
        
    }
    
    
    //get  all the pieces that are on this board  square
    pub fn get_pieces_on_board_square(&self, boardsquareid: &(u8,u8)) -> HashSet<u16>{
        
        let mut toreturn = HashSet::new();
        
        
        //for each piece
        for physicalid in self.pieces.iter(){
            
            //if that piece is on a board square
            if let Some( curpieceboardsquareid ) = self.get_board_square_piece_is_on(*physicalid){
                
                //if the board square this piece is on is the one thats requested
                //add it to the hashset being returned
                if boardsquareid == &curpieceboardsquareid {
                    
                    toreturn.insert( *physicalid );
                    
                }
                
            }
        }
        
        
        toreturn
        
    }
    
    
    

    //get the id of every object
    pub fn get_object_ids(&self) -> Vec<u16>{

        let mut toreturn = Vec::new();

        for curid in &self.pieces{

            toreturn.push(*curid);

        };

        for (curpos, curid) in &self.boardsquares{

            toreturn.push(*curid);
        } 


        toreturn
    }


    
    
    
    
    //get the id of the boardsquare by its position
    fn get_id_of_boardsquare(&self, pos: (u8,u8) ) -> Option<u16>{
        
        if let Some(bsid) = self.boardsquares.get(&pos){
            
            return Some(*bsid);
        }
        else{
            
            return None;
        };
        
    }
    
    //get the board square that a certain piece is on
    pub fn get_board_square_piece_is_on(&self, pieceid: u16) -> Option<(u8,u8)>{
        
        //get its position
        let (mut xpos, mut ypos, mut zpos) = self.physicsengine.get_translation(&pieceid);
        
        
        //if its yposition is below zero, its not considered "on" any particular board square
        if ypos < -2.0{
            
            return None ;
            
        };
        
        
        return convert_physical_pos_to_id_pos(xpos, zpos);
    }
    
    //get a pieces offset on the square its on
    fn piece_on_square_offset(&self, id: u16) -> Option<(f32,f32)>{
        
        
        if let Some(bsid) = self.get_board_square_piece_is_on(id){
            
            
            let physicalbs = convert_id_pos_to_physical_pos(bsid);
            
            //get the pieces x and z position and subtract the position of the piece its on from it
            let xoffset = self.physicsengine.get_translation(&id).0 - physicalbs.0;
            let yoffset = self.physicsengine.get_translation(&id).0 - physicalbs.0;
            
            return Some( (xoffset, yoffset) );
            
        }
        else{
            
            return None;
        }
        
        
        
    }
    
    
    
    
    
    //an associated function to prevent borrowing errors, might not want an associated function for other purposes
    //set a mission only if there are no other missions on the object currently
    //and return if it passed and was set, or failed, and not set
    fn associated_set_mission(gameobjectIDtoMission: &mut HashMap<u16, Mission>, objectid: u16, mission: Mission) -> bool{
        
        //if there is already a mission for this object
        if gameobjectIDtoMission.contains_key(&objectid){
            
            //dont set the mission and return none
            return(false);
        }
        else{
            
            //set the mission and return true
            gameobjectIDtoMission.insert(objectid, mission);
            
            return(true);
        }
        
        
    }
    
    
    
    
    
    
}



















//takes the shapeid and returns the convexhull of the shape
pub struct ShapeIDtoConvexHull{
}

impl ShapeIDtoConvexHull{
    
    pub fn horizontalwall() -> ConvexHull<f32>{
        
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
    
    pub fn verticalwall() -> ConvexHull<f32>{
        
        
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
    
    
    pub fn dischull() -> ConvexHull<f32>{
        
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
    
    pub fn shapeidtoconvexhull(shapeID: &u32) -> ConvexHull<f32>{
        
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







//convert  the object center to what board square its on
//and if it isnt on any board square, return None
fn convert_physical_pos_to_id_pos( xpos: f32, zpos: f32 ) -> Option<(u8,u8)>{
    
    
    //println!("x and y {:?}", (xpos, zpos));
    
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




//convert the id of a board square, to the position at the center of that board square
fn convert_id_pos_to_physical_pos( boardsquare:(u8,u8) ) -> (f32,f32) {
    
    
    let mut xpos = boardsquare.0 as f32;
    let mut zpos = boardsquare.1 as f32;
    
    //subtract 3.5
    
    xpos = xpos - 3.5;
    zpos = zpos - 3.5;
    
    (xpos, zpos)
    
}











//a mission
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Mission{
    
    //the current tick the mission is currently on
    currenttick: u32,
    
    //START is inclusive
    //END is exclusive
    
    //the force (impulse) to apply when the current tick is in range
    //a vector with the scalar of the force
    impulses: Vec< (u32, u32, Vector3<f32>) >,
    
    //the change in position to apply when the current tick is in range
    //(call the "disable gravity for a tick" when this is being called)
    positionchanges: Vec< (u32, u32, Vector3<f32>) >,
    
    
}

impl Mission{
    
    
    //tick the mission
    //the tick should be done after performing the effects of the mission
    //so tick 0 is run
    pub fn tick(&mut self){
        
        self.currenttick += 1;
    }
    
    
    //make the mission of flicking a piece
    pub fn make_flick_mission(direction: f32, force: f32) -> Mission{
        
        let mut impulses = Vec::new();
        
        impulses.push( (0,1, Vector3::new( direction.cos()*force, 0.0 , direction.sin()*force )   ) );
        
        
        
        let toreturn = Mission{
            
            currenttick: 0,
            
            impulses: impulses,
            
            positionchanges: Vec::new(),
            
        };
        
        
        toreturn
        
    }
    
    pub fn make_lift_mission(relativepos: (f32,f32)) -> Mission{
        
        
        let mut positionchanges = Vec::new();
        
        
        //the timesteps at which the states change
        let lifttomove = 10;
        let movetodrop = 20;
        let endtick = 30;
        
        
        let liftphysics = (0, lifttomove, Vector3::new(0.0, 0.1, 0.0)  );
        positionchanges.push( liftphysics );
        
        
        let totalmoveticks = movetodrop - lifttomove;
        let xchangepertick = relativepos.0 / (totalmoveticks) as f32;
        let zchangepertick = relativepos.1 / (totalmoveticks) as f32;
        
        let movephysics = (lifttomove, movetodrop, Vector3::new(xchangepertick, 0.0, zchangepertick) );
        positionchanges.push(movephysics);
        
        
        let lowerphysics = (movetodrop, endtick, Vector3::new(0.0, -0.1, 0.0) );
        positionchanges.push(lowerphysics);
        
        
        
        
        
        
        Mission{
            currenttick: 0,
            
            impulses: Vec::new(),
            
            positionchanges: positionchanges,
            
        }
        
        
        
        
        
        
    }
    
    //make a slide mission given the relative position for the piece to slide to
    pub fn make_slide_mission(relativepos: (f32,f32)) -> Mission{
        
        
        let mut positionchanges = Vec::new();
        
        
        //get the distance so i can determine how long to make the slide
        let slidedistance = (relativepos.0 * relativepos.0 + relativepos.1 * relativepos.1).sqrt();
        
        //the timesteps at which the states change
        let ticks = (slidedistance as u32 * 10);
        //how long to wait before starting the movement
        let waitbefore = 24;
        
        
        let xchangepertick = relativepos.0 / (ticks) as f32;
        let zchangepertick = relativepos.1 / (ticks) as f32;
        
        let slidephysics = (waitbefore, ticks + waitbefore, Vector3::new(xchangepertick, 0.0, zchangepertick) );
        positionchanges.push(slidephysics);        
        
        
        Mission{
            currenttick: 0,
            impulses: Vec::new(),
            positionchanges: positionchanges,
            
        }
        
        
    }
    
    //a drop and raise mission for a board square
    pub fn make_drop_and_raise() -> Mission{
        
        let mut positionchanges = Vec::new();
        
        
        //when the object stops dropping
        let enddrop = 10;
        let restoretime = 12;
        let kickerstart = 14;
        let kickerend = 18;
        let kickerrestore = 20;
        
        
        
        let dropphysics = (0, enddrop, Vector3::new(0.0, -0.5, 0.0) );
        positionchanges.push(dropphysics);
        
        //shoot the object to the left so nothing can stay on it
        let leftphysics = (enddrop, restoretime, Vector3::new(-2.0, 0.0, 0.0) );
        positionchanges.push(leftphysics);
        
        //return the piece back to its original position
        let restorephysics = (restoretime, kickerstart, Vector3::new(2.0, 2.5, 0.0) );
        positionchanges.push(restorephysics);
        
        
        //pop upwards after returning to the original position to kick any piece that didnt fall
        //that was on the square upwards and out of the game
        let kickup = (kickerstart, kickerend, Vector3::new(0.0, 0.2, 0.0));
        positionchanges.push(kickup);
        
        let kickrestore = ( kickerend, kickerrestore, Vector3::new(0.0, -0.4, 0.0) );
        positionchanges.push(kickrestore);
        
        
        Mission{
            currenttick: 0,
            impulses: Vec::new(),
            positionchanges: positionchanges,
            
        }
        
        
    }
    
    
    pub fn make_lengthed_drop_and_raise(ticks: u32) -> Mission{
        
        let mut positionchanges = Vec::new();
        
        
        //when the object stops dropping
        let enddrop = 5;
        let endleft = 10;
        let waitstillend = 10 + ticks;
        let restoreend = 10 + ticks + 10;
        
        
        
        let dropphysics = (0, enddrop, Vector3::new(0.0, -1.0, 0.0) );
        positionchanges.push(dropphysics);
        
        //shoot the object to the left so nothing can stay on it
        let leftphysics = (enddrop, waitstillend, Vector3::new(-1.0, 0.0, 0.0) );
        positionchanges.push(leftphysics);
        
        
        //shoot the object to the left so nothing can stay on it
        let waitphysics = (enddrop, waitstillend, Vector3::new(0.0, 0.0, 0.0) );
        positionchanges.push(waitphysics);
        
        
        //return the piece back to its original position
        let restorephysics = (waitstillend, restoreend, Vector3::new(1.0, 1.0, 0.0) );
        positionchanges.push(restorephysics);
        
        
        
        
        Mission{
            currenttick: 0,
            impulses: Vec::new(),
            positionchanges: positionchanges,
            
        }
        
        
        
    }
    
    
    //is there a position change currently going on?
    //this should be plural but i just never seem to have plurals in method titles as a rule
    pub fn is_current_position_change(&self) -> bool{
        
        
        //for every one of the position changes in the list
        
        
        for (starttick, endtick, vector) in &self.positionchanges{
            
            if  self.currenttick >= *starttick {
                
                if  self.currenttick < *endtick {
                    return(true);                    
                }
                
            }
            
        }
        
        
        return(false);
        
    }
    
    pub fn is_current_impulse(&self) -> bool{
        
        
        for (starttick, endtick, vector) in &self.impulses{
            
            if self.currenttick >= *starttick {
                
                if self.currenttick < *endtick {
                    
                    return(true);
                    
                }
            }
            
        }
        
        
        return(false);
        
        
        
        
    }
    
    pub fn get_current_position_change(&self) -> Vector3<f32>{
        
        
        let mut totalpositionchange = Vector3::<f32>::new(0.0,0.0,0.0);
        
        for (starttick, endtick, vector) in &self.positionchanges{
            
            if  self.currenttick >= *starttick {
                
                if  self.currenttick < *endtick {
                    
                    totalpositionchange += vector;
                }
                
            }
            
        }
        
        
        totalpositionchange
        
        
        
    } 
    
    pub fn get_current_impulse(&self) -> Vector3<f32>{
        
        
        let mut totalimpulse = Vector3::<f32>::new(0.0,0.0,0.0);
        
        for (starttick, endtick, vector) in & self.impulses{
            
            if  self.currenttick >= *starttick {
                
                if  self.currenttick < *endtick {
                    
                    totalimpulse += vector;
                    
                }
                
            }
            
        }
        
        
        totalimpulse
        
        
        
    } 
    
    
    //if this mission is finished
    pub fn is_finished(&self) -> bool{
        
        let mut isfinished = true;
        
        //see if theres any impulse or position change currently or in the future
        
        for (starttick, endtick, vector) in &self.positionchanges{
            
            if endtick >= &self.currenttick{
                
                isfinished = false;
                
            }
            
            
        }
        
        for (starttick, endtick, vector) in &self.impulses{
            
            if endtick >= &self.currenttick{
                
                isfinished = false;
                
            }
            
            
        }
        
        
        isfinished
        
        
    }
    
}