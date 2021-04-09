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
pub struct RapierPhysicsWrapper{
    
    
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
    no_gravity_for_tick: HashSet<u16>,
    
    
    
    
    //the misions
    //the map of piece to mission
    missions: HashMap<u16, Mission>,
    
    //how long until this mission, the object its applied to, the mission
    futuremissions: Vec<(i32, u16, Mission)>,
    
}






//a getter and setter for the state of the physics engine
//its the 
impl RapierPhysicsWrapper{
    
    
    pub fn new() -> RapierPhysicsWrapper{
        
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
        RapierPhysicsWrapper{
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
            no_gravity_for_tick: HashSet::new(),
            
            missions: HashMap::new(),
            futuremissions: Vec::new(),
        }
        
    }
    
    
    
    
    //add an object to this physics world
    //return the ID for the object
    pub fn add_object(&mut self, isstatic: bool) -> u16{
        
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
        
        
        if isstatic{
            self.make_static(&objectid);
        }
        
        return objectid;        
        
    }
    
    
    
    
    pub fn remove_object(&mut self, id:&u16){
        
        if let Some(rhandle) = self.bodyhandles.remove(id){
            
            self.shapehandles.remove(id);
            
            self.no_gravity_for_tick.remove(id);
            
            self.bodies.remove(rhandle, &mut self.colliders, &mut self.joints);
        }
        
    }
    
    
    
    
    
    
    fn remove_gravity_for_tick(&mut self, ID: &u16){
        self.no_gravity_for_tick.insert(*ID);
    }
    
    
    //physical property getters and setters
    
    
    //apply a change of position to an object
    fn apply_delta_position(&mut self, ID: &u16, deltapos: (f32,f32,f32) ){
        
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
    fn apply_delta_impulse(&mut self, ID: &u16, impulse: Vector3<f32>){
        
        
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
    fn make_static(&mut self, ID: &u16){  
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        rigidbody.body_status = BodyStatus::Static;
    }
    
    
    
    pub fn tick(&mut self){
        
        self.tick_missions();
        
        //make the object static for the objects it should be static for this tick
        for objid in &self.no_gravity_for_tick{
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
        for objid in &self.no_gravity_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
            
            rigidbody.set_gravity_scale(1.0, false);
        };
        
        
        
        //clear the objects that had their gravity disabled this tick
        self.no_gravity_for_tick = HashSet::new();
        
    }
    
    
    
    fn does_object_with_id_exist(&self, id: &u16) -> bool{
        
        let rbhandle = self.bodyhandles.get(id).unwrap();
        
        if let Some(_) = self.bodies.get(*rbhandle){
            
            return true;
        }
        else{
            
            return false;
        }
        
    } 
    
    
    
    
    
    
    
    
    
    //get all the active missions and the object they're for
    pub fn get_active_missions(&self ) -> Vec<(u16, Mission)>{
        
        let mut toreturn = Vec::new();
        
        //for each active mission
        for (objectid, mission) in &self.missions{
            
            toreturn.push( (*objectid, mission.clone()) );
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
    
    
    pub fn end_mission(&mut self, id: &u16){
        
        
        //if this object is on a mission
        if let Some(mission) = self.missions.get(id){
            
            //if this object has a default isometry
            if let Some((pos, rot)) = mission.get_default_isometry(){
                
                self.set_translation(id, pos);
                self.set_rotation(id, rot);
            }
        }
        
        self.missions.remove(id);
    }
    
    
    fn tick_missions(&mut self){
        
        
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
                        //make the mission started
                        mission.started = true;
                        
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
        for (physicalid, mission) in self.missions.clone().iter(){
            
            //if an object with that physical ID exists in the engine
            if self.does_object_with_id_exist(physicalid){
                
                //if there is an impulse
                if mission.is_current_impulse(){
                    let currentimpulsevector = mission.get_current_impulse();
                    
                    self.apply_delta_impulse(physicalid, currentimpulsevector);
                }
                
                if mission.is_current_position_change(){          
                    
                    let poscvector = mission.get_current_delta_position();
                    
                    self.apply_delta_position(physicalid, (poscvector.x, poscvector.y, poscvector.z) );
                    
                    //and set it to be static for a tick
                    self.remove_gravity_for_tick(physicalid);
                }
                
            }
            
            
            
            
        }
        
        
        for (physicalid, mission) in self.missions.iter_mut(){
            
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
            
            self.end_mission(objectid);
        }
    }
    
    
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
    
    
    //if the mission has started, it cant have its position, impulse or default position change
    started: bool,
    
    
    
    //user data
    data: u16,
    
}

impl Mission{
    
    
    pub fn set_mission_data(&mut self, data: u16){
        
        self.data = data;
    }
    
    pub fn get_mission_data(&self) -> u16{
        self.data
    }
    
    
    pub fn add_position_change(&mut self, starttick: u32, endtick: u32, stepchange: (f32,f32,f32)){
        
        self.positionchanges.push( (starttick, endtick, Vector3::new(stepchange.0 , stepchange.1 , stepchange.2) ) );
    }
    
    pub fn add_impulse_change(&mut self, starttick: u32, endtick: u32, stepchange: (f32,f32,f32)){
        
        self.impulses.push( (starttick, endtick, Vector3::new(stepchange.0 , stepchange.1 , stepchange.2) ) );
    }
    
    pub fn default_mission(data: u16) -> Mission{
        
        Mission{
            currenttick: 0,
            
            impulses: Vec::new(),
            
            positionchanges: Vec::new(),
            
            defaultpos: None,
            
            started: false,
            
            data: data,
        }
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
    
    
    
    
    pub fn set_default_isometry(&mut self, pos: (f32,f32,f32), rot: (f32,f32,f32)){
        
        self.defaultpos = Some( (pos, rot) );
    }
    
    pub fn get_default_isometry(&self) -> Option<((f32,f32,f32), (f32,f32,f32))>{
        
        self.defaultpos
    }
    
    
}
