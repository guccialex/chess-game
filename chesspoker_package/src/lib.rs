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







//the maingame creates and returns these objects as its fuctions

pub struct MainGame{
    
    
    totalplayers: u8,
    
    //the list of players
    players: HashSet<u8>,
    
    
    
    //the board game engine
    boardgame: GameEngine,
    
    //the card interface
    cards: CardsInterface,
    
    
    
    
    //the manager for who has a turn turn currently is
    turnmanager: TurnManager,
    
    
    
    //the last input of each player
    queuedinputs: HashMap<u8, Option<PlayerInput>>,
    
    
    
}

impl MainGame{
    
    //create a game with two players
    pub fn new_two_player() -> MainGame{
        
        
        //create a new 2 player game        
        let mut toreturn = MainGame{
            
            cards: CardsInterface::new_two_player(),
            totalplayers: 0,
            players: HashSet::new(),
            turnmanager: TurnManager::new_two_player(1, 2),            
            boardgame: GameEngine::new(1,2),
            queuedinputs: HashMap::new(),
            
        };
        
        
        //add two players
        toreturn.add_player();
        toreturn.add_player();
        
        toreturn.queuedinputs.insert(1, None);
        toreturn.queuedinputs.insert(2, None);
        
        
        toreturn.start_poker_game(1, 2);
        
        
        toreturn
        
    }
    

    pub fn get_game_information_string(&self, playerid: u8) -> String{

        "somestring".to_string()

    }
    
    
    
    //card getter functions
    //get player 1 and 2s hand as a list of ids
    pub fn get_cards_in_hands_ids(&self) -> Vec<u16>{
        
        self.cards.get_all_card_ids()
    }
    pub fn get_card_by_id(&self, cardid: u16) -> Card{
        
        self.cards.get_card_unsafe(cardid)
    }
    pub fn get_card_owner(&self, cardid: u16) -> u8{

        if self.cards.does_player_own_card(1, cardid){
            return 1;
        }
        else if self.cards.does_player_own_card(2, cardid){
            return 2;
        }
        else{
            panic!("well apparently nobody owns the card");
        }
    }
    pub fn get_card_position_in_hand(&self, cardid: u16) -> u8{

        //get the owner of the card
        let owner = self.get_card_owner(cardid);

        //get the hand of that player
        let ownershand = self.cards.get_cards_in_hand(owner);

        //get the position of this card in the hand
        let mut curpos = 0;

        for curcardid in ownershand{

            if curcardid == cardid{
                break;
            }

            curpos += 1;
        }

        return curpos;
    }
    pub fn get_cards_in_game(&self) -> Option< (Vec<Card>, Vec<Card>, Vec<Card>) >{
        
        self.cards.get_cards_in_game()
    }
    



    pub fn get_board_game_object_ids(&self) -> Vec<u16>{
        self.boardgame.get_object_ids()
    }
    pub fn get_board_game_object_translation(&self, objectid: u16) -> (f32,f32,f32){
        self.boardgame.get_object_translation(objectid)
    }
    pub fn get_board_game_object_rotation(&self, objectid: u16) -> (f32,f32,f32){
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

    
    
    

    //get the objects on the board that that the card can interact with, and the associated input for it
    pub fn get_boardobject_actions_allowed_by_card(&self, playerid: u8, cardid: u16) -> HashMap<u16, PlayerInput> {
        
        let card = self.cards.get_card_unsafe(cardid);
        
        //get every possible input
        //then if its allowed
        
        let mut allowedinputs = HashMap::new();

        
        //if this card can drop or raise a square
        if card.effect == CardEffect::dropsquare || card.effect == CardEffect::raisesquare{
            
            //for every board square
            for boardsquareid in self.boardgame.get_squares(){

                let input = PlayerInput::playcardonsquare(cardid, boardsquareid);

                allowedinputs.insert( boardsquareid, input );
            }
        }
        
        
        return allowedinputs;
    }


    //the actions allowed by the piece and the objects it captures or lands on
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec<(PieceAction, Vec<u16> )>){
        
        //get the actions allowed by the piece
        //if the owner is allowed to perform piece actions right now
        self.boardgame.get_actions_allowed_by_piece(pieceid)
        
    }
    
    
    
    //get the input that a player sends and set it to be performed next tick
    //return whether this input is valid for this player to have queued
    pub fn receive_input(&mut self, playerid: u8, input: PlayerInput) -> bool{        
        
        
        
        //get if the input is valid for this player
        if  self.is_input_valid(playerid, &input ) {
            
            self.queuedinputs.insert(playerid, Some(input));
            
            return true ;
            
        }
        else{
            
            
            return false ;
        };
        
        
    }
    
    
    //get what pieces are captures in the game engine and remove them from here
    pub fn tick(&mut self){
        
        
        //get each player whos turn it currently is
        let currentturnplayers = self.turnmanager.get_current_players();
        
        
        
        
        for playerid in currentturnplayers.clone(){
            
            
            //if an action was taken
            let mut actionwastaken = false;
            
            
            
            //if this player has a queued input
            if let Some(playerinput) = self.queuedinputs.get(&playerid).unwrap(){
                
                
                //if its valid to perform it
                if self.is_input_valid(playerid, &playerinput){
                    
                    self.perform_input(&playerid, &playerinput.clone());
                    
                    actionwastaken = true;
                    
                }
                else{
                    actionwastaken = false;
                }
                
                
            }
            
            
            
            //if an action was taken, let the turnmanager know that that player took their turn
            if (actionwastaken){
                self.turnmanager.player_took_action(playerid);
                
                //and clear queud inputs
                self.queuedinputs.insert(playerid, None);
            }
            
            
            
        }
        
        
        
        //let the turn manager know that a tick has happeneds
        self.turnmanager.tick();
        
        //tick the physical game engine
        self.boardgame.tick();
        
    }
    
    
    
    
    
    
    
    
    
    //add a player
    fn add_player(&mut self){
        
        //the number of players starts counting at 1
        //so the first players id is 1 not 0
        let currentplayer = self.totalplayers + 1;
        
        self.players.insert(currentplayer);
        
        
        //give that player a random card
        self.cards.give_new_random_card(currentplayer);
        self.cards.give_new_random_card(currentplayer);
        self.cards.give_new_random_card(currentplayer);
        self.cards.give_new_random_card(currentplayer);
        
        
        
        self.totalplayers += 1;
        
    }
    
    
    
    
    
    
    //check if input is valid rather than just if the action is
    //if the player is the one sending the request or some shit like that i guess
    fn is_input_valid(&self, playerid: u8, input: &PlayerInput) -> bool{
        
        
        //if the player doesnt own the piece or the card its not valid
        //return false before proceeding
        {
            
            //if its a card action, what the id of it is
            let mut maybecardid: Option<u16> = None;
            
            //if its a play card alone action
            if let PlayerInput::playcardonboard(cardid) = input {
                maybecardid = Some(*cardid);
            }
            //if its a play card alone action
            else if let PlayerInput::playcardonpiece(cardid, pieceid) = input {
                maybecardid = Some(*cardid);
            }
            //if its a play card alone action
            else if let PlayerInput::playcardonsquare(cardid, boardsquareid) = input {
                maybecardid = Some(*cardid);
            }
            
            if let Some(cardid) = maybecardid {
                
                let owns = self.cards.does_player_own_card(playerid, cardid);
                
                if owns == false{
                    return false;
                }
            }
            
            if let PlayerInput::pieceaction(pieceid, _ ) = input.clone(){
                
                let owner = self.boardgame.get_owner_of_piece( (pieceid as u16) );
                if &owner == &playerid{
                    return false;
                }
            }
            
        }
        
        
        
        
        
        //if its a play card alone action
        if let PlayerInput::playcardonboard(cardid) = input {
            return self.is_play_card_on_board_action_valid(&playerid, cardid) ;
        }
        
        //if its a play card alone action
        else if let PlayerInput::playcardonpiece(cardid, pieceid) = input {
            return self.is_play_card_on_piece_action_valid(&playerid, cardid, pieceid)
        }
        
        //if its a play card alone action
        else if let PlayerInput::playcardonsquare(cardid, boardsquareid) = input {
            return self.is_play_card_on_square_action_valid(&playerid, cardid, boardsquareid)
        }
        
        //if its a piece action
        //get if its valid        
        else if let PlayerInput::pieceaction(pieceid, pieceaction) = input.clone(){
            return self.is_piece_action_valid( &playerid, &(pieceid as u16), &pieceaction);
        }
        
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
        
    }
    //can this card be played alone
    fn is_play_card_on_board_action_valid(&self, playerid: &u8, cardid: &u16) -> bool{
        
        
        if self.cards.is_player_allowed_to_play_card(*playerid) {
            return(true);
        }
        
        let cardeffect = self.cards.get_card_unsafe( *cardid).effect;
        
        
        //if the card effect is to make a blackjack game
        if cardeffect == CardEffect::blackjackgame{
            return true;
        }
        
        //if the card effect is to make a poker game
        if cardeffect == CardEffect::pokergame{
            return true;
        }
        
        
        return false;
        
    }
    //if this card can be played on this piece 
    fn is_play_card_on_piece_action_valid(&self, playerid: &u8, cardid: &u16, pieceid: &u16) -> bool{
        
        
        return false;
        
    }   
    //if this card can be played on this boardsquare
    fn is_play_card_on_square_action_valid(&self, playerid: &u8, cardid: &u16, boardsquareid: &u16 ) -> bool{
        
        
        //get if this card has an effect that can be played on a board square
        let cardeffect = self.cards.get_card_unsafe(*cardid).effect.clone();
        
        
        //if its a drops a board square
        if cardeffect == CardEffect::dropsquare{
            //see if thats a board square valid to be dropped
            return(true);
        }
        
        
        //if its trying to lift a board square
        if cardeffect == CardEffect::raisesquare{
            //see if thats a board square valid to be raised
            return(true);
        }
        
        
        
        true
        
    }
    //only called when the player is the one who owns the piece
    fn is_piece_action_valid(&self, playerid: &u8, pieceid: &u16,  pieceaction: &PieceAction) -> bool{
        
        
        //if the piece action is a slide or lift action
        if  let PieceAction::slide(_,_) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.boardgame.get_slide_and_lift_actions_allowed_for_piece(*pieceid);
            
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
            let allowedactions = self.boardgame.get_slide_and_lift_actions_allowed_for_piece(*pieceid);
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return(true);                
            }
            else{
                return(false);
            }
            
        }
        else if let PieceAction::flick(direction, force) = pieceaction{            
            
            //and return it
            return true;
            
        }
        
        panic!(" dont know what kind of mission this is..");
        
        
    }
    



    //perform an input that is valid, and it is the turn of the player
    fn perform_input(&mut self, playerid: &u8 ,playerinput: &PlayerInput) {
        
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = playerinput {
            
            
            if let PieceAction::liftandmove(relativeposition) = pieceaction{
                
                
                
            }
            else if let PieceAction::slide(slidedirection, slidedistance) = pieceaction{
                
                
                
                
            }
            else if let PieceAction::flick(direction, force) = pieceaction{
                
                
                
            }
            
            
        }
        
        //or if the input is a card action
        else if let PlayerInput::playcardonboard(cardid) = playerinput{
            
            
        }
        
        else if let PlayerInput::playcardonpiece(cardid, pieceid) = playerinput{
            
            
        }
        
        
        else if let PlayerInput::playcardonsquare(cardid, squareid) = playerinput{
            
        };
        
        
        
    }
    
    //start a blackjack game with the given players
    fn start_blackjack_game(&mut self, player1: u8, player2:u8){
        
        self.cards.start_blackjack_game(player1, player2);
    }
    
    //start a poker game with the given players
    fn start_poker_game(&mut self, player1: u8, player2:u8){
        
        self.cards.start_poker_game(player1, player2);
        
    }
    
    
    
    
}











use serde::{Serialize, Deserialize};



//a request for how the client wants to join a game
#[derive(Serialize, Deserialize)]
pub enum GameToConnectTo{
    
    
    joinpublicgame,
    
    joinprivategame(u32),
    
    createprivategame,
    
    
}


//the message sent when a client is connected to a game on the server
//and the game is active
#[derive(Serialize, Deserialize)]
pub struct ConnectedToGame{
    
    //what is your player id in the game
    playerid: u32,
    
    
}

impl ConnectedToGame{
    
    pub fn new(playerid: u32) -> ConnectedToGame{
        
        ConnectedToGame{
            
            playerid: playerid
        }
        
        
    }
    
    
}









