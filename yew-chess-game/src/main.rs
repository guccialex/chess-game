//mod chesscheckers;

use gloo_console::log;

use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};

//extern crate console_error_panic_hook;
use std::panic;
use bevy_mod_picking::*;



#[derive(PartialEq, Eq)]
struct HeldMouse(bool);

struct SincePlayerAction(i32);


fn main() {

    panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    let window = web_sys::window().expect("no global `window` exists");

    let windowsize = (window.inner_width().unwrap().as_f64().unwrap() as f32  ,window.inner_height().unwrap().as_f64().unwrap() as f32);

    
    App::new()
        .add_plugins(DefaultPlugins)
        
        // .add_plugins_with(DefaultPlugins, |group| {
        //     // The web asset plugin must be inserted in-between the
        //     // `CorePlugin' and `AssetPlugin`. It needs to be after the
        //     // CorePlugin, so that the IO task pool has already been constructed.
        //     // And it must be before the `AssetPlugin` so that the asset plugin
        //     // doesn't create another instance of an assert server. In general,
        //     // the AssetPlugin should still run so that other aspects of the
        //     // asset system are initialized correctly.
        //     group.add_before::<bevy::asset::AssetPlugin, _>( bevy_web_asset::WebAssetPlugin)
        // })

        //.add_plugins( WebAssetPlugin )
        .insert_resource::<Option<PlayerInterface>>( None )
        .insert_resource::<Option<GameObject>>(None)
        .insert_resource(SincePlayerAction(60))
        .insert_resource(HeldMouse(false))
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(WindowDescriptor {
            width: windowsize.0,
            height: windowsize.1,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)
        //.add_system(counter)
        .add_system( add_player_interface )
        .add_system( tick_game )
        .add_system( update_objects )
        .add_system( held_mouse )

        .add_system( control_camera )
        //.add_system( click_input_event )
        .add_system( print_events )

        .add_startup_system( spawn_tasks )



        .run();
        // .insert_resource(WindowDescriptor {
        //     width: 300.,
        //     height: 300.,
        //     ..Default::default()
        // })
        // .add_plugins(DefaultPlugins)
        // .add_startup_system(hello_wasm_system)
        
        // .add_system(track_input_events)
        // .run();
}


fn held_mouse(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut held_mouse: ResMut<HeldMouse>
) {

    use bevy::input::ElementState;

    for event in mouse_button_input_events.iter() {

        //log!("some mouse inputs");

        if event.button == MouseButton::Left && event.state == ElementState::Released{
            *held_mouse = HeldMouse(false);
        }

        if event.button == MouseButton::Left && event.state == ElementState::Pressed{
            *held_mouse = HeldMouse(true);
        }
    }
}


use bevy::tasks::AsyncComputeTaskPool;

fn spawn_tasks(thread_pool: Res<AsyncComputeTaskPool>){


    let task = thread_pool.spawn(async move {

        log!("async runnign");


        // let body = reqwest::get("https://www.rust-lang.org")
        //     .await.unwrap()
        //     .text()
        //     .await.unwrap();


        // log!( &format!("thing {:?}", body) );

        //https://i.imgur.com/0vCAeat.png



        let body = reqwest::get("https://cdn.pixabay.com/photo/2017/02/01/10/09/basics-2029357_960_720.png")
            .await.unwrap();


        log!( &format!("thing {:?}", body) );



        /*
        let mut rng = rand::thread_rng();
        let start_time = Instant::now();
        let duration = Duration::from_secs_f32(rng.gen_range(0.05..0.2));
        while Instant::now() - start_time < duration {
            // Spinning for 'duration', simulating doing hard
            // compute work generating translation coords!
        }

        // Such hard work, all done!
        Transform::from_xyz(x as f32, y as f32, z as f32)
        */
    });



}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    //asset_server: Res<AssetServer>,
) {


    //log!("THING GET!");
    //let queen: Handle<Image> = asset_server.load("/static/xxb.png");
    //log!(&format!("queen{:?}", queen));


    /*
    let mut mesh = Mesh::from(shape::Cube { size: 3.0 });

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some( queen ),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    */

    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(   mesh ),
    //     material: material_handle,
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     ..Default::default()
    // })
    // .insert_bundle(PickableBundle::default());


    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-6.0, 8.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert_bundle(PickingCameraBundle::default());
}



fn tick_game( mut game: ResMut< Option< PlayerInterface>  > ,  mut sinceaction: ResMut< SincePlayerAction  > ){

    if let Some( game) = &mut*game{

        game.tick();



        if sinceaction.0 > 100{

            game.opponent_takes_action();
            
        }

        sinceaction.0 += 1;


    }


}


use std::collections::{HashSet, HashMap};
fn update_objects(
    game: Res< Option< PlayerInterface>  >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut selected: ResMut< Option< GameObject >  >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut objects: Query<(&mut Transform, &BoardObjectID, Entity, &Handle<StandardMaterial>)>,
    asset_server: Res<AssetServer>,
    ){



    if let Some( game) = &*game{

        let visiblegamestate = game.get_visible_game_state( &*selected );


        let mut visibleobjects: HashMap<BoardObjectID, &VisibleGameBoardObject> = HashMap::new();

        for object in &visiblegamestate.boardobjects{

            visibleobjects.insert( BoardObjectID(object.id.clone()) , object );
        }



        for (mut transform, objectid, entity, materialhandle) in objects.iter_mut(){

            if let Some(object) = visibleobjects.remove( objectid ){

                let xyz = object.isometry.translation.vector.as_slice();

                let x = xyz[0];
                let y = xyz[1];            
                let z = xyz[2];

                transform.translation.x = x;
                transform.translation.y = y;
                transform.translation.z = z;


                //log!( &format!("handle{:?}", materialhandle));
                //materials.get_handle(handle)


                if let Some(material) = materials.get_mut( &*materialhandle){

                    let newcolor = Color::rgb(object.color.0, object.color.1, object.color.2);

                    material.base_color = newcolor;
                }

            }
            else{
                //log!("despawning", objectid.0.id());
                commands.entity(entity).despawn();
            }
        }



        for (id, object) in visibleobjects{

            //log!("Spawning ", id.0.id() );

            let xyz = object.isometry.translation.vector.as_slice();

            let x = xyz[0];
            let y = xyz[1];            
            let z = xyz[2];

            let color = Color::rgb(0., 0., 0.);//.into(); //Color::rgb(object.color.0, object.color.1, object.color.2).into();

            use chessengine::TypedShape;

            let mut mesh = Mesh::from(shape::Cube { size: 1.0 });

            let typedshape = object.shape.as_typed_shape();

            if let TypedShape::Cylinder(cylinder) = typedshape{

                // mesh = Mesh::from(shape::Capsule{
                //     radius: 0.7,
                //     depth: 0.5,
                //     rings: 10,
                //     latitudes: 4,
                //     longitudes: 2,
                //     uv_profile: shape::CapsuleUvProfile::Uniform,
                // });

                mesh = Mesh::from( shape::Box::new(0.65, 0.5, 0.65) );
            
            }
            else if let TypedShape::Cuboid( cuboid ) = typedshape{


            }

            

            let mut material_handle = materials.add(StandardMaterial {
                base_color: color,
                //base_color_texture: Some( queen ),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..Default::default()
            });

            

            if let Some(texture) = &object.texturelocation{

                let texture = "/static/pieceart/".to_string() +texture;
                log!( &format!("texture {:?}", texture) );


                let texture: Handle<Image> = asset_server.load(&texture);

                //log!( &format!("texture {:?}", texture) );
                material_handle = materials.add(StandardMaterial {
                    base_color: color,
                    base_color_texture: Some(texture),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..Default::default()
                });

            }

                
            let shape = Mesh::from(shape::Cube { size: 1.0 });

            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(   mesh ),
                material: material_handle,
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            })
            .insert(  id )
            .insert_bundle(PickableBundle::default());

        }


    }

}





fn control_camera(
    mut mouse_held: Res<HeldMouse>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera: Query<(&mut Transform, &Camera )>,
    windows: Res<Windows>,
) {

    for (mut transform,_) in camera.iter_mut(){

        //log!("got camera");


        if *mouse_held == HeldMouse(true){

            //log!("left mouse inputs");

            let mut rotation_move = Vec2::ZERO;

            for ev in mouse_motion_events.iter() {

                //log!("mouse motion");

                rotation_move += ev.delta;
            }

            //the amount its rotated
            //add to teh cameras rotation the delta
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                //if pan_orbit.upside_down { -delta } else { delta }
                delta
            };

            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis

        
            let radius = 13.;

            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, radius));
        }
    }

}


fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

//get 
//if the mouse is down 
//move the camera to hte left or right or up or down
//and 


//get what's clicked
//get all the game objects
//get which one was clicked, send it to the game
fn print_events(
    mut game: ResMut< Option< PlayerInterface>  >,
    mut selected: ResMut< Option< GameObject >  >,
    mut events: EventReader<PickingEvent>,
    objects: Query<(&BoardObjectID, Entity)>,

    mut sinceaction: ResMut< SincePlayerAction  >
    ) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => {
            },
            PickingEvent::Hover(e) => {
            },
            PickingEvent::Clicked(e) =>
            { 

                //log!( &format!("{:?}", e));

                //let mut clickedobject = None;

                for (objectid, entity) in objects.iter(){

                    if &entity == e{
                        //clickedobject = Some(objectid);
                        //log!( &format!("clicked{:?}", objectid.0.id() ));


                        if let Some(game) = &mut *game{

                            //log!("got game now selecte");

                            let nowselected = game.clicked_object( selected.clone(), Some(GameObject::BoardObject( objectid.0.clone() )) );

                            //if something was clicked, and nothing is selected anymore, treat that as an action
                            if nowselected.is_none(){
                                sinceaction.0 = 0;
                            }
        
                            *selected = nowselected;

                            //log!( &format!("selected {:?}", &*selected) );

                            // if let Some(nowselected) = nowselected{
                            //     *selected = GameObject::BoardObject(nowselected);
                            // }
                            // *selected = None;
                            //log!( &format!("now selected {:?}", nowselected) );
                            
                        }
                    }
                }







                info!("Gee Willikers, it's a click! {:?}", e)
            
            },
        }
    }
}



#[derive(Component, Hash, Eq, PartialEq)]
struct BoardObjectID( BoardObject );


fn add_player_interface( mut game: ResMut< Option< PlayerInterface>  > ){

    if game.is_none(){

        *game = Some( PlayerInterface::new(1) );
    }

}


//the game struct
use chessengine::{PlayerInterface, GameObject};
use chessengine::BoardObject;

use chessengine::VisibleGameState;
//use chessengine::GameObject;
use chessengine::nalgebra;
use chessengine::rapier3d;

//use chessengine::B

use chessengine::VisibleGameBoardObject;


#[derive(Component)]
struct VisibleGameBoardObjectComponent(VisibleGameBoardObject);
