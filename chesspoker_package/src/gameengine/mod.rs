mod physicsengine;

use physicsengine::PhysicsEngine;
use nalgebra::Vector3;



use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};



use ncollide3d::shape::ConvexHull;

//this doesnt need to be a struct
//just a function
//but whatever, i like it more like this

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



use serde::{Serialize, Deserialize};


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
    fn tick(&mut self){
        
        self.currenttick += 1;
    }
    
    
    //make the mission of flicking a piece
    fn make_flick_mission(direction: f32, force: f32) -> Mission{
        
        let mut impulses = Vec::new();
        
        impulses.push( (0,1, Vector3::new( direction.cos()*force, 0.0 , direction.sin()*force )   ) );
        
        
        
        let toreturn = Mission{
            
            currenttick: 0,
            
            impulses: impulses,
            
            positionchanges: Vec::new(),
            
        };
        
        
        toreturn
        
    }
    
    fn make_lift_mission(relativepos: (f32,f32)) -> Mission{
        
        
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
    fn make_slide_mission(relativepos: (f32,f32)) -> Mission{
        
        
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
    fn make_drop_and_raise() -> Mission{
        
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
    
    
    fn make_lengthed_drop_and_raise(ticks: u32) -> Mission{
        
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
    fn is_current_position_change(&self) -> bool{
        
        
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
    
    fn is_current_impulse(&self) -> bool{
        
        
        for (starttick, endtick, vector) in &self.impulses{
            
            if self.currenttick >= *starttick {
                
                if self.currenttick < *endtick {
                    
                    return(true);
                    
                }
            }
            
        }
        
        
        return(false);
        
        
        
        
    }
    
    fn get_current_position_change(&self) -> Vector3<f32>{
        
        
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
    
    fn get_current_impulse(&self) -> Vector3<f32>{
        
        
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
    fn is_finished(&self) -> bool{
        
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



#[derive(Clone, Serialize, Deserialize)]
pub struct PhysicalObject{
    
    //the shape id of it
    shapeid: u32,
    
    //position
    position: (f32,f32,f32),
    
    rotation: (f32,f32,f32),
    
    linear_velocity: (f32,f32,f32),
    
    angular_velocity: (f32,f32,f32),
    
    //weight?? i only need to add the things that I can change about the physical object
    //so i shouldnt need to add anything that I dont have a method for in the physics engine
    //aside from pos, rotation, and velocities, which change through the execution of the engine
    //other properties of the objects should not (imagine if it was a more realistic physics engine)
    
}


//the state of the game engine
#[derive(Clone, Serialize, Deserialize)]
pub struct GameEngineState{
    
    gameobjecttophysicalstate: Vec< (GameObjectID, PhysicalObject) >,
    
    gameobjecttomission: Vec< (GameObjectID, Mission)>,
    
}

impl GameEngineState{
    
    fn new_empty() -> GameEngineState{
        
        GameEngineState{
            
            gameobjecttophysicalstate: Vec::new(),
            
            gameobjecttomission: Vec::new(),
            
        }
        
        
        
        
    }
    
    
    fn add_gameobject(&mut self, ID: GameObjectID, physicalpiece: PhysicalObject){
        
        self.gameobjecttophysicalstate.push( (ID, physicalpiece) );
        
    }
    
    
    fn add_gameobjecttomission(&mut self, ID: GameObjectID, mission: Mission){
        
        self.gameobjecttomission.push( (ID, mission) );
        
    }
    
    
    fn pop_gameobject(&mut self) -> Option< (GameObjectID, PhysicalObject) >{
        
        self.gameobjecttophysicalstate.pop()
    }
    
    
    
    fn pop_mission(&mut self) -> Option<(  GameObjectID, Mission )>{
        
        self.gameobjecttomission.pop()
        
    }
    
    
    
}



//the ID of a game object
#[derive(Eq, Hash, PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
enum GameObjectID{
    
    boardsquare(u8,u8),
    piece(u32),
    
}

impl GameObjectID{
    
    //create a new gameobjectid for a boardsquare
    fn new_boardsquare(x:u8, y: u8) -> GameObjectID{
        
        GameObjectID::boardsquare(x,y)
        
    }
    
    fn new_piece(id: u32) -> GameObjectID{
        GameObjectID::piece(id)
    }
    
    
    
}


use std::collections::HashMap;
use std::collections::HashSet;


//this should be serializable in all parts but "physics engine"

//uses the physics engine to make a game
pub struct GameEngine{
    
    
    
    //the physics engine inside this game engine
    physicsengine: PhysicsEngine,
    
    
    //the new structs
    gameobjectIDtophysicalID: HashMap<GameObjectID, u16>,
    
    gameobjectIDtoMission: HashMap<GameObjectID, Mission>,
    
    gameobjectIDtoshapeID: HashMap<GameObjectID, u32>,
    
    
    //the missions that are yet to occur
    futuremissions: Vec<(u32, GameObjectID, Mission)>,
    
    
    
}


impl GameEngine{
    
    //public methods
    pub fn new() -> GameEngine{
        
        
        let mut gameengine = GameEngine{
            
            physicsengine: PhysicsEngine::new(),
            
            gameobjectIDtophysicalID: HashMap::new(),
            
            gameobjectIDtoMission: HashMap::new(),
            
            gameobjectIDtoshapeID: HashMap::new(),
            
            
            futuremissions: Vec::new(),
            
            
            
        };
        
        
        
        //set up the boardsquares
        {
            let boardxsize = 8;
            let boardysize = 8;
            let halfboardxsize = 4.0;
            let halfboardysize = 4.0;
            
            
            //create the 64 squares of the board as objects
            for x in 0..boardxsize{
                
                for z in 0..boardysize{
                    
                    
                    let physicalid = gameengine.physicsengine.add_object();
                    
                    
                    let ypos = 0.0;
                    let (xpos, zpos) = convert_id_pos_to_physical_pos( (x, z) ); 
                    
                    gameengine.physicsengine.set_position( &physicalid, ( xpos , ypos ,zpos  ) );
                    gameengine.physicsengine.toggle_gravity( &physicalid, false);
                    gameengine.physicsengine.make_static(&physicalid);
                    
                    
                    
                    let boardsquareshape = ShapeIDtoConvexHull::shapeidtoconvexhull( &(1 as u32) );
                    
                    
                    gameengine.physicsengine.set_shape(&physicalid, boardsquareshape);
                    
                    
                    
                    let gameobjectid = GameObjectID::new_boardsquare(x,z);
                    gameengine.gameobjectIDtophysicalID.insert( gameobjectid, physicalid  );
                    
                    
                }
                
            }
            
        }
        
        
        //create a sensor for the top elimination and bottom elimination box
        
        
        //create the 4 invisible walls bordering the game
        {
            
            
            let physicalid = gameengine.physicsengine.add_object();
            gameengine.physicsengine.set_position( &physicalid,  (0.0,0.0,-5.0) );
            gameengine.physicsengine.make_static(&physicalid);
            gameengine.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::horizontalwall() );
            
            
            let physicalid = gameengine.physicsengine.add_object();
            gameengine.physicsengine.set_position( &physicalid,  (0.0,0.0,5.0) );
            gameengine.physicsengine.make_static(&physicalid);
            gameengine.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::horizontalwall() );
            
            
            
            
            let physicalid = gameengine.physicsengine.add_object();
            gameengine.physicsengine.set_position( &physicalid,  (-5.0,0.0,0.0) );
            gameengine.physicsengine.make_static(&physicalid);
            gameengine.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::verticalwall() );
            
            
            let physicalid = gameengine.physicsengine.add_object();
            gameengine.physicsengine.set_position( &physicalid,  (5.0,0.0,0.0) );
            gameengine.physicsengine.make_static(&physicalid);
            gameengine.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::verticalwall() );
            
        }        
        
        
        //im doing this
        //just to be safe
        //maybe a little dumb to do
        gameengine.tick();
        
        
        gameengine
        
        
        
    }
    
    
    //add a piece with this id to a certain position on the board
    //return object id
    pub fn add_piece(&mut self, pieceid:u32, mut pos:(u8,u8), shapeid: u32  ) -> u16{
        
        
        let pos = convert_id_pos_to_physical_pos( (pos.0, pos.1)  );
        
        
        
        let physicalid = self.physicsengine.add_object();
        
        self.physicsengine.set_position( &physicalid, ( pos.0 , 6.0 , pos.1 ) );
        
        self.physicsengine.toggle_gravity( &physicalid, true);
        
        self.physicsengine.set_shape(&physicalid, ShapeIDtoConvexHull::shapeidtoconvexhull(&2));
        
        
        
        
        let objectid = GameObjectID::new_piece(pieceid);
        
        
        self.gameobjectIDtoshapeID.insert(objectid, shapeid);
        
        self.gameobjectIDtophysicalID.insert( objectid, physicalid);
        
        physicalid
        
    }
    
    
    //flick a piece in a direction (radians), with a force
    pub fn flick_piece(&mut self, pieceid: u32, direction: f32, force: f32){
        
        //create a mission
        let flickmission = Mission::make_flick_mission( direction, force);
        
        
        
        //put that mission into the lists of missions
        let objectid = GameObjectID::new_piece(pieceid);
        self.gameobjectIDtoMission.insert(objectid, flickmission );
        
        
    }
    
    //get a pieces offset on the square its on
    fn piece_on_square_offset(&self, pieceid: &u32) -> Option<(f32,f32)>{
        
        if let Some(bsid) = self.get_board_square_piece_is_on(pieceid){
            
            let physicalbs = convert_id_pos_to_physical_pos(bsid);
            
            //get the pieces x and z position and subtract the position of the piece its on from it
            let xoffset = self.get_piece_translation(pieceid).0 - physicalbs.0;
            let yoffset = self.get_piece_translation(pieceid).2 - physicalbs.1;
            
            return( Some( (xoffset, yoffset) ) );
        }
        else{
            
            return(None);
        }
        
        
    }
    
    
    //lift and move a piece to another position
    pub fn lift_and_move_piece_to(&mut self, pieceid: &u32, mut relativepos:(f32,f32)){
        
        //get the difference between this piece and the center of the board square its on
        //and add that to the relative position its moving to, to "recenter it" on the piece its arriving at
        
        if let Some(offset) = self.piece_on_square_offset(pieceid){
            
            relativepos.0 = relativepos.0 - offset.0;
            relativepos.1 = relativepos.1 - offset.1;
            
            
            
            
            let liftandmovemission = Mission::make_lift_mission( relativepos );
            
            //put that mission into the lists of missions
            let objectid = GameObjectID::new_piece(*pieceid);
            self.gameobjectIDtoMission.insert(objectid, liftandmovemission );
            
            
            
            //get the position of this piece currently
            let piecexpos = self.get_piece_translation(pieceid).0 + relativepos.0;
            let piecezpos = self.get_piece_translation(pieceid).2 + relativepos.1;
            
            if let Some(bsid) = convert_physical_pos_to_id_pos(piecexpos, piecezpos){
                
                //raise and drop that board square 5 ticks in the future
                let bsobjectid = GameObjectID::new_boardsquare( bsid.0, bsid.1 );
                
                self.set_future_drop_and_raise(5, bsobjectid);
                
                
            }
            
            
            
            
        }
        
        
        
        
        
        
        
        
        
    }
    
    
    //slide a piece to a position given the relative position it should slide to
    pub fn slide_piece(&mut self, pieceid: &u32, slidestepchange: (i32,i32), slidedistance: u8){
        
        
        
        //get the board square this piece is on
        if let Some(boardsquareid) = self.get_board_square_piece_is_on(pieceid){
            
            //make the slide mission want to go to the middle of the square its going to
            let mut relativepos = ((slidestepchange.0 * slidedistance as i32) as f32, (slidestepchange.1 * slidedistance as i32) as f32);
            let pieceoffset = self.piece_on_square_offset(pieceid).unwrap();
            
            relativepos.0 = relativepos.0 - pieceoffset.0;
            relativepos.1 = relativepos.1 - pieceoffset.1;
            
            //slide to the center of a piece
            let slidemission = Mission::make_slide_mission( relativepos);
            
            
            //put that mission into the lists of missions
            let objectid = GameObjectID::new_piece(*pieceid);
            self.gameobjectIDtoMission.insert(objectid, slidemission );
            
            
            
            
            
            //convert it to a physical position
            let mut curpos = convert_id_pos_to_physical_pos(boardsquareid);
            
            let mut stepnumber = 0;
            
            //how long into the future to drop the piece
            let mut curdroptick = 0;
            
            
            //for the series of board squares its going to pass over            
            while let Some(cursquareid) = convert_physical_pos_to_id_pos(curpos.0, curpos.1){
                
                
                //raise and drop that board square 10 ticks in the future
                let bsobjectid = GameObjectID::new_boardsquare(cursquareid.0, cursquareid.1);
                self.set_future_drop_and_raise(curdroptick, bsobjectid);
                
                
                //take a step in the direction
                curpos = (curpos.0 + slidestepchange.0 as f32, curpos.1 + slidestepchange.1 as f32);
                stepnumber += 1;
                
                //take 10 ticks per unit of distance
                curdroptick += (10.0 * ((slidestepchange.0 as f32).powf(2.0) + (slidestepchange.1 as f32).powf(2.0)).sqrt()) as u32;
                
                //if the step number goes over, break
                if stepnumber as u8 > slidedistance{
                    break;
                }
                
                
            }
            
            
            
            
            
        };
        
    }
    
    pub fn set_long_boardsquare_drop(&mut self, length: u32, boardsquareid: (u8,u8)){
        
        let objectid = GameObjectID::boardsquare( boardsquareid.0 , boardsquareid.1  );
        
        let longliftanddropmission = Mission::make_lengthed_drop_and_raise(length);
        
        self.futuremissions.push(  (0, objectid , longliftanddropmission)  );
        
    }
    
    
    //private function that sets a drop and raise mission for a piece
    //at a certain amount of ticks in the future
    fn set_future_drop_and_raise(&mut self, ticksuntil: u32, objectid: GameObjectID){
        
        let liftanddropmission = Mission::make_drop_and_raise();
        
        self.futuremissions.push(  (ticksuntil, objectid , liftanddropmission)  );
        
    }
    
    
    //an associated function to prevent borrowing errors, might not want an associated function for other purposes
    //set a mission only if there are no other missions on the object currently
    //and return if it passed and was set, or failed, and not set
    fn associated_set_mission(gameobjectIDtoMission: &mut HashMap<GameObjectID, Mission>, objectid: GameObjectID, mission: Mission) -> bool{
        
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
    
    
    
    
    pub fn tick(&mut self){
        
        //the future missions
        for thing in self.futuremissions.iter_mut(){
            
            
            let (tick, objectid, mission) = thing;
            
            //tick it
            *tick = *tick - 1;
            
            
            //if its time to start the mission, just start it by putting it in the list of missions 
            if *tick <= 0{
                GameEngine::associated_set_mission( &mut self.gameobjectIDtoMission, *objectid, mission.clone());
            }
            
        }
        
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
        
        
        
        //the piece ids of the missions that are expired
        let mut finishedmissions: Vec<GameObjectID> = Vec::new();
        
        //for each mission
        for (gameobjectid, mission ) in self.gameobjectIDtoMission.iter_mut(){
            
            //get the id of the object by the piece id
            let physicalid = self.gameobjectIDtophysicalID.get(gameobjectid).unwrap();
            
            
            
            //if there is an impulse
            if (mission.is_current_impulse()){
                
                let currentimpulsevector = mission.get_current_impulse();
                
                //make the vector into a force
                let currentimpulse = nphysics3d::algebra::Force3::new( currentimpulsevector, Vector3::new(0.0,0.0,0.0) ); 
                
                self.physicsengine.apply_delta_impulse(physicalid, currentimpulse);
                
            }
            
            
            if (mission.is_current_position_change()){
                
                
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
                finishedmissions.push(*gameobjectid);
            }
            
            
        }
        
        //remove each finished mission
        for finishedmissionpieceid in finishedmissions{
            self.gameobjectIDtoMission.remove( &finishedmissionpieceid);
        }
        
        
        
        //tick the physics world
        self.physicsengine.tick();
        
        
        
        
        
    }
    
    
    
    
    
    //get the board square that a certain piece is on
    pub fn get_board_square_piece_is_on(&self, pieceid: &u32) -> Option<(u8,u8)>{
        
        let physicalid = self.gameobjectIDtophysicalID.get( &GameObjectID::new_piece(*pieceid) ).unwrap();
        
        
        //get its position
        let (mut xpos, mut ypos, mut zpos) = self.physicsengine.get_translation(physicalid);
        
        println!("{}", ypos);
        
        //if its yposition is below zero, its not considered "on" any particular board square
        if ypos < -2.0{
            
            return( None );
            
        };
        
        
        return convert_physical_pos_to_id_pos(xpos, zpos);
        
        
    }
    
    //get  all the pieces that are on this board  square
    pub fn get_pieces_on_board_square(&self, boardsquareid: &(u8,u8)) -> HashSet<u32>{
        
        //i could just do "get_boardsquare_piece_is_on" for every piece
        //it just takes a bit of runtime
        
        //i could also just make a sensor, on top of a board square, and get every piece that intersects with it
        //but that sort of has an issue that it finds what pieces are on top of what boards squares with different standards
        //than the method of fingindi what board square a piece is on
        
        //i could just do that to find "plausible" candidates, and then narrow it down with the "get_board_square_piece_is_on"
        //which I want to if this method ends up being too slow
        
        let mut toreturn = HashSet::new();
        
        //for each object
        for (objectid, physicalid) in self.gameobjectIDtophysicalID.iter(){
            
            //if its a piece
            if let GameObjectID::piece(pieceid) = objectid{
                
                
                //if that piece is on a board square
                if let Some( curpieceboardsquareid ) = self.get_board_square_piece_is_on(&pieceid){
                    
                    //if the board square this piece is on is the one thats requested
                    //add it to the hashset being returned
                    if boardsquareid == &curpieceboardsquareid {
                        
                        toreturn.insert( *pieceid);
                        
                    }
                    
                    
                    
                }
            }
            
            
            
        }
        
        
        toreturn
        
    }
    
    
    
    
    pub fn get_piece_translation(&self, pieceid: &u32) -> (f32,f32,f32){
        
        let gameobjectid = GameObjectID::new_piece(*pieceid);
        let physicalid = self.gameobjectIDtophysicalID.get( &gameobjectid).expect("why does this piece not have an associated object?");
        
        //return what "get translation" returns
        self.physicsengine.get_translation(physicalid)
        
        
    }
    
    pub fn get_piece_rotation(&self, pieceid: &u32) -> (f32,f32,f32){
        
        
        let gameobjectid = GameObjectID::new_piece(*pieceid);
        let physicalid = self.gameobjectIDtophysicalID.get( &gameobjectid).expect("why does this piece not have an associated object?");
        
        //return what "get rotation" returns
        self.physicsengine.get_rotation(physicalid)
        
        
    }
    
    pub fn get_board_square_translation(&self, boardsquareid: &(u8,u8)) -> (f32,f32,f32){
        
        
        let gameobjectid = GameObjectID::new_boardsquare(boardsquareid.0, boardsquareid.1);
        let physicalid = self.gameobjectIDtophysicalID.get( &gameobjectid).expect("why does this piece not have an associated object?");
        
        self.physicsengine.get_translation( physicalid )
        
    }
    
    pub fn get_board_square_rotation(&self, boardsquareid: &(u8,u8)) -> (f32,f32,f32){
        
        let gameobjectid = GameObjectID::new_boardsquare(boardsquareid.0, boardsquareid.1);
        let physicalid = self.gameobjectIDtophysicalID.get( &gameobjectid).expect("why does this piece not have an associated object?");
        
        self.physicsengine.get_rotation( physicalid )
        
    }
    
    
    
    //export the state of the game engine
    pub fn get_game_engine_state(&self) -> GameEngineState{
        
        
        let mut gameenginestate = GameEngineState::new_empty();
        
        
        //ask the physics engine to get the information about every game object
        for (objectid, physicalid) in &self.gameobjectIDtophysicalID{
            
            let translation = self.physicsengine.get_translation(&physicalid);
            let rotation = self.physicsengine.get_rotation(&physicalid);
            let linearvelocity = self.physicsengine.get_linear_velocity(&physicalid);
            let angularvelocity = self.physicsengine.get_angular_velocity(&physicalid);
            
            
            let shapeid = *self.gameobjectIDtoshapeID.get(&objectid).unwrap();
            
            
            let physicalobject = PhysicalObject{
                
                position: translation,
                rotation: rotation,
                linear_velocity: linearvelocity,
                angular_velocity: angularvelocity,
                shapeid: shapeid,
            };
            
            
            gameenginestate.add_gameobject(objectid.clone(), physicalobject);
            
        };
        
        
        
        //get the information about every mission
        for (objectid, mission) in &self.gameobjectIDtoMission{
            
            gameenginestate.add_gameobjecttomission( objectid.clone(), mission.clone());
            
        }
        
        
        
        return gameenginestate;
        
    }
    
    
    
    //import information about the state of this game engine
    pub fn set_game_engine_state(&mut self, mut data: GameEngineState){
        
        
        
        //while theres info about the objects left to pop
        while let Some( (objectid, physicalstate) ) = data.pop_gameobject(){
            
            let mut physicalid = 0;
            
            
            //get the id of this object if it exists
            if let Some(tempid) = self.gameobjectIDtophysicalID.get(&objectid){                
                physicalid = *tempid;
            }
            //create and get the id if it doesnt exist
            else{
                panic!("I need to create the object if it doesnt exist")
            }
            
            
            //now set the information about its physical state
            self.physicsengine.set_position( &physicalid, physicalstate.position );
            self.physicsengine.set_rotation( &physicalid, physicalstate.rotation );
            self.physicsengine.set_angular_velocity( &physicalid, physicalstate.angular_velocity);
            self.physicsengine.set_linear_velocity( &physicalid, physicalstate.linear_velocity);
            
            
        }
        
        
        
        
        //while theres a mission left to pop
        while let Some( (objectid, mission) ) = data.pop_mission(){
            
            self.gameobjectIDtoMission.insert( objectid, mission);
            
        }
        
        
    }
    
    
    
    
}





//fn is_boardsquare_id_in_range()


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