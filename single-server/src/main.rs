

use physicsengine::MainGame;


use physicsengine::GameToConnectTo;
use physicsengine::ConnectedToGame;


use std::sync::Arc;



use std::net::TcpListener;

use std::net::TcpStream;
use std::thread::spawn;

use tungstenite::accept_hdr;
use tungstenite::handshake::server::{Request, Response};

use tungstenite::server::accept;


use std::collections::HashMap;
use std::collections::HashSet;

use tungstenite::{connect, Message};



use url::Url;
use  std::sync::Mutex;
use std::{thread, time};



use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;




fn main() {
    println!("Hello, world!");
    
    
    let webaddress = "127.0.0.1".to_string();
    let gameport = 3012.to_string();
    
    let listener = TcpListener::bind(webaddress + ":" + &gameport).unwrap();
    
    
    //tell agones that the game is ready to accept player connections
    let thegame = Game::new();
    
    let mutexgame = Arc::new(Mutex::new( thegame ));
    
    
    
    
    //tick the game
    let mutexgamecopy = mutexgame.clone();
    spawn(move || {
        
        loop{
            
            println!("ticking");
            
            let sleeptime = time::Duration::from_millis(700);
            thread::sleep( sleeptime );
            
            
            //taking ownership of the "games" list
            //to tick the game
            {
                let mut games = mutexgamecopy.lock().unwrap();
                
                games.tick();    
            }
            
        }
        
        
    });
    
    
    
    //for each websocket stream this server gets
    for stream in listener.incoming() {
        
        
        //accept a new websocket 10 times every second
        let sleeptime = time::Duration::from_millis(100);
        thread::sleep( sleeptime );
        
        let mutexgamecopy = mutexgame.clone();
        
        
        //spawn a new thread for the connection
        spawn(move || {
            
            let stream = stream.unwrap();
            
            handle_connection(stream, mutexgamecopy);
            
            
        });
        
        
        
        
        
        
    }
    
    
    
    
    
    
    
}



//handle a connection for the game
fn handle_connection(mut stream: TcpStream, game: Arc< Mutex< Game >>){
    
    
    
    //the password needed to connect to the game as a certain player
    let player1password = "somepassword";
    let player2password = "1241354353";
    
    
    
    stream.set_nonblocking(true);
    
    let callback = |req: &Request, mut response: Response| {
        Ok(response)
    };
    
    //panic and exit the thread if its not a websocket connection
    let mut websocket = accept_hdr(stream, callback).unwrap();
    
    
    
    //wait 2000 millis
    let sleeptime = time::Duration::from_millis(2000);
    thread::sleep( sleeptime );
    
    
    
    //if theres a message
    //only read the first message, if the first message isnt used,  
    let potentialmessage = websocket.read_message();
    
    if let Ok(msg) = potentialmessage{
        
        println!("the message received: {:?}", msg);
        
        
        //if the message im receiving is a string
        if let Ok(textmsg) = msg.into_text(){
            
            
            //if that string is the password for player 1
            if textmsg == player1password{
                
                //connect to the game as player 1 and give it this websocket stream
                {
                    let mut unlockedgame = game.lock().unwrap();
                    unlockedgame.connect_player1(websocket);
                }
            }
            
            else if textmsg == player2password{
                
                //connect to the game as player 2 and gove it this websocket stream
                {
                    let mut unlockedgame = game.lock().unwrap();
                    unlockedgame.connect_player2(websocket);
                }
                
            }
            
            
            //if its not the password for either, do nothing
            //just let the websocket connection end
            
            
        }
        
        
    }
    
    
    
}





//a single game
struct Game{
    
    thegame: MainGame,
    
    //if everything about the game is valid enough for it to tick
    gameon: bool,
    
    
    player1active: bool,
    player2active: bool,
    
    
    player1websocket: Option< tungstenite::WebSocket<std::net::TcpStream> >,
    player2websocket: Option< tungstenite::WebSocket<std::net::TcpStream> >,
    
    
    
}

impl Game{
    
    fn new() -> Game{
        
        
        Game{
            
            thegame: MainGame::new_two_player(),
            
            gameon: false,
            
            player1active: false,
            player2active: false,
            
            player1websocket: None,
            
            player2websocket: None,
            
        }
        
    }
    
    
    fn connect_player1(&mut self, websocket: tungstenite::WebSocket<std::net::TcpStream> ){
        
        //if player 1 does not have their websocket connection set
        
        if self.player1websocket.is_none(){
            self.player1websocket = Some(websocket);
            
            self.player1active = true;
            
            
            
            let player1msg = Message::text("connected to game");
            self.player1websocket.as_mut().unwrap().write_message(player1msg).unwrap();
        }
        
        
        
    }
    
    
    fn connect_player2(&mut self, websocket: tungstenite::WebSocket<std::net::TcpStream>){
        
        
        //if player 2 does not have their websocket connection set
        if self.player2websocket.is_none(){
            self.player2websocket = Some(websocket);
            
            self.player2active = true;
            
            
            
            let player2msg = Message::text("connected to game");
            self.player2websocket.as_mut().unwrap().write_message(player2msg).unwrap();
            
        }
        
        
    }
    
    
    fn tick(&mut self){
        
        
        //set the game to be on if both players are active
        //and off if either player is inactive
        if self.player1active && self.player2active{
            self.gameon = true;
        }
        else{
            self.gameon = false;
        }
        
        
        //if the game state is valid to tick it
        if self.gameon{
            
            
            //tick the game
            self.thegame.tick();
            
            
            //receive player 1's queued input if there is any
            {
                
            }
            //receive player 2's queued input if there is any
            {
                
            }
            
            
            
            
            //send the states of the game through the websocket
            
            {
                let gamestatestringto1 = self.thegame.get_game_information_string(1);
                let player1msg = Message::text(gamestatestringto1);
                self.player1websocket.as_mut().unwrap().write_message(player1msg).unwrap();
                
                
                
                let gamestatestringto2 = self.thegame.get_game_information_string(2);
                let player2msg = Message::text(gamestatestringto2);
                self.player2websocket.as_mut().unwrap().write_message(player2msg).unwrap();
            }
            
        }
        
        
    }
    
    
}

