//abstraction to the physical state of the game

//HAHA
//I had to restructure the entire thing to figure out that basically my earlier design for it was basically the most ideal
//on one hand I wont have to re-export functions if I put it all in game engine
//on the other hand, that module would easily get over 2000 lines long


//apply certain physical actions to objects

mod physicsengine;

use physicsengine::RapierPhysicsWrapper;
use physicsengine::Mission;

use rapier3d::na::Isometry3;

use serde::{Serialize, Deserialize};


//this struct doesnt know anything about the properties of the pieces as defined by the rules of the game
//it only knows about the physical properties of each of the objects, because its just wrapping the game engine

#[derive(Serialize, Deserialize)]
pub struct BoardPhysics{

    physics: RapierPhysicsWrapper,


}

use rapier3d::na;
use na::Point3;
use na::Vector3;


impl BoardPhysics{


    pub fn new() -> BoardPhysics{
        
        let mut boardgame = RapierPhysicsWrapper::new();
        
        //create the 4 invisible walls bordering the game
        {
            let horizontalwalldimensions = (20.0 , 20.0 , 4.0);
            let verticalwalldimensions = (4.0 , 20.0 , 20.0 );

            //se their collision group to "1"
            //and then ignore that group when doing queries
            
            /*
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (0.0,0.0,-6.0) );
            boardgame.set_shape_cuboid(&physicalid, horizontalwalldimensions );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (0.0,0.0,6.0) );
            boardgame.set_shape_cuboid(&physicalid, horizontalwalldimensions );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (-6.0,0.0,0.0) );
            boardgame.set_shape_cuboid(&physicalid, verticalwalldimensions );
            
            let physicalid = boardgame.add_object(true);
            boardgame.set_translation( &physicalid,  (6.0,0.0,0.0) );
            boardgame.set_shape_cuboid(&physicalid, verticalwalldimensions );
            */
        }

        BoardPhysics{
            physics: boardgame
        }
    }

    pub fn get_flat_pos(&self, id: &u16) -> (f32,f32){

        let pos = self.physics.get_translation(id);

        return (pos.0, pos.2);
    }

    pub fn get_object_intersection(& self, ray: (Point3<f32>, Vector3<f32>) ) -> Option<u16>{

        self.physics.get_object_intersection(ray )
    }
    
    pub fn create_piece_object(&mut self, id: u16,  pos: (f32,f32,f32) ) {
        
        let objectid = self.physics.add_object(id, false);
        
        self.physics.set_shape_cylinder(&objectid, 0.5, 0.7 );
        self.physics.set_materials(&objectid, 0.5, 0.5);
        self.physics.set_translation( &objectid, ( pos.0 , pos.1 , pos.2 ) );
    }

    
    pub fn create_boardsquare_object(&mut self, id: u16, pos: (f32, f32, f32) ) {
        
        let objectid = self.physics.add_object( id, true );
        
        self.physics.set_shape_cuboid(&objectid, (1.0, 1.0, 1.0) );
        self.physics.set_materials(&objectid, 0.0, 0.0);        
        self.physics.set_translation( &objectid, ( pos.0 , pos.1,  pos.2  ) );        
    }
    
    //get object 1's x&z position relative to object 2's
    fn flat_plane_object_offset(&self, object1: u16, object2: u16 ) -> (f32,f32){
        
        let object1pos = self.physics.get_translation(&object1);
        let object2pos = self.physics.get_translation(&object2);
        
        //get the pieces x and z position and subtract the position of the piece its on from it
        let xoffset = object1pos.0 - object2pos.0;
        let zoffset = object1pos.2 - object2pos.2;
        
        return (xoffset, zoffset);
    }
    
    //is this object in this range of positions?
    pub fn is_object_in_position_range(&self, objectid: u16, xrange: (f32,f32), yrange: (f32,f32), zrange: (f32,f32) ) -> bool{
        
        //get its position
        let (x,y,z) = self.physics.get_translation( &objectid );
        
        if x >= xrange.0 && x<= xrange.1{
            
            if y >= yrange.0 && y<= yrange.1{
                
                if x>= zrange.0 && z<= zrange.1{
                    
                    return true;
                }
            }
        }
        
        return false;
    }


    pub fn tick(&mut self){
        self.physics.tick();
    }

    //used for the VisibleGameBoardObject constructor
    pub fn get_isometry(&self, objectid: &u16) -> Isometry3<f32>{
        self.physics.get_isometry(objectid)
    }



    
    //how many ticks to complete
    pub fn slide_object(&mut self, objectid: u16, relativepos: (f32,f32), ticks: u32 ){
        
        let mut slidemission = Mission::default_mission(  );
        
        let xchangepertick = relativepos.0 / (ticks) as f32;
        let zchangepertick = relativepos.1 / (ticks) as f32;
        
        slidemission.add_position_change(0, ticks, (xchangepertick, 0.0, zchangepertick));    

        self.physics.set_future_mission(0, objectid, slidemission);
    }
    
    //flick a piece in a direction (radians), with a force
    pub fn flick_object(&mut self, objectid: u16, direction: f32, force: f32){
        
        let mut flickmission = Mission::default_mission(  );
        
        flickmission.add_impulse_change( 0,1, (direction.cos()*force, 10.0 , direction.sin()*force) );
        
        self.physics.set_mission(objectid, flickmission );
    }
    

    //lift and move a piece to another position
    pub fn lift_and_move_object(&mut self,  objectid: u16, relativepos: (f32,f32), ticks: u32){

        println!("performing list and move");
                
        let mut mission = Mission::default_mission(  );

        //ticks is at least 3
        //1 tick to raise
        //1 tick to move
        //1 tick to drop

        let ticks = 3.max( ticks );

        let liftend = ticks/3;
        let moveend = ticks/3 + liftend;
        let dropend = ticks/3 + moveend;

        
        let totalmoveticks = moveend - liftend;
        let xchangepertick = relativepos.0 / (totalmoveticks) as f32;
        let zchangepertick = relativepos.1 / (totalmoveticks) as f32;
        
        let verticalspeed = 1.0 / ticks as f32 + 0.05;

        
        mission.add_position_change(0, liftend, (0.0, verticalspeed, 0.0) );
        
        mission.add_position_change(liftend, moveend,(xchangepertick, 0.0, zchangepertick) );

        mission.add_position_change(moveend, dropend, (0.0, -verticalspeed, 0.0) );
        
        
        self.physics.set_future_mission( 0, objectid, mission );
    }

    
    pub fn set_long_drop(&mut self, length: u32, objectid: u16){

        let mut mission = Mission::default_mission(  );
        
        //when the object stops dropping
        let enddrop = 5;
        let waitstillend = 5 + length;
        let restoreend = waitstillend + 5;
        
        //lower
        mission.add_position_change(0, enddrop, (0.0, -2.0, 0.0) );
        
        //wait
        mission.add_position_change(enddrop, waitstillend, (0.0, 0.0, 0.0) );        
        
        //return back to its original position
        mission.add_position_change(waitstillend, restoreend, (0.0, 2.0, 0.0) );
        
        
        self.physics.set_mission(objectid, mission);
    }
    

    pub fn set_long_raise(&mut self, length: u32, objectid: u16){

        let mut mission = Mission::default_mission(  );
        
        //when the object stops dropping
        let endraise = 5;
        let wait = 5 + length;
        let restore = 5 + length + 5;
        
        mission.add_position_change(  0, endraise, (0.0, 0.2, 0.0)     );
        
        mission.add_position_change(  endraise, wait, (0.0, 0.0, 0.0)  );
        
        mission.add_position_change(  wait, restore, (0.0, -0.2, 0.0)  );
        
        self.physics.set_mission(objectid, mission);
    }
    


    pub fn set_drop(&mut self,  bsid: u16){


        let mut mission = Mission::default_mission(  );
        
        
        //the object stops dropping
        //starts moving to the left
        let enddrop = 3;
        //the object stops moving to the left
        //starts raising
        let endleft = 6;
        //the object raises up
        let endraise = 9;
        //the object comes back to where it was
        let endright = 12;
        //the object shoots back down into its original position
        let endrestore = 21;
        
        
        mission.add_position_change(0, enddrop,  (0.0, -1.5, 0.0)   );
        mission.add_position_change(enddrop, endleft, (-6.0, 0.0, 0.0)  );
        mission.add_position_change(endleft, endraise, (0.0, 3.0, 0.0)  );
        mission.add_position_change(endraise, endright, (6.0, 0.0, 0.0)  );
        mission.add_position_change( endright, endrestore, (0.0, -0.50, 0.0) );
        
        
        self.physics.set_future_mission(0, bsid, mission);
    }


    pub fn end_mission(&mut self, id: &u16){

        self.physics.end_mission(id);
    }




    //set the current position of the object as the default position on the object
    fn mission_set_current_pos_as_default(&self, id: &u16,  mission: &mut Mission){
        
        mission.set_default_isometry( self.physics.get_isometry(id) );

    }    
    
    pub fn is_object_on_mission(&self, id: &u16) -> bool{
        
        self.physics.is_object_on_mission(&id)
    }

    pub fn remove_object(&mut self, id: &u16){

        self.physics.remove_object(id);
    }
    


    
    pub fn get_isometry_and_shape(& self, id: &u16) -> (Isometry3<f32>, Box<dyn Shape>){

        return ( self.physics.get_isometry(id), self.physics.get_shape(id) );
    }
    

    /*
    pub fn get_objects_on_long_raise_mission(&self) -> Vec<u16>{

        self.get_objects_on_mission_of_type( &MissionType::LongRaise)
    }
    */




    /*
    fn get_objects_on_mission_of_type(&self, missiontype: &MissionType) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        for (affectedobjectid, mission) in self.physics.get_active_missions(){
            if &mission.mission_type() == missiontype{
                toreturn.push( affectedobjectid);
            }
        }
        
        toreturn        
    }
    */


}
use rapier3d::geometry::Shape;








/*
//the types of missions
#[derive(PartialEq)]
enum MissionType{
    
    LongDrop,
    
    LongRaise,
    
    ShortDrop,
    
    Slide,
    
    LiftAndMove,
    
    Flick,
}

impl MissionType{
    
    fn to_number(&self) -> u16{
        match *self{
            MissionType::LongDrop => 0,
            MissionType::LongRaise => 1,
            MissionType::ShortDrop => 2,
            MissionType::Slide => 3,
            MissionType::LiftAndMove => 4,
            MissionType::Flick => 5,
        }
    }
    
    fn from_number(number: u16) -> MissionType{
        
        match number{
            0 => MissionType::LongDrop,
            1 => MissionType::LongRaise,
            2 => MissionType::ShortDrop,
            3 => MissionType::Slide,
            4 => MissionType::LiftAndMove,
            5 => MissionType::Flick,
            _ => panic!("what number is this?"),
        }
    }   
}
*/