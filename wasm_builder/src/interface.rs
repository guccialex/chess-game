use physicsengine::PlayerInput;
use physicsengine::PieceAction;
use physicsengine::Card;

use std::collections::HashMap;

use physicsengine::MainGame;


use physicsengine::GameData;




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
    
    //when a message is received on the javascript side
    //get it and send it directly here
    //to be used as an update on the state of the game
    pub fn receive_game_state_data(&mut self, websocketdata: GameData) {
        
        self.thegame.set_game_information(websocketdata);
        
    }
    
    
    //given an object
    //get the list of other objects that can be clicked on when this object is selected
    pub  fn get_this_objects_selectable_objects(&self, objectid: ObjectType ) -> Vec<ObjectType>{
        
        let mut toreturn = Vec::new();
        
        
        //if the object is a piece
        if let ObjectType::piece(pieceid) = objectid{
            
            //get the board squares reachable by the piece
            let (pieceids, squareids) = self.thegame.get_pieces_and_squares_reachable_by_piece(&pieceid);
            
            for (boardsquareidx, boardsquareidy) in squareids{
                let objectid = ObjectType::boardsquare(boardsquareidx, boardsquareidy);
                toreturn.push(objectid);
            };

            for pieceid in pieceids{
            
                let objectid = ObjectType::piece(pieceid);
                toreturn.push(objectid);
            
            }

            
        }
        //if the object is a card
        else if let ObjectType::card(cardid) = objectid{
            
            //get the actions allowed by the card
            let (pieceids, boardsquareids) = self.thegame.get_pieces_and_squares_actable_by_card( self.playerid, cardid );
            
            for pieceid in pieceids{
                
                let objectid = ObjectType::piece(pieceid);
                toreturn.push(objectid);
                
                
            }
            
            for boardsquareid in boardsquareids{
                
                let objectid = ObjectType::boardsquare(boardsquareid.0 , boardsquareid.1 );
                toreturn.push(objectid);
                
            }
            
            
        }
        //if the object is a board square
        else if let ObjectType::boardsquare(bsidx, bsidy) = objectid{
            
            //dont do anything to fill the list to return
            //because no actions can be performed by a board square
            
        }
        
        
        return toreturn;
        
    }
    
    
    //gets a map of every valid player input for this given object
    //mapped by the id of the object that needs to be clicked on for it to be performed
    fn get_inputs_of_object(&self, objectid: ObjectType) -> HashMap< ObjectType, PlayerInput >{
        
        let mut toreturn = HashMap::new();
        
        
        //if the object is a piece
        if let ObjectType::piece(pieceid) = objectid{
            
            
            //get the actions allowed by the piece
            let (_, actionsallowedbypiece) = self.thegame.get_actions_allowed_by_piece(&pieceid);
            
            
            //for every action allowed, get the objectid of the board square and the piece id associated it can capture
            for (action, boardsquare, pieces) in actionsallowedbypiece{
                
                //for the board squares
                let objectid = ObjectType::boardsquare(boardsquare.0, boardsquare.1);
                let playerinput = PlayerInput::pieceaction( pieceid, action );
                toreturn.insert(  objectid, playerinput.clone() );
                
                
                //and for every piece
                for pieceid in pieces{
                    
                    let objectid = ObjectType::piece(pieceid);
                    toreturn.insert( objectid, playerinput.clone() );
                    
                }
                
                
            }
            
            
            
        }
        //if the object is a card
        else if let ObjectType::card(cardid) = objectid{
            
            //get the pieces and squares actable by the card
            let (pieceinputs, boardsquareinputs) = self.thegame.get_piece_and_square_actions_allowed_by_card(self.playerid, cardid);

            //panic!("the inputs allowed{:?}", boardsquareinputs);
            
            //for each piece and board square, add it to the return list
            for (pieceid, input) in pieceinputs{
                
                toreturn.insert( ObjectType::piece(pieceid), input );
                
            }
            for ((bsidx, bsidy), input) in boardsquareinputs{
                
                toreturn.insert( ObjectType::boardsquare(bsidx , bsidy ), input );
                
            }
            
            
        }
        //if the object is a board square
        else if let ObjectType::boardsquare(bsidx, bsidy) = objectid{
            
            //dont do anything to fill the list to return
            //because no actions can be performed by a board square
            
        }
        
        
        toreturn
    }
    
    
    
    
    
    
    
    //given the id of an main object, and then an object that its trying to perform an action on
    //try to perform that action and return whether it succeded and was sent to be performed or not
    pub fn try_to_perform_action(&mut self, object1: ObjectType, object2: ObjectType) -> bool{
        
        let objecttoinput = self.get_inputs_of_object(object1);

        
        //if there is a player input that lets object1 perform some action on object 2
        if let Some(playerinput) = objecttoinput.get(&object2){

            
            //send that input to the game and return true
            self.thegame.receive_input( self.playerid, playerinput.clone());
            
            return true;
            
        };
        
        
        //otherwise do nothing and return false
        return false;
        
    }
    
    
    pub fn try_to_flick_piece(&mut self, pieceid: u32, direction: f32, force: f32 ) -> bool{
        
        
        let flickaction = PieceAction::flick(direction, force);
        
        let flickinput = PlayerInput::pieceaction(pieceid, flickaction);
        
        //give the flick input to the game
        self.thegame.receive_input(self.playerid, flickinput);
        
        true
        
        
    }
    
    
    //try to play the selected card for its effect on the game board
    pub fn try_to_play_card_on_game_board(&mut self, cardid: u16){
        
        
        let input = PlayerInput::playcardonboard(cardid);
        
        self.thegame.receive_input( self.playerid, input);
        
    }
    
    
    //try to play the selected card for its effect on the card board
    pub fn try_to_play_card_on_card_board(&mut self, cardid: u16){

        let input = PlayerInput::playcardonboard(cardid);
        
        self.thegame.receive_input( self.playerid, input);
        
    }
    
    
    
    
    
    
    //get the appearance of this object
    fn get_object_appearance(&self, objectid: ObjectType) -> ObjectAppearance{
        
        //if its a card
        if let ObjectType::card(cardid) = objectid{
            
            //if i can get the card from this players perspective
            let card = self.thegame.get_card(&cardid, &self.playerid);
            
            //get the player whos hand it is in 
            let ownersid = self.thegame.get_card_owner(&cardid);
            //get its index in that players hand
            let handposition = self.thegame.get_card_position_in_hand(&cardid);
            
            
            let objectname = objecttype_to_objectname(objectid);
            
            let appearanceid = LocalGameInterface::get_appearance_id_of_card(&card);
            
            let mut xpos = handposition as f32 * 2.5;
            let mut ypos = 2.0;
            let mut zpos = 5.0;
            
            let mut xrot = 0.0;
            let mut yrot = 0.0;
            let mut zrot = 0.0;
            
            
            if ownersid == 1{
                
                zpos = -5.0;
            }
            if ownersid == 2{
                
                zpos = 5.0;
                
                
            }
            
            
            let toreturn = ObjectAppearance{
                
                
                //the name of the object
                objectname: objectname,
                
                //the appearanceid
                appearanceid: appearanceid,
                
                
                //the position
                xposition: xpos,
                yposition: ypos,
                zposition: zpos,
                
                //the rotation
                xrotation: xrot,
                yrotation: yrot,
                zrotation: zrot,
                
                
                
                
                
                isselected: false,
                
                ishighlighted: false,
                
                
            };
            
            return( toreturn );
            
            
            
            
            
            
        }
        else if let ObjectType::piece(pieceid) = objectid{
            
            
            //get its position
            let (xpos, ypos, zpos) = self.thegame.get_piece_translation( pieceid);
            //get its isometry
            let (xrot, yrot, zrot) = self.thegame.get_piece_rotation( pieceid);
            
            //get its appearance id
            
            //starting with 2 is black
            //starting with 3 is white
            //1 pawn
            //2 knight
            //3 bishop
            //4 rook
            //5 queen
            //6 king
            let appearanceid = 10;
            
            //and its name
            let objectname = objecttype_to_objectname(objectid);
            
            
            let toreturn = ObjectAppearance{
                
                objectname: objectname,
                xposition: xpos,
                yposition: ypos,
                zposition: zpos,
                xrotation: xrot,
                yrotation: yrot,
                zrotation: zrot,
                appearanceid: appearanceid,
                
                
                isselected: false,
                ishighlighted: false,
            };
            
            return(toreturn);
            
            
            
            
        }
        else if let ObjectType::boardsquare(boardsquareidx, boardsquareidy) = objectid{
            
            //get its position
            let (xpos, ypos, zpos) = self.thegame.get_board_square_translation( &(boardsquareidx, boardsquareidy) );
            let (xrot, yrot, zrot) = self.thegame.get_board_square_rotation( &(boardsquareidx, boardsquareidy) );
            
            //if board square id x + id y is even
            let iseven = (boardsquareidx + boardsquareidy) % 2;
            
            let appearanceid = 20 + iseven;
            
            
            
            let objectname = objecttype_to_objectname(objectid);
            
            let toreturn = ObjectAppearance{
                
                objectname: objectname,
                xposition: xpos,
                yposition: ypos,
                zposition: zpos,
                xrotation: xrot,
                yrotation: yrot,
                zrotation: zrot,
                appearanceid: appearanceid,
                
                
                isselected: false,
                ishighlighted: false,
            };
            
            return(toreturn);
            
            
            
            
        }
        else{
            
            panic!("why isnt the object id matching with an object of any of these types?");
            
        };
        
    }
    
    
    //get a list of each object in the game by id (objecttype)
    fn get_objects(&self) -> Vec<ObjectType>{
        
        let pieceids = self.get_piece_ids();
        let boardsquareids = self.get_board_square_ids();
        
        let mut toreturn = Vec::new();
        
        
        for curpieceid in pieceids{
            let objectid = ObjectType::piece(curpieceid);
            toreturn.push(objectid);
        };
        for (curboardsquareidx, curboardsquareidy) in boardsquareids{
            let objectid = ObjectType::boardsquare(curboardsquareidx, curboardsquareidy);
            toreturn.push(objectid);
        };

        
        
        
        toreturn
        
    }
    
    
    //get an objects flat position on the plane
    pub fn get_object_flat_plane_position(&self, objectid: ObjectType) -> (f32,f32){
        
        if let ObjectType::piece(pieceid) = objectid{
            
            //get its position
            let (xpos, ypos, zpos) = self.thegame.get_piece_translation( pieceid);
            
            return(  (xpos,zpos ) );
            
            
        }
        
        (0.0,0.0)
        
        //should panic if its not a piece being dragged
        //panic!("it shouldnt be anything but a ")
    }
    
    
    
    
    fn get_appearance_id_of_card(card: &Card) -> u32{
        
        
        //giving a card of every suit and value a unique ID
        let toreturn =  100 + 4 * card.numbervalue() + card.suitvalue();
        
        
        toreturn as u32
        
    }
    
    
    fn get_cards_in_cardgame_appearance(&self) -> Vec<ObjectAppearance>{
        
        let mut toreturn = Vec::new();
        
        
        //get the state of the cards  in the game if there is a game
        if let Some( (player1hand, rivercards, player2hand) ) = self.thegame.get_cards_in_game(){
            
            let mut uniquecardidnumb = 100;
            
            
            let mut xpositioninsection = 0.0;
            
            for card in player1hand{
                
                uniquecardidnumb += 1;
                
                xpositioninsection += 2.5;
                
                let appearance = ObjectAppearance{
                    
                    //the name of the object
                    objectname: "G".to_string()+&uniquecardidnumb.to_string(),
                    
                    //the appearanceid
                    appearanceid: LocalGameInterface::get_appearance_id_of_card(&card),
                    
                    
                    //the position
                    xposition: xpositioninsection + 5.0,
                    yposition: 0.0,
                    zposition: -5.0,
                    
                    //the rotation
                    xrotation: 0.0,
                    yrotation: 0.0,
                    zrotation: 0.0,
                    
                    
                    isselected: false,
                    ishighlighted: false,
                    
                };
                
                
                toreturn.push(appearance);
                
                
            }
            
            
            
            let mut xpositioninsection = 0.0;
            
            for card in rivercards{
                
                uniquecardidnumb += 1;
                
                xpositioninsection += 2.5;
                
                let appearance = ObjectAppearance{
                    
                    //the name of the object
                    objectname: "G".to_string()+&uniquecardidnumb.to_string(),
                    
                    //the appearanceid
                    appearanceid: LocalGameInterface::get_appearance_id_of_card(&card),
                    
                    
                    //the position
                    xposition: xpositioninsection + 5.0,
                    yposition: 0.0,
                    zposition: 0.0,
                    
                    //the rotation
                    xrotation: 0.0,
                    yrotation: 0.0,
                    zrotation: 0.0,
                    
                    
                    isselected: false,
                    ishighlighted: false,
                    
                };
                
                
                toreturn.push(appearance);
                
                
            }
            
            

            let mut xpositioninsection = 0.0;
            
            for card in player2hand{
                
                uniquecardidnumb += 1;
                
                xpositioninsection += 2.5;
                
                let appearance = ObjectAppearance{
                    
                    //the name of the object
                    objectname: "G".to_string()+&uniquecardidnumb.to_string(),
                    
                    //the appearanceid
                    appearanceid: LocalGameInterface::get_appearance_id_of_card(&card),
                    
                    
                    //the position
                    xposition: xpositioninsection + 5.0,
                    yposition: 0.0,
                    zposition: 5.0,
                    
                    //the rotation
                    xrotation: 0.0,
                    yrotation: 0.0,
                    zrotation: 0.0,
                    
                    
                    isselected: false,
                    ishighlighted: false,
                    
                };
                
                
                toreturn.push(appearance);
                
                
            }
            
            
            
        }
        
        
        toreturn
        
    }
    
    
    
    
    pub fn get_full_appearance_state(&self) -> FullAppearanceState{
        
        let mut toreturn = FullAppearanceState::new();
        
        
        //get the piece ids
        //get the board square ids
        //get the card ids of the cards in the players main hands
        let objectids = self.get_objects();
        
        //get the object appearance of these objects
        let mut objectsappearance: Vec<ObjectAppearance> = Vec::new();
        
        for objectid in objectids{
            
            let objectappearance = self.get_object_appearance(objectid);
            objectsappearance.push ( objectappearance );
            
        };
        
        
        let cardsingameappearance = self.get_cards_in_cardgame_appearance();
        
        
        
        //for every object in the game add it to the full appearance to return
        for objectappearance in objectsappearance{
            
            toreturn.add_object(objectappearance);
        }
        
        
        //for every object in the card game add it to the full appearance to return
        for objectappearance in cardsingameappearance.clone(){
            toreturn.add_object(objectappearance);
        }
        
        
        //if there are cards in the card game
        if cardsingameappearance.len() != 0{
            
            
            
            let cardboard = ObjectAppearance{
                
                xposition: 7.0,
                yposition: -2.0,
                zposition: 0.0,
                
                xrotation: 0.0,
                yrotation: 0.0,
                zrotation: 0.0,
                
                
                objectname: "G201".to_string(),
                
                
                appearanceid: 201,
                
                
                isselected: false,
                ishighlighted: false,
                
                
            };
            
            //add the card board
            toreturn.add_object(cardboard);
            
            
            
        }
        
        
        
        
        toreturn
        
    }
    
    
    
    
    
}







#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum ObjectType{
    
    card(u16),
    boardsquare(u8,u8),
    piece(u32),
    
}


use serde::{Serialize, Deserialize};



//appearance data for an object
#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectAppearance{
    
    //the position
    pub xposition: f32,
    pub yposition: f32,
    pub zposition: f32,
    
    //the rotation
    pub xrotation: f32,
    pub yrotation: f32,
    pub zrotation: f32,
    
    
    //the name of the object
    pub objectname: String,
    
    
    //the appearanceid
    //for shape
    pub appearanceid: u32,
    
    
    
    //if this object is currently selected
    pub isselected: bool,
    
    //if this object is currently highlighted as an object that can be used to form an action in
    //coaliation with the object that is currently selected
    pub ishighlighted: bool,
    
}




//a struct representing the entire state of a games physical appearance
#[derive(Serialize, Deserialize, Clone)]
pub struct FullAppearanceState{
    
    //this is optional but used when the position of the camera is to be set
    cameraposition: Option<u32>,
    
    
    //the list of every object and its appearance
    objects: Vec<ObjectAppearance>,
    
}

impl FullAppearanceState{
    
    fn new() -> FullAppearanceState{
        
        FullAppearanceState{
            cameraposition: None,
            objects: Vec::new(),
        }
        
    }
    
    
    fn add_object(&mut self, objectappearance: ObjectAppearance){
        
        self.objects.push(objectappearance);
        
    }
    
    pub fn make_object_highlighted(&mut self, objectname: String){
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.objectname == objectname{
                
                curobject.ishighlighted = true;
                
            }
            
        }
        
        
    }
    
    
    pub fn append_object_list(&mut self, objectlist: Vec<ObjectAppearance>){
        
        for object in objectlist{
            
            self.objects.push(object);
            
        }
        
        
    }
    
    
}





//turn an object name into an object type and its ID
pub fn objectname_to_objecttype(objectname: String) -> Option<ObjectType> {
    
    //if the first character of the objects name is "P"
    if objectname.chars().nth(0).unwrap() == 'P'{
        
        //get the rest of the name and try to convert it to an int
        let stringpieceid = objectname[1..].to_string();
        let intpieceid = stringpieceid.parse::<u32>().unwrap();
        let toreturn = ObjectType::piece(intpieceid);
        
        return(Some (toreturn) );
        
        
        
    }
    //if the first character of the objects name is "C"
    else if objectname.chars().nth(0).unwrap() == 'C'{
        
        
        //get the rest of the name and try to convert it to an int
        let stringcardid = objectname[1..].to_string();
        let intcardid = stringcardid.parse::<u16>().unwrap();
        let toreturn = ObjectType::card(intcardid);
        
        return(Some (toreturn));
        
        
    }
    //if the first character of the objects name is "B"
    else if objectname.chars().nth(0).unwrap() == 'B'{
        
        //turn the second character into an id and the third character into an id
        let boardsquarexid = objectname.chars().nth(1).unwrap().to_digit(10).unwrap();
        let boardsquareyid = objectname.chars().nth(2).unwrap().to_digit(10).unwrap();
        
        let toreturn = ObjectType::boardsquare(boardsquarexid as u8, boardsquareyid as u8);
        
        return(Some (toreturn));
        
    }
    else{
        
        //panic!("cant convert this object name to an object type");
        
        return(None);
        
    }
    
}


//turn an object type into its object name
pub fn objecttype_to_objectname(inputobjecttype: ObjectType) -> String {
    
    if let ObjectType::piece(pieceid) = inputobjecttype{
        
        let toreturn = "P".to_string() + &pieceid.to_string();
        return(toreturn);
        
    }
    else if let ObjectType::boardsquare(boardsquareidx, boardsquareidy) = inputobjecttype{
        
        let toreturn = "B".to_string() + &boardsquareidx.to_string() + &boardsquareidy.to_string();
        return(toreturn);
        
    }
    else if let ObjectType::card(cardid) = inputobjecttype{
        
        let toreturn = "C".to_string() + &cardid.to_string();
        return(toreturn);
        
    }
    else{
        panic!("cant convert object type to a string");
    }
    
    
}




//the name of an object to the board that object is on
//0 none
//1 game
//2 card
pub fn objectname_to_board(objectname: String) -> u32{
    
    if objectname.len() == 0{
        
        return(0);
        
    }
    
    
    //if its a piece
    if objectname.chars().nth(0).unwrap() == 'P'{
        return 1;
    }
    //if its a card in a game
    if objectname.chars().nth(0).unwrap() == 'G'{

        return 2;
    }
    //if its a card in a hand or outside a game
    if objectname.chars().nth(0).unwrap() == 'C'{
        return 0;
    }
    //if its a boardsquare
    if objectname.chars().nth(0).unwrap() == 'B'{
        return 1;
    }
    
    
    
    //otherwise its not a game or a card
    0
    
}