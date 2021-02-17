mod gameengine;

use gameengine::GameEngine;
pub use gameengine::PieceAction;


use std::collections::HashSet;
use std::collections::HashMap;


mod datastructs;



//import the data structures needed

//make these public, and visible to the game interface
pub use datastructs::PlayerInput;
pub use datastructs::TurnManager;






mod cardstructs;
pub use cardstructs::Card;
use cardstructs::CardEffect;
use cardstructs::CardsInterface;



use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct GameEffects{

    //information about the game
    //information that can be represented as cards


    //does this player have their king replaced by another
    //high value piece when their king is captured
    //aka ANTI "do i lose when I lose my king?"
    arekingsreplaced: bool,


    //if pawns become queens when they reach the back rank
    arepawnspromoted: bool,


    //the effects applied to the state of the game
    /*
    the time left for each player
    the amount of time each player has to take each turn
    if its a pool game (each piece is a pool object)
    if certain squares are lowered for a duration
    if all the pieces can be flicked (pieces are flickable)
    */





}

impl GameEffects{


    pub fn new() -> GameEffects{

        GameEffects{

            arekingsreplaced: false,

            arepawnspromoted: true,

        }
    }


    pub fn arekingsreplaced(&self) -> bool{

        self.arekingsreplaced

    }


    pub fn arepawnspromoted(&self) -> bool{

        self.arepawnspromoted

    }


    pub fn card_drawn(&mut self){

        self.arekingsreplaced = true;
        self.arepawnspromoted = false;
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







    //the last input of each player
    queuedinputs: HashMap<u8, Option<PlayerInput>>,

    //how many ticks the game has been ended for
    //if its been 3000 ticks, panic, to stop running
    ticksgamehasbeenoverfor: u32,

    //if the game is finished, and who the winner is
    gameover: Option<u8>,


}

impl MainGame{
    
    //create a game with two players
    pub fn new_two_player() -> MainGame{
        
        
        //create a new 2 player game        
        let mut toreturn = MainGame{
            turnmanager: TurnManager::new_two_player(1, 2),            
            boardgame: GameEngine::new(1,2),
            queuedinputs: HashMap::new(),
            gameover: None,
            ticksgamehasbeenoverfor: 0,
            gameeffects: GameEffects::new(),
        };
        
        
        toreturn        
    }
    
    
    fn is_game_over(&self) -> Option<u8>{
        
        //win / lose conditions
        //no pieces left
        //king taken
        //no time left
        
        
        self.gameover
    }
    
    //get if it is the players turn, and if it is, how many ticks they have left in their turn
    //0 means it is not their turn
    fn get_players_turn_ticks_left(&self, playerid: u8) -> u32{
        
        if let Some(ticksleft) = self.turnmanager.get_ticks_left_for_players_turn(playerid){
            return ticksleft;
        }
        else{
            return 0;
        }
        
    }
    //get the total amount of time the player has lefts
    fn get_players_total_ticks_left(&self, playerid: u8) -> u32{
        
        self.turnmanager.get_players_total_ticks_left(playerid)
        
    }
    
    
    
    
    //get every player with an active turn
    pub fn get_active_players(&self) -> HashSet<u8>{
        
        self.turnmanager.get_current_players()
    }
    
    
    
    
    pub fn get_board_game_object_ids(&self) -> Vec<u16>{
        self.boardgame.get_object_ids()
    }
    fn get_board_game_object_translation(&self, objectid: u16) -> (f32,f32,f32){
        self.boardgame.get_object_translation(objectid)
    }
    fn get_board_game_object_rotation(&self, objectid: u16) -> (f32,f32,f32){
        self.boardgame.get_object_rotation(objectid)
    }
    
    //is this board game object a square
    pub fn is_board_game_object_square(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_square(objectid)
    }
    //is this board game object a piece
    pub fn is_board_game_object_piece(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_piece(objectid)
    }
    
    //get a string representing teh type of the piece
    fn get_piece_type_name(&self, pieceid: u16) -> Option<String>{
        
        //get if the piece exists
        if self.boardgame.does_piece_have_owner(pieceid){
            
            return Some(self.boardgame.get_piece_type_name(pieceid));
        }
        
        return None;
    }
    
    pub fn get_board_game_object_owner(&self, objectid: u16) -> Option<u8>{
        
        //get if the piece exists
        if self.boardgame.does_piece_have_owner(objectid){
            
            return  Some(self.boardgame.get_owner_of_piece(objectid)) ;
        }
        
        return None;
        
    }
    
    //true if its white false if its black
    fn is_boardsquare_white(&self, boardsquareid: u16) -> bool{
        
        self.boardgame.is_boardsquare_white(boardsquareid)
    }
    
    //the actions allowed by the piece and the objects it captures or lands on
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec<(PieceAction, Vec<u16> )>){
        
        let mut toreturn = Vec::new();
        
        let owner = self.get_board_game_object_owner(pieceid).unwrap();
        
        
        
        //get the actions allowed by the piece on the board
        let (canflick, actions) = self.boardgame.get_actions_allowed_by_piece(pieceid);

        
        //get the pieces targeted by every action
        for action in actions{
            
            let objects = self.boardgame.get_objects_targeted_by_action(pieceid, action.clone());
            
            toreturn.push( (action, objects) );
        }

        
        (canflick, toreturn)
    }
    

    //get the last card played and how many ticks its been since its been played
    
    
    
    
    //get what pieces are captures in the game engine and remove them from here
    pub fn tick(&mut self){
        
        
        //get each player whos turn it currently is
        let currentturnplayers = self.turnmanager.get_current_players();
        
        
        //if the game isnt over, process input
        //and tick the turn manager
        if self.gameover.is_none(){
            
            for playerid in currentturnplayers.clone(){
                
                //if an action was taken
                let mut actionwastaken = false;
                
                //if this player has a queued input
                if let Some(playerinput) = self.queuedinputs.get(&playerid).unwrap(){
                    
                    //if its valid to perform it
                    if self.is_input_valid(&playerid, &playerinput){
                        
                        self.perform_input(&playerid, &playerinput.clone());
                        actionwastaken = true;
                    }
                }
                
                
                //if an action was taken, let the turnmanager know that that player took their turn
                if actionwastaken{    
                    self.turnmanager.player_took_action(playerid);
                    
                    //and clear queud input for this player
                    self.queuedinputs.insert(playerid, None);
                }
            }
            
            //let the turn manager know that a tick has happeneds
            self.turnmanager.tick();
            
            
            self.ticksgamehasbeenoverfor +=1;
            
            if self.ticksgamehasbeenoverfor > 3000{
                panic!("Game has been over for long enough. Pod is going to be restarted now");
            }
        }
        
    
        



        let arepawnspromoted = self.gameeffects.arepawnspromoted();
        let arekingsreplaced = self.gameeffects.arekingsreplaced();


        //tick the physical game engine
        self.boardgame.tick(arekingsreplaced, arepawnspromoted);


                
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
    
    
    
    
    
    
    //can a player do a draw card action
    fn can_player_draw(& self, playerid: &u8) -> bool{

        //if its past turn 10
        if self.turnmanager.get_turn_number() > 10{
            return true;
        }


        return false;
    }
    
    
    
    
    
    //check if input is valid rather than just if the action is
    //if the player is the one sending the request or some shit like that i guess
    fn is_input_valid(&self, playerid: &u8, input: &PlayerInput) -> bool{
        

        if let PlayerInput::pieceaction(pieceid, pieceaction) = input.clone(){
            return self.is_piece_action_valid( &playerid, &(pieceid as u16), &pieceaction);
        }
        
        else if let PlayerInput::drawcard = input{
            return self.can_player_draw(playerid);
        }
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
        
    }
    
    
    
    
    fn is_piece_action_valid(&self, playerid: &u8, pieceid: &u16,  pieceaction: &PieceAction) -> bool{
        
        
        
        
        //if the piece action is a slide or lift action
        if  let PieceAction::slide(_,_) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.boardgame.get_actions_allowed_by_piece(*pieceid).1;
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return(true);                
            }
            else{
                return(false);
            }
            
            
        }
        else if let PieceAction::liftandmove( _ ) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.boardgame.get_actions_allowed_by_piece(*pieceid).1;
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return(true);                
            }
            else{
                return(false);
            }
            
        }
        else if let PieceAction::flick(direction, force) = pieceaction{            
            
            //get the slide and lift actions allowed for the piece
            let canflick = self.boardgame.get_actions_allowed_by_piece(*pieceid).0;
            
            return canflick;
            
        }
        
        panic!(" dont know what kind of mission this is..");
        
        
    }
    
    
    fn apply_card_effect_to_board(&mut self, playerid: &u8, cardeffect: CardEffect){
        

        if cardeffect == CardEffect::makepoolgame{
            self.boardgame.make_pool_game();
        }
        else if cardeffect == CardEffect::backtobackturns{
            self.turnmanager.players_take_2_turns_in_a_row();
        }
        else if cardeffect == CardEffect::halvetimeleft{
            self.turnmanager.halve_time_left();
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
            
            self.apply_card_effect_to_board(playerid, CardsInterface::get_joker_card_effect());
            
        }
        else{
            panic!("unhandled input to be performed {:?}", playerinput);
        }
        
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
    


    //get the state of the game
    pub fn get_visible_game_state(&self, playerid: &u8) -> VisibleGameState{


        let mut boardobjects = Vec::new();


        let boardobjectids = self.get_board_game_object_ids();

        for objectid in boardobjectids{

            let position = self.get_board_game_object_translation(objectid);
            let rotation = self.get_board_game_object_rotation(objectid);
            
            //panic!("position {:?}", position);
            if self.is_board_game_object_piece(objectid){

                let mut towner = 0;

                if let Some(owner) = self.get_board_game_object_owner(objectid){
                    towner = owner;

                }

                let mut name = "pawn".to_string();

                if let Some(piecename) = self.get_piece_type_name(objectid){
                    name = piecename;
                }


                let visiblegamepiece = VisibleGamePieceObject{
                    owner: towner,
                    typename: name,
                };

                let boardobject = VisibleGameBoardObject{
                    position: position,
    
                    rotation: rotation,
    
                    id: objectid,

                    objecttype: VisibleGameObjectType::Piece(visiblegamepiece),
                };
    
                boardobjects.push(boardobject);
            }


            if self.is_board_game_object_square(objectid){

                let visiblegamesquare = VisibleGameSquareObject{

                    iswhite: self.is_boardsquare_white(objectid),

                };

                let boardobject = VisibleGameBoardObject{
                    position: position,

                    rotation: rotation,

                    id: objectid,

                    objecttype: VisibleGameObjectType::Square(visiblegamesquare),
                };

                boardobjects.push(boardobject);
            }

        }


        let playerswithactiveturns = self.get_active_players();


        VisibleGameState{
            
            isgameover: self.is_game_over(),

            drawactionvalid: self.can_player_draw(playerid),

            player1totalticksleft: self.get_players_total_ticks_left(1),

            player2totalticksleft: self.get_players_total_ticks_left(2),

            player1ticksleft: self.get_players_turn_ticks_left(1),
            
            player2ticksleft: self.get_players_turn_ticks_left(2),

            playerswithactiveturns: playerswithactiveturns,

            boardobjects: boardobjects,

        }

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


    //the last card drawn and how long ago it was drawn
    


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



//the different effects that the game can represent as a card
#[derive(Eq, PartialEq)]
pub enum VisibleGameEffect{

    //if the king is captured will it be replaced
    KingReplacable(bool),


    //will pawns be promoted when reaching opponents back ranks
    PawnPromotable(bool),
    

    //how many ticks does each player have to take their turns
    TurnLengthLimit(u32),


    //is it in a pool game mode
    PoolGame(bool),


    //do players take 2 turns back to back
    PlayersTakeTurnsBackToBack(bool),


    //basically, i want
    //the visible game effect to let the player know
    //what the state of the game is by looking at this
    //in combination with
    //the appearance of the pieces

    /*
    I want the player to be able to see the state of the game without knowing the history of events
    So I kinda want the STATE of the game saved in some... struct that doesnt hide in some other data

    I want the state of the game to be fully explained by:

    Position of each piece
    Data associated with each piece
    The turn manager and whos turn it currently is, how much time they have left on their turn and in total
    PLUS THIS -> Some struct of rules that the game is currently operating on


    */

}




#[derive(Eq, PartialEq)]
pub struct VisibleCardDrawn{

    //the different names of the cards that can be drawn

    //there should be one of these for each "joker effect"



}