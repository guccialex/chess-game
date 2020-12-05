use physicsengine::PlayerInput;
use physicsengine::Card;

use std::collections::HashMap;

use physicsengine::MainGame;

use physicsengine::PieceAction;

use std::collections::HashSet;



//the interface the "fullgame" has with the rust chesscheckers game
pub struct LocalGameInterface{
    
    
    //the id of the player
    playerid: u8,
    
    //the actual rust game
    thegame: MainGame,


    //the MeshType of each object in the game
    //to the mesh it was previously
    objecttolastmesh: HashMap<String, MeshType>,
    
    
}


//What methods do I want the game to interface with?
impl LocalGameInterface{
    
    
    //create a game with a certain ID
    pub fn new(playerid: u8) -> LocalGameInterface{
        
        
        let thegame = MainGame::new_two_player();
        
        LocalGameInterface{
            
            playerid: playerid,
            thegame:thegame,
            objecttolastmesh: HashMap::new(),
            
        }
    }
    
    
    //tick the local game
    pub fn tick(&mut self) {
        
        self.thegame.tick();
        
    }
    
    
    //the name of an object to the board that object is on
    //0 none
    //1 game
    //2 card
    pub fn objectname_to_board(&self, objectname: String) -> u32{

        if let Some(objecttype) = objectname_to_objecttype(objectname){

            //if its a piece
            if let ObjectType::piece(pieceid) = objecttype{
                return 1;
            }

            else if let ObjectType::card(cardid) = objecttype{

                //if this card has an owner
                if let Some(cardowner) = self.thegame.get_card_owner(cardid){
                    //its not on any board
                    return 0;
                }
                //if it doesnt, its on the card board
                else{
                    return 2;
                }

            }
            //if its a boardsquare
            else if let ObjectType::boardsquare(bsid) = objecttype{
                return 1;
            }


        }

        
        //otherwise its not a game or a card
        0
    }
    
    
    
    
    //returns true if i am the owner of this object, false otherwise
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
            
        }
        
        
        
        return false;
        
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
    
    
    pub fn try_to_flick_piece(&mut self, pieceid: u16, direction: f32, force: f32 ) {
        
        
        let flickaction = PieceAction::flick(direction, force.sqrt() * 40.0);
        
        let flickinput = PlayerInput::pieceaction(pieceid, flickaction);
        
        //give the flick input to the game
        self.thegame.receive_input(self.playerid, flickinput);
        
        
    }
    
    
    //try to play the selected card for its effect on the game board
    pub fn try_to_play_card(&mut self, cardid: u16){
        
        let input = PlayerInput::playcardonboard(cardid);
        
        self.thegame.receive_input( self.playerid, input);
        
    }
    
    
    pub fn try_to_draw_card(&mut self){
        
        let input = PlayerInput::drawcard;
        
        self.thegame.receive_input(self.playerid, input);
        
    }
    
    
    
    //get the appearance of this object
    fn get_object_appearance(&mut self, objectid: ObjectType) -> ObjectAppearance{
        
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
                zpos = -4.0;
                xpos += 4.0;
            }
            else if field == 4{
                zpos = 4.0;
                xpos += 4.0;
            }
            else{
                zpos = 0.0;
                xpos += 4.0;
            }
            
            
            
            let toreturn = ObjectAppearance::new_card( objectname, (xpos, ypos, zpos), (xrot, yrot, zrot), card );
            
            
            return toreturn ;
            
        }
        else if let ObjectType::piece(pieceid) = objectid{
            
            let position = self.thegame.get_board_game_object_translation( pieceid );
            let rotation = self.thegame.get_board_game_object_rotation( pieceid );
            let objectname = objecttype_to_objectname(objectid);
            
            let ownerid = self.thegame.get_board_game_object_owner(pieceid);
            
            let typename = self.thegame.get_piece_type_name(pieceid);

            //if this is a new mesh
            
            let toreturn = ObjectAppearance::new_piece( objectname, typename, position, rotation, ownerid, &mut self.objecttolastmesh );
            
            
            return toreturn;
        }
        else if let ObjectType::boardsquare(bsid) = objectid{
            
            let position = self.thegame.get_board_game_object_translation( bsid );
            let rotation = self.thegame.get_board_game_object_rotation( bsid );
            let objectname = objecttype_to_objectname(objectid);
            
            let issquarewhite = self.thegame.is_boardsquare_white(bsid);
            
            let toreturn = ObjectAppearance::new_boardsquare( objectname, position, rotation, issquarewhite );
            
            
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
        let mut objectsappearance: Vec<ObjectAppearance> = Vec::new();
        
        for objectid in objectids{
            let objectappearance = self.get_object_appearance(objectid);
            objectsappearance.push ( objectappearance );
        };
        
        
        //for every object in the game add it to the full appearance to return
        for objectappearance in objectsappearance{
            toreturn.add_object(objectappearance);
        };        
        
        
        
        
        let deckappearance = ObjectAppearance::new_deck();
        toreturn.add_object(deckappearance);
        
        //add the appearance of the timer for the player and the opponent
        let player1totaltimeleft = self.thegame.get_players_total_ticks_left(1);
        let iscurrentlyturn = (self.thegame.get_players_turn_ticks_left(1) > 0);
        let player1timer = ObjectAppearance::new_timer(1, player1totaltimeleft, iscurrentlyturn);
        toreturn.add_object(player1timer);
        
        
        let player2totaltimeleft = self.thegame.get_players_total_ticks_left(2);
        let iscurrentlyturn = (self.thegame.get_players_turn_ticks_left(2) > 0);
        let player2timer = ObjectAppearance::new_timer(2, player2totaltimeleft, iscurrentlyturn);
        toreturn.add_object(player2timer);
        
        
        
        //if theres a card game going on, get a poker board to render
        
        
        toreturn
    }
    
}


#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum ObjectType{
    
    card(u16),
    boardsquare(u16),
    piece(u16),
    
}


use serde::{Serialize, Deserialize};



//appearance data for an object
#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectAppearance{
    
    //the shared elements
    position: (f32,f32,f32),
    rotation: (f32,f32,f32),
    name: String,
    colour: (u8,u8,u8),

    meshupdated: bool,

    
    mesh: MeshType,
}

impl ObjectAppearance{
    
    pub fn new_cue(pos: (f32,f32,f32), rot: (f32,f32,f32)) -> ObjectAppearance{
        
        let mesh = CubeMesh{
            dimensions: (0.2, 0.2, 1.2),
            texture: None,
        };
        
        let meshtype = MeshType::Cube(mesh);
        
        ObjectAppearance{
            name: "dragindicator".to_string(),
            position: pos,
            rotation: rot,
            colour: (0,0,0),

            meshupdated: false,

            mesh: meshtype,
        }
    }
    
    
    
    pub fn new_deck() -> ObjectAppearance{
        
        let texturename = "cardart/cardback.jpg".to_string();
        
        let mesh = CubeMesh{
            dimensions: (0.6, 1.96, 1.4),
            texture: Some(texturename),
        };
        
        let meshtype = MeshType::Cube(mesh);
        
        
        ObjectAppearance{
            name: "deck".to_string(),
            position: (-7.0,0.0,0.0),
            rotation: (0.0,0.0,0.0),
            colour: (255,255,255),

            meshupdated: false,

            mesh: meshtype,
        }
        
    }
    
    
    pub fn new_timer(playerid: u32, ticksleft: u32, currentlyturn: bool) -> ObjectAppearance{
        

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
        
        let mesh = TimerMesh{
            timeleft: timeleft,
            currentlyturn: currentlyturn,
        };
        
        let meshtype = MeshType::Timer(mesh);
        
        ObjectAppearance{
            position: position,
            rotation: (0.0,0.0,-0.5),
            name: name,
            colour: (255,200,255),

            meshupdated: false,

            mesh: meshtype,
        }
        
    }
    
    pub fn new_piece(objectname: String, typename: String ,position: (f32,f32,f32), rotation: (f32,f32,f32), ownerid: u8, prevmeshmap: &mut HashMap<String, MeshType>) -> ObjectAppearance{
        
        let texturename;
        let colour;
        
        if ownerid == 1{
            colour = (255,255,255);
            texturename = "pieceart/".to_string() + &typename + &".png";
        }
        else{
            colour = (255,255,255);
            texturename = "pieceart/b_".to_string() + &typename + &".png";
            
        }

        let meshtype;
        
        if typename == "poolball"{

            let mesh = CircleMesh{
                diameter: 0.7,
                texture: Some("testball.png".to_string()),
            };

            meshtype = MeshType::Circle(mesh);

        }
        else{

            let mesh = CylinderMesh{
                dimensions: (0.5, 0.72),
                texture: Some(texturename),
            };
            
            meshtype = MeshType::Cylinder(mesh);
        }


        let meshupdated;

        //if theres a meshtype for this object name
        if let Some(prevmeshtype) = prevmeshmap.get(&objectname){

            //if that mesh type is the same as this mesh type, set meshupdated to true
            if prevmeshtype != &meshtype{
                meshupdated = true;
            }
            else{
                meshupdated = false;
            }

        }
        else{
            meshupdated = true;
        }

        prevmeshmap.insert(objectname.clone(), meshtype.clone());


        
        ObjectAppearance{
            name: objectname,
            position: position,
            rotation: rotation,
            colour: colour,
            meshupdated: meshupdated,
            mesh: meshtype,
        }
        
    }
    
    pub fn new_card(name: String, position: (f32,f32,f32), mut rotation: (f32,f32,f32), card: Card ) -> ObjectAppearance{
        
        let texturename = LocalGameInterface::get_name_of_cards_texture(&card);
        
        
        let mesh = CubeMesh{
            dimensions: (0.1, 1.96, 1.4),
            texture: Some(texturename),
        };
        
        let meshtype = MeshType::Cube(mesh);
        
        
        rotation.1 += 3.14159 / 2.0;
        
        ObjectAppearance{
            name: name,
            position: position,
            rotation: rotation,
            colour: (255,255,255),

            meshupdated: false,

            mesh: meshtype,
        }
        
        
    }
    
    
    pub fn new_boardsquare(name: String, position: (f32,f32,f32), rotation: (f32,f32,f32), white: bool ) -> ObjectAppearance{
        
        
        let mesh = CubeMesh{
            dimensions: (1.0, 1.0, 1.0),
            texture: None,
        };
        
        let meshtype = MeshType::Cube(mesh);
        
        let colour;
        
        if white{
            colour = (255,255,255);
        }
        else{
            colour = (0,0,0);
        }
        
        
        ObjectAppearance{
            name: name,
            position: position,
            rotation: rotation,
            colour: colour,

            meshupdated: false,

            mesh: meshtype,
        }
        
        
    }
    
    
    /*
    //display a large version of the card along with its effect when the mouse hovers over the card or clicks on it
    pub fn new_card_display(card:Card) -> ObjectAppearance{


    }
    */

    
}




#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum MeshType{
    Cube(CubeMesh),
    Cylinder(CylinderMesh),
    Circle(CircleMesh),
    Timer(TimerMesh)
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct TimerMesh{

    timeleft: String,
    currentlyturn: bool,
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CubeMesh{
    
    //the size
    dimensions: (f32,f32,f32),
    
    //the name of the texture
    texture: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CylinderMesh{
    
    //the size
    dimensions: (f32,f32),
    
    //the name of the texture
    texture: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CircleMesh{
    
    //the size
    diameter: f32,
    
    //the name of the texture
    texture: Option<String>,
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
            
            if curobject.name == objectname{
                
                let greencolourfloat = (0.0,255.0,0.0);
                let colourfloat = (curobject.colour.0 as f32, curobject.colour.1 as f32, curobject.colour.2 as f32);
                
                
                let mixedr = greencolourfloat.0 * 0.8 + colourfloat.0 * 0.2;
                let mixedg = greencolourfloat.1 * 0.8 + colourfloat.1 * 0.2;
                let mixedb = greencolourfloat.2 * 0.8 + colourfloat.2 * 0.2;
                
                //make its colour closer to green
                curobject.colour = (mixedr as u8, mixedg as u8, mixedb as u8);
                
            }
        }
    }
    
    pub fn make_object_selected(&mut self, objectname: String){
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.name == objectname{
                
                let yellowcolourfloat = (255.0,255.0,0.0);
                let colourfloat = (curobject.colour.0 as f32, curobject.colour.1 as f32, curobject.colour.2 as f32);
                
                
                let mixedr = yellowcolourfloat.0 * 0.8 + colourfloat.0 * 0.2;
                let mixedg = yellowcolourfloat.1 * 0.8 + colourfloat.1 * 0.2;
                let mixedb = yellowcolourfloat.2 * 0.8 + colourfloat.2 * 0.2;
                
                //make its colour closer to green
                curobject.colour = (mixedr as u8, mixedg as u8, mixedb as u8);
                
                
                
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
        let intpieceid = stringpieceid.parse::<u16>().unwrap();
        let toreturn = ObjectType::piece(intpieceid);
        
        return Some (toreturn) ;
        
        
        
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
    else{
        panic!("cant convert object type to a string");
    }
    
    
}




