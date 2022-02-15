use yew::prelude::*;

//use yew::services::console::ConsoleService;
//use yew::format::{Nothing};
//use yew::services::interval::{IntervalService, IntervalTask};
//use std::time::Duration;
//use yew::services::fetch::{FetchService, FetchTask, Request, Response};
//use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};


mod game;
use game::InterfaceChannels;


use gloo_console::log;






pub fn numbertoaddress(number: u32) -> String{

    // /api/game/{id}
    // /api/game/{id}/get_players
    // ws://{  }/api/game/{id}/ws/

    let _base = "ws://bbbbeeee.com/".to_string();
    let _extension = "api/game/".to_string() + &number.to_string();
    let _socket = "/ws/";


    let toreturn = format!( "ws://bbbbeeee.com/api/game/{}/ws/", number ) ;

    return toreturn;

}



pub enum Msg{

    Tick,

    MouseDown(f32,f32),
    MouseUp(f32,f32),

    //to create the game
    Start(u8),

    //when I read a texture from file
    FetchedTexture( String, Vec<u8> ),

    WSResponse( Vec<u8> ),

    Error,
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    
    pub multiplayer: Option<u32>,
}


pub struct ChessCheckers {

    fetch_tasks: Vec<FetchTask>,

    //the websocket connection to the server
    ws: Option<WebSocketTask>,



    link: ComponentLink<Self>,
    _task: IntervalTask,

    windowsize: (u32,u32),

    channels: Option<InterfaceChannels>,

}


impl Component for ChessCheckers {
    type Message = Msg;
    type Properties = Props;


    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let window = yew::web_sys::window().expect("no global `window` exists");


        let windowsize = (window.inner_width().unwrap().as_f64().unwrap() as u32  ,window.inner_height().unwrap().as_f64().unwrap() as u32);

        let callback = link.callback(|_| Msg::Tick);
        let task = IntervalService::spawn(Duration::from_millis(33), callback);



        let mut ws = None;


        if let Some(id) = props.multiplayer{

            let address = numbertoaddress( id );


            if let Ok(task) = WebSocketService::connect_binary(
                &address,
                link.callback(|x: Result< Vec<u8>, anyhow::Error > | {

                    if let Ok(s) = x{
                        Msg::WSResponse(s) 
                    }
                    else{
                        Msg::Error
                    }
                
                } ),
                link.callback(|_x: WebSocketStatus| Msg::Error )
            )
            {

                ConsoleService::log(" connected ");

                ws = Some( task );
            }

        }





        Self{

            link,
            ws,
            _task: task,
            windowsize,
            channels: None,
            fetch_tasks: Vec::new(),
        }

    }


    fn update(&mut self, message: Self::Message) -> ShouldRender {

        match message{

            Msg::Tick =>{


                if let Some(channels) = &mut self.channels{
                    
                    channels.sendtick.send(()).unwrap(); 

                    if let Ok(name) = channels.receivetexturerequest.try_recv(){

                        
                        let texture = "/static/chessgame/".to_string()+&name;                        

                        let request = Request::get(texture).body(Nothing).expect("Could not build request.");

                        // 2. construct a callback
                        let callback = self.link.callback(move |response: Response<Result<Vec<u8>, anyhow::Error>>| {

                            let response = response.into_body();

                            if let Ok( stringdata) = response{

                                //let image = image::load_from_memory(&stringdata);
                                return Msg::FetchedTexture( name.clone(), stringdata );
                            }
                            return Msg::Error;
                        });

                        // 3. pass the request and callback to the fetch service
                        let task = FetchService::fetch_binary(request, callback).expect("failed to start request");
                        // 4. store the task so it isn't canceled immediately
                        self.fetch_tasks.push(task);

                        //ConsoleService::log(&texture);
                    };

                    
                    if let Ok(binary) = channels.receiveoutgoing.try_recv(){

                        ConsoleService::log("sending binary output");


                        if let Some(ws) = &mut self.ws{
                            ws.send_binary( Ok( binary ) );
                        }
                    };
                    
                };
            },
            Msg::MouseDown(_x,_y) =>{

            },
            Msg::MouseUp(x,y) =>{
                
                if let Some(channels) = &mut self.channels{
                    channels.sendclicks.send( (x,y) ).unwrap();
                }
            },
            Msg::Start(playerid) =>{

                if self.channels.is_none(){

                    //if there isnt a websocket, its single player, not otherwise
                    let issingleplayer = self.ws.is_none();


                    self.channels = Some( game::new(playerid, issingleplayer, self.windowsize) );
                }

            },
            Msg::FetchedTexture( name, texture) =>{

                if let Some(channels) = &mut self.channels{
                    channels.sendfetchedtexture.send( (name, texture) ).expect("texture couldnt send");
                };
            },
            Msg::WSResponse(binary) =>{


                if binary.len() == 1{

                    if binary[0] == 0{
                        self.link.send_message( Msg::Start(1) );
                    }
                    else{
                        self.link.send_message( Msg::Start(2) );
                    }

                }
                else if let Some(channels) = &mut self.channels{
                    channels.sendincoming.send( binary ).expect("cant send binary message");
                };


            }
            Msg::Error =>{  },
        }

        false
    }
    
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        ConsoleService::log("RENDERED. SHOULDNT HAPPEN MORE THAN ONCE");

        use yew::MouseEvent;

        

        



        let down = self.link.callback( |x: MouseEvent|{
            Msg::MouseDown(  x.client_x() as f32 , x.client_y() as f32)
        });

        let up = self.link.callback( |x: MouseEvent|{

            Msg::MouseUp(  x.client_x() as f32 , x.client_y() as f32)
        });


        //if the game isnt multiplayer start it if it hadnt started now
        if self.ws.is_none(){

            self.link.send_message( Msg::Start(2) );
        }




        let style = format!(
            " position: absolute;
            left: 0px;
            top: 0px;
            width: {}px;
            height: {}px; ",
            self.windowsize.0,
            self.windowsize.1
        );

        let whitebg = format!(
            "
            background-color: yellow;
            position: absolute;
            left: 0px;
            top: 0px;
            width: {}px;
            height: {}px;
            ",
            self.windowsize.0,
            self.windowsize.1
        );


        html! {

            <>
                <div style=whitebg>
                //{"sorry. Something went wrong"}
                </div>
                <canvas id="canvas" onmousedown=down onmouseup=up style=style>
                </canvas>
            </>
            
        }
    }
}
