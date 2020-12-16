use physicsengine::PlayerInput;
use physicsengine::Card;

use std::collections::HashMap;

use physicsengine::MainGame;

use physicsengine::PieceAction;
use physicsengine::BlackJackAction;
use physicsengine::PokerAction;
use physicsengine::CardAction;

use std::collections::HashSet;



//the interface the "fullgame" has with the rust chesscheckers game
pub struct LocalGameInterface{
    
    
    //the id of the player
    playerid: u8,
    
    //the actual rust game
    thegame: MainGame,
    
    
    
    
}


//What methods do I want the game to interface with?
impl LocalGameInterface{
    
    
    //create a game with a certain ID
    pub fn new(playerid: u8) -> LocalGameInterface{
        
        
        let thegame = MainGame::new_two_player();
        
        LocalGameInterface{
            
            playerid: playerid,
            thegame:thegame,
            
        }
    }
    
    
    //tick the local game
    pub fn tick(&mut self) {
        
        self.thegame.tick();
        
    }
    
    
    
    pub fn receive_game_update(&mut self, string: Vec<u8>){
        
        if let Ok(newgame) = bincode::deserialize::<MainGame>(&string){
            
            self.thegame = newgame;
        }
        else{
            panic!("didnt work");
        }
        
        
    }
    
    pub fn get_value_of_offered_pieces(&self, pieces: Vec<u16>) -> Option<u8>{
        
        self.thegame.get_value_of_offered_pieces(self.playerid, pieces)
    }
    
    pub fn get_cost_to_check(&self) -> Option<u8>{
        
        self.thegame.get_cost_to_check(&self.playerid)
    }
    
    
    
    //returns true if i am the owner of this object
    //OR if its an object which im allowed to select, like raise, check, deck
    //false otherwise
    pub fn do_i_own_object(&self, object: ObjectType) -> bool{
        
        
        if self.does_object_still_exist(object){
            
            if let ObjectType::card(cardid) = object{
                
                if let Some(ownerid)= self.thegame.get_card_owner(cardid){
                    if ownerid  == self.playerid{
                        return true;
                    }
                }
            }
            else if let ObjectType::piece(pieceid) = object{
                
                if self.playerid == self.thegame.get_board_game_object_owner(pieceid){
                    return true;
                }
            }
            else if let ObjectType::deck = object{
                return true;
            }
            else if let ObjectType::foldbutton = object{
                return true;
            }
            else if let ObjectType::raisebutton = object{
                return true;
            }
            else if let ObjectType::checkbutton = object{
                return true;
            }
            
        }
        
        
        
        return false;
        
    }
    
    
    //if this piece can be proposed to be offered by this player
    pub fn can_piece_be_offered(&self, pieceid: u16) -> bool{
        
        self.thegame.can_piece_be_offered(self.playerid, pieceid)
    }
    
    
    //if theres a cardgame going on
    pub fn is_cardgame_ongoing(&mut self) -> bool{
        
        self.thegame.is_pokergame_ongoing()
    }
    
    
    //gets a map of every valid player input for this given object
    //mapped by the id of the object that needs to be clicked on for it to be performed
    fn get_inputs_of_object(&self, objectid: ObjectType) -> HashMap< ObjectType, PlayerInput >{
        
        let mut toreturn = HashMap::new();
        
        
        //if the object is a piece
        if let ObjectType::piece(pieceid) = objectid{
            
            //get the actions allowed by the piece
            let actionsandobjects = self.thegame.get_actions_allowed_by_piece(pieceid);
            
            //for every action allowed, get the objectid of the board square and the piece id associated it can capture
            for (action, objectids) in actionsandobjects.1{
                
                let input = PlayerInput::pieceaction(pieceid, action);
                
                //for every object id
                for objectid in objectids{
                    
                    let objecttype;
                    
                    //if the object is a piece
                    if self.thegame.is_board_game_object_piece(objectid){
                        
                        objecttype = ObjectType::piece(objectid);
                    }
                    else if self.thegame.is_board_game_object_square(objectid){
                        
                        objecttype = ObjectType::boardsquare(objectid);
                    }
                    else{
                        panic!("apparently its neither boardsquare or piece");
                    }
                    
                    toreturn.insert( objecttype, input.clone() );
                }
                
            }
            
        }
        //if the object is a card
        else if let ObjectType::card(cardid) = objectid{
            
            //get the pieces and squares actable by the card
            let idtoinput = self.thegame.get_boardobject_actions_allowed_by_card(self.playerid, cardid);
            
            
            for (id, input) in idtoinput{
                
                if self.thegame.is_board_game_object_piece(id){
                    toreturn.insert( ObjectType::piece(id), input );
                }
                else if self.thegame.is_board_game_object_square(id){
                    toreturn.insert( ObjectType::boardsquare(id), input );
                }
                
            }
            
            
        }
        //if the object is a board square
        else if let ObjectType::boardsquare(id) = objectid{
            
            //dont do anything to fill the list to return
            //because no actions can be performed by a board square
            
        }
        
        
        toreturn
    }
    
    pub fn get_this_objects_selectable_objects(&self, objectid: ObjectType) -> Vec<ObjectType>{
        
        let objecttoinput = self.get_inputs_of_object(objectid);
        
        let mut toreturn = Vec::new();
        
        for (objectid, input) in objecttoinput{
            toreturn.push(objectid);
        };
        
        toreturn
        
    }
    
    
    //returns if this piece can be flicked or not
    pub fn can_piece_be_flicked(&self, pieceid: u16) -> bool{
        
        //if i own this piece
        //and its not a boardgame active
        if self.do_i_own_object( ObjectType::piece(pieceid) ){
            
            return self.thegame.get_actions_allowed_by_piece(pieceid).0;
        }
        
        return false;
    }
    
    
    //given the id of an main object, and then an object that its trying to perform an action on
    //return if an input was sent to the game, and if it was, what the serialized string of it is
    pub fn try_to_perform_action(&mut self, object1: ObjectType, object2: ObjectType) -> Option<String>{
        
        
        //if object 1 and 2 are the same card, play that card
        if let ObjectType::card(cardid1) = object1{
            
            if let ObjectType::card(cardid2) = object2{
                
                if cardid1 == cardid2{
                    
                    //if onl
                    
                    return Some( self.try_to_play_card(cardid1) );
                }
            }
        }
        
        
        
        let objecttoinput = self.get_inputs_of_object(object1);
        
        //if there is a player input that lets object1 perform some action on object 2
        if let Some(playerinput) = objecttoinput.get(&object2){
            
            //send that input to the game and return true
            self.thegame.receive_input( self.playerid, playerinput.clone());
            
            return Some( serde_json::to_string(playerinput).unwrap() );
            
        };
        
        
        
        
        //otherwise do nothing and return false
        return None;
        
    }
    
    
    pub fn try_to_flick_piece(&mut self, pieceid: u16, direction: f32, force: f32 ) -> String{
        
        let flickaction = PieceAction::flick(direction, force.sqrt() * 3.0);
        
        let flickinput = PlayerInput::pieceaction(pieceid, flickaction);
        
        //give the flick input to the game
        self.thegame.receive_input(self.playerid, flickinput.clone());
        
        
        return serde_json::to_string(&flickinput).unwrap() ;
        
    }
    
    pub fn try_to_play_card(&mut self, cardid: u16) -> String{
        
        
        let action = CardAction::playcardonboard;
        let input = PlayerInput::cardaction(cardid, action);
        
        self.thegame.receive_input( self.playerid, input.clone());
        
        
        return serde_json::to_string( &input).unwrap() ;
        
    }
    
    pub fn try_to_draw_card(&mut self) -> String{
        
        let input = PlayerInput::drawcard;
        
        self.thegame.receive_input(self.playerid, input.clone());
        
        
        return serde_json::to_string( &input).unwrap() ;
        
    }
    
    
    pub fn try_to_check(&mut self, pieces: Vec<u16>) -> String{
        
        let action = PokerAction::check(pieces);
        let input = PlayerInput::pokeraction(action);
        
        self.thegame.receive_input(self.playerid, input.clone());
        
        
        return serde_json::to_string( &input).unwrap() ;
    }
    
    pub fn try_to_raise(&mut self, pieces: Vec<u16>) -> String{
        
        let action = PokerAction::raise(pieces);
        let input = PlayerInput::pokeraction(action);
        
        self.thegame.receive_input(self.playerid, input.clone());
        
        
        return serde_json::to_string( &input).unwrap() ;
    }
    pub fn try_to_fold(&mut self) -> String{
        
        let action = PokerAction::fold;
        let input = PlayerInput::pokeraction(action);
        
        self.thegame.receive_input(self.playerid, input.clone());
        
        
        return serde_json::to_string( &input).unwrap() ;
    }
    pub fn try_to_settle_debt(&mut self, pieces: Vec<u16>) -> String{
        
        let input = PlayerInput::settledebt(pieces);
        
        self.thegame.receive_input(self.playerid, input.clone());
        
        
        return serde_json::to_string( &input).unwrap() ;
    }
    
    
    
    
    
    
    
    //get the appearance of this object
    fn get_object_appearance(&mut self, objectid: ObjectType) -> AppearanceData{
        
        //if its a card
        if let ObjectType::card(cardid) = objectid{
            
            let card = self.thegame.get_card_by_id(cardid);
            
            let (field, cardposition, fieldsize) = self.thegame.where_is_card(cardid);
            
            let objectname = objecttype_to_objectname(objectid);
            
            
            let mut xpos = cardposition as f32 * 2.0;
            let ypos = 0.0;
            let zpos;
            
            let xrot = 0.0;
            let yrot = 0.0;
            let zrot = 0.0;
            
            
            if field == 1{
                zpos = -6.0;
            }
            else if field == 2{
                zpos = 6.0;
            }
            else if field == 3{
                zpos = -3.0;
                xpos += 5.5;
            }
            else if field == 4{
                zpos = 3.0;
                xpos += 5.5;
            }
            else{
                zpos = 0.0;
                xpos += 5.5;
            }
            
            
            
            let toreturn = AppearanceData::new_card( objectname, (xpos, ypos, zpos), (xrot, yrot, zrot), card );
            
            
            return toreturn ;
            
        }
        else if let ObjectType::piece(pieceid) = objectid{
            
            let position = self.thegame.get_board_game_object_translation( pieceid );
            let rotation = self.thegame.get_board_game_object_rotation( pieceid );
            let objectname = objecttype_to_objectname(objectid);
            
            let ownerid = self.thegame.get_board_game_object_owner(pieceid);
            
            let typename = self.thegame.get_piece_type_name(pieceid);
            
            //if this is a new mesh
            
            let toreturn = AppearanceData::new_piece( objectname, typename, position, rotation, ownerid );
            
            
            return toreturn;
        }
        else if let ObjectType::boardsquare(bsid) = objectid{
            
            let position = self.thegame.get_board_game_object_translation( bsid );
            let rotation = self.thegame.get_board_game_object_rotation( bsid );
            let objectname = objecttype_to_objectname(objectid);
            
            let issquarewhite = self.thegame.is_boardsquare_white(bsid);
            
            let toreturn = AppearanceData::new_boardsquare( objectname, position, rotation, issquarewhite );
            
            
            return toreturn;
        }
        else{
            panic!("why isnt the object id matching with an object of any of these types?");
        };
    }
    
    
    //get a list of each object in the game by id (objecttype)
    //every piece, board square, and card
    fn get_objects(&self) -> Vec<ObjectType>{
        
        let boardobjectids = self.thegame.get_board_game_object_ids();
        let cardobjectids = self.thegame.get_card_ids();
        
        let mut toreturn = Vec::new();
        
        
        for boardobjectid in boardobjectids{
            
            //get if this is a card or a boardsquare
            if self.thegame.is_board_game_object_piece(boardobjectid){
                let objectid = ObjectType::piece(boardobjectid);
                
                toreturn.push(objectid);
            }
            else if self.thegame.is_board_game_object_square(boardobjectid){
                let objectid = ObjectType::boardsquare(boardobjectid);
                
                toreturn.push(objectid);
            };
            
            
        };
        
        for cardobjectid in cardobjectids{
            let objectid = ObjectType::card(cardobjectid);
            
            toreturn.push(objectid);
        };
        
        
        
        toreturn
    }
    
    
    //get an objects flat position on the plane
    pub fn get_object_flat_plane_position(&self, objectid: ObjectType) -> (f32,f32){
        
        if let ObjectType::piece(objectid) = objectid{
            
            //get its position
            let (xpos, ypos, zpos) = self.thegame.get_board_game_object_translation(objectid);
            
            return  (xpos,zpos ) ;
            
            
        }
        
        (0.0,0.0)
    }
    
    
    fn get_appearance_id_of_card(card: &Card) -> u32{
        
        //giving a card of every suit and value a unique ID
        let toreturn =  4 * (card.numbervalue() -1) + card.suitvalue()  + 1;
        
        toreturn as u32
    }
    
    
    //get the name of this cards texture
    fn get_name_of_cards_texture(card: &Card) -> String{
        
        let cardappearanceid = LocalGameInterface::get_appearance_id_of_card(card);
        let cardappearancestring = format!("{:03}", cardappearanceid );
        "cardart/card_".to_string() + &cardappearancestring + ".jpg"
        
    }
    
    
    
    //returns whether this object exists in the game
    fn does_object_still_exist(&self, object: ObjectType) -> bool{
        
        if let ObjectType::piece(pieceid) = object{
            if self.thegame.get_board_game_object_ids().contains(&pieceid){
                return true;
            }
            else{
                return false;
            }
        }
        else if let ObjectType::card(cardid) = object{
            if self.thegame.get_card_ids().contains(&cardid){
                return true;
            }
            else{
                return false;
            }
        }
        else{
            return true ;
        };
    }
    
    
    pub fn get_full_appearance_state(&mut self) -> FullAppearanceState{
        
        let mut toreturn = FullAppearanceState::new();
        
        
        //get the piece ids
        //get the board square ids
        //get the card ids of the cards in the players main hands
        let objectids = self.get_objects();
        
        //get the object appearance of these objects
        let mut objectsappearance: Vec<AppearanceData> = Vec::new();
        
        for objectid in objectids{
            let objectappearance = self.get_object_appearance(objectid);
            objectsappearance.push ( objectappearance );
        };
        
        
        //for every object in the game add it to the full appearance to return
        for objectappearance in objectsappearance{
            toreturn.add_object(objectappearance);
        };        
        
        
        
        
        let deckappearance = AppearanceData::new_deck();
        toreturn.add_object(deckappearance);
        
        //add the appearance of the timer for the player and the opponent
        let player1totaltimeleft = self.thegame.get_players_total_ticks_left(1);
        let iscurrentlyturn = (self.thegame.get_players_turn_ticks_left(1) > 0);
        let player1timer = AppearanceData::new_timer(1, player1totaltimeleft, iscurrentlyturn);
        toreturn.add_object(player1timer);
        
        
        let player2totaltimeleft = self.thegame.get_players_total_ticks_left(2);
        let iscurrentlyturn = (self.thegame.get_players_turn_ticks_left(2) > 0);
        let player2timer = AppearanceData::new_timer(2, player2totaltimeleft, iscurrentlyturn);
        toreturn.add_object(player2timer);

        
        
        let debtowed = self.thegame.get_debt_of_player(&self.playerid);
        
        if debtowed != 0{
            toreturn.add_object( AppearanceData::new_debt_owed_button(debtowed) );
            
        }
        else{


            //if theres a poker game going on
            //give the check, fold and raise buttons
            if self.thegame.is_pokergame_ongoing() {
                
                let checkbutton = AppearanceData::new_check_button();
                toreturn.add_object(checkbutton);
                
                let foldbutton = AppearanceData::new_fold_button();
                toreturn.add_object(foldbutton);
                
                let raisebutton = AppearanceData::new_raise_button();
                toreturn.add_object(raisebutton);    

                let costtocheck = AppearanceData::new_cost_to_check(debtowed);
                toreturn.add_object(costtocheck);
            }



        }
        
        
        
        
        
        toreturn
    }
    
}






use serde::{Serialize, Deserialize};



impl AppearanceData{
    
    pub fn new_cue(pos: (f32,f32,f32), rot: (f32,f32,f32)) -> AppearanceData{
        
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions:  (0.2, 0.2, 1.2),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: pos,
            
            rotation: rot,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "dragindicator".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_deck() -> AppearanceData{
        
        let imagename = "cardart/cardback.jpg".to_string();
        
        
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: Some(imagename),
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions: (0.6, 1.96, 1.4),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (-7.0,0.0,0.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "deck".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_timer(playerid: u32, ticksleft: u32, currentlyturn: bool) -> AppearanceData{
        
        
        //the time left should be as minutes then seconds
        let seconds = ticksleft / 30;
        
        let minutestext = (seconds / 60).to_string();
        let secondstext = format!("{:02}", seconds % 60);
        
        
        let timeleft = minutestext + ":" + &secondstext;
        
        
        let position;
        let name;
        
        if playerid == 1{
            position = (-7.0,0.0,-3.0);
            name = "player".to_string() + &playerid.to_string() + "timer";
        }
        else if playerid == 2{
            position = (-7.0,0.0,3.0);
            name = "player".to_string() + &playerid.to_string() + "timer";
        }
        else{
            panic!("ahhh");
        }
        
        
        let colour;
        
        if currentlyturn{
            colour = (0,255,0);
        }
        else{
            colour = (255,255,255);
        }
        
        
        let text = Text{
            fontsize: 30,
            position: (0.0,30.0),
            text: timeleft,
        };
        
        let texture = Texture{
            
            colour: colour,
            image: None,
            text: Some(text),
        };
        
        
        let shape = CubeShape{
            dimensions: (0.01, 2.0, 2.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: name,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_piece(objectname: String, typename: String ,position: (f32,f32,f32), rotation: (f32,f32,f32), ownerid: u8) -> AppearanceData{
        
        let texturename;
        let colour;
        
        if ownerid == 1{
            
            colour = (255,255,255);
            texturename = "pieceart/".to_string() + &typename + &".png";
            
        }
        else if ownerid == 2{
            
            colour = (255,255,255);
            texturename = "pieceart/b_".to_string() + &typename + &".png";
            
        }
        else{
            
            colour = (255,5,255);
            texturename = "pieceart/".to_string() + &typename + &".png";
            
        }
        
        let shapetype;
        
        if typename == "poolball"{
            
            let shape = CircleShape{
                diameter: 0.7,
            };
            
            shapetype = ShapeType::Circle(shape);
            
        }
        else{
            
            let shape = CylinderShape{
                dimensions: (0.5, 0.7),
            };
            
            shapetype = ShapeType::Cylinder(shape);
            
        }
        
        
        
        let texture = Texture{
            colour: colour,
            image: Some(texturename),
            text: None,
        };
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: rotation,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: objectname,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
        
    }
    
    pub fn new_card(name: String, position: (f32,f32,f32), mut rotation: (f32,f32,f32), card: Card ) -> AppearanceData{
        
        let texturename = LocalGameInterface::get_name_of_cards_texture(&card);
        
        
        rotation.1 += 3.14159 / 2.0;
        
        
        let texture = Texture{
            colour: (200,200,200),
            image: Some(texturename),
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions: (0.1, 1.96, 1.4),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: rotation,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: name,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    
    pub fn new_boardsquare(name: String, position: (f32,f32,f32), rotation: (f32,f32,f32), white: bool ) -> AppearanceData{
        
        
        let colour;
        
        if white{
            colour = (255,255,255);
        }
        else{
            colour = (0,0,0);
        }
        
        
        let texture = Texture{
            colour: colour,
            image: None,
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions: (1.0, 1.0, 1.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: rotation,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: name,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    
    pub fn new_check_button() -> AppearanceData{
        
        let text = format!("check");
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.1, 1.5),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (5.5,0.0,-6.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "check button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_fold_button() -> AppearanceData{
        
        let text = format!("fold");
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.1, 1.5),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (7.5,0.0,-6.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "fold button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_raise_button() -> AppearanceData{
        
        let text = format!("raise");
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.1, 1.5),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (9.5,0.0,-6.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "raise button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_piece_value_offered(valuex: u8) -> AppearanceData{
        
        
        let text = format!("{} selected", valuex);
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.01, 2.0),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (-9.0,0.0,0.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "piece value".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_debt_owed_button(debt: u8) -> AppearanceData{
        
        let text = format!("PAY ANTE OF {}", debt);
        
        let text = Text{
            fontsize: 10,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CubeShape{
            dimensions: (0.01, 3.0, 3.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (-6.0,1.0,0.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "debt button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }

    pub fn new_cost_to_check(costtocheck: u8) -> AppearanceData{

        let text = format!("check {}", costtocheck);
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CubeShape{
            dimensions: (0.0, 2.0, 2.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (12.0, 0.0, -6.0),
            
            rotation: (0.0,0.0,0.0),
        };
        
        let appearancedata = AppearanceData{
            
            name: "cost to check".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata



    }
    
}








//a struct representing the entire state of a games physical appearance
#[derive(Serialize, Deserialize, Clone)]
pub struct FullAppearanceState{
    
    //the list of every object and its appearance
    objects: Vec<AppearanceData>,
    
}

impl FullAppearanceState{
    
    fn new() -> FullAppearanceState{
        FullAppearanceState{
            objects: Vec::new(),
        }   
    }
    
    fn add_object(&mut self, objectappearance: AppearanceData){
        
        self.objects.push(objectappearance);
    }
    
    
    pub fn make_object_colour(&mut self, objectname: String, colour: (f32,f32,f32)){
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.name == objectname{
                
                let unmixedcolourfloat = colour;
                let colourfloat = (curobject.texture.colour.0 as f32, curobject.texture.colour.1 as f32, curobject.texture.colour.2 as f32);
                
                let mixedr = unmixedcolourfloat.0 * 0.8 + colourfloat.0 * 0.2;
                let mixedg = unmixedcolourfloat.1 * 0.8 + colourfloat.1 * 0.2;
                let mixedb = unmixedcolourfloat.2 * 0.8 + colourfloat.2 * 0.2;
                
                //make its colour closer to green
                curobject.texture.colour = (mixedr as u8, mixedg as u8, mixedb as u8);
                
            }
        }
    }
    
    
    //add an object to display that displays X / X
    pub fn append_value_selected(&mut self, valuex: u8){
        
        self.add_object( AppearanceData::new_piece_value_offered(valuex) );
    }
    
    
}






#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum ObjectType{
    
    card(u16),
    boardsquare(u16),
    piece(u16),
    
    deck,
    foldbutton,
    raisebutton,
    checkbutton,
    debtbutton,
    
}








//turn an object name into an object type and its ID
pub fn objectname_to_objecttype(objectname: String) -> Option<ObjectType> {
    
    
    if objectname == "deck"{
        return Some( ObjectType::deck  );
    }
    else if objectname == "raise button"{
        return Some( ObjectType::raisebutton );
    }
    else if objectname == "fold button"{
        return Some( ObjectType::foldbutton );
    }
    else if objectname == "check button"{
        
        return Some( ObjectType::checkbutton );
    }
    else if objectname == "debt button"{
        
        return Some( ObjectType::debtbutton );
    }
    //if the first character of the objects name is "P"
    else if objectname.chars().nth(0).unwrap() == 'P'{
        
        //get the rest of the name and try to convert it to an int
        let stringpieceid = objectname[1..].to_string();
        let intpieceid = stringpieceid.parse::<u16>().unwrap();
        let toreturn = ObjectType::piece(intpieceid);
        
        return Some (toreturn);
        
    }
    //if the first character of the objects name is "C"
    else if objectname.chars().nth(0).unwrap() == 'C'{
        
        //get the rest of the name and try to convert it to an int
        let stringcardid = objectname[1..].to_string();
        let intcardid = stringcardid.parse::<u16>().unwrap();
        let toreturn = ObjectType::card(intcardid);
        
        return Some (toreturn);
        
    }
    //if the first character of the objects name is "B"
    else if objectname.chars().nth(0).unwrap() == 'B'{
        
        //get the rest of the name and try to convert it to an int
        let bsid = objectname[1..].to_string();
        let intbsid = bsid.parse::<u16>().unwrap();
        let toreturn = ObjectType::boardsquare(intbsid);
        
        return Some (toreturn);
    }
    else{
        
        return None;
    }
    
}


//turn an object type into its object name
pub fn objecttype_to_objectname(inputobjecttype: ObjectType) -> String {
    
    if let ObjectType::piece(pieceid) = inputobjecttype{
        
        let toreturn = "P".to_string() + &pieceid.to_string();
        return toreturn ;
    }
    else if let ObjectType::boardsquare(boardsquareid) = inputobjecttype{
        
        let toreturn = "B".to_string() + &boardsquareid.to_string();
        return toreturn ;
    }
    else if let ObjectType::card(cardid) = inputobjecttype{
        
        let toreturn = "C".to_string() + &cardid.to_string();
        return toreturn ;
    }
    else if let ObjectType::deck = inputobjecttype{
        return "deck".to_string();
    }
    else if let ObjectType::raisebutton = inputobjecttype{
        return "raise button".to_string();
    }
    else if let ObjectType::foldbutton = inputobjecttype{
        return "fold button".to_string();
    }
    else if let ObjectType::checkbutton = inputobjecttype{
        return "check button".to_string();
    }
    else if let ObjectType::debtbutton = inputobjecttype{
        
        return "debt button".to_string();
    }
    else{
        panic!("cant convert object type to a string");
    }
    
    
}










//the most complete way form of an object
//for babylon to take and display

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AppearanceData{
    
    name: String,
    
    //the shape
    shape: Shape,
    
    //the texture
    texture: Texture,
    
}



#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Shape{
    
    shapetype: ShapeType,
    
    position: (f32,f32,f32),
    rotation: (f32,f32,f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ShapeType{
    Cube(CubeShape),
    Cylinder(CylinderShape),
    Circle(CircleShape),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CubeShape{
    
    dimensions: (f32,f32,f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CylinderShape{
    
    dimensions: (f32,f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CircleShape{
    diameter: f32,
}



#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Texture{
    
    colour: (u8,u8,u8),
    
    image: Option<String>,
    
    text: Option<Text>,
}



#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Text{
    
    text: String,
    
    position: (f32,f32),
    
    fontsize: u32,
    
}