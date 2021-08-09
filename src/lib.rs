mod gameengine;

use std::collections::HashSet;
use std::collections::HashMap;

use gameengine::GameEngine;


mod objects;



mod inputs;
pub use inputs::GameObject;
use inputs::GameInput;


mod piecedata;


mod effects;
pub use effects::CardEffect;
pub use effects::GameEffects;



mod turnmanager;
use turnmanager::TurnManager;


mod boardsquarestructs;



use serde::{Serialize, Deserialize};


pub use rapier3d::na as nalgebra;


pub use gameengine::VisibleGameBoardObject;


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


    //only id and appearance
    //just click objects is the input
    pub boardobjects: Vec<VisibleGameBoardObject>,
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
    
    //the input of each player if they have an input queued
    queuedinputs: HashMap<u8, GameInput>,

    
    //if the game is finished, and who the winner is
    gameover: Option<u8>,
    
    //what players if any, have their actions performed automatically by an ai
    aiplayer: HashSet<u8>,

    //the tick number that the last action was performed
    sincelastaction: u32,

}



impl MainGame{
    
    
    fn default_game() -> MainGame{

        use log::Level;
        use log::info;
        console_log::init_with_level(Level::Debug);
        
        info!("it works");

        
        
        //create a new 2 player game without any pieces or effects
        MainGame{
            turnmanager: TurnManager::new_two_player(1, 2),            
            boardgame: GameEngine::new(1,2),
            queuedinputs: HashMap::new(),
            
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
        toreturn.apply_card_effect(&1, CardEffect::TurnsTimed(60));
        toreturn.apply_card_effect(&1, CardEffect::TurnsUntilDrawAvailable(10));
        
        toreturn.apply_card_effect(&1, CardEffect::PawnsPromoted);
        toreturn.apply_card_effect(&1, CardEffect::LossWithoutKing);    
        
        
        //player 2 is AI
        toreturn.aiplayer.insert(2);
        //toreturn.aiplayer.insert(1);
        
        toreturn
    }
    
}

use rapier3d::na::Point3;
use rapier3d::na::Vector3;



//THE METHODS REQUIRED FOR THE TICK METHOD
impl MainGame{ 
    

    
    //given the object that the user has selected
    //and a ray of where they clicked
    //return either the new thing they will have selected, or perform an action and return the string
    //they need to have something selected
    pub fn click(&mut self, playerid: u8, selected: Option<GameObject>, ray: (Point3<f32>, Vector3<f32>) ) -> (Option<GameObject>, Option<String> ){


        //if an object is clicked
        if let Some(clicked) = self.boardgame.get_object_intersection(ray){

            let gameinput = GameInput::new( selected, GameObject::BoardObject(clicked) );

            
            //if that input is valid
            if let Some(serialized) = self.receive_input( playerid, gameinput){
                log::info!("valid");

                return (None, Some(serialized));
            }

            //if that input isnt valid
            //set the selected object to the one clicked
            return ( Some( GameObject::BoardObject(clicked) ), None  );

        }
        //if no object is clicked, set selected to none
        else{
            return (None, None);
        }

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
    
    
    
    
    //perform an input and return if it is valid or not
    fn perform_input(&mut self, playerid: &u8, input: &GameInput) {
                
        self.sincelastaction = 0;

        if GameObject::Deck == input.clicked {

            let randomcardeffect = self.gameeffects.get_random_card_effect();
        
            self.apply_card_effect(playerid, randomcardeffect );
            
            self.apply_card_effect(playerid, CardEffect::TurnsUntilDrawAvailable(5)  );

        }
        
        if let Some(boardinput) = input.to_boardinput() {

            log::info!("is boardinput");
            
            self.boardgame.perform_boardinput( &boardinput );
        }
    }
    




    
    //get what pieces are captures in the game engine and remove them from here
    pub fn tick(&mut self){
        
        
        
        //get each player whos turn it currently is
        let currentturnplayers = self.turnmanager.get_current_players();
        
        
        for playerid in currentturnplayers.clone(){
            
            //if this player has a queued input
            if let Some(playerinput) = self.queuedinputs.get(&playerid).clone(){

                let playerinput = playerinput.clone();

                //if it performs successfully
                if  self.is_input_valid(&playerid, &playerinput) {

                    log::info!("input performed");

                    self.perform_input(&playerid, &playerinput);
                    
                    self.turnmanager.player_took_action(playerid);
                    
                    self.queuedinputs.remove(&playerid);
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
        
        
        
        
        use inputs::BoardInput;
        
        
        //for each player that is controlled by an AI
        for playerid in self.aiplayer.clone().iter(){
            
            //if its been 30 ticks since the last tick
            //and its this players turn
            if self.sincelastaction > 30{

                if self.turnmanager.get_current_players().contains(playerid){

                    if self.totalticks % 10 == 0{

                        let input = self.boardgame.get_best_fullaction_for_player(playerid);
                    
                        self.receive_input(*playerid, input.to_gameinput() );
                    }    
                }
            }
        }
        


        self.sincelastaction +=1;
        
    }
    
    
    
    
    //a method only for the server
    pub fn receive_string_input(&mut self, playerid: &u8, stringinput: String) -> Result<(), ()>{
        
        //try to convert to player input with bincode
        
        if let Ok(playerinput) = bincode::deserialize::<GameInput>(&bincode_string_to_bytes(&stringinput) ){
            
            self.receive_input(*playerid, playerinput);
            
            return Ok ( () );
        }
        
        return Err( () );
    }


    fn is_input_valid(&self, playerid: &u8, input: &GameInput) -> bool{

        if GameObject::Deck == input.clicked {
            if self.can_player_draw(playerid){
                return true;
            }
        }
        else if let Some(boardinput) = input.to_boardinput() {

            return self.boardgame.is_boardinput_valid(playerid, &boardinput );
        }

        return false;
    }


    //get the input that a player sends and set it to be performed next tick
    //return whether this input is valid for this player to have queued
    fn receive_input(&mut self, playerid: u8, input: GameInput) -> Option<String>{
        
        //get if the input is valid for this player
        if  self.is_input_valid(&playerid, &input ) {

            self.queuedinputs.insert(playerid, input.clone() );

            return Some( bincode_bytes_to_string( &bincode::serialize(&input).unwrap() ) );
        }
        
        log::info!("input isnt valid?");

        return None;
    }







    
    //get the state of the game
    //given the player id, and the object they have selected
    pub fn get_visible_game_state(&self, playerid: &u8, selected: &Option<GameObject>) -> VisibleGameState{
        
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



        let mut boardobjects = self.boardgame.get_visible_board_game_objects();


        let mut highlightedobjects = Vec::new();

        if let Some(GameObject::BoardObject(selected)) = selected{

            highlightedobjects.extend( self.boardgame.get_valid_targets( playerid, selected ) );
        }



        for object in boardobjects.iter_mut(){

            if let Some(GameObject::BoardObject(selected)) = selected{

                if object.id == *selected{

                    object.color = (0.2,4.0,0.3);
                }

                if highlightedobjects.contains(&object.id){

                    object.color = (3.0,3.0,0.0);
                }
            }
        }

        
        
        VisibleGameState{
            
            isgameover: self.gameover,
            
            turnsuntildrawavailable: self.gameeffects.get_turns_until_draw_available(),
            
            player1totalticksleft: self.turnmanager.get_players_total_ticks_left(1),
            
            player2totalticksleft: self.turnmanager.get_players_total_ticks_left(2),
            
            player1ticksleft: self.turnmanager.get_ticks_left_for_players_turn(1),
            
            player2ticksleft: self.turnmanager.get_ticks_left_for_players_turn(2),
            
            playerswithactiveturns: activeturnsandlength,
            
            boardobjects,
            
            gameeffects: self.gameeffects.clone(),
            
            lastcardeffect: lastcard,
        }
    }


}



impl MainGame{


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





    //can a player do a draw card action
    fn can_player_draw(& self, playerid: &u8) -> bool{
        self.gameeffects.is_draw_available()
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