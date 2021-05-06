mod gameengine;

use std::collections::HashSet;
use std::collections::HashMap;


use gameengine::GameEngine;

//pub use gameengine::GameEngine::is_board_game_object_square;

pub use gameengine::FullAction;


mod datastructs;
use datastructs::TurnManager;

pub use datastructs::CardEffect;
pub use datastructs::GameEffects;


use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerInput{
    
    //perform an action on a piece
    pieceaction(u16, FullAction),
    
    //draw card from the deck
    drawcard,
}







//the maingame creates and returns these objects as its fuctions
#[derive(Serialize, Deserialize)]
pub struct MainGame{
    
    
    //the board game engine
    boardgame: GameEngine,
    
    //the manager for who has a turn turn currently is
    turnmanager: TurnManager,
    
    //the effects on the game
    gameeffects: GameEffects,
    
    
    
    totalticks: u32,
    
    //the list of the last cards played and how many more ticks until that card has
    //been displayed long enough
    lastcardeffect: Vec<(CardEffect, i32)>,
    
    //the last input of each player
    queuedinputs: HashMap<u8, Option<PlayerInput>>,
    
    //if the game is finished, and who the winner is
    gameover: Option<u8>,
    
    
    //what players if any, have their actions performed automatically by an ai
    aiplayer: HashSet<u8>,


    //the tick number that the last action was performed
    sincelastaction: u32,
}


impl MainGame{
    
    
    fn default_game() -> MainGame{
        
        
        let mut queuedinputs = HashMap::new();
        queuedinputs.insert(1, None);
        queuedinputs.insert(2, None);
        
        
        //create a new 2 player game without any pieces or effects
        MainGame{
            turnmanager: TurnManager::new_two_player(1, 2),            
            boardgame: GameEngine::new(1,2),
            queuedinputs: queuedinputs,
            
            totalticks: 0,
            gameover: None,
            gameeffects: GameEffects::new(),
            lastcardeffect: Vec::new(),
            
            
            aiplayer: HashSet::new(),

            sincelastaction: 10000,
        }
        
    }
    
    
    //create a game with two players
    pub fn new_two_player(  ) -> MainGame{
        
        let mut toreturn = MainGame::default_game();
        
        
        toreturn.apply_card_effect(&1, CardEffect::AddChessPieces);
        toreturn.apply_card_effect(&1, CardEffect::TurnsUntilDrawAvailable(10));
        
        toreturn.apply_card_effect(&1, CardEffect::LossWithoutKing);
        toreturn.apply_card_effect(&1, CardEffect::PawnsPromoted);
        
        
        toreturn        
    }
    
    
    
    //this is what the client calls and the game that it defaults to when not receiving websockets from the server
    pub fn new_solo_game() -> MainGame{
        
        let mut toreturn = MainGame::default_game();
        
        
        toreturn.apply_card_effect(&1, CardEffect::AddChessPieces);
        //toreturn.apply_card_effect(&1, CardEffect::TurnsTimed(60));
        toreturn.apply_card_effect(&1, CardEffect::TurnsUntilDrawAvailable(10));
        
        toreturn.apply_card_effect(&1, CardEffect::PawnsPromoted);
        toreturn.apply_card_effect(&1, CardEffect::LossWithoutKing);    
        
        
        //player 1 is AI
        toreturn.aiplayer.insert(2);
        //toreturn.aiplayer.insert(1);
        
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
            let doubleturns = self.gameeffects.get_double_turns();
            let turnlength = self.gameeffects.get_turn_length();
            
            self.turnmanager.tick(doubleturns, turnlength );
        }  
        
        
        {   
            let arepawnspromoted = self.gameeffects.get_are_pawns_promoted();
            let arekingsreplaced = self.gameeffects.get_are_kings_replaced();
            
            //self.boardgame set raised and removed squares
            
            self.boardgame.tick(arekingsreplaced, arepawnspromoted);
        }
        
        
        
        self.apply_game_effects();
        
        
        
        
        self.set_if_game_is_over();
        
        
        
        
        if let Some( (effect, tickslefttodisplay) ) = &mut self.lastcardeffect.first_mut(){
            
            *tickslefttodisplay += -1;
        }
        
        if let Some( (effect, tickslefttodisplay) ) = &mut self.lastcardeffect.first().clone(){
            
            if tickslefttodisplay <= &0{
                self.lastcardeffect.remove(0);
            }
        }
        
        
        
        
        self.totalticks +=1;
        
        
        
        
        //if the game has been running for more than 1000 seconds (~16 minutes)
        if self.totalticks > 30000{
            panic!("Game has been over for long enough. Pod is going to be restarted now");
        }
        
        
        
        
        
        //for each player that is controlled by an AI
        for playerid in self.aiplayer.clone().iter(){
            
            //if its been 30 seconds since the last tick
            //and its this players turn
            if self.sincelastaction > 60{

                if self.turnmanager.get_current_players().contains(playerid){

                    if self.totalticks % 10 == 0{

                        let action = self.boardgame.get_best_fullaction_for_player(playerid);
                    
                        let input = PlayerInput::pieceaction( action.0, action.1);
                        
                        self.receive_input(*playerid, input);     
                    }    
                }
            }
        }


        self.sincelastaction +=1;
        
    }
    
    
    //apply the effects of the game
    fn apply_game_effects(&mut self){
        
        
        
        //if it is a new turn this tick
        if self.turnmanager.did_turn_change(){
            
            
            self.gameeffects.decrement_turns_until_draw_available();
            self.gameeffects.decrement_raised_and_dropped_squares();
            
            
            let raisedsquares = self.gameeffects.get_raised_squares();
            let droppedsquares = self.gameeffects.get_dropped_squares();
            
            self.boardgame.set_randomly_raised_squares(raisedsquares);
            self.boardgame.set_randomly_dropped_squares(droppedsquares);
            
            
            if self.gameeffects.get_knightified(){
                self.boardgame.knightify();
            }
            else{
                self.boardgame.unaugment_abilities();
            }
            
        }
        
        
        
    }
    
    
    
    fn set_if_game_is_over(&mut self){
        
        //if the game isnt already over
        if self.gameover.is_none(){
            
            if self.gameeffects.get_loss_without_king() == true{
                
                //if the player doesnt have a king
                //and neither player has drawn a card yet
                if ! self.boardgame.does_player_have_king(&1){
                    self.gameover = Some(2);
                }
                if ! self.boardgame.does_player_have_king(&2){
                    self.gameover = Some(1);
                }
            }
            
            
            if ! self.boardgame.does_player_have_pieces(&1){  self.gameover = Some(2);   }
            if ! self.boardgame.does_player_have_pieces(&2){  self.gameover = Some(1);   }
            
            
            //check if either player has no time left on their clock
            if self.turnmanager.get_players_total_ticks_left(1) == 0{   self.gameover = Some(2);  }
            if self.turnmanager.get_players_total_ticks_left(2) == 0{   self.gameover = Some(1);  }
            
        }
        
    }
    
    
    
    
    //check if input is valid rather than just if the action is
    //if the player is the one sending the request or some shit like that i guess
    fn is_input_valid(&self, playerid: &u8, input: &PlayerInput) -> bool{
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = input.clone(){
            
            return self.boardgame.is_action_allowed(&pieceaction, &pieceid);
            //return self.is_piece_action_valid( &(pieceid as u16), &pieceaction);
        }
        
        else if let PlayerInput::drawcard = input{
            return self.can_player_draw(playerid);
        }
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
    }
    
    
    
    
    
    fn apply_card_effect(&mut self, playerid: &u8, cardeffect: CardEffect){
        
        
        //set the last card effect if its any effect other than "turns until draw available"
        if let CardEffect::TurnsUntilDrawAvailable(_ ) = cardeffect{
        }
        else{
            self.lastcardeffect.push( (cardeffect.clone(), 100  ) );
        }
        
        
        /*
        self.gameeffects.remove_card_effect(CardEffect::LossWithoutKing);
        self.gameeffects.remove_card_effect(CardEffect::PawnsPromoted);
        */
        
        
        
        match cardeffect{
            
            CardEffect::MakePoolGame => {
                panic!("pool games not working");
            },
            CardEffect::BackToBackTurns 
            | CardEffect::TurnsTimed(_) 
            | CardEffect::TurnsUntilDrawAvailable(_) 
            | CardEffect::Knight 
            | CardEffect::LossWithoutKing  
            | CardEffect::PawnsPromoted
            | CardEffect::KingsReplaced
            => {
                self.gameeffects.set_card_effect(cardeffect);
            },
            CardEffect::HalveTimeLeft => {
                self.turnmanager.halve_time_left();
            },
            CardEffect::RaiseSquares(number) => {    
                self.gameeffects.set_card_effect(cardeffect);
                
                self.boardgame.set_randomly_raised_squares(  self.gameeffects.get_raised_squares()    );
            },
            CardEffect::RemoveSquares(number) => {    
                self.gameeffects.set_card_effect(cardeffect);
                
                self.boardgame.set_randomly_dropped_squares(  self.gameeffects.get_dropped_squares()   );
            },
            CardEffect::AddChessPieces => {
                self.boardgame.add_chess_pieces();
            },
            CardEffect::AddCheckersPieces => {
                self.gameeffects.remove_card_effect(CardEffect::PawnsPromoted);
                self.gameeffects.remove_card_effect(CardEffect::LossWithoutKing);
                
                self.boardgame.add_checkers_pieces();
            },
            CardEffect::SplitPieceIntoPawns =>{
                self.gameeffects.remove_card_effect(CardEffect::PawnsPromoted);
                self.gameeffects.remove_card_effect(CardEffect::LossWithoutKing);
                
                self.boardgame.split_highest_piece_into_pawns();
            },
            CardEffect::Checkerify =>{
                self.gameeffects.remove_card_effect(CardEffect::PawnsPromoted);
                self.gameeffects.remove_card_effect(CardEffect::LossWithoutKing);
                
                self.boardgame.checkerify();
            },
            CardEffect::Chessify =>{
                self.gameeffects.set_card_effect(CardEffect::PawnsPromoted);
                self.gameeffects.set_card_effect(CardEffect::LossWithoutKing);
                
                
                self.boardgame.chessify();
            },
            
            
            
        }
        
        
        
    }
    
    
    
    //perform an input that is valid, and it is the turn of the player
    fn perform_input(&mut self, playerid: &u8, playerinput: &PlayerInput) {
        
        
        self.sincelastaction = 0;

        if let PlayerInput::pieceaction(pieceid, pieceaction) = playerinput {
            
            self.boardgame.perform_action( *pieceid, pieceaction.clone() );
        }
        else if let PlayerInput::drawcard = playerinput{
            
            let randomcardeffect = self.gameeffects.get_random_card_effect();
            
            self.apply_card_effect(playerid, randomcardeffect );
            
            self.apply_card_effect(playerid, CardEffect::TurnsUntilDrawAvailable(5)  );
        }
        else{
            panic!("unhandled input to be performed {:?}", playerinput);
        }
        
    }
    
    
    //can a player do a draw card action
    fn can_player_draw(& self, playerid: &u8) -> bool{
        
        self.gameeffects.is_draw_available()
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
        
        //try to convert to player input with bincode
        
        if let Ok(playerinput) = bincode::deserialize::<PlayerInput>(&bincode_string_to_bytes(&stringinput) ){
            
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
            
            return Some( bincode_bytes_to_string( &bincode::serialize(&input).unwrap() ) );
            
        }
        else{
            
            return None;
        };
    }
    
    
    
    //the actions allowed by the piece and the objects it captures or lands on
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec< (FullAction, Vec<u16> ) >){
        
        self.boardgame.get_piece_valid_actions_and_targets(&pieceid)
    }
    
    
    
    
    pub fn is_object_selectable(&self, playerid: &u8, objectid: &u16) -> bool{
        
        return self.boardgame.does_player_own_object(playerid, objectid);
    }
    
    
    
    //get the state of the game
    pub fn get_visible_game_state(&self, playerid: &u8) -> VisibleGameState{
        
        
        
        let lastcard;
        
        if let Some( (lastcardeffect, _) ) = &self.lastcardeffect.first(){
            lastcard = Some( lastcardeffect.clone() );
        }
        else{
            lastcard = None;
        }
        
        
        let mut activeturnsandlength: HashMap<u8, u32> = HashMap::new();
        
        for activeplayer in self.turnmanager.get_current_players(){
            
            let ticksleft = self.turnmanager.get_ticks_left_for_players_turn(activeplayer);
            
            activeturnsandlength.insert( activeplayer, ticksleft );
        }
        
        
        
        VisibleGameState{
            
            isgameover: self.gameover,
            
            turnsuntildrawavailable: self.gameeffects.get_turns_until_draw_available(),
            
            player1totalticksleft: self.turnmanager.get_players_total_ticks_left(1),
            
            player2totalticksleft: self.turnmanager.get_players_total_ticks_left(2),
            
            player1ticksleft: self.turnmanager.get_ticks_left_for_players_turn(1),
            
            player2ticksleft: self.turnmanager.get_ticks_left_for_players_turn(2),
            
            playerswithactiveturns: activeturnsandlength,
            
            boardobjects: self.boardgame.get_visible_board_game(),
            
            gameeffects: self.gameeffects.clone(),
            
            lastcardeffect: lastcard,
        }
    }
}







pub use gameengine::VisibleGameBoardObject;
pub use gameengine::VisibleGameObjectType;



//the information the client needs to know at every frame to render it
//the information the client needs to render the current frame
pub struct VisibleGameState{
    
    //has either player won
    pub isgameover: Option<u8>,
    
    //the deck
    //whether the move is available
    pub turnsuntildrawavailable: Option<u32>,
    
    pub player1totalticksleft: u32,
    pub player2totalticksleft: u32,
    
    pub player1ticksleft: u32,
    pub player2ticksleft: u32,
    
    //the players whos turn it is right now
    //and how much ticks they have left for their turn
    pub playerswithactiveturns: HashMap<u8, u32>,
    
    //the effects currently applied to the game
    pub gameeffects: GameEffects,
    
    //the most recent card effect applied
    pub lastcardeffect: Option< CardEffect >,    
    
    pub boardobjects: Vec<VisibleGameBoardObject>,
}








fn bincode_bytes_to_string(bytes: &Vec<u8>) -> String{
    
    let vecofchar = bytes.iter().map(|b| *b as char).collect::<Vec<_>>();
    let string = vecofchar.iter().collect::<String>();
    
    return string;
}


fn bincode_string_to_bytes(string: &String) -> Vec<u8>{
    
    let vecofchar = string.chars().collect::<Vec<_>>();
    let gamebin = vecofchar.iter().map(|c| *c as u8).collect::<Vec<_>>();
    
    return gamebin ;
}