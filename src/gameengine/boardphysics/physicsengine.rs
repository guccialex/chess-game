use rapier3d::dynamics::{CCDSolver, JointSet, RigidBodySet, IntegrationParameters};
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

use rapier3d::dynamics::RigidBody;
use rapier3d::dynamics::RigidBodyHandle;
use rapier3d::geometry::Collider;
use rapier3d::geometry::ColliderHandle;
use rapier3d::geometry::SharedShape;


use rapier3d::geometry::Cuboid;
use rapier3d::geometry::Cylinder;



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
    
    ccdsolver: CCDSolver,
    
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




use rapier3d::na::Point3;
use rapier3d::geometry::Ray;
use rapier3d::geometry::InteractionGroups;

use rapier3d::pipeline::QueryPipeline;
//a getter and setter for the state of the physics engine
//its the 
impl RapierPhysicsWrapper{

    
    //pass in a position, and the direction of looking
    pub fn get_object_intersection(& self, ray: (Point3<f32>, Vector3<f32>) ) -> Option<u16>{

        let ray = Ray::new( ray.0, ray.1 );
        let max_toi = 1000.0;
        let solid = true;

        let groups = InteractionGroups::all();


        let mut wallhandles: Vec<ColliderHandle> = Vec::new();

        for wallid in 0..4{
            wallhandles.push( self.shapehandles.get(&wallid).unwrap().clone()  );
        }


        let tclosure = move |handle, _: &Collider| { 

            let wallhandles = wallhandles.clone();

            if wallhandles.contains(&handle){

                return false;
            }

            true
        } ;

        //to filter out the wall, aka, the things with ID 0-3
        let filter: std::option::Option<&dyn for<'r> std::ops::Fn(rapier3d::geometry::ColliderHandle, &'r rapier3d::geometry::Collider) -> bool>
         = Some( & tclosure );


        let mut query_pipeline = QueryPipeline::new();

        query_pipeline.update(&self.bodies, &self.colliders);




        if let Some((handle, toi)) = query_pipeline.cast_ray(
            &self.colliders, &ray, max_toi, solid, groups, filter
        ) {
            
            //get the id of the shape
            for (id, curhandle) in &self.shapehandles.clone(){

                if &handle == curhandle{

                    return Some(*id);
                }
            }
            
            panic!("ray intersects with a shape that doesnt have an id?");
        }else{

            //panic!(" no intersection");

            return None;
        }

    }
    
    
    
    pub fn new() -> RapierPhysicsWrapper{
        
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
        //integration_parameters.multiple_ccd_substep_sensor_events_enabled = true;
        integration_parameters.set_inv_dt(30.0);
        
        
        
        // Run the simulation in the game loop.
        RapierPhysicsWrapper{
            gravity,
            integration_parameters,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            joints,
            ccdsolver: CCDSolver::new(),

            
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
        .translation( 0.0, 0.0, 0.0 )
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
    
    
    pub fn get_mut_rigidbody(&mut self, ID: &u16) -> &mut RigidBody{
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        self.bodies.get_mut(*rbhandle).unwrap()
    }

    pub fn get_rigidbody(& self, ID: &u16) -> & RigidBody{

        let rbhandle = self.bodyhandles.get(ID).unwrap();
        self.bodies.get(*rbhandle).unwrap()
    }

    
    //apply a change of position to an object
    fn apply_delta_position(&mut self, ID: &u16, deltapos: (f32,f32,f32) ){
        
        use nalgebra::geometry::Translation3;
        
        let rigidbody = self.get_mut_rigidbody(ID);
        
        let mut pos = *rigidbody.position();
        
        let tra = Translation3::new(deltapos.0, deltapos.1, deltapos.2);
        
        pos.append_translation_mut(&tra);
        
        rigidbody.set_position(pos, false);
    }
    
    //apply a impulse force to an object
    fn apply_delta_impulse(&mut self, ID: &u16, impulse: Vector3<f32>){
        self.get_mut_rigidbody(ID).apply_impulse(impulse, false);
    }


    //get the translation of the position of an object
    pub fn get_translation(&self, ID: &u16) -> (f32,f32,f32){
        
        let translation = self.get_rigidbody(ID).position().translation;
        
        (translation.x, translation.y, translation.z)
    }
    
    pub fn set_translation(&mut self, ID: &u16, position: (f32,f32,f32)  ) {
        
        let pos = Isometry3::translation(position.0, position.1, position.2);
        self.get_mut_rigidbody(ID).set_position(pos, false);
    }

    
    pub fn get_isometry(&self, ID: &u16) -> Isometry3<f32>{
        self.get_rigidbody(ID).position().clone()
    }

    pub fn set_isometry(&mut self, ID: &u16, isometry: Isometry3<Real>){
        self.get_mut_rigidbody(ID).set_position(isometry, false);
    }

    
    //get the translation of an object
    pub fn get_rotation(&self, ID: &u16) -> (f32,f32,f32){
        
        let rotation = self.get_rigidbody(ID).position().rotation.euler_angles();
        
        (rotation.0, rotation.1, rotation.2)
    }
    
    pub fn set_rotation(&mut self, ID: &u16, rotation:(f32,f32,f32) ){

        let oldisometry = self.get_mut_rigidbody(ID).position();
        let oldtranslation = oldisometry.translation;
        
        use nalgebra::geometry::UnitQuaternion;
        
        let newrotation = UnitQuaternion::from_euler_angles( rotation.0, rotation.1, rotation.2);
        
        let newisometry = Isometry3::from_parts(oldtranslation, newrotation);
        
        self.get_mut_rigidbody(ID).set_position(newisometry, false);
    }

    
    /*
    //get its velocity in each dimension
    pub fn get_linear_velocity(&self, ID: &u16) -> (f32,f32,f32){
        
        let linvel = self.get_mut_rigidbody(ID).linvel();
        
        (linvel.x, linvel.y, linvel.z)
    }
    
    //get its velocity in each dimension
    pub fn set_linear_velocity(&mut self, ID: &u16, linearvelocity:(f32,f32,f32) ){
        
        let linvel = Vector3::new(linearvelocity.0, linearvelocity.1, linearvelocity.2);
        
        self.get_mut_rigidbody(ID).set_linvel(linvel, false);
    }

    //get its velocity in each dimension
    pub fn get_angular_velocity(&self, ID: &u16) -> (f32,f32,f32){
        
        let linvel = self.get_mut_rigidbody(ID).angvel();
        
        (linvel.x, linvel.y, linvel.z)
    }
    
    //get its velocity in each dimension
    pub fn set_angular_velocity(&mut self, ID: &u16, angularvelocity:(f32,f32,f32) ){
        
        let angvel = Vector3::new(angularvelocity.0, angularvelocity.1, angularvelocity.2);
        
        self.get_mut_rigidbody(ID).set_angvel(angvel, false);
    }
    */
    

    pub fn get_mut_collider(&mut self, ID: &u16) -> &mut Collider{

        let chandle = self.shapehandles.get(ID).unwrap();

        self.colliders.get_mut( *chandle ).unwrap()
    }

    pub fn get_collider(& self, ID: &u16) -> & Collider{

        let chandle = self.shapehandles.get(ID).unwrap();

        self.colliders.get( *chandle ).unwrap()
    }


    pub fn get_shape(& self, ID: &u16) -> Box<dyn Shape>{

        self.get_collider(ID).shape().clone_box()
    }

    
    pub fn set_shape_sphere(&mut self, ID: &u16, diameter: f32){
        
        let radius = diameter /2.0;

        self.get_mut_collider(ID).set_shape( SharedShape::new( Ball::new(radius) ) );
    }
    
    pub fn set_shape_cuboid(&mut self, ID: &u16, dimensions: (f32,f32,f32)){
        
        let dimensions = (dimensions.0 / 2.0, dimensions.1/2.0, dimensions.2 / 2.0);
        
        self.get_mut_collider(ID).set_shape( SharedShape::new( Cuboid::new( Vector3::new(dimensions.0, dimensions.1, dimensions.2) ) ) );
    }
    
    pub fn set_shape_cylinder(&mut self, ID: &u16, height: f32, diameter: f32){
        
        let halfheight = height /2.0;
        let radius = diameter /2.0;
        
        self.get_mut_collider(ID).set_shape( SharedShape::new( Cylinder::new( halfheight, radius) ) ) ;   
    }
    
    
    
    pub fn set_materials(&mut self, ID: &u16, elasticity: f32, friction: f32){
        
        let collider = self.get_mut_collider(ID);

        collider.friction = friction;
        collider.restitution = elasticity;
    }
    
    
    fn make_static(&mut self, ID: &u16){  
        self.get_mut_rigidbody(ID).set_body_status(BodyStatus::Static) ;
    }
    
    
    
    pub fn tick(&mut self){
        
        self.tick_missions();

        let mut oldstatus = HashMap::new();
        
        //make the object static for the objects it should be static for this tick
        for objid in &self.no_gravity_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
            
            rigidbody.set_gravity_scale(0.0, false);

            oldstatus.insert( objid , rigidbody.body_status() );

            rigidbody.set_body_status(BodyStatus::Static) ;            
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
            &mut self.ccdsolver,
            &(),
            &(),
        );
        
        
        //restore the objects made static to what they were before
        for objid in &self.no_gravity_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
            
            rigidbody.set_gravity_scale(1.0, false);
            
            rigidbody.set_body_status( *oldstatus.get(objid).unwrap() );
        };
        
        
        
        //clear the objects that had their gravity disabled this tick
        self.no_gravity_for_tick = HashSet::new();
        
    }
    
    
    
    fn does_object_with_id_exist(&self, id: &u16) -> bool{
        
        if let Some(rbhandle) = self.bodyhandles.get(id){

            if let Some(_) = self.bodies.get(*rbhandle){
            
                return true;
            }
        }
            
        return false;

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
    
    
    pub fn end_mission(&mut self, ID: &u16){

        //if this object is on a mission
        if let Some(mission) = self.missions.get(ID){
            
            //if this object has a default isometry
            if let Some(isometry) = mission.get_default_isometry(){
                
                self.set_isometry(ID, isometry);
            }
        }
        
        self.missions.remove(ID);
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





use rapier3d::math::Real;



//a mission
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Mission{
    
    //the current tick the mission is currently on
    currenttick: u32,
    
    //START is inclusive
    //END is exclusive
    
    //the force (impulse) to apply when the current tick is in range
    //a vector with the scalar of the force
    impulses: Vec< (u32, u32, Vector3<Real>) >,
    
    //the change in position to apply when the current tick is in range
    //(call the "disable gravity for a tick" when this is being called)
    positionchanges: Vec< (u32, u32, Vector3<Real>) >,
    
    //the position and velocity that this object should be in when this mission is over
    defaultpos: Option< Isometry3<Real> >,
    
    
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
    
    
    pub fn set_default_isometry(&mut self, isometry: Isometry3<f32> ){
        self.defaultpos = Some( isometry );
    }
    
    fn get_default_isometry(&self) -> Option<Isometry3<Real>>{
        self.defaultpos
    }
    
}
