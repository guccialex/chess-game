


use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

use kiss3d::window::{State, Window};

use yew::services::console::ConsoleService;


use chessengine::VisibleGameState;
use chessengine::GameObject;
use chessengine::PlayerInterface;
use chessengine::nalgebra;


use chessengine::rapier3d;

use rapier3d::geometry::Shape;
use rapier3d::geometry::TypedShape;



use nalgebra::Vector3;
use nalgebra::Vector2;
use nalgebra::Point3;
use nalgebra::Point2;
use std::collections::HashMap;


use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use  kiss3d::resource::TextureManager;

use std::collections::HashSet;

use nalgebra::geometry::UnitQuaternion;


mod gui;
use gui::GuiObject;





fn ticks_to_string(ticks: u32) -> String{

    use std::time::Duration;

    //a tick is 33 ms
    let dur = Duration::from_millis(  ticks as u64 * 66 );

    //minutes, seconds, centiseconds
    let text = format!("{}:{:02}:{:02}", dur.as_secs() / 60 , dur.as_secs() % 60 , (dur.as_millis() % 1000) / 10 );

    text
}




//fetch the texture and return true if its been fetched
fn fetch_texture(channels: &mut GameChannels, texturesfetched: &mut HashSet<String>,  texture: &String) -> bool{



    //if the texture isnt fetched yet, fetch it and set it as being fetched
    if ! texturesfetched.contains(texture){

        ConsoleService::log( &format!( "Fetching texture: {:?}", texture )  );

        texturesfetched.insert( texture.clone() );

        channels.sendtexturerequest.send( texture.clone() );
    }

    //the textures that are fetched, set in the global texture manager by the name used to fetch them
    if let Ok( (name, bytes) ) = channels.receivefetchedtexture.try_recv(){

        //if this is a valid image

        //kiss3d::image;

        TextureManager::get_global_manager(|manager|{

            manager.add_image_from_memory( &bytes, &name );
        });
    }

    //if the texture exists
    let mut textureexists = false;
    TextureManager::get_global_manager(|manager|{
        if manager.get(&texture).is_some(){
            textureexists = true;
        }
    });

    return textureexists;
}





struct BoardObject{

    //the meshes of the pieces and squares
    body: SceneNode,

    //if this piece has a symbol
    symbol: Option<SceneNode>,

}



struct InnerGame{

    channels: GameChannels,

    //the meshes and data for the gui
    game: PlayerInterface,


    //if it is single player, set the opponents actions like every 10th tick
    singleplayer: Option<i32>,

    
    boardobjects: HashMap<u16, BoardObject>,



    guiobjects: HashMap<String, GuiObject>,



    texturesfetched: HashSet<String>,

    selectedobject: Option<GameObject>,

    camera: ArcBall,

    windowsize: (u32,u32),
}


impl InnerGame{


    fn new(playerid: u8, issingleplayer: bool, channels: GameChannels, window: &mut Window, windowsize: (u32,u32) ) -> InnerGame{


        let pos;
        let at = Point3::new(0.,0.,0.);

        if playerid == 1{
            pos = Point3::new(0. , 10., -10.);
        }
        else{
            pos = Point3::new(0. , 10., 10.);
        }

        let mut camera = kiss3d::camera::ArcBall::new( pos, at);

        camera.set_max_dist( 18. );
        camera.set_min_dist( 12. );


        
        //let cardsize = (2.25, 0.05, 3.5);



        let mut player1position = 0.;
        let mut player2position = 95.;

        if playerid == 1{

            player1position = 95.;
            player2position = 0.;

        }


        let mut guiobjects = HashMap::new();

        guiobjects.insert( "player1totalticksleft".to_string() , GuiObject::new( (0., player1position),  (10.,5.), &windowsize,  window  )  );
        guiobjects.insert( "player2totalticksleft".to_string() , GuiObject::new( (0., player2position),  (10.,5.), &windowsize,  window  )  );


        let mut x =  GuiObject::new( (15., player1position),  (10.,5.), &windowsize,  window  ) ;
        x.set_texture("green.png".to_string());
        guiobjects.insert( "player1ticksleft".to_string() , x );


        let mut x = GuiObject::new( (15., player2position),  (10.,5.), &windowsize,  window  );
        x.set_texture("green.png".to_string() );
        guiobjects.insert( "player2ticksleft".to_string() ,  x );



        guiobjects.insert( "clickcard".to_string() , GuiObject::new_align_left( (0., 15.), 20.,  0.7   , &windowsize,  window  ) );
        guiobjects.get_mut("clickcard").unwrap().set_texture("clickcard.png".to_string());



        guiobjects.insert( "pile0".to_string() , GuiObject::new_align_left( (0., 40.), 20.,  0.7   , &windowsize,  window  ) );
        guiobjects.insert( "pile1".to_string() , GuiObject::new_align_left( (11., 40.), 20., 0.7  , &windowsize,  window  ) );
        guiobjects.insert( "pile2".to_string() , GuiObject::new_align_left( (0., 65.),  20., 0.7, &windowsize,  window  ) );
        guiobjects.insert( "pile3".to_string() , GuiObject::new_align_left( (11., 65.), 20., 0.7, &windowsize,  window  ) );


        guiobjects.insert( "lastcardeffect".to_string() , GuiObject::new( (11., 15.),  (10.,20.), &windowsize,  window  ) );
        


        guiobjects.insert( "activeeffects".to_string() , GuiObject::new_align_left( (85., 0.), 18.,  0.7   , &windowsize,  window  ) );
        guiobjects.get_mut("activeeffects").unwrap().set_texture("activeeffects.png".to_string());


        for y in 0..5{

            for x in 0..2{

                let name = "effect".to_string() + & (x*5 + y).to_string();

                let xf = 90. - (x as f32 * 10.) ;

                let yf = y as f32 * 20. + 20.;

                guiobjects.insert( name , GuiObject::new_align_left( (xf, yf),  18. , 0.7, &windowsize,  window  ) );

            }

        }


        let mut singleplayer = None;

        if issingleplayer == true{

            singleplayer = Some( 10 );
        }



        InnerGame{

            channels,

            game: PlayerInterface::new(playerid),

            boardobjects: HashMap::new(),

            texturesfetched: HashSet::new(),

            guiobjects,

            camera,

            selectedobject: None,

            windowsize,

            singleplayer,
        }

    }



    fn tick(&mut self){

        if let Some(singleplayer ) = &mut self.singleplayer{

            *singleplayer += -1;

            if *singleplayer  == 0{
                self.game.opponent_takes_action();
            }

            //if it misses making a move at 0 for some reason
            if *singleplayer < -100{
                *singleplayer = 10;
            }
        }


        if let Ok(toset) = self.channels.receiveincoming.try_recv(){

            self.game.set_game_string_state( toset );
        }

        
        self.game.tick();
    }




    fn click(&mut self, pos: (f32,f32) ){


        let upos = (pos.0 as u32, pos.1 as u32);

        if self.guiobjects.get( "pile0" ).unwrap().is_clicked(upos){
            self.game.draw(0);
        }
        else if self.guiobjects.get( "pile1" ).unwrap().is_clicked(upos){
            self.game.draw(1);
        }
        else if self.guiobjects.get( "pile2" ).unwrap().is_clicked(upos){
            self.game.draw(2);
        }
        else if self.guiobjects.get( "pile3" ).unwrap().is_clicked(upos){
            self.game.draw(3);
        }
        else {

            let pos = Point2::new( pos.0, pos.1  );

            let ray = self.camera.unproject( &pos , &Vector2::new( self.windowsize.0 as f32, self.windowsize.1 as f32) );

            let (selected, tosend) = self.game.click(  self.selectedobject.clone() , ray );

            ConsoleService::log( &format!("{:?}", selected) );
    
            self.selectedobject = selected;
    
            if let Some(tosend) = tosend{
                self.channels.sendoutgoing.send(tosend).unwrap();
            }
        }


        if let Some(singleplayer) = &mut self.singleplayer{
            *singleplayer = 35;
        }


    }





    //update the gui scene nodes and text values
    fn update_gui(&mut self, visible: &VisibleGameState ){



        if visible.playerswithactiveturns.contains(&1){
            self.guiobjects.get_mut("player1totalticksleft").unwrap().set_texture( "green.png".to_string() );
        }
        else{
            self.guiobjects.get_mut("player1totalticksleft").unwrap().set_texture( "white.png".to_string() );
        }

        if visible.playerswithactiveturns.contains(&2){
            self.guiobjects.get_mut("player2totalticksleft").unwrap().set_texture( "green.png".to_string() );
        }
        else{
            self.guiobjects.get_mut("player2totalticksleft").unwrap().set_texture( "white.png".to_string() );
        }



        self.guiobjects.get_mut("player1totalticksleft").unwrap().set_text(  ticks_to_string( visible.player1totalticksleft )  );
        self.guiobjects.get_mut("player2totalticksleft").unwrap().set_text(  ticks_to_string( visible.player2totalticksleft )  );



        if let Some(x) = visible.player1ticksleft{
            self.guiobjects.get_mut("player1ticksleft").unwrap().set_text( ticks_to_string( x ) );
            self.guiobjects.get_mut("player1ticksleft").unwrap().set_visibility( true );
        }
        else{
            self.guiobjects.get_mut("player1ticksleft").unwrap().set_text( "".to_string() );
            self.guiobjects.get_mut("player1ticksleft").unwrap().set_visibility( false );
        }


        if let Some(x) = visible.player2ticksleft{
            self.guiobjects.get_mut("player2ticksleft").unwrap().set_text( ticks_to_string( x ) );
            self.guiobjects.get_mut("player2ticksleft").unwrap().set_visibility(true);
        }
        else{
            self.guiobjects.get_mut("player2ticksleft").unwrap().set_text( "".to_string() );
            self.guiobjects.get_mut("player2ticksleft").unwrap().set_visibility(false);
        }




        //remove the "xturnstilldraw" effect and set it 

        //the card effects
        let mut gameeffects = visible.gameeffects.clone();


        let mut turnstilldraw = None;

        for x in 0..10{

            let name = format!("{}turnsuntildraw.png", x);

            if let Some(pos) = gameeffects.iter().position(|r| r == &name){

                turnstilldraw = Some( gameeffects.remove(pos) );
            }
        }



        if visible.gameeffects.contains( &"0turnsuntildraw.png".to_string() ){

            self.guiobjects.get_mut("clickcard").unwrap().set_visibility( true );
            self.guiobjects.get_mut("clickcard").unwrap().set_texture( "clickcard.png".to_string() );

        }
        else if let Some(texture) = turnstilldraw{
            self.guiobjects.get_mut("clickcard").unwrap().set_visibility( true );
            self.guiobjects.get_mut("clickcard").unwrap().set_texture( texture );
        }
        else{

            self.guiobjects.get_mut("clickcard").unwrap().set_visibility( false );
        }

        


        
        for id in 0..10{

            let name = "effect".to_string() + &id.to_string();

            if let Some(texture) = gameeffects.pop(){

                self.guiobjects.get_mut(&name).unwrap().set_visibility( true );
                self.guiobjects.get_mut(&name).unwrap().set_texture(  texture  );

            }
            else{

                self.guiobjects.get_mut(&name).unwrap().set_visibility( false);
            }

        }



        let name = "lastcardeffect".to_string();

        if let Some(texture) = & visible.lastcardeffect{

            self.guiobjects.get_mut(&name).unwrap().set_visibility( true );
            self.guiobjects.get_mut(&name).unwrap().set_texture(  texture.clone()  );

        }
        else{

            self.guiobjects.get_mut(&name).unwrap().set_visibility( false);
        }








        
        let mut id = 0;

        for texture in &visible.piles{

            let name = "pile".to_string() + &id.to_string();
            self.guiobjects.get_mut(&name).unwrap().set_texture( texture.clone() );
            id += 1;
        }





        for (_, object) in self.guiobjects.iter_mut(){

            let texture = object.get_texture();

            let texture = "cards/".to_string() + &texture;

            if fetch_texture( &mut self.channels, &mut self.texturesfetched , &texture ){
                object.get_node().set_texture_with_name(&texture);
            }
        }




    }


    //render the text of the gui
    fn render_gui_text(&mut self, window: &mut Window){

        for (_, guiobject) in self.guiobjects.iter_mut(){
            guiobject.render_text( window );
        }

    }

}


use kiss3d::camera::Camera;
use kiss3d::camera::ArcBall;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::renderer::Renderer;
use kiss3d::post_processing::post_processing_effect::PostProcessingEffect;

impl State for InnerGame{


    fn cameras_and_effect_and_renderer(&mut self) -> (Option<&mut dyn Camera>, Option<&mut dyn PlanarCamera>, Option<&mut dyn Renderer>, Option<&mut dyn PostProcessingEffect>){

        return ( Some( &mut self.camera ), None, None, None);
    }




    fn step(&mut self, window: &mut Window) {
        
        self.render_gui_text( window );

        if let Ok(_) = self.channels.receivetick.try_recv(){

            self.tick();

            let visible = self.game.get_visible_game_state( &self.selectedobject );


            self.update_gui( &visible );


            let mut colors = Vec::new();

            for x in &visible.boardobjects{

                colors.push( x.id );
            }


            let mut objectstoremove: HashSet<u16> = self.boardobjects.keys().map(|x| *x).collect();


            for object in visible.boardobjects{

                objectstoremove.remove( &object.id );
            

                //if the node exists
                if let Some(boardobject) = self.boardobjects.get_mut(&object.id){

                    let bodynode = &mut boardobject.body;

                    bodynode.set_local_transformation( object.isometry );

                    bodynode.set_color( object.color.0, object.color.1, object.color.2 );


                    //if it has a texture
                    if let Some(texture) = object.texturelocation{

                        let texture = "pieceart/".to_string() + &texture;
                        
                        //if it has a symbol, set the texture to the symbol instead of the body
                        if let Some(symbolnode) = &mut boardobject.symbol{
                            
                            symbolnode.set_color( object.color.0, object.color.1, object.color.2 );

                            if fetch_texture(&mut self.channels, &mut self.texturesfetched, &texture){
                                symbolnode.set_texture_with_name(&texture);
                            }
                        }
                        //if this square has a texture?
                        else{

                            if fetch_texture(&mut self.channels, &mut self.texturesfetched, &texture){
                                bodynode.set_texture_with_name(&texture);
                            }
                        }                 
                    }

                }
                //only dealing with boardobjects here
                else{
                    
                    let bodynode = add_shape_to_window(&*object.shape,  window);

                    let mut boardobject = BoardObject{ body: bodynode, symbol: None  };

                    //if this is a cylinder, add a symbol child to it
                    let typedshape = object.shape.as_typed_shape();
                
                    if let TypedShape::Cylinder(cylinder) = typedshape{

                        let mut symbolnode = boardobject.body.add_cube( 0.71, 0.01, 0.71 );

                        symbolnode.set_local_translation( nalgebra::geometry::Translation::from( Vector3::new(0., cylinder.half_height, 0.) ) );

                        if self.game.get_id() == 2{
                            symbolnode.append_rotation_wrt_center( &UnitQuaternion::from_euler_angles( 0., 3.1415, 0. ) );                          
                        }

                        symbolnode.set_color(1., 2., 1.);
                        boardobject.symbol = Some (symbolnode);
                    };


                    self.boardobjects.insert( object.id , boardobject );

                }

            }


            for objectid in objectstoremove{

                let mut node = self.boardobjects.remove(&objectid).unwrap();

                window.remove_node( &mut node.body );

                if let Some(symbol) = &mut node.symbol{

                    window.remove_node( symbol );
                }
            };


        }



        if let Ok( pos ) = self.channels.receiveclicks.try_recv(){
            self.click( pos );
        }



    }

}


fn add_shape_to_window(shape: &dyn Shape, window: &mut Window) -> SceneNode{

    let typedshape = shape.as_typed_shape();

    if let TypedShape::Cuboid(cuboid) = typedshape{

        return window.add_cube( cuboid.half_extents.x*2., cuboid.half_extents.y*2., cuboid.half_extents.z*2.  );
    }
    else if let TypedShape::Cylinder(cylinder) = typedshape{

        return window.add_cylinder( cylinder.radius, cylinder.half_height*2.);
    }
    else{

        panic!("cant add shape to window");
    }
    
}











struct GameChannels{

    //the websocket channels
    sendoutgoing: Sender< Vec<u8> >,
    receiveincoming: Receiver< Vec<u8> >,

    //the users input
    receiveclicks: Receiver<(f32,f32)>,
    receivetick: Receiver<()>,

    sendtexturerequest: Sender<String>,

    receivefetchedtexture: Receiver<(String, Vec<u8>)>,
}


pub struct InterfaceChannels{

    //websocket messages
    pub receiveoutgoing: Receiver<Vec<u8>>,
    pub sendincoming: Sender<Vec<u8>>,

    pub sendclicks: Sender<(f32,f32)>,
    pub sendtick: Sender<()>,


    pub receivetexturerequest: Receiver<String>,

    //the name of the texture and its representation as bytes
    pub sendfetchedtexture: Sender<(String, Vec<u8>)>,
}

fn new_channels() -> (GameChannels, InterfaceChannels){

    let (sendoutgoing, receiveoutgoing) = channel();
    let (sendincoming, receiveincoming) = channel();        
    let (sendclicks, receiveclicks) = channel();
    let (sendtick, receivetick) = channel();
    let (sendtexturerequest, receivetexturerequest) = channel();
    let (sendfetchedtexture, receivefetchedtexture) = channel();

    let gamechannels = GameChannels{
        sendoutgoing,
        receiveincoming,
        receiveclicks,
        receivetick,
        sendtexturerequest,
        receivefetchedtexture,
    };

    let interfacechannels = InterfaceChannels{

        receiveoutgoing,
        sendincoming,
        sendclicks,
        sendtick,
        receivetexturerequest,
        sendfetchedtexture,
    };

    (gamechannels,  interfacechannels)
}




pub fn new(playerid: u8, issingleplayer: bool, windowsize: (u32,u32)) -> InterfaceChannels{


    let mut window = Window::new("Kiss3d: wasm example");

    window.set_background_color(0.7, 0.9, 0.9);

    //window.set_light(Light::StickToCamera);
    window.set_light( Light::Absolute( Point3::new(1.,5.,2.) )   );


    let (gamechannel, interfacechannel) = new_channels();

    let innergame = InnerGame::new( playerid, issingleplayer, gamechannel , &mut window , windowsize);


    window.render_loop(innergame);


    interfacechannel

}




