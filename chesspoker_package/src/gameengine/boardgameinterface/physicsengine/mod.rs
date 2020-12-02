

use nalgebra::{Point3, RealField, Vector3};
use ncollide3d::shape::{Cuboid, Ball, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;

use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use nphysics3d::object::DefaultBodyHandle;
use nphysics3d::object::DefaultColliderHandle;

use nalgebra::geometry::Isometry3;

use nalgebra::geometry::Translation3;

use ncollide3d::shape::ConvexHull;

use nphysics3d::object::Body;

use nphysics3d::object::BodyStatus;



use nphysics3d::math::{Force, ForceType};



use std::collections::HashMap;
use std::collections::HashSet;

pub struct PhysicsEngine{
    
    totalobjects: u16,
    
    
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
    
    
    //objectid to its rigidbody handle
    bodyhandles: HashMap<u16, DefaultBodyHandle>,
    
    //the main shape/collider associated with each body
    bodytoshape: HashMap<u16, DefaultColliderHandle>,
    
    //a list of sensors, mapped to their colliderid
    sensors: HashMap<u16, DefaultColliderHandle>,
    
    
    
    //objects whos gravity will be disabled for only the next physical tick
    gravity_off_for_tick: HashSet<u16>,

    //objects who will be considered static for the next physical tick
    static_for_tick: HashSet<u16>,
    
}




impl PhysicsEngine{
    
    
    pub fn new() -> PhysicsEngine{
        
        

        //increase gravity so stuff drops quicker
        let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -70.00, 0.0));
        let geometrical_world = DefaultGeometricalWorld::new();
        let mut bodies = DefaultBodySet::<f32>::new();
        let mut colliders = DefaultColliderSet::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();
        
        
        
        PhysicsEngine{
            
            totalobjects: 0,
            
            mechanical_world: mechanical_world,
            geometrical_world: geometrical_world,
            bodies: bodies,
            colliders: colliders,
            joint_constraints: joint_constraints,
            force_generators: force_generators,
            
            
            bodyhandles: HashMap::new(),
            bodytoshape: HashMap::new(),
            
            sensors: HashMap::new(),
            
            gravity_off_for_tick: HashSet::new(),
            static_for_tick: HashSet::new(),
        }
        
        
        
    }
    
    
    //add an object to this physics world
    //return the ID for the object
    pub fn add_object(&mut self) -> u16{
        
        //maybe dont specify anything about the object but allow it to be changed in post before the tick
        
        let ID = self.totalobjects;
        self.totalobjects +=1;
        
        
        
        let cuboid = ShapeHandle::new(Cuboid::new(Vector3::repeat(1.0)));

        use nalgebra::base::Matrix3;
        
        // Build the rigid body.
        let rb = RigidBodyDesc::<f32>::new()
        .status(BodyStatus::Dynamic)
        //dont let it sleep, or it wont drop when the square is moved out from under
        .sleep_threshold(None)
        //.angular_inertia( Matrix3::new_rotation(0.00001) )
        .build();
        
        let rb_handle = self.bodies.insert(rb);
        
        // Build the collider.
        let co = ColliderDesc::new(cuboid.clone())
        .density(1.0)
        .build(BodyPartHandle(rb_handle, 0));
        
        let colliderhandle = self.colliders.insert(co);
        
        
        
        self.bodyhandles.insert(ID, rb_handle);
        self.bodytoshape.insert(ID, colliderhandle);
        
        
        ID
    }

    
    


    pub fn tick(&mut self){
        
        //turn the gravity off of objects that it should be turned off for
        for objid in &self.gravity_off_for_tick{
            
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
            
            rigidbody.enable_gravity(false);
            
        }

        //save the status of the body before making it static so to restore it to the proper state after
        let mut previousbodystatusbyobjid: HashMap<u16, BodyStatus> = HashMap::new();

        //make the object static for the objects it should be static for this tick
        for objid in &self.static_for_tick{

            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();

            previousbodystatusbyobjid.insert(*objid, rigidbody.status());
            
            rigidbody.set_status(BodyStatus::Static);

        }
        
        
        
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
        
        
        //reenable gravity for those objects
        for objid in &self.gravity_off_for_tick{
            
            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
            
            rigidbody.enable_gravity(true);
            
        }

        //set the  body status back to what it was
        for objid in &self.static_for_tick{

            let rbhandle = self.bodyhandles.get(&objid).unwrap();
            let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
            
            rigidbody.set_status( *previousbodystatusbyobjid.get(objid).unwrap() );

        }


        
        
        
        //clear the objects that had their gravity disabled this tick
        self.gravity_off_for_tick = HashSet::new();
        self.static_for_tick = HashSet::new();
        
    }

    
    //turn the force of gravity off for a piece for a single tick
    pub fn turn_gravity_off_for_tick(&mut self, ID: &u16){
        
        self.gravity_off_for_tick.insert(*ID);
        
    }


    pub fn make_static_for_tick(&mut self, ID: &u16){

        self.static_for_tick.insert(*ID);
    }

    


    //given the id of the object they're attached to
    //attach a sensor
    //then set its shape using the method "set_shape()"
    pub fn attach_sensor(&mut self, ID: &u16) -> u16{
        
        let sensorid = self.totalobjects;
        self.totalobjects += 1;
        
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbodypart = BodyPartHandle(*rbhandle, 0);
        
        
        let shape = ShapeHandle::new(Ball::new(1.5));
        
        let sensor = ColliderDesc::<f32>::new(shape)
        .sensor(true)
        .build(rigidbodypart);
        
        let sensor_handle = self.colliders.insert(sensor);
        
        self.sensors.insert(sensorid, sensor_handle);
        
        sensorid
        
        
    }


    //used to get the contacts of a sensor
    //given the id of a sensor, get the id of every object that is in contact with the sensor currently
    pub fn get_sensor_proximities(&self, sensorid: &u16) -> HashSet<u16>{
        
        let mut contacts = HashSet::new();
        
        //the handle of the sensor
        let sensorhandle = self.sensors.get(sensorid).unwrap();
        
        //get the things it contacts with
        for (colliderhandle1, collider1, colliderhandle2, collider2, _, contactmanifold) in self.geometrical_world.proximities_with(&self.colliders, *sensorhandle, true).unwrap(){
            
            //if the sensor is intersecting with the object
            if contactmanifold == ncollide3d::query::Proximity::WithinMargin{
                
                //return the collider (that isnt a sensor) that this sensor interacts with
                for (objectid, colliderhandle) in &self.bodytoshape{
                    
                    if (*colliderhandle == colliderhandle1 || *colliderhandle == colliderhandle2){
                        
                        contacts.insert(*objectid);
                        
                    }
                    
                }
                
            }
            
        }
        
        //return the objects this sensor intersects with
        contacts
    }
    


    


    //physical property getters and setters



    //apply a change of position to an object
    pub fn apply_delta_position(&mut self, ID: &u16, deltapos: Vector3<f32> ){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
        
        
        let oldisometry = rigidbody.position();
        let oldtra = oldisometry.translation;
        let oldrotation = oldisometry.rotation;
        
        let newtranslation = Translation3::new(deltapos.x + oldtra.x, deltapos.y+oldtra.y, deltapos.z+oldtra.z);
        
        
        let newisometry = Isometry3::from_parts(newtranslation, oldrotation);
        
        
        rigidbody.set_position(newisometry);
    }
    
    //apply a impulse force to an object
    pub fn apply_delta_impulse(&mut self, ID: &u16, impulse: Force<f32>){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
        
        
        
        rigidbody.apply_force(0, &impulse , ForceType::Impulse, true);
    }
    
    
    //set the position of an object with its ID
    //set its position without changing its rotation
    pub fn set_position(&mut self, ID: &u16, position: (f32,f32,f32)  ) {
        
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
        
        
        let oldisometry = rigidbody.position();
        let oldrotation = oldisometry.rotation;
        
        let newtranslation = Translation3::new(position.0, position.1, position.2);
        
        
        let newisometry = Isometry3::from_parts(newtranslation, oldrotation);
        
        
        rigidbody.set_position(newisometry);
        
    }

    //get the translation of the position of an object
    pub fn get_translation(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body(*rbhandle).unwrap();
        
        let translation = rigidbody.position().translation;
        
        (translation.x, translation.y, translation.z)
    }

    //get the translation of an object
    pub fn get_rotation(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body(*rbhandle).unwrap();
        
        let rotation = rigidbody.position().rotation.euler_angles();
        
        (rotation.0, rotation.1, rotation.2)
        
        
    }
    
    pub fn set_rotation(&mut self, ID: &u16, rotation:(f32,f32,f32) ){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
        
        
        let oldisometry = rigidbody.position();
        let oldtranslation = oldisometry.translation;


        use nalgebra::geometry::UnitQuaternion;
        
        let newrotation = UnitQuaternion::from_euler_angles( rotation.0, rotation.1, rotation.2);
        
        let newisometry = Isometry3::from_parts(oldtranslation, newrotation);
        
        rigidbody.set_position(newisometry);
        
    }
    
    //get its velocity in each dimension
    pub fn get_linear_velocity(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body(*rbhandle).unwrap();
        
        let velocity = rigidbody.velocity().linear;
        
        (velocity.x, velocity.y, velocity.z)
        
        
    }
    
    //get its velocity in each dimension
    pub fn set_linear_velocity(&mut self, ID: &u16, linearvelocity:(f32,f32,f32) ){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();

        let vector = Vector3::new( linearvelocity.0, linearvelocity.1, linearvelocity.2 );
        
        rigidbody.set_linear_velocity( vector );
        
    }
    
    //get its velocity in each dimension
    pub fn get_angular_velocity(&self, ID: &u16) -> (f32,f32,f32){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body(*rbhandle).unwrap();
        
        let velocity = rigidbody.velocity().angular;
        
        (velocity.x, velocity.y, velocity.z)
        
        
    }
    
    //get its velocity in each dimension
    pub fn set_angular_velocity(&mut self, ID: &u16, angularvelocity:(f32,f32,f32) ){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();

        let vector = Vector3::new( angularvelocity.0, angularvelocity.1, angularvelocity.2 );
        
        rigidbody.set_angular_velocity(vector);
        
    }
    
    pub fn set_shape(&mut self, ID: &u16, shape: ConvexHull<f32>){
        
        //change the shape of the main collider associated with the object passed by ID
        
        
        let colliderhandle = self.bodytoshape.get_mut(ID).unwrap();
        let mut collider = self.colliders.get_mut(*colliderhandle).unwrap();
        
        collider.set_shape( ShapeHandle::new( shape ) );
        
    }
    
    pub fn toggle_gravity(&mut self, ID: &u16, gravityornot: bool){
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
        
        rigidbody.enable_gravity(gravityornot);
        
    }
    
    pub fn make_static(&mut self, ID: &u16){
        
        
        let rbhandle = self.bodyhandles.get(&ID).unwrap();
        let rigidbody = self.bodies.rigid_body_mut(*rbhandle).unwrap();
        
        rigidbody.set_status(BodyStatus::Static);
        
        
        
    }

    
    
}


