use rapier3d::dynamics::{JointSet, RigidBodySet, IntegrationParameters};
use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier3d::pipeline::PhysicsPipeline;

use rapier3d::dynamics::{BodyStatus, RigidBodyBuilder};

use std::collections::HashMap;
use std::collections::HashSet;


use rapier3d::na as nalgebra;
use nalgebra::{Vector3, Isometry3};

//use ncollide3d::shape::ConvexHull;
use rapier3d::geometry::{ColliderBuilder, Shape, Ball};


use serde::{Serialize, Deserialize};



use rapier3d::dynamics::RigidBodyHandle;
use rapier3d::geometry::ColliderHandle;



#[derive(Serialize, Deserialize)]
pub struct RapierPhysicsEngine{
    
    
    //this should be stored but not serialized. hmm
    //pipeline: PhysicsPipeline,
    gravity: Vector3<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    
    
    //objectid to its rigidbody handle
    bodyhandles: HashMap<u16, RigidBodyHandle>,
    //the main shape/collider associated with each body
    shapehandles: HashMap<u16, ColliderHandle>,
    
    totalobjects: u16,
    
    
    //objects who will be considered static for the next physical tick
    static_for_tick: HashSet<u16>,
    
    
}






//a getter and setter for the state of the physics engine
//its the 
impl RapierPhysicsEngine{
    
    
    pub fn new() -> RapierPhysicsEngine{
        
        //this should be stored but not serialized. hmm
        //let mut pipeline = PhysicsPipeline::new();
        
        
        let gravity = Vector3::new(0.0, -20.0, 0.0);
        let mut integration_parameters = IntegrationParameters::default();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let mut joints = JointSet::new();
        // We ignore contact events for now.
        let event_handler = ();
        
        
        
        integration_parameters.warmstart_coeff = 1.0;
        integration_parameters.max_ccd_substeps = 10;
        integration_parameters.multiple_ccd_substep_sensor_events_enabled = true;
        integration_parameters.set_inv_dt(30.0);
        
        
        
        // Run the simulation in the game loop.
        RapierPhysicsEngine{
            gravity: gravity,
            integration_parameters: integration_parameters,
            broad_phase: broad_phase,
            narrow_phase: narrow_phase,
            bodies: bodies,
            colliders: colliders,
            joints: joints,
            
            bodyhandles: HashMap::new(),
            shapehandles: HashMap::new(),
            totalobjects: 0,
            static_for_tick: HashSet::new(),
        }
        
    }
    
    
    
    
    //add an object to this physics world
    //return the ID for the object
    pub fn add_object(&mut self) -> u16{
        
        let objectid = self.totalobjects;
        self.totalobjects += 1;
        
        
        let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .angular_damping(3.5)
        .linear_damping(0.0)
        .can_sleep(false)
        .build();
        
        
        let rbhandle = self.bodies.insert(rigid_body);
        
        self.bodyhandles.insert(objectid, rbhandle);
        
        
        
        let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
        .translation(0.0, 0.0, 0.0)
        .density(1.3)
        .friction(0.8)
        .build();
        
        
        let chandle = self.colliders.insert(collider, rbhandle, &mut self.bodies);
        
        self.shapehandles.insert(objectid, chandle);
        
        return objectid;        
        
    }
    
    
    
    
    pub fn remove_object(&mut self, id:&u16){
        
        if let Some(rhandle) = self.bodyhandles.remove(id){
            
            self.shapehandles.remove(id);
            
            self.static_for_tick.remove(id);
            
            self.bodies.remove(rhandle, &mut self.colliders, &mut self.joints);
        }
        
    }
    
    
    
    
    
    
    pub fn make_static_for_tick(&mut self, ID: &u16){
        self.static_for_tick.insert(*ID);
    }
    
    
    //physical property getters and setters
    
    
    //apply a change of position to an object
    pub fn apply_delta_position(&mut self, ID: &u16, deltapos: (f32,f32,f32) ){
        
        use  nalgebra::geometry::Translation3;
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        let mut pos = *rigidbody.position();
        
        let tra = Translation3::new(deltapos.0, deltapos.1, deltapos.2);
        
        pos.append_translation_mut(&tra);
        
        if !rigidbody.is_moving() || true {
            rigidbody.set_position(pos, false);
        }
        
    }
    
    //apply a impulse force to an object
    pub fn apply_delta_impulse(&mut self, ID: &u16, impulse: Vector3<f32>){
        
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();

        //rigidbody.wake_up(true);
        
        if !rigidbody.is_moving() || true {
            rigidbody.apply_impulse(impulse, false);
        }

        
    }
    
    
    pub fn set_translation(&mut self, ID: &u16, position: (f32,f32,f32)  ) {
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        let pos = Isometry3::translation(position.0, position.1, position.2);
        
        if !rigidbody.is_moving() || true {
            rigidbody.set_position(pos, false);
        }

    }
    
    //get the translation of the position of an object
    pub fn get_translation(&self, ID: &u16) -> (f32,f32,f32){
        
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get(*rbhandle).unwrap();
        
        let translation = rigidbody.position().translation;
        
        (translation.x, translation.y, translation.z)
        
    }
    
    //get the translation of an object
    pub fn get_rotation(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get(*rbhandle).unwrap();
        
        let rotation = rigidbody.position().rotation.euler_angles();
        
        (rotation.0, rotation.1, rotation.2)
        
    }
    
    pub fn set_rotation(&mut self, ID: &u16, rotation:(f32,f32,f32) ){
        
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        let mut rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        
        let oldisometry = rigidbody.position();
        let oldtranslation = oldisometry.translation;
        
        
        use nalgebra::geometry::UnitQuaternion;
        
        let newrotation = UnitQuaternion::from_euler_angles( rotation.0, rotation.1, rotation.2);
        
        let newisometry = Isometry3::from_parts(oldtranslation, newrotation);
        

        if !rigidbody.is_moving() || true {
            rigidbody.set_position(newisometry, false);
        }
        
    }
    
    //get its velocity in each dimension
    pub fn get_linear_velocity(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        let rigidbody = self.bodies.get(*rbhandle).unwrap();
        
        let linvel = rigidbody.linvel();
        
        (linvel.x, linvel.y, linvel.z)
    }
    
    //get its velocity in each dimension
    pub fn set_linear_velocity(&mut self, ID: &u16, linearvelocity:(f32,f32,f32) ){
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        
        let linvel = Vector3::new(linearvelocity.0, linearvelocity.1, linearvelocity.2);
        
        rigidbody.set_linvel(linvel, false);
    }
    
    //get its velocity in each dimension
    pub fn get_angular_velocity(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        let rigidbody = self.bodies.get(*rbhandle).unwrap();
        
        let linvel = rigidbody.angvel();
        
        (linvel.x, linvel.y, linvel.z)
        
    }
    
    //get its velocity in each dimension
    pub fn set_angular_velocity(&mut self, ID: &u16, angularvelocity:(f32,f32,f32) ){
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        
        let angvel = Vector3::new(angularvelocity.0, angularvelocity.1, angularvelocity.2);
        
        rigidbody.set_angvel(angvel, false);
    }
    
    
    pub fn set_shape_sphere(&mut self, ID: &u16, diameter: f32){
        
        let radius = diameter /2.0;
        self.remove_bodies_colldiers(ID);
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let collider = ColliderBuilder::ball(radius).build();
        
        
        
        
        let oldfriction = collider.friction;
        let oldrestitution = collider.restitution;
        
        let chandle = self.colliders.insert(collider, *rbhandle, &mut self.bodies);
        self.shapehandles.insert(*ID, chandle);
        
        let collider = self.colliders.get_mut(chandle).unwrap();
        collider.friction = oldfriction;
        collider.restitution = oldrestitution;
        
    }
    
    
    pub fn set_shape_cuboid(&mut self, ID: &u16, dimensions: (f32,f32,f32)){
        
        let dimensions = (dimensions.0 / 2.0, dimensions.1/2.0, dimensions.2 / 2.0);
        
        self.remove_bodies_colldiers(ID);
        
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let collider = ColliderBuilder::cuboid(dimensions.0, dimensions.1, dimensions.2).build();
        
        
        
        
        let oldfriction = collider.friction;
        let oldrestitution = collider.restitution;
        
        let chandle = self.colliders.insert(collider, *rbhandle, &mut self.bodies);
        self.shapehandles.insert(*ID, chandle);
        
        let collider = self.colliders.get_mut(chandle).unwrap();
        collider.friction = oldfriction;
        collider.restitution = oldrestitution;
        
    }
    
    pub fn set_shape_cylinder(&mut self, ID: &u16, height: f32, diameter: f32){
        
        
        
        let halfheight = height/2.0;
        let radius = diameter /2.0;
        
        self.remove_bodies_colldiers(ID);
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let collider = ColliderBuilder::cylinder(halfheight, radius).build();
        
        
        
        
        
        let oldfriction = collider.friction;
        let oldrestitution = collider.restitution;
        
        let chandle = self.colliders.insert(collider, *rbhandle, &mut self.bodies);
        self.shapehandles.insert(*ID, chandle);
        
        let collider = self.colliders.get_mut(chandle).unwrap();
        collider.friction = oldfriction;
        collider.restitution = oldrestitution;
    }
    
    //remove the colliders from the body of this object
    fn remove_bodies_colldiers(&mut self, ID: &u16){
        
        //get the colliders associated with the rigidbody with this ID
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap().clone();
        
        let colliders = rigidbody.colliders();
        
        for colliderhandle in colliders{
            //remove them from the collider set
            self.colliders.remove(*colliderhandle, &mut self.bodies, true );
        }
        
    }
    
    
    pub fn set_materials(&mut self, ID: &u16, elasticity: f32, friction: f32){
        let colliderid = self.shapehandles.get(ID).unwrap();
        
        let collider = self.colliders.get_mut(*colliderid).unwrap();
        
        collider.friction = friction;
        collider.restitution = elasticity;
    }
    
    
    //DO NOT SET THIS AFTER CREATION
    pub fn make_static(&mut self, ID: &u16){  
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        rigidbody.body_status = BodyStatus::Static;
    }
    
    
    
    pub fn tick(&mut self){
        
        
        
        //make the object static for the objects it should be static for this tick
        for objid in &self.static_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
            
            rigidbody.set_gravity_scale(0.0, false);

        }
        
        
        let mut temppipeline = PhysicsPipeline::new();
        
        
        temppipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &(),
            &(),
        );
        

        //restore the objects made static to what they were before
        for objid in &self.static_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
            
            rigidbody.set_gravity_scale(1.0, false);
        };
        
        
        
        //clear the objects that had their gravity disabled this tick
        self.static_for_tick = HashSet::new();
        
    }
    
}







#[derive(Serialize, Deserialize)]
pub struct RapierMissionExtender{
    
    
    //the misions
    //the map of piece to mission
    missions: HashMap<u16, Mission>,
    
    //how long until this mission, the object its applied to, the mission
    futuremissions: Vec<(i32, u16, Mission)>,
    
}


impl RapierMissionExtender{
    
    pub fn new() -> RapierMissionExtender{
        
        RapierMissionExtender{
            missions: HashMap::new(),
            futuremissions: Vec::new(),
        }
    }
    
    //for each active mission, get its mission type
    pub fn get_active_missions_of_type(&self, wantedmissiontype: MissionType ) -> Vec<u16>{
        
        
        
        let mut toreturn = Vec::new();
        
        //for each active mission
        for (objectid, mission) in &self.missions{
            
            let missiontype = mission.get_mission_type();
            
            if missiontype == wantedmissiontype{
                
                toreturn.push(*objectid);
            }
        }
        
        
        toreturn
        
    }
    
    
    
    pub fn is_object_on_mission(&self, objectid: &u16) -> bool{
        
        self.missions.contains_key(objectid)
        
    }
    
    pub fn set_mission(&mut self, pieceid: u16, mission: Mission){
        
        //if that piece already has a mission, end its
        self.set_future_mission(0, pieceid, mission);
        
    }
    
    pub fn set_future_mission(&mut self, ticks: u32, pieceid: u16, mission: Mission){
        
        
        self.futuremissions.push( (ticks as i32, pieceid, mission) );        
    }
    
    
    
    
    
    
    pub fn end_missions(&mut self, id: &u16, physicsengine: &mut RapierPhysicsEngine){
        
        
        //if this object is on a mission
        if let Some(mission) = self.missions.get(id){
            
            //if this object has a default isometry
            if let Some((pos, rot)) = mission.get_default_isometry(){
                
                physicsengine.set_translation(id, pos);
                physicsengine.set_rotation(id, rot);
                
            }
        }
        
        self.missions.remove(id);
    }
    
    
    
    
    
    pub fn tick_missions(&mut self, physicsengine: &mut RapierPhysicsEngine){
        
        
        //the future missions
        {   
            
            //tick the future missions down and start it if the tick is 0
            for thing in self.futuremissions.iter_mut(){
                
                let (tick, objectid, mission) = thing;
                
                *tick = *tick - 1;
                
                //if its time to start the mission, just start it by putting it in the list of missions 
                //if its less than zero
                if *tick <= 0 { 
                    
                    //if there is already a mission for this object
                    if self.missions.contains_key(&objectid){
                    }
                    else{
                        //set the mission and return true
                        self.missions.insert(*objectid, mission.clone());
                    }

                }

            };
            
            
            
            //remove the future mission if the tick is 0
            self.futuremissions.retain(|(tick, objectid, mission)|{            
                
                //if the tick is 0 or less
                if *tick <= 0 {
                    
                    //remove it
                    return false;
                }
                else{
                    //keep it
                    return true;
                }
                
            });
            
            
        }
        
        
        
        
        //the ids of the missions that are expired
        let mut finishedmissions: Vec<u16> = Vec::new();
        
        //for each mission
        for (physicalid, mission) in self.missions.iter_mut(){
            
            //panic!("IMPULSSE HERE");
            
            //if there is an impulse
            if mission.is_current_impulse(){
                
                let currentimpulsevector = mission.get_current_impulse();


                physicsengine.apply_delta_impulse(physicalid, currentimpulsevector);

                
            }
            
            if mission.is_current_position_change(){          
                
                let poscvector = mission.get_current_delta_position();
                
                physicsengine.apply_delta_position(physicalid, (poscvector.x, poscvector.y, poscvector.z) );
                
                //and set it to be static for a tick
                physicsengine.make_static_for_tick(physicalid);
            }
            
            //physicsengine.apply_delta_position(physicalid, (0.0,5.0,0.0) );
            
            
            //then tick the mission
            //end and remove it if it needs to be ended and removed
            //and remove the sensor that the piece had on that mission
            mission.tick();
            
            if mission.is_finished() {
                finishedmissions.push(*physicalid);
            }
            
        }
        
        
        
        //remove each finished mission
        for objectid in &finishedmissions{
            
            self.end_missions(objectid, physicsengine);
            
        }
        
        
        
        
    }
    
    
    
    
    
    
    
    
}







//what type of mission it is
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MissionType{
    
    RaiseSquare,
    
    DropSquare,
    
    LoopAround,
    
    
    Slide,
    
    LiftAndMove,
    
    
    None
    
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
    
    
    
    //the position and velocity that this object should be in when this mission is over
    defaultpos: Option<((f32,f32,f32), (f32,f32,f32))>,
    
    
    missiontype: MissionType
    
}

impl Mission{
    
    
    fn default_mission() -> Mission{
        
        Mission{
            currenttick: 0,
            
            impulses: Vec::new(),
            
            positionchanges: Vec::new(),
            
            defaultpos: None,
            
            missiontype: MissionType::None
        }
    }
    
    
    fn get_mission_type(&self) -> MissionType{
        
        self.missiontype.clone()
    }
    
    //tick the mission
    //the tick should be done after performing the effects of the mission
    //so tick 0 is run
    fn tick(&mut self){
        
        self.currenttick += 1;
    }
    


    
    //if this mission is finished
    fn is_finished(&self) -> bool{
        
        let mut isfinished = true;
        
        //see if theres any position change currently or in the future
        for (starttick, endtick, vector) in &self.positionchanges{
            
            if endtick >= &self.currenttick{
                
                isfinished = false;   
            }
        }
        
        //see if theres any impulses currently or in the future
        for (starttick, endtick, vector) in &self.impulses{
            if endtick >= &self.currenttick{
                isfinished = false;   
            }   
        }
        
        isfinished   
    }
    
    
    fn get_current_delta_position(&self) -> Vector3<f32>{
        
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
    
    
    


    



    
    
    //make the mission of flicking a piece
    pub fn make_flick_mission(direction: f32, force: f32) -> Mission{
        
        let mut toreturn = Mission::default_mission();
        
        toreturn.impulses.push( (0,1, Vector3::new( direction.cos()*force, 0.0 , direction.sin()*force )   ) );
        
        toreturn
    }
    
    
    //for pieces
    pub fn make_lift_mission(relativepos: (f32,f32)) -> Mission{
        
        let mut toreturn = Mission::default_mission();
        
        //the timesteps at which the states change
        let lifttomove = 10;
        let movetodrop = 20;
        let endtick = 30;
        
        
        let liftphysics = (0, lifttomove, Vector3::new(0.0, 0.1, 0.0)  );
        toreturn.positionchanges.push( liftphysics );
        
        
        let totalmoveticks = movetodrop - lifttomove;
        let xchangepertick = relativepos.0 / (totalmoveticks) as f32;
        let zchangepertick = relativepos.1 / (totalmoveticks) as f32;
        let movephysics = (lifttomove, movetodrop, Vector3::new(xchangepertick, 0.0, zchangepertick) );
        toreturn.positionchanges.push(movephysics);
        
        
        let lowerphysics = (movetodrop, endtick, Vector3::new(0.0, -0.1, 0.0) );
        toreturn.positionchanges.push(lowerphysics);
        
        toreturn
    }
    
    //make a slide mission given the relative position for the piece to slide to
    pub fn make_slide_mission(relativepos: (f32,f32)) -> Mission{
        
        let mut toreturn = Mission::default_mission();
        
        
        //get the distance so i can determine how long to make the slide
        let slidedistance = (relativepos.0 * relativepos.0 + relativepos.1 * relativepos.1).sqrt();
        
        //the total amount of ticks
        let ticks = (slidedistance * 5.0).ceil() as u32;
        
        
        let xchangepertick = relativepos.0 / (ticks) as f32;
        let zchangepertick = relativepos.1 / (ticks) as f32;
        
        
        let slidephysics = (0, ticks, Vector3::new(xchangepertick, 0.0, zchangepertick) );
        
        toreturn.positionchanges.push(slidephysics);        
        
        
        toreturn
    }
    
    
    //a mission for a boardsquare that drops it then makes it sink from the top back to teh bottom
    pub fn make_drop_and_loop_around() -> Mission{
        
        
        let mut toreturn = Mission::default_mission();
        
        
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
        
        
        
        let dropphysics = (0, enddrop, Vector3::new(0.0, -1.5, 0.0) );
        toreturn.positionchanges.push(dropphysics);
        
        let leftphysics = (enddrop, endleft, Vector3::new(-6.0, 0.0, 0.0) );
        toreturn.positionchanges.push(leftphysics);
        
        let raisephysics = (endleft, endraise, Vector3::new(0.0, 3.0, 0.0) );
        toreturn.positionchanges.push(raisephysics);
        
        let rightphysics = (endraise, endright, Vector3::new(6.0, 0.0, 0.0));
        toreturn.positionchanges.push(rightphysics);
        
        let restorephysics = ( endright, endrestore, Vector3::new(0.0, -0.50, 0.0) );
        toreturn.positionchanges.push(restorephysics);
        
        
        toreturn
        
    }
    
    
    pub fn make_lengthed_drop(ticks: u32) -> Mission{
        
        let mut toreturn = Mission::default_mission();
        
        
        //when the object stops dropping
        let enddrop = 5;
        let endleft = 10;
        let waitstillend = 10 + ticks;
        let restoreend = 10 + ticks + 5;
        
        
        let dropphysics = (0, enddrop, Vector3::new(0.0, -2.0, 0.0) );
        toreturn.positionchanges.push(dropphysics);
        
        //shoot the object to the left so nothing can stay on it
        let leftphysics = (enddrop, endleft, Vector3::new(-3.0, 0.0, 0.0) );
        toreturn.positionchanges.push(leftphysics);
        
        
        let waitphysics = (endleft, waitstillend, Vector3::new(0.0, 0.0, 0.0) );
        toreturn.positionchanges.push(waitphysics);
        
        
        //return the piece back to its original position
        let restorephysics = (waitstillend, restoreend, Vector3::new(3.0, 2.0, 0.0) );
        toreturn.positionchanges.push(restorephysics);


        toreturn.missiontype = MissionType::DropSquare;
        
        
        toreturn
    }
    
    pub fn make_lengthed_raise(ticks: u32) -> Mission{
        
        let mut toreturn = Mission::default_mission();
        
        //when the object stops dropping
        let endraise = 5;
        let wait = 5 + ticks;
        let restore = 5 + ticks + 5;
        
        
        let raisephysics = (0, endraise, Vector3::new(0.0, 0.2, 0.0) );
        toreturn.positionchanges.push(raisephysics);
        
        let waitphysics = (endraise, wait, Vector3::new(0.0, 0.0, 0.0) );
        toreturn.positionchanges.push(waitphysics);
        
        //return the piece back to its original position
        let restorephysics = (wait, restore, Vector3::new(0.0, -0.2, 0.0) );
        toreturn.positionchanges.push(restorephysics);

        toreturn.missiontype = MissionType::RaiseSquare;
        
        
        toreturn
    }
    
    
    
    
    
    
    pub fn set_default_isometry(&mut self, pos: (f32,f32,f32), rot: (f32,f32,f32)){
        
        self.defaultpos = Some( (pos, rot) );
    }
    
    
    pub fn get_default_isometry(&self) -> Option<((f32,f32,f32), (f32,f32,f32))>{
        
        self.defaultpos
    }
    
    
}
