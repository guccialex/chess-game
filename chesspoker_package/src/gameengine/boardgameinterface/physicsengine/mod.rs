use rapier3d::dynamics::{JointSet, RigidBodySet, IntegrationParameters};
use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier3d::pipeline::PhysicsPipeline;

use rapier3d::dynamics::{BodyStatus, RigidBodyBuilder};

use std::collections::HashMap;
use std::collections::HashSet;

use nalgebra::{Vector3, Isometry3};

use ncollide3d::shape::ConvexHull;
use rapier3d::geometry::{ColliderBuilder, Shape, Ball};


use serde::{Serialize, Deserialize};


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
    bodyhandles: HashMap<u16,rapier3d::data::arena::Index>,
    
    //the main shape/collider associated with each body
    shapehandles: HashMap<u16, rapier3d::data::arena::Index>,
    
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
        
        
        
        integration_parameters.warmstart_coeff = 0.9;
        integration_parameters.max_ccd_substeps = 10;
        integration_parameters.multiple_ccd_substep_sensor_events_enabled = true;
        integration_parameters.set_inv_dt(60.0);
        
        
        
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
    
    
    pub fn tick(&mut self){

        //save the status of the body before making it static so to restore it to the proper state after
        let mut previousbodystatusbyobjid: HashMap<u16, BodyStatus> = HashMap::new();

        //make the object static for the objects it should be static for this tick
        for objid in &self.static_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
            previousbodystatusbyobjid.insert(*objid, rigidbody.body_status);
                    
            rigidbody.body_status = BodyStatus::Static;
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
            None,
            None,
            &()
        );




        
        //restore the objects made static to what they were before
        for objid in &self.static_for_tick{
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();

            let rbstatus = previousbodystatusbyobjid.get(&objid).unwrap();
                    
            rigidbody.body_status = *rbstatus;
        }
        
        
        
        //clear the objects that had their gravity disabled this tick
        self.static_for_tick = HashSet::new();
        
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
        
        rigidbody.set_position(pos, true);
        
    }
    
    //apply a impulse force to an object
    pub fn apply_delta_impulse(&mut self, ID: &u16, impulse: Vector3<f32>){
        
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        rigidbody.apply_impulse(impulse, true);
        
    }
    
    
    pub fn set_translation(&mut self, ID: &u16, position: (f32,f32,f32)  ) {
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        let pos = Isometry3::translation(position.0, position.1, position.2);
        
        rigidbody.set_position(pos, true);
        
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
        
        rigidbody.set_position(newisometry, true);
        
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
        
        rigidbody.set_linvel(linvel, true);
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
        
        rigidbody.set_angvel(angvel, true);
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
    
    
    
    pub fn make_static(&mut self, ID: &u16){
        
        
        let rbhandle = self.bodyhandles.get(ID).unwrap();
        
        let rigidbody = self.bodies.get_mut(*rbhandle).unwrap();
        
        rigidbody.body_status = BodyStatus::Static;
    }
    
}

