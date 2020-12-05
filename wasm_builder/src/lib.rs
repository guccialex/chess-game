//the rust project that is compiled into webassembly to be used by the javascript

use std::panic;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}



#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    
    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}



mod interface;
use interface::LocalGameInterface;
use interface::ObjectType;
use interface::FullAppearanceState;
use interface::ObjectAppearance;
use interface::objectname_to_objecttype;
use interface::objecttype_to_objectname;











#[wasm_bindgen]
//the only methods the fullgame should have:
//-set the input of the user
//-set the websocket message received from the server
//-get the state of the game
//-get the websocket message to send to the server
//-tick

pub struct FullGame{
    
    //the local client side version of the game if it exists
    localgame: LocalGameInterface,
    
    queuedoutgoingsocketmessages: Vec< String>,
    
    //the name of the object that is selected
    selectedobject: Option<ObjectType>,
    
    //the list of objects that will be added to the game appearance
    gameappearancetoappend: Vec<ObjectAppearance>,
    
    
    dragged: Option<Dragged>,
    
}



#[wasm_bindgen]
impl FullGame{
    
    pub fn new(playerid: u8) -> FullGame{
        
        //set the panic hook so i get real error reporting
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        FullGame{
            
            localgame: LocalGameInterface::new(playerid),
            queuedoutgoingsocketmessages: Vec::new(),
            selectedobject: None,
            gameappearancetoappend: Vec::new(),
            dragged: None,
        }
        
    }
    
    
    
    //give this wasm struct a message from the server
    pub fn get_incoming_socket_message(&mut self, message: String){
        
        /*
        //if it is a "gamedata" struct
        if let Ok(gamedata) = serde_json::from_str::<GameData>( &message ){
            
            //give it to the local game
            self.localgame.receive_game_state_data(gamedata);
            
        }
        */
    }
    
    
    //if there is an outgoing socket message to pop
    pub fn is_outgoing_socket_message_queued(&self) -> bool{
        
        !self.queuedoutgoingsocketmessages.is_empty()
    }
    
    pub fn pop_outgoing_socket_message(&mut self) -> String{
        
        //get and remove the first element
        self.queuedoutgoingsocketmessages.remove(0)
    }
    
    
    pub fn tick(&mut self){
        
        //tick
        self.localgame.tick();
        
        //and queue the outgoing socket messages
        //which should just be the input of the player, the player actions
        //the same ones that the local game received
        //or should this just be done on the "inputs"
    }
    
    
    
    
    
    //return an object with the data of what the game should look like currently
    pub fn get_appearance_data(&mut self) -> JsValue{
        
        let mut toreturn = self.localgame.get_full_appearance_state();
        
        
        //the list of objects that can be selectable with the currently selected object
        let mut highlightedobjects = Vec::new();
        
        if let Some(selectedobject) = self.selectedobject{
            highlightedobjects = self.localgame.get_this_objects_selectable_objects(selectedobject);
            toreturn.make_object_selected( objecttype_to_objectname(selectedobject) );
        }
        
        
        //set those objects to highlighted in the struct being returned
        for highlightedobject in highlightedobjects{
            let highlightedobjectname = objecttype_to_objectname(highlightedobject);
            toreturn.make_object_highlighted(highlightedobjectname);
        }
        
        
        //append the "gameappearancetoappend" to the appearance data being returned
        toreturn.append_object_list( self.gameappearancetoappend.clone() );
        
        
        //turn it into a json object and return as a jsvalue
        JsValue::from_serde( &toreturn ).unwrap()
        
    }
    
    
    
    
    //return whether the object passed in is the selected one or not
    pub fn is_object_selected(&self, objectname: String) -> bool{
        
        //if it can be converted from an object name to an objecttype
        if let Some(pickedobject) = objectname_to_objecttype(objectname){
            
            if let Some(selectedobject) = self.selectedobject{
                
                if (selectedobject == pickedobject){
                    return(true);
                }
                else{
                    return(false);
                }
                
            }
            else{
                return(false);
            }
        }
        else{
            
            return(false);
        }
        
    }
    
    pub fn get_selected_object_name(&self) -> Option<String>{
        
        if let Some(selectedobject) =self.selectedobject{
            return Some( objecttype_to_objectname(selectedobject) ) ;
        }
        else{
            return None;
        }
    }
    

    //player input functions
    
    
    //a player clicks on an object
    pub fn click_object(&mut self, objectname: String){
        
        //if the object name is empty
        //set "selectedobject" to None
        if objectname == ""{
            self.selectedobject = None;
        }
        //if an object been clicked
        else{
            
            //if it can be converted from an object name to an objecttype
            if let Some(pickedobject) = objectname_to_objecttype(objectname.clone()){
                
                //if the selected object is currently none
                if self.selectedobject == None{
                    
                    //if the picked object is owned by me
                    if self.localgame.do_i_own_object(pickedobject){
                        
                        //set that object to be the selected one
                        self.selectedobject = Some( pickedobject );
                    }
                }
                //if theres an object already selected
                else if let Some(currentlyselectedobject) = self.selectedobject{
                    
                    //get if the new object selected can form an action with the selected one
                    //if it can, send that action as input to the game
                    //and set selected to be none
                    self.localgame.try_to_perform_action(currentlyselectedobject, pickedobject);

                    self.selectedobject = None;
                }
            }
            //if its name is "deck" create a draw action
            else if objectname == "deck"{

                self.localgame.try_to_draw_card();
            }
            else{

                self.selectedobject = None;
            }

        }        
    }
    
    //if the mouse is being dragged
    //and what object its being dragged over
    //and how far its being dragged
    //maybe i should be getting the objectname its over, and converting the name of the object its over into the board its over here hmmmm
    pub fn drag_selected_object(&mut self, relativedistancex: f32, relativedistancey: f32, objectovername: String ){
        
        //get the board the cursor is over by what object the object being hovered over is on
        let boardover = self.localgame.objectname_to_board(objectovername);
        
        //if an object is selected
        if let Some(selectedobject) = self.selectedobject{
            
            //if the selected object is a piece
            if let ObjectType::piece(pieceid) = selectedobject{
                
                //get the position of the selected piece
                let selectedposition = self.localgame.get_object_flat_plane_position(selectedobject);
                
                //the distance plus the length of half the cue
                let mut curtotaldistance = (relativedistancex * relativedistancex + relativedistancey * relativedistancey).sqrt();
                
                //if the distance of the que is farther or closer than it should be, change the scalar to render it within range
                let mut distancescalar = 1.0;
                
                //if the distance of the que is less than 2 units away from the piece, make it two units away
                if curtotaldistance <= 1.0{
                    distancescalar = 1.0 / curtotaldistance ;
                }
                
                //their direction should be that of the rotation
                //their distance should be that of half the pool cue
                
                //0 + the ratio of the hypotenuse length to x length * cue length
                let xcuedistance = (relativedistancex / curtotaldistance ) * 1.0 ;
                //0 + the ratio of the hypotenuse length to y length * cue length
                let ycuedistance = (relativedistancey / curtotaldistance ) * 1.0 ;
                
                
                //i want it to circle around the selected pieces position
                //facing inwards
                
                let xdistancefromselected = (relativedistancex * distancescalar) + xcuedistance;
                let zdistancefromselected = (relativedistancey * distancescalar) + ycuedistance;
                
                let xrotation = relativedistancex.atan2(relativedistancey);
                


                let position = (selectedposition.0 + xdistancefromselected, 0.8, selectedposition.1 + zdistancefromselected);
                let rotation = (0.0, xrotation, 0.0);
                
                let dragindicatorappearance = ObjectAppearance::new_cue(position, rotation);
                self.gameappearancetoappend.push( dragindicatorappearance );
                
                
                
                //only make a flick mission if the mouse is further away from the piece than 1
                if curtotaldistance >= 1.0{
                    
                    //and the force is proportional to the distance the cue is pulled back
                    //not the distance the mouse is from the piece center
                    //ergo, the minus 1
                    self.dragged = Some( Dragged::piece(-xrotation - (3.14159 / 2.0), (curtotaldistance - 1.0) * 1.0) );

                }
                
            }
            
            //if the object is a card (in the hand)
            else if let ObjectType::card(cardid) = selectedobject{
                self.dragged = Some ( Dragged::card( (relativedistancex, relativedistancex), boardover) );
            }
        }
    }
    
    
    //the mouse is raised
    pub fn mouse_up(&mut self){
        
        //if there is an object being dragged
        if let Some( dragged ) = self.dragged.clone() {
            
            
            
            //if its a piece being dragged
            if let Dragged::piece( flickx, flicky ) = dragged{
                
                //if there is an object selected and it (what is being dragged) is a piece
                if let Some(ObjectType::piece(pieceid)) = self.selectedobject{
                    
                    //try to flick that piece
                    self.localgame.try_to_flick_piece(pieceid, flickx, flicky);
                    
                }
                
            }
            
            //if its a card being dragged
            else if let Dragged::card( relativepos, boardover ) = dragged{
                
                //if there is an object selected and it (what is being dragged) is a card                
                if let Some(ObjectType::card(cardid)) = self.selectedobject{
                    
                    //if its over 0, dont do anything
                    if boardover == 0{
                    }
                    //if its over 1, send a mission to play the card over the game board
                    else if boardover == 1{
                        
                        self.localgame.try_to_play_card(cardid);
                        
                    }
                    //if its over 2, send a mission to play the card over the card board
                    else if boardover == 2{
                        
                        self.localgame.try_to_play_card(cardid);
                        
                    }
                    else{
                        panic!("Why does boardover have a value other than 1,2,3 ?");
                    }
                    
                }

            }
            
            
            
            //if there was an object being dragged, clear the selected object
            self.selectedobject = None;
            
        }
        
        
        //clear the drag indicator
        self.gameappearancetoappend = Vec::new();
        
        //clear the object being dragged
        self.dragged = None;
        
    }
    
    
}






//if something is dragged
#[derive(Clone)]
enum Dragged{
    
    //if its a piece
    piece(f32,f32),
    
    //or a card being dragged
    card( (f32,f32), u32),
    
}