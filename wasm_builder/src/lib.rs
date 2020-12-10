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
use interface::AppearanceData;
use interface::objectname_to_objecttype;
use interface::objecttype_to_objectname;









use std::collections::HashSet;

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
    
    
    dragged: Option<Dragged>,
    
    
    
    //the pieces and values of each put up to offer
    //for either a need to raise, check, or settle the debt
    piecesforoffer: HashSet<u16>,
    
    
    
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
            dragged: None,
            piecesforoffer: HashSet::new(),
        }
        
    }
    
    //give this wasm struct a message from the server
    pub fn get_incoming_socket_message(&mut self, message: String){
        
        let backtovecofchar = message.chars().collect::<Vec<_>>();
        let backtogamebin = backtovecofchar.iter().map(|c| *c as u8).collect::<Vec<_>>();
        
        self.localgame.receive_game_update( backtogamebin );
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
    }
    
    
    
    
    
    //return an object with the data of what the game should look like currently
    pub fn get_appearance_data(&mut self) -> JsValue{
        
        let mut toreturn = self.localgame.get_full_appearance_state();
        
        
        //the list of objects that can be selectable with the currently selected object
        let mut highlightedobjects = Vec::new();
        
        if let Some(selectedobject) = self.selectedobject{
            highlightedobjects = self.localgame.get_this_objects_selectable_objects(selectedobject);
            toreturn.make_object_colour( objecttype_to_objectname(selectedobject), (10.0,10.0,254.0) );
        }
        
        
        //set those objects to highlighted in the struct being returned
        for highlightedobject in highlightedobjects{
            let highlightedobjectname = objecttype_to_objectname(highlightedobject);
            toreturn.make_object_colour(highlightedobjectname, (0.0,255.0,0.0));
        }
        
        
        
        /*
        //if theres a piece being dragged
        //append the cue to the appearance data being returned
        
        if let Some(dragged) = self.dragged{
            
            if let Dragged::piece(position, rotation) = dragged{
                
                
                
                
                
            }
            
        }
        let cueappearance = AppearanceData::new_cue(pos: (f32,f32,f32), rot: (f32,f32,f32))
        
        toreturn.add_object(  );
        */
        
        
        
        
        //highlight the list of objects up for offer
        for pieceid in &self.piecesforoffer{
            
            let objecttype = ObjectType::piece(*pieceid);
            let highlightedobjectname = objecttype_to_objectname(objecttype);
            
            toreturn.make_object_colour(highlightedobjectname, (0.0,255.0,0.0));
        }
        
        
        
        let vecofpieces: Vec<u16> = self.piecesforoffer.clone().into_iter().collect();
        let valueoffered = self.localgame.get_value_of_offered_pieces(vecofpieces);
        let valuetocheck = self.localgame.get_cost_to_check();
        
        if let Some(valueoffered) = valueoffered{
            if let Some(valuetocheck) = valuetocheck{
                
                toreturn.append_value_out_of_value(valueoffered, valuetocheck);
                
            }
        }
        
        
        
        //turn it into a json object and return as a jsvalue
        JsValue::from_serde( &toreturn ).unwrap()
    }
    
    
    //return whether the object passed in is the selected one or not
    pub fn is_object_selected(&self, objectname: String) -> bool{
        
        //if it can be converted from an object name to an objecttype
        if let Some(pickedobject) = objectname_to_objecttype(objectname){
            
            if let Some(selectedobject) = self.selectedobject{
                
                if selectedobject == pickedobject{
                    return true;
                }
                else{
                    return false;
                }
                
            }
            else{
                return false;
            }
        }
        else{
            
            return false;
        }
        
    }
    
    
    
    fn click_in_value_gathering_mode(&mut self, objecttype: ObjectType){
        
        
        //if its a piece select / deselect it
        if let ObjectType::piece(pieceid) = objecttype{
            
            //if this piece is already offered
            if self.piecesforoffer.contains(&pieceid){
                
                //remove it from the pieces offered
                self.piecesforoffer.remove(&pieceid);
                
            }
            //if its not
            else {
                
                //if it can be offered, add it to the pieces that can be offered
                if self.localgame.can_piece_be_offered(pieceid){
                    
                    self.piecesforoffer.insert(pieceid);
                }
            }
        }
        
        //the check fold and raise buttons should only be available to be clicked if
        //the pieces for offer add to the valid amount
        //and clear the list of pieces for offer
        
        //if its name is "check button"
        else if ObjectType::checkbutton == objecttype{
            
            let vecofpieces: Vec<u16> = self.piecesforoffer.clone().into_iter().collect();
            
            let input = self.localgame.try_to_check(vecofpieces);
            self.queuedoutgoingsocketmessages.push(input);
            
            self.piecesforoffer = HashSet::new();
        }
        //if its name is "fold button"
        else if ObjectType::foldbutton == objecttype{
            
            let input = self.localgame.try_to_fold();
            self.queuedoutgoingsocketmessages.push(input);
            
            self.piecesforoffer = HashSet::new();
        }
        //if its name is "raise button"
        else if ObjectType::raisebutton == objecttype{
            
            let vecofpieces: Vec<u16> = self.piecesforoffer.clone().into_iter().collect();
            
            let input = self.localgame.try_to_raise(vecofpieces);
            self.queuedoutgoingsocketmessages.push(input);
            
            self.piecesforoffer = HashSet::new();
        }
        
        
    }
    
    
    
    //player input functions
    
    
    //a player clicks on an object
    pub fn click_object(&mut self, objectname: String){
        
        
        
        //if it can be converted from an object name to an objecttype
        if let Some(pickedobject) = objectname_to_objecttype(objectname.clone()){
            
            
            //if there is a card game going on
            if self.localgame.is_cardgame_ongoing(){
                
                //click the pieces in the value gathering mode
                self.click_in_value_gathering_mode(pickedobject);
            }
            //if theres an object already selected
            else if let Some(currentlyselectedobject) = self.selectedobject{
                
                //get if the new object selected can form an action with the selected one
                //if it can, send that action as input to the game
                //and set selected to be none
                let possibleinput = self.localgame.try_to_perform_action(currentlyselectedobject, pickedobject);
                if let Some(input) = possibleinput{
                    self.queuedoutgoingsocketmessages.push(input);
                }
                
                
                self.selectedobject = None;
                
            }
            //if its name is "deck" create a draw action
            else if ObjectType::deck == pickedobject{
                let input = self.localgame.try_to_draw_card();
                self.queuedoutgoingsocketmessages.push(input);
            }
            //if its a card, play the card
            else if let ObjectType::card(cardid) = pickedobject{

                let input = self.localgame.try_to_play_card(cardid);
                self.queuedoutgoingsocketmessages.push(input);
                
            }
            //if the selected object is currently none
            else if self.selectedobject == None{
                
                //if the picked object is owned by me
                //or selectable by me (button / deck)
                if self.localgame.do_i_own_object(pickedobject){
                    
                    //set that object to be the selected one
                    self.selectedobject = Some( pickedobject );
                }
            }
            
            
            //if the object selected is some other object i didnt handle
            else{
                self.selectedobject = None;
            }
            
            
        }
        //if it cant be converted to an objecttype
        else{
            
            self.selectedobject = None;
        }
        
        
    }        
    
    
    
    
    
    
    //if the mouse is being dragged
    //and what object its being dragged over
    //and how far its being dragged
    //maybe i should be getting the objectname its over, and converting the name of the object its over into the board its over here hmmmm
    pub fn drag_selected_object(&mut self, relativedistancex: f32, relativedistancey: f32, objectovername: String ){
        
        
        //if an object is selected
        if let Some(selectedobject) = self.selectedobject{
            
            //if the selected object is a piece
            if let ObjectType::piece(pieceid) = selectedobject{
                
                //if the piece cant be flicked, end
                if ! self.localgame.can_piece_be_flicked(pieceid){
                    return ();
                }
                
                //get the position of the selected piece
                let selectedposition = self.localgame.get_object_flat_plane_position(selectedobject);
                
                
                let (position, rotation) = get_position_and_rotation_of_cue_indicator(selectedposition, relativedistancex, relativedistancey);
                
                
                //only make a flick mission if the mouse is further away from the piece than 1
                if let Some( (rotation, distance) ) = get_flick_force(relativedistancex, relativedistancey){
                    
                    self.dragged = Some( Dragged::piece( rotation, distance ) );
                }
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
                    let input = self.localgame.try_to_flick_piece(pieceid, flickx, flicky);
                    self.queuedoutgoingsocketmessages.push(input);
                }
                
            }
            
            //if there was an object being dragged, clear the selected object
            self.selectedobject = None;
        }
        
        
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



//return the distance from the piece
//and the rotation relative to the piece
//if its been dragged far enough to flick
fn get_flick_force(relativedistancex: f32, relativedistancey: f32) -> Option<(f32, f32)>{
    
    //the distance plus the length of half the cue
    let curtotaldistance = (relativedistancex * relativedistancex + relativedistancey * relativedistancey).sqrt();
    
    //if the distance of the que is farther or closer than it should be, change the scalar to render it within range
    let mut distancescalar = 1.0;
    
    //if the distance of the que is less than 2 units away from the piece, make it two units away
    if curtotaldistance <= 1.0{
        distancescalar = 1.0 / curtotaldistance ;
    }
    
    
    let xrotation = relativedistancex.atan2(relativedistancey);
    
    
    if curtotaldistance >= 1.0{
        
        return Some( (-xrotation - (3.14159 / 2.0), (curtotaldistance - 1.0) * 1.0) );
        
    };
    
    
    return None;
    
    
    
}



fn get_position_and_rotation_of_cue_indicator(piecepos: (f32,f32), reldistx: f32, reldisty: f32) -> ((f32,f32,f32), (f32,f32,f32)){
    
    //the distance plus the length of half the cue
    let curtotaldistance = (reldistx * reldistx + reldisty * reldisty).sqrt();
    
    //if the distance of the que is farther or closer than it should be, change the scalar to render it within range
    let mut distancescalar = 1.0;
    
    //if the distance of the que is less than 2 units away from the piece, make it two units away
    if curtotaldistance <= 1.0{
        distancescalar = 1.0 / curtotaldistance ;
    }
    
    
    //0 + the ratio of the hypotenuse length to x length * cue length
    let xcuedistance = (reldistx / curtotaldistance ) * 1.0 ;
    //0 + the ratio of the hypotenuse length to y length * cue length
    let ycuedistance = (reldisty / curtotaldistance ) * 1.0 ;
    
    
    //i want it to circle around the selected pieces position
    //facing inwards
    
    let xdistancefromselected = (reldistx * distancescalar) + xcuedistance;
    let zdistancefromselected = (reldisty * distancescalar) + ycuedistance;
    
    let xrotation = reldistx.atan2(reldisty);
    
    
    
    let position = (piecepos.0 + xdistancefromselected, 0.8, piecepos.1 + zdistancefromselected);
    let rotation = (0.0, xrotation, 0.0);
    
    
    
    return (position, rotation) ;
    
}