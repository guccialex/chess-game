mod gameengine;

use std::collections::HashSet;
use std::collections::HashMap;




use gameengine::GameEngine;

//pub use gameengine::GameEngine::is_board_game_object_square;

pub use gameengine::PieceAction;


mod datastructs;
use datastructs::TurnManager;

use serde::{Serialize, Deserialize};





#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerInput{
    
    
    //perform an action on a piece
    pieceaction(u16, PieceAction),
    
    //draw card from the deck
    drawcard,
    
}




//the effect of the card it can have
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardEffect{
    
    //joker effects
    
    backtobackturns, 
    
    halvetimeleft,
    
    makepoolgame,

    turnstimed(u32),
    
    //what other game effects?
    removesomesquares(u32),
    
    raisesomesquares(u32),
    
    checkerify,
}


impl CardEffect{  
    
    //get a random card effect playable on the board
    fn get_joker_card_effect() -> CardEffect{
        
        
        use rand::Rng;
        
        let mut jokereffects = Vec::new();
        jokereffects.push(CardEffect::backtobackturns);
        jokereffects.push(CardEffect::halvetimeleft);
        jokereffects.push(CardEffect::makepoolgame);
        //jokereffects.push(CardEffect::raisesomesquares(7));
        jokereffects.push(CardEffect::removesomesquares(7));
        //jokereffects.push(CardEffect::checkerify);
        //jokereffects.push(CardEffect::turnstimed(30) );
        
        let mut rng = rand::thread_rng();
        let effectnumb = rng.gen_range(0, jokereffects.len() );
        let jokereffect = jokereffects[effectnumb].clone();
        
        jokereffect    
    }
}





//the game effects which change the game from its DEFAULT of normal chess game
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum GameEffect{
    
    
    //if the kings are replaced when dying
    KingsReplaced,
    
    //if the pawns arent promoted
    PawnsNotPromoted,
    
    //if players take 2 turns in a row
    DoubleTurns,
    
    
    //if its a pool game
    PoolGame,
    
    
    //what other game effects?
    RemovedSquares(u32),
    
    RaisedSquares(u32),
    
    Checkerified,
    
    //if turns take time
    TurnsTimed(u32),
    
    
}




#[derive(Serialize, Deserialize)]
pub struct GameEffects{
    
    list: HashSet<GameEffect>,

    totalraisedsquares: u32,

    totalremovedsquares: u32,
}

impl GameEffects{
    
    pub fn new() -> GameEffects{
        
        GameEffects{
            
            list: HashSet::new(),

            totalraisedsquares: 0,

            totalremovedsquares: 0,
        }
    }
    
    pub fn set_pawns_not_promoted(&mut self){
        
        self.list.insert( GameEffect::PawnsNotPromoted );
    }
    pub fn get_pawns_not_promoted(&self) -> bool{
        
        return self.list.contains( &GameEffect::PawnsNotPromoted ) ;
    }
    
    
    pub fn set_kings_replaced(&mut self){
        
        self.list.insert( GameEffect::KingsReplaced );
    }
    pub fn get_kings_replaced(&self) -> bool{
        
        return self.list.contains( &GameEffect::KingsReplaced ) ;
    }
    
    
    pub fn set_double_turns(&mut self){
        
        self.list.insert( GameEffect::DoubleTurns );
    }
    pub fn get_double_turns(&self) -> bool{
        
        return self.list.contains( &GameEffect::DoubleTurns ) ;
        
    }
    
    
    pub fn set_pool_game(&mut self){
        self.list.remove(&GameEffect::Checkerified);
        
        self.list.insert( GameEffect::PoolGame );
    }
    pub fn get_pool_game(&self) -> bool{
        
        self.list.contains( &GameEffect::PoolGame ) 
    }


    pub fn set_checkerified(&mut self){

        self.list.remove(&GameEffect::PoolGame);

        self.list.insert(GameEffect::Checkerified);
    }

    pub fn get_checkerified(&mut self) -> bool{

        self.list.contains(&GameEffect::Checkerified)
    }


    //set the total raised squares
    pub fn set_raised_squares(&mut self, number: u32){

        self.totalraisedsquares = number;
    }
    pub fn add_raised_squares(&mut self, number: u32){

        self.totalraisedsquares += number;
    }
    pub fn subtract_raised_squares(&mut self, tosubtract: u32){

        self.totalraisedsquares = self.totalraisedsquares.saturating_sub(tosubtract);
    }
    //get the total number of raised squares
    pub fn get_raised_squares(&self) -> u32{

        self.totalraisedsquares

    }





    pub fn add_removed_squares(&mut self, number: u32){

        self.totalremovedsquares += number;
    }

    pub fn subtract_removed_squares(&mut self, number: u32){

        self.totalremovedsquares = self.totalremovedsquares.saturating_sub(number);
    }
    pub fn set_removed_squares(&mut self, number: u32){

        self.totalremovedsquares = number;
    }

    //get the total number of raised squares
    pub fn get_removed_squares(&self) -> u32{

        self.totalremovedsquares

    }



    //a few ways I can have the invarants I want satisfied
    //the setter establishes the invariant
    //the getter makes sure the invariant is delivered
    //the tick function reestablishes the invariants


    //set the ticks that a player has for their turn
    pub fn set_turn_length(&mut self, ticks: u32){

        let mut prevturnstimedvalue = 0;

        //get if the list already contains a TurnsTimed(anyvalue)

        for effect in &self.list{

            if let GameEffect::TurnsTimed(somevalue) = effect{

                prevturnstimedvalue = *somevalue;
            }
        }


        self.list.remove( &GameEffect::TurnsTimed(prevturnstimedvalue)  );

        self.list.insert( GameEffect::TurnsTimed(ticks) );
    }


    pub fn get_turn_length(&mut self) -> Option<u32>{

        for effect in &self.list{

            if let GameEffect::TurnsTimed(somevalue) = effect{

                return Some(*somevalue);
            }
        }


        return None;
    }


    
    

    
    pub fn card_drawn(&mut self){
        
        self.list.insert( GameEffect::KingsReplaced );
        self.list.insert( GameEffect::PawnsNotPromoted );
    }
    
    
    
    //get visible game effects
    pub fn get_visible_game_effects(&self) -> Vec<GameEffect>{
        
        use std::iter::FromIterator;
        
        let mut toreturn = Vec::from_iter( self.list.clone() );

        if self.totalraisedsquares > 0{
            toreturn.push( GameEffect::RaisedSquares(self.totalraisedsquares) );
        }

        if self.totalremovedsquares > 0{

            toreturn.push( GameEffect::RemovedSquares(self.totalremovedsquares) );
        }
        
        toreturn
    } 
    
    
}





//the maingame creates and returns these objects as its fuctions
#[derive(Serialize, Deserialize)]
pub struct MainGame{
    
    
    //the things that basically constitute the state of the game
    
    //the board game engine
    boardgame: GameEngine,
    
    //the manager for who has a turn turn currently is
    turnmanager: TurnManager,
    
    //the effects on the game
    gameeffects: GameEffects,
    
    
    
    
    totalticks: u32,
    
    
    //the last card effect applied and how long ago it was
    lastcardeffect: Option<(CardEffect, u32)>,
    
    
    //the last input of each player
    queuedinputs: HashMap<u8, Option<PlayerInput>>,
    
    
    //if the game is finished, and who the winner is
    gameover: Option<u8>,
    
    
}

impl MainGame{
    
    //create a game with two players
    pub fn new_two_player() -> MainGame{
        
        let mut queuedinputs = HashMap::new();
        queuedinputs.insert(1, None);
        queuedinputs.insert(2, None);
        
        
        //create a new 2 player game        
        let mut toreturn = MainGame{
            turnmanager: TurnManager::new_two_player(1, 2),            
            boardgame: GameEngine::new(1,2),
            queuedinputs: queuedinputs,
            
            totalticks: 0,
            gameover: None,
            gameeffects: GameEffects::new(),
            lastcardeffect: None,
        };
        
        
        toreturn        
    }
    
    
}



//THE METHODS REQUIRED FOR THE TICK METHOD
impl MainGame{ 
    
    
    
    //get what pieces are captures in the game engine and remove them from here
    pub fn tick(&mut self){
        
        //get each player whos turn it currently is
        let currentturnplayers = self.turnmanager.get_current_players();
        
        
        for playerid in currentturnplayers.clone(){
            
            //if this player has a queued input
            if let Some(playerinput) = self.queuedinputs.get(&playerid).unwrap(){
                
                //if its valid to perform it
                if self.is_input_valid(&playerid, &playerinput){
                    
                    self.perform_input(&playerid, &playerinput.clone());
                    
                    self.turnmanager.player_took_action(playerid);
                    
                    //and clear queud input for this player
                    self.queuedinputs.insert(playerid, None);

                }
            }
        }
        
        
        //tick turn managers
        
        {   
            let temptest = Some(20);
            //let turnlength = self.gameeffects.get_turn_length();
            
            self.turnmanager.tick(self.gameeffects.get_double_turns(), temptest );
        }  
        
        
        
        {   
            let arepawnspromoted = ! self.gameeffects.get_pawns_not_promoted();
            let arekingsreplaced = self.gameeffects.get_kings_replaced();
            let ispoolgame = self.gameeffects.get_pool_game();
            let raisedsquares = self.gameeffects.get_raised_squares();
            let removedsquares = self.gameeffects.get_removed_squares();
            let checkersgame = self.gameeffects.get_checkerified();
            
            self.boardgame.tick(arekingsreplaced, arepawnspromoted, ispoolgame, raisedsquares, removedsquares, checkersgame);
        }
        
        
        
        self.set_if_game_is_over();


        
        
        
        if let Some( (_, tickssince) ) = &mut self.lastcardeffect{
            *tickssince += 1;
        }
        
        
        self.totalticks +=1;


        //every 30th tick
        if self.totalticks %30 == 0{

            self.gameeffects.subtract_raised_squares(1);

            self.gameeffects.subtract_removed_squares(1);
        }


        //if the game has been running for more than 1000 seconds (~16 minutes)
        if self.totalticks > 30000{
            panic!("Game has been over for long enough. Pod is going to be restarted now");
        }
        
    }
    
    
    fn set_if_game_is_over(&mut self){
        
        //update if the game is over and what player won
        
        
        //if the player doesnt have a king
        //and neither player has drawn a card yet
        if ! self.boardgame.does_player_have_king(1){
            self.gameover = Some(2);
        }
        if ! self.boardgame.does_player_have_king(2){
            self.gameover = Some(1);
        }
        
        
        //check if either player has no time left on their clock
        if self.turnmanager.get_players_total_ticks_left(1) == 0{
            self.gameover = Some(2);
        }
        if self.turnmanager.get_players_total_ticks_left(2) == 0{
            self.gameover = Some(1);
        }
        
        
    }
    
    
    
    
    
    
    
    
    
    
    
    //check if input is valid rather than just if the action is
    //if the player is the one sending the request or some shit like that i guess
    fn is_input_valid(&self, playerid: &u8, input: &PlayerInput) -> bool{
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = input.clone(){
            return self.is_piece_action_valid( &(pieceid as u16), &pieceaction);
        }
        
        else if let PlayerInput::drawcard = input{
            return self.can_player_draw(playerid);
        }
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
    }
    
    
    
    
    fn is_piece_action_valid(&self, pieceid: &u16,  pieceaction: &PieceAction) -> bool{
        
        //if the piece action is a slide or lift action
        if  let PieceAction::slide(_,_) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.boardgame.get_actions_allowed_by_piece(*pieceid).1;
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return true;                
            }
            else{
                return false;
            }
            
        }
        else if let PieceAction::liftandmove( _ ) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.boardgame.get_actions_allowed_by_piece(*pieceid).1;
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return true;                
            }
            else{
                return false;
            }
            
        }
        else if let PieceAction::flick(direction, force) = pieceaction{            
            
            let canflick = self.boardgame.get_actions_allowed_by_piece(*pieceid).0;
            
            return canflick;
            
        }
        
        panic!(" dont know what kind of mission this is..");
        
    }
    
    
    
    
    
    
    
    
    fn apply_card_effect(&mut self, playerid: &u8, cardeffect: CardEffect){
        
        self.lastcardeffect = Some((cardeffect.clone(), 0));
        
        
        if cardeffect == CardEffect::makepoolgame{
            self.gameeffects.set_pool_game();
        }
        else if cardeffect == CardEffect::backtobackturns{
            self.gameeffects.set_double_turns();
        }
        else if cardeffect == CardEffect::halvetimeleft{
            self.turnmanager.halve_time_left();
        }
        else if cardeffect == CardEffect::checkerify{

            self.gameeffects.set_checkerified();
        }
        else if let CardEffect::raisesomesquares(number) = cardeffect{


            self.gameeffects.add_raised_squares(number );
        }
        else if let CardEffect::removesomesquares(number) = cardeffect{

            self.gameeffects.add_removed_squares(number);
        }
        else if let CardEffect::turnstimed(ticks) = cardeffect{

            self.gameeffects.set_turn_length(ticks);
        }
        else{
            //otherwise panic, because this card should not have been allowed to be played
            //and it will fuck shit if i get here without actually having a valid action
            
            panic!("I dont know what a {:?} is", cardeffect);
        }
    }
    
    
    
    //perform an input that is valid, and it is the turn of the player
    fn perform_input(&mut self, playerid: &u8, playerinput: &PlayerInput) {
        
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = playerinput {
            
            self.boardgame.perform_action( *pieceid, pieceaction.clone() );
        }
        else if let PlayerInput::drawcard = playerinput{
            
            self.gameeffects.card_drawn();
            
            self.apply_card_effect(playerid, CardEffect::get_joker_card_effect() );
        }
        else{
            panic!("unhandled input to be performed {:?}", playerinput);
        }
        
    }
    
    
    
    
    
    
}





//i am adopting the idea that if a function is a single line and only called in one other place
//even if theres a potential for the way of that value being determined, you should probably get rid of that function










//getters
//NONE OF THE FUNCTIONS IN THE FIRST IMPL BLOCK SHOULD RELY ON THESE FUNCTIONS
//the functions the user interfaces
//generally these two groups of functions dont depend on each other
impl MainGame{
    
    
    
    
    //can a player do a draw card action
    fn can_player_draw(& self, playerid: &u8) -> bool{
        
        //if its past turn 10
        if self.turnmanager.get_turn_number() > 10{
            return true;
        }
        
        
        return false;
    }
    
    
    //is this board game object a square
    pub fn is_board_game_object_square(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_square(objectid)
    }
    //is this board game object a piece
    pub fn is_board_game_object_piece(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_piece(objectid)
    }
    
    
    //get the state of the game as a string
    pub fn get_string_state(&self) -> String{
        
        let binstate = bincode::serialize(&self).unwrap();        
        let vecofchar = binstate.iter().map(|b| *b as char).collect::<Vec<_>>();
        let stringstate = vecofchar.iter().collect::<String>();

        
        stringstate
    }
    
    
    //set the state of the game using a string, returns error if the string is invalid
    pub fn set_string_state(&mut self, stringstate: String) -> Result<(), ()>{
        
        let vecofchar = stringstate.chars().collect::<Vec<_>>();
        let gamebin = vecofchar.iter().map(|c| *c as u8).collect::<Vec<_>>();
        
        if let Ok(gamestate) = bincode::deserialize::<MainGame>(&gamebin){
            
            *self = gamestate;
            
            return Ok( () ); 
        }
        else{
            return Err( () );
        }
        
    }
    
    pub fn receive_string_input(&mut self, playerid: &u8, stringinput: String) -> Result<(), ()>{
        
        //try to convert to player input with serde json
        
        if let Ok(playerinput) = serde_json::from_str::<PlayerInput>(&stringinput){
            
            self.receive_input(*playerid, playerinput);
            
            return Ok ( () );
        }
        
        return Err( () );
    }
    
    
    pub fn get_board_game_object_ids(&self) -> Vec<u16>{
        self.boardgame.get_object_ids()
    }
    
    
    //get the input that a player sends and set it to be performed next tick
    //return whether this input is valid for this player to have queued
    pub fn receive_input(&mut self, playerid: u8, input: PlayerInput) -> Option<String>{
        
        //get if the input is valid for this player
        if  self.is_input_valid(&playerid, &input ) {
            
            self.queuedinputs.insert(playerid, Some( input.clone() ));
            
            return Some( serde_json::to_string(&input).unwrap() );
            
        }
        else{
            
            return None;
        };
    }
    
    
    pub fn get_board_game_object_owner(&self, objectid: u16) -> Option<u8>{
        
        Some(self.boardgame.get_owner_of_piece(objectid))
        
    }
    
    
    //get the state of the game
    pub fn get_visible_game_state(&self, playerid: &u8) -> VisibleGameState{
        
        
        let mut boardobjects = Vec::new();
        
        
        let boardobjectids = self.get_board_game_object_ids();
        
        for objectid in boardobjectids{
            
            let position = self.boardgame.get_object_translation(objectid);
            let rotation = self.boardgame.get_object_rotation(objectid);
            
            let isonmission = self.boardgame.is_object_on_mission(objectid);
            
            if self.is_board_game_object_piece(objectid){
                
                let visiblegamepiece = VisibleGamePieceObject{
                    owner: self.boardgame.get_owner_of_piece(objectid),
                    typename: self.boardgame.get_piece_type_name(objectid),
                };
                
                let boardobject = VisibleGameBoardObject{
                    position: position,
                    rotation: rotation,
                    id: objectid,
                    isonmission: isonmission,
                    objecttype: VisibleGameObjectType::Piece(visiblegamepiece),
                };
                
                boardobjects.push(boardobject);
            }
            else if self.is_board_game_object_square(objectid){
                
                let visiblegamesquare = VisibleGameSquareObject{
                    iswhite: self.boardgame.is_boardsquare_white(objectid),
                };
                
                let boardobject = VisibleGameBoardObject{
                    position: position,
                    rotation: rotation,
                    id: objectid,
                    isonmission: isonmission,
                    objecttype: VisibleGameObjectType::Square(visiblegamesquare),
                };
                
                boardobjects.push(boardobject);
            }
            
        }
        
        
        
        VisibleGameState{
            
            isgameover: self.gameover,
            
            drawactionvalid: self.can_player_draw(playerid),
            
            player1totalticksleft: self.turnmanager.get_players_total_ticks_left(1),
            
            player2totalticksleft: self.turnmanager.get_players_total_ticks_left(2),
            
            player1ticksleft: self.turnmanager.get_ticks_left_for_players_turn(1),
            
            player2ticksleft: self.turnmanager.get_ticks_left_for_players_turn(2),
            
            playerswithactiveturns: self.turnmanager.get_current_players(),
            
            boardobjects: boardobjects,
            
            gameeffects: self.gameeffects.get_visible_game_effects(),
            
            lastcardeffect: self.lastcardeffect.clone(),
            
        }
        
    }
    
    
    //the actions allowed by the piece and the objects it captures or lands on
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec<(PieceAction, Vec<u16> )>){
        
        let mut toreturn = Vec::new();
        
        
        //get the actions allowed by the piece on the board
        let (canflick, actions) = self.boardgame.get_actions_allowed_by_piece(pieceid);
        
        
        //get the pieces targeted by every action
        for action in actions{
            
            let objects = self.boardgame.get_objects_targeted_by_action(pieceid, action.clone());
            
            toreturn.push( (action, objects) );
        }
        
        
        (canflick, toreturn)
    }
    
    
}




//the information the client needs to know at every frame to render it
//the information the client needs to render the current frame
pub struct VisibleGameState{
    
    //has either player won
    pub isgameover: Option<u8>,
    
    
    //the deck
    //whether the move is available
    pub drawactionvalid: bool,
    
    pub player1totalticksleft: u32,
    pub player2totalticksleft: u32,
    
    pub player1ticksleft: u32,
    pub player2ticksleft: u32,
    
    //the players whos turn it is right now
    pub playerswithactiveturns: HashSet<u8>,
    
    
    pub boardobjects: Vec<VisibleGameBoardObject>,
    
    
    //the effects currently applied to the game
    pub gameeffects: Vec<GameEffect>,
    
    
    //the most recent card effect applied, and how long since
    pub lastcardeffect: Option< (CardEffect, u32) >,    
    
    
}

impl VisibleGameState{
    
    
    pub fn get_piece_plane_position(&self, id: u16) -> Option< (f32,f32) >{
        
        for curobject in &self.boardobjects{
            
            if id == curobject.id{
                
                return Some( (curobject.rotation.0, curobject.rotation.2) );
                
            }
        }
        
        None
    }
    
}

pub struct VisibleGameBoardObject{
    
    pub position: (f32,f32,f32),
    
    pub rotation: (f32,f32,f32),
    
    pub id: u16,

    pub isonmission: bool,
    
    pub objecttype: VisibleGameObjectType,
    
}


#[derive(Eq, PartialEq)]
pub enum VisibleGameObjectType{
    
    Piece(VisibleGamePieceObject),
    Square(VisibleGameSquareObject),
}

#[derive(Eq, PartialEq)]
pub struct VisibleGamePieceObject{
    
    pub owner: u8,
    
    pub typename: String,
    
}

#[derive(Eq, PartialEq)]
pub struct VisibleGameSquareObject{
    
    pub iswhite: bool,
    
}