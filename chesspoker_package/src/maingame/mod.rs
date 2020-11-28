mod gameengine;

use gameengine::GameEngine;


//export and make public the GameEngineState struct
pub use gameengine::GameEngineState;



use std::collections::HashSet;
use std::collections::HashMap;



mod datastructs;




//import the data structures needed

//make these public, and visible to the game interface
pub use datastructs::PlayerInput;
pub use datastructs::PieceAction;
pub use datastructs::Card;

use datastructs::CardEffect;
use datastructs::CardValue;
use datastructs::CardSuit;
//pub use datastructs::CardGame;

pub use datastructs::TurnManager;

use datastructs::direction_change_to_slide_id_from_objective_perspective;
use datastructs::slide_id_to_direction_change_from_objective_perspective;


pub use datastructs::PieceTypeData;




//the maingame creates and returns these objects as its fuctions





//extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}




pub struct MainGame{
    
    
    totalplayers: u8,
    
    //a map of objects to what type of object they are
    totalpieces: u32,
    
    //the total number of cards ever, used for setting ID
    totalcards: u16,
    
    
    
    //the list of players
    players: HashSet<u8>,
    
    
    //the pieces owned by each player
    playertopiece: HashMap<u8, HashSet<u32> >,
    
    //the direction the player i facing, of the 8 cardinal directions
    playertodirection: HashMap<u8, u8>,
    
    
    
    
    
    piecetypedata: HashMap<u32,PieceTypeData>,
    
    
    
    
    //the card struct by ID
    cards: HashMap<u16, Card>,
    
    //the cards each player has in their hand as a list
    playertocards: HashMap<u8, Vec<u16> >,
    
    
    
    //what players the turn currently is
    turnmanager: TurnManager,
    
    
    //the games running, and the players of that game
    physicalgameengine: GameEngine,
    
    
    //the games and the players of the games
    //with the id of player 1 and 2
    cardgame: Option<CardGame>,
    
    
    //the last input of each player
    queuedinputs: HashMap<u8, Option<PlayerInput>>,
    
    
    
    /*
    the boards
    
    the pieces on each board
    
    the cards on each board
    
    
    
    
    
    
    the turn manager
    */
    
    
    
    
}

impl MainGame{
    
    
    //the functions needed by the wasm
    pub fn get_all_piece_ids(&self) -> Vec<u32>{
        
        let mut toreturn = Vec::new();
        
        for (playerid, pieceidmap) in self.playertopiece.iter() {
            
            for pieceid in pieceidmap{
                
                toreturn.push(*pieceid);
                
            };
            
            
        };
        
        toreturn
        
        
        
    }
    pub fn get_all_board_square_ids(&self) -> Vec<(u32,u32)>{
        
        //assume, for important reasons
        //(because I shouldnt have any other or different board squares and i dont want to make)
        //(the function call a function, call a function, call a function, call a function)
        //that the board squares are just the normal 64 board squares of a normal chess board
        //and id'd the normal way
        
        let  mut toreturn = Vec::new();
        
        for x in 0..8{
            for y in 0..8{
                
                toreturn.push( (x,y) );
                
            };
        };
        
        
        toreturn
        
        
    }
    pub fn get_all_card_ids(&self) ->  Vec<u16> {
        
        let mut toreturn = Vec::new();
        
        for (playerid, pieceidmap) in self.playertocards.iter() {
            
            for pieceid in pieceidmap{
                
                toreturn.push(*pieceid);
                
            };
            
            
        };
        
        toreturn
        
    }
    
    
    
    
    pub fn get_piece_position(&self, pieceid: &u32) -> (f32,f32,f32){
        
        self.physicalgameengine.get_piece_translation(pieceid)
        
    } 
    pub fn get_piece_rotation(&self, pieceid: &u32) -> (f32,f32,f32){
        
        self.physicalgameengine.get_piece_rotation(pieceid)
    }
    pub fn get_board_square_translation(&self, boardsquareid:& (u32,u32)) -> (f32,f32,f32){
        
        self.physicalgameengine.get_board_square_translation( boardsquareid)
        
        
    }
    pub fn get_board_square_rotation(&self, boardsquareid:& (u32,u32)) -> (f32,f32,f32){
        
        self.physicalgameengine.get_board_square_rotation( boardsquareid)
        
        
    }
    
    
    //the fuctions for reading & writing the state used by the game interface to turn this game into
    pub fn get_queuedinputs(&self) ->  HashMap<u8, Option<PlayerInput>>{
        self.queuedinputs.clone()
    }
    pub fn set_queuedinputs(&mut self, queuedinputs: HashMap<u8, Option<PlayerInput>>){
        
        self.queuedinputs = queuedinputs;
    }
    pub fn get_totalplayers(&self) -> u8{
        self.totalplayers.clone()
    }
    pub fn set_totalplayers(&mut self, totalplayers: u8){
        self.totalplayers = totalplayers;
    }
    pub fn get_totalpieces(&self) -> u32{
        self.totalpieces.clone()
    }
    pub fn set_totalpieces(&mut self, totalpieces: u32){
        self.totalpieces = totalpieces;
    }
    pub fn get_players(&self)-> HashSet<u8>{
        self.players.clone()
    }
    pub fn set_players(&mut self, players: HashSet<u8>){
        self.players = players;
    }
    pub fn get_playertopiece(&self) -> HashMap<u8, HashSet<u32> >{
        
        self.playertopiece.clone()
    }
    pub fn set_playertopiece(&mut self, playertopiece: HashMap<u8, HashSet<u32> >){
        
        self.playertopiece = playertopiece;
        
        
    }
    pub fn get_piecetypedata(&self) -> HashMap<u32, PieceTypeData>{
        self.piecetypedata.clone()
    }
    pub fn set_piecetypedata(&mut self, piecetypedata: HashMap<u32, PieceTypeData>){
        self.piecetypedata = piecetypedata;
    }   
    pub fn get_cards(&self) -> HashMap<u16, Card>{
        self.cards.clone()
    }
    pub fn set_cards(&mut self, cards: HashMap<u16, Card>){
        
        self.cards = cards;
        
    }
    pub fn get_playertocards(&self) -> HashMap<u8, Vec<u16> >{
        
        self.playertocards.clone()
    }
    pub fn set_playertocards(&mut self, playertocards: HashMap<u8, Vec<u16> >){
        
        self.playertocards = playertocards;
    }
    pub fn get_turnmanager(&self) -> TurnManager{
        self.turnmanager.clone()
    }
    pub fn set_turnmanager(&mut self, turnmanager: TurnManager){
        self.turnmanager = turnmanager;
    }
    pub fn get_blackjackgame(&self) -> Option<CardGame>{
        
        self.cardgame.clone()
    }
    pub fn set_blackjackgame(&mut self, blackjackgame: Option<CardGame>){
        
        self.cardgame = blackjackgame;
    }
    
    //get the physics engine state as a "GameEngineState"
    pub fn get_game_engine_state(&self) -> GameEngineState{
        
        self.physicalgameengine.get_game_engine_state()
    }
    pub fn set_game_engine_state(&mut self, data: GameEngineState){
        
        self.physicalgameengine.set_game_engine_state(data);
    }
    
    


    //get the id of the board square a piece is on if its on that board square and none if its not
    pub fn get_board_square_piece_is_on(&self, pieceid: &u32) -> Option<(u32,u32)>{
        
        return self.physicalgameengine.get_board_square_piece_is_on(pieceid);
        
    }
    
    
    //get if a piece is allowed to move to a certain 
    pub fn get_board_squares_reachable_by_piece(&self, pieceid: &u32) -> HashSet<(u32,u32)>{
        
        let mut toreturn = HashSet::new();
        
        //get  the slide and lift actions allowed by the piece
        let actionsallowed = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);
        
        //for each action allowed, get the board square it would take the piece, and add it to return
        for action in actionsallowed{
            
            let boardsquareid = self.get_square_that_action_takes_piece(pieceid, action);
            
            toreturn.insert(boardsquareid);
        }
        
        
        //if its not on any board square, its handled by the "get_slide_and_lift_actions" function by returnign empty vec
        toreturn
        
    }
    
    
    
    pub fn get_pieces_on_board_square(&self, boardsquareid: &(u32, u32)) -> HashSet<u32>{
        
        self.physicalgameengine.get_pieces_on_board_square(boardsquareid)
        
    }
    
    pub fn get_if_piece_can_be_flicked(&self, pieceid: &u32) -> bool{
        
        true
        
    }
    

    pub fn get_pieces_and_squares_actable_by_card(&self, cardid: &u16) -> ( Vec<u32>, Vec<(u8,u8)> ){

        let mut boardsquaresallowed = Vec::new();
        let mut piecesallowed = Vec::new();


        //get this card
        if let Some(card) = self.cards.get(cardid){

            //if its a drop effect 
            if CardEffect::dropsquare == card.effect{
                
                //add every boardsquare to the list of board squares allowed
                for x in 0..8{
                    for y in 0..8{
                        boardsquaresallowed.push( (x,y) );
                    }
                }

            }

            //if its a raise and lift effect
            else if CardEffect::raisesquare == card.effect{
                
                //add every boardsquare to the list of board squares allowed
                for x in 0..8{
                    for y in 0..8{
                        boardsquaresallowed.push( (x,y) );
                    }
                }

            }
            


        }



        return (piecesallowed, boardsquaresallowed);
        


    }

    
    //get the card from the perspective of the player
    pub fn get_card(&self, cardid: &u16, playerid: &u8) -> Option<Card>{
        
        //if the card is owned by the player, return the card
        if self.playertocards.get(playerid).unwrap().contains(cardid){
            
            if let Some(card) = self.cards.get(cardid){
                
                return Some (card.clone()) ;
            }
            else{
                return None ;
            }
            
            
        }
        //otherwise return an unknown card
        else{
            return None;
        }
        
        
        
    }
    
    
    
    
    //create a game with two players
    pub fn new_two_player() -> MainGame{
        
        
        //create a new 2 player game        
        let mut toreturn = MainGame{
            
            cards: HashMap::new(),
            
            playertocards: HashMap::new(),
            
            totalplayers: 0,
            totalpieces: 0,
            totalcards: 0,
            players: HashSet::new(),
            playertodirection: HashMap::new(),
            playertopiece: HashMap::new(),
            piecetypedata: HashMap::new(),
            turnmanager: TurnManager::new_two_player(1, 2),            
            physicalgameengine: GameEngine::new(),
            queuedinputs: HashMap::new(),
            
            cardgame: None,
            
        };
        
        
        //add two players
        toreturn.add_player();
        toreturn.add_player();
        
        toreturn.queuedinputs.insert(1, None);
        toreturn.queuedinputs.insert(2, None);
        
        
        
        //add the standard configuration of chess pieces
        //owned by each player
        toreturn.initialize_pieces();
        
        
        toreturn.start_poker_game(1, 2);
        
        
        toreturn
        
    }
    
    //add a player
    fn add_player(&mut self){
        
        //the number of players starts counting at 1
        //so the first players id is 1 not 0
        let currentplayer = self.totalplayers + 1;
        
        self.players.insert(currentplayer);
        
        if currentplayer == 1{
            
            //player 1 faces direction "0"
            self.playertodirection.insert(1, 0);
            
        }
        else if currentplayer == 2{
            
            //player 2 faces direction "4"
            self.playertodirection.insert(2, 4);
            
        }
        else{
            panic!("not implemented for anything other than 2 players and probably never will be");
        }
        
        
        
        self.playertopiece.insert(currentplayer, HashSet::new());
        
        
        self.playertocards.insert(currentplayer, Vec::new());
        
        //give that player a random card
        self.give_new_random_card(currentplayer);
        self.give_new_random_card(currentplayer);
        self.give_new_random_card(currentplayer);
        self.give_new_random_card(currentplayer);
        

        
        self.totalplayers += 1;
        
    }
    
    //add a piece
    //specify which player
    fn add_piece(&mut self, playerid: u8, position:(u32,u32)) -> u32{
        
        //println!("I ADDED PIECE, TESTING LOGGIN");
        let pieceid = self.totalpieces;
        
        
        //set this piece as a Queen
        self.piecetypedata.insert( pieceid, PieceTypeData::new() );
        
        
        //set this piece to be owned by that player
        
        self.playertopiece.get_mut( &playerid).unwrap().insert(pieceid);
        //self.playertopiece.insert( playerid, pieceid);
        
        //add the piece in the game engine
        //at square (3,3), and with shapeid  of 0
        self.physicalgameengine.add_piece( pieceid, (position.0, position.1), 0);
        
        
        self.totalpieces += 1;
        
        return(pieceid);
        
    }
    
    //generate a new card and give it to this player
    //give this player a random card
    fn give_new_random_card(&mut self, playerid: u8){
        
        let cardid = self.totalcards;
        self.totalcards += 1;
        
        let thecard = Card::new_random_card();
        
        //put it into the list of cards
        self.cards.insert(cardid, thecard);
        
        self.playertocards.get_mut(&playerid).unwrap().push(cardid);
        
        
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
                
                let owns = self.playertocards.get(&playerid).unwrap().contains(&cardid);
                if owns == false{
                    return false;
                }
            }
            
            if let PlayerInput::pieceaction(pieceid, _ ) = input.clone(){
                
                let owns = self.playertopiece.get(&playerid).unwrap().contains(&pieceid);
                if owns == false{
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
            return self.is_piece_action_valid( &playerid, &pieceid, &pieceaction);
        }
        
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
        
    }
    
    
    //can this card be played alone
    fn is_play_card_on_board_action_valid(&self, playerid: &u8, cardid: &u16) -> bool{
        
        
        //if there is a card game going on
        if let Some(cardgame) = &self.cardgame{
            
            //if a card game allows this player to play a card
            let canplayerplaycard = cardgame.is_player_allowed_to_play_card(*playerid);
            

            //return if the player is allowed to play a card on the active card game
            return canplayerplaycard;
            
        }
        else{
            
            
            //get if this card has an effect that can be played on the board
            //get the cards effect
            let cardeffect = self.cards.get(cardid).unwrap().effect.clone();
            
            
            //if the card effect is to make a blackjack game
            if cardeffect == CardEffect::blackjackgame{
                
                return true;
            }
            //if the card effect is to make a poker game
            if cardeffect == CardEffect::pokergame{
                
                return true;
            }
            
            
            //and other effect isnt valid to be played on a board game, so return false
            return false;
        }

        
        
    }
    
    //if this card can be played on this piece 
    fn is_play_card_on_piece_action_valid(&self, playerid: &u8, cardid: &u16, pieceid: &u32) -> bool{
        
        //if a card game requires this player to play a card
        let doesplayerhavetoplaycard = self.is_player_forced_to_play_card_on_cardgame(playerid);
        
        //if its true, return false
        if doesplayerhavetoplaycard {
            return false;
        }
        
        
        
        
        //get the effect of the card
        
        //get if this card has an effect that can be played this a piece
        let cardeffect = self.cards.get(cardid).unwrap().effect.clone();
        
        
        
        
        
        return false;
        
        
        
    }
    
    //if this card can be played on this boardsquare
    fn is_play_card_on_square_action_valid(&self, playerid: &u8, cardid: &u16, boardsquareid: &(u8,u8) ) -> bool{
        
        //if a card game requires this player to play a card
        let doesplayerhavetoplaycard = self.is_player_forced_to_play_card_on_cardgame(playerid);
        
        //if its true, return false
        if doesplayerhavetoplaycard {
            return false;
        }
        
        
        
        //get the effect of the card
        
        //get if this card has an effect that can be played on a board square
        let cardeffect = self.cards.get(cardid).unwrap().effect.clone();
        
        
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
    fn is_piece_action_valid(&self, playerid: &u8, pieceid: &u32,  pieceaction: &PieceAction) -> bool{
        
        //if a card game requires this player to play a card
        let doesplayerhavetoplaycard = self.is_player_forced_to_play_card_on_cardgame(playerid);
        
        //if its true, return false
        if doesplayerhavetoplaycard {
            return false;
        }
        
        
        
        
        //if the piece action is a slide or lift action
        if  let PieceAction::slide(_,_) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);
            
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
            let allowedactions = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return(true);                
            }
            else{
                return(false);
            }
            
        }
        else if let PieceAction::flick(direction, force) = pieceaction{
            
            //get its piece data
            let piecetypedata = self.piecetypedata.get(pieceid).unwrap();
            
            
            //get if this piece can perform this flick
            let canperformflick = piecetypedata.get_if_this_flick_allowed(*direction, *force);
            
            
            //and return it
            return(canperformflick);
            
        }
        
        panic!(" dont know what kind of mission this is..");
        
        
    }
    
    
    
    
    //get if a card game exists and if this player is forced to play a card on a 
    fn is_player_forced_to_play_card_on_cardgame(&self, playerid: &u8) -> bool {
        
        
        //if there is an active card game
        if let Some(cardgame) = &self.cardgame{
            
            //if the card game allows this player to play a card
            if cardgame.is_player_allowed_to_play_card(*playerid){
                
                return true;
            }
            
        };
        
        
        //if there isnt an active cardgame that requires this player to play a card, return false
        return false;
        
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //set the type of piece a piece is
    //what actions it can perform
    //what it looks like
    fn set_piece_type(&mut self, pieceid: &u32, typeid: &u32){
        
        /*
        //1 pawn
        //2 knight
        //3 bishop
        //4 rook
        //5 queen
        //6 king
        OTHER: "all"
        */
        
        let mut piecetypedata = self.piecetypedata.get_mut(pieceid).expect("why doesnt htis piece ID have a piecetype associated?");
        
        
        //set the piece type data according to this 
        
        if typeid == &1{
            piecetypedata.set_pawn();
        }
        else if typeid == &2{
            piecetypedata.set_knight();
        }
        else if typeid == &3{
            piecetypedata.set_bishop();
        }
        else if typeid == &4{
            piecetypedata.set_rook();
        }
        else if typeid == &5{
            
            piecetypedata.set_queen();
        }
        else if typeid == &6{
            piecetypedata.set_king();
        }
        
        
    }
    
    //create all the pieces for both players
    fn initialize_pieces(&mut self){
        
        
        //add all the pawns
        for x in 0..8{
            
            for y in 1..2{
                
                let player1pawnid = self.add_piece(1,  (x,y) );
                self.set_piece_type(&player1pawnid, &1);
                
                let player2pawnid = self.add_piece(2,  (x,y+5) );
                self.set_piece_type(&player2pawnid, &1);
            }
            
            
        }
        
        //add the rooks
        let rook = self.add_piece(1, (0,0) );
        self.set_piece_type(&rook, &4);
        let rook = self.add_piece(1, (7,0) );
        self.set_piece_type(&rook, &4);
        let rook = self.add_piece(2, (0,7) );
        self.set_piece_type(&rook, &4);
        let rook = self.add_piece(2, (7,7) );
        self.set_piece_type(&rook, &4);
        
        
        //add the knights
        let piece = self.add_piece(1, (1,0) );
        self.set_piece_type(&piece, &2);
        let piece = self.add_piece(1, (6,0) );
        self.set_piece_type(&piece, &2);
        let piece = self.add_piece(2, (1,7) );
        self.set_piece_type(&piece, &2);
        let piece = self.add_piece(2, (6,7) );
        self.set_piece_type(&piece, &2);
        
        
        //add the bishops
        let piece = self.add_piece(1, (2,0) );
        self.set_piece_type(&piece, &3);
        let piece = self.add_piece(1, (5,0) );
        self.set_piece_type(&piece, &3);
        let piece = self.add_piece(2, (2,7) );
        self.set_piece_type(&piece, &3);
        let piece = self.add_piece(2, (5,7) );
        self.set_piece_type(&piece, &3);
        
        
        //add the queens
        let piece = self.add_piece(1, (3,0) );
        self.set_piece_type(&piece, &5);
        let piece = self.add_piece(2, (3,7) );
        self.set_piece_type(&piece, &5);
        
        
        
        //add the kings
        let piece = self.add_piece(1, (4,0) );
        self.set_piece_type(&piece, &6);
        let piece = self.add_piece(2, (4,7) );
        self.set_piece_type(&piece, &6);
        
        
    }
    
    //get the list of pieceactions allowed by the specified piece (not just what the struct says are allowed, but all the positions to move)
    pub fn get_slide_and_lift_actions_allowed_for_piece(&self, pieceid: &u32) -> Vec<PieceAction>{
        
        let mut toreturn: Vec<PieceAction> = Vec::new();
        
        
        
        //get the piece data of this piece
        let piecetypedata = self.piecetypedata.get(pieceid).unwrap();
        
        
        
        
        //get the board square the piece is on
        let maybeboardsquareid = self.get_board_square_piece_is_on(pieceid);
        
        //if its not on a board square its not allowed to take any actions
        if maybeboardsquareid == None{
            return Vec::new();
        };
        
        let boardsquareid = maybeboardsquareid.unwrap();
        
        //get the owner of this piece
        let ownerofpiece = self.get_owner_of_piece(pieceid);
        
        //get the facing direction of the owner
        let ownerdirection = self.playertodirection.get(&ownerofpiece).unwrap();
        
        
        //get the slide actions
        let slide_actions = piecetypedata.get_allowed_slide_actions(ownerdirection);
        
        //get the lift and move actions
        let lift_and_move_actions = piecetypedata.get_allowed_lift_and_move(ownerdirection);
        
        
        
        //for each direction its allowed to slide
        for (direction, maxdistance, hastocapture, cancapture) in slide_actions.iter(){
            
            let mut currentboardsquare = (boardsquareid.0 as i32, boardsquareid.1 as i32);
            
            //get the change in position every step from the direction
            let (xstep, zstep) = slide_id_to_direction_change_from_objective_perspective(*direction);
            
            
            for stepnumber in 1..*maxdistance+1{
                
                //step in the direction from the current position
                currentboardsquare.0 += xstep;
                currentboardsquare.1 += zstep;
                
                
                //if the board square gets out of range, break immediately
                if currentboardsquare.0 < 0 || currentboardsquare.0 > 7{
                    break;
                }
                if currentboardsquare.1 < 0 || currentboardsquare.1 > 7{
                    break;
                }
                
                
                let currentboardsquareid = (currentboardsquare.0 as u32, currentboardsquare.1 as u32);
                
                let piecesonboardsquare = self.physicalgameengine.get_pieces_on_board_square(&currentboardsquareid);
                
                //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                let mut onlyopponentspieces = true;
                for otherpieceid in piecesonboardsquare.iter(){
                    
                    //if this piece is owned by the same player that owns the "pieceid" entered
                    //set "onlyopponentspieces" to false
                    let ownerofotherpiece = self.get_owner_of_piece(&otherpieceid);
                    
                    if ownerofotherpiece == ownerofpiece{   
                        onlyopponentspieces = false;
                    }
                    
                }
                
                
                
                //if this is an empty board square, and im not forced to capture to move, add this
                if piecesonboardsquare.is_empty(){
                    
                    if ! hastocapture{
                        
                        let action_to_slide_here = PieceAction::slide(*direction, stepnumber);
                        
                        toreturn.push(action_to_slide_here);
                        
                    }
                    
                    
                }
                else{
                    
                    //if this square has a piece and only has opponents pieces, and im allowed to capture, add this
                    if onlyopponentspieces{
                        
                        if *cancapture{
                            let action_to_slide_here = PieceAction::slide(*direction, stepnumber);
                            
                            toreturn.push(action_to_slide_here);
                            
                            
                        }
                    }
                }
                
                
                
                
                //if there is a piece on this board square break and end after this loop
                
                if ! piecesonboardsquare.is_empty() {
                    
                    break;
                    
                }
                
                
                
                
            }
            
            
            
        }
        
        
        
        //for each position it can be lifted and moved to
        for (currelativeposition, hastocapture, cancapture ) in lift_and_move_actions.iter(){
            
            //the position of the piece + the direction this move wants to send it
            let currentboardsquare = (currelativeposition.0 + boardsquareid.0 as i8, currelativeposition.1 + boardsquareid.1 as i8);
            
            //if the board square considered is out of range, dont add it
            if currentboardsquare.0 < 0 || currentboardsquare.0 > 7{
                
            }
            else if currentboardsquare.1 < 0 || currentboardsquare.1 > 7{
                
            }
            else{
                
                
                let currentboardsquareid = (currentboardsquare.0 as u32, currentboardsquare.1 as u32);
                
                
                //if this board square does not have any of my pieces on it
                let piecesonboardsquare = self.physicalgameengine.get_pieces_on_board_square(& currentboardsquareid);
                
                //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                let mut onlyopponentspieces = true;
                
                for otherpieceid in piecesonboardsquare.iter(){
                    
                    let ownerofotherpiece = self.get_owner_of_piece(&otherpieceid);
                    if ownerofotherpiece == ownerofpiece{   
                        onlyopponentspieces = false;
                    }
                    
                }
                
                
                //if this is an empty board square, and im not forced to capture to move, add this
                if piecesonboardsquare.is_empty(){
                    
                    if ! hastocapture{
                        
                        let lift_action_to_get_here = PieceAction::liftandmove( (currelativeposition.0 as i32, currelativeposition.1 as i32) );
                        
                        toreturn.push(lift_action_to_get_here);
                        
                    }
                    
                    
                }
                else{
                    
                    //if this square has a piece and only has opponents pieces, and im allowed to capture, add this
                    if onlyopponentspieces{
                        
                        if *cancapture{
                            
                            let lift_action_to_get_here = PieceAction::liftandmove( (currelativeposition.0 as i32, currelativeposition.1 as i32) );
                            
                            toreturn.push(lift_action_to_get_here);
                            
                        }
                        
                    }
                    
                }
                
            }
            
            
            
        }
        
        
        //if it can castle
        
        
        
        
        
        toreturn
        
        
        
    }
    
    //assume the action is valid, and the piece is on a board square
    //try not to call this method any tick after the "get actions allowed for piece" is called
    //get the id of the board square that this action will take this piece
    pub fn get_square_that_action_takes_piece(&self, pieceid: &u32, pieceaction: PieceAction) -> (u32,u32){
        
        //get the board square id this piece is on
        let boardsquareid = self.get_board_square_piece_is_on(pieceid).unwrap();
        
        let mut boardsquarepos = (boardsquareid.0 as i32, boardsquareid.1 as i32);
        
        
        //if its a slide action
        if let PieceAction::slide(direction, distance) = pieceaction{
            
            let (xdiff, zdiff) = slide_id_to_direction_change_from_objective_perspective(direction);
            
            let distance = distance as i32;
            
            let xdiff = xdiff * distance;
            let zdiff = zdiff * distance;
            
            boardsquarepos.0 += xdiff;
            boardsquarepos.1 += zdiff;
            
        };
        
        
        //if its a lift and move action
        if let PieceAction::liftandmove( (xdiff, zdiff) ) = pieceaction{
            
            boardsquarepos.0 += xdiff;
            boardsquarepos.1 += zdiff;
            
        }
        
        
        //make sure its within range
        //which it SHOULD BE if this method is called appropriately (and I dont have errors)
        //(and i cant get panic messages when running as wasm.... fuck)
        
        if boardsquarepos.0 < 0 || boardsquarepos.0 > 7{
            panic!("board square not within range");
        }
        if boardsquarepos.1 < 0 || boardsquarepos.1 > 7{
            panic!("board square not within range");
        }
        
        
        
        return  ( boardsquarepos.0 as u32, boardsquarepos.1 as u32 )   ;
        
    }
    
    //perform an input that is valid, and it is the turn of the player
    fn perform_input(&mut self, playerinput: &PlayerInput) {
        
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = playerinput {
            
            
            if let PieceAction::liftandmove(relativeposition) = pieceaction{
                
                let relativeposition = (relativeposition.0 as f32, relativeposition.1 as f32);
                
                self.physicalgameengine.lift_and_move_piece_to(pieceid, relativeposition);
                
                
            }
            if let PieceAction::slide(slidedirection, slidedistance) = pieceaction{
                
                
                self.physicalgameengine.slide_piece(pieceid, slide_id_to_direction_change_from_objective_perspective(*slidedirection), *slidedistance );
                
                
            }
            if let PieceAction::flick(direction, force) = pieceaction{
                
                self.physicalgameengine.flick_piece(*pieceid, *direction, *force);
                
                
            }
            
            
        };
        
        
        //if the input is a card action
        if let PlayerInput::playcardonboard(cardid) = playerinput{


            
            //get the player whos hand the card is in
            let playerid = self.get_owner_of_card(cardid);
            let thecard  =  self.get_card(cardid, &playerid).clone();
            

            //take it out, and put it in the game under that players ownership
            let playershand = self.playertocards.get_mut(&playerid).unwrap();


            let mut removedcard: Option<Card> = None;

            //remove the card from the players hand
            playershand.retain(|cardidinhand| {
                
                let delete = {

                    removedcard = thecard.clone();
                    cardidinhand == cardid
                };

                !delete
            });


            //remove the card from the card id of the hand right?
            //i kinda dont need to
            //but it just uses up memory


            //put that card into the game
            if let Some(cardgame) = &mut self.cardgame{

                cardgame.play_card(playerid, removedcard.unwrap());

            }


            

            //self.cardgame.unwrap().playcard()
            
            
            
            
        };
        
        
        if let PlayerInput::playcardonpiece(cardid, pieceid) = playerinput{
            
            
        };
        
        
        if let PlayerInput::playcardonsquare(cardid, squareid) = playerinput{

            //set a long drop mission for that board square
            self.physicalgameengine.set_long_boardsquare_drop(50, *squareid);
            
            
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
                    
                    self.perform_input( &playerinput.clone());
                    
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
        self.physicalgameengine.tick();
        
    }
    
    
    
    
    
    
    
    
    
    
    
    
    //private functions
    fn get_owner_of_piece(&self, pieceid: &u32) -> u8{
        
        
        //go over every player and get if they own this piece
        //if none do, panic
        
        for (playerid, piecemap) in self.playertopiece.iter(){
            
            if piecemap.contains(pieceid){
                
                return *playerid ;
            }
            
        }
        
        panic!(" piece not found to be owned by any player");
        
        
    }
    
    
    pub fn get_owner_of_card(&self, cardid: &u16) -> u8{
        
        //for each player
        //get if it owns this card
        //if none do, panic
        
        for (playerid, cardidmap) in self.playertocards.iter(){
            
            if cardidmap.contains(cardid){
                
                return *playerid;
            }
        }
        
        
        
        panic!("card owner not found");
        
    }
    
    
    pub fn get_position_of_card_in_hand(&self, cardid: &u16) -> u32{
        
        //for each player
        //get if it owns this card
        //and return its position in the hand
        
        for (playerid, cardidmap) in self.playertocards.iter(){

            let mut cardpos = 0;
            
            for handcardid in cardidmap{


                if handcardid == cardid{

                    return cardpos;

                }


                cardpos += 1;

            }
        }
        
        
        
        panic!("card owner not found");
        
    }
    

    
    
    
    //if there is a card game, get the cards in it
    pub fn get_cards_in_game(&self) -> Option< (Vec<Card>, Vec<Card>, Vec<Card>) >{
        
        
        //if there is an active card game
        
        if let Some(cardgame) = &self.cardgame{
            
            
            let player1hand = cardgame.playerscards.get(&1).unwrap().clone();
            
            let river = cardgame.river.clone();
            
            let player2hand = cardgame.playerscards.get(&2).unwrap().clone();

            
            
            return  Some( ( player1hand, river, player2hand) ) ;
            
        }
        else{
            return(None);
        }
        
        
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //start a blackjack game with the given players
    fn start_blackjack_game(&mut self, player1: u8, player2:u8){
        
        self.cardgame = Some( CardGame::new_blackjack() );
        
    }
    
    
    //start a poker game with the given players
    fn start_poker_game(&mut self, player1: u8, player2:u8){
        
        
        
        //create the river to be passed in
        let mut river = Vec::new();
        
        for x in 0..5{
            river.push( Card::new_random_card() );
        }
        
        
        self.cardgame = Some( CardGame::new_poker(river)  );
        
    }
    
    
    
}












use serde::{Serialize, Deserialize};    


//a card game
#[derive(Clone, Serialize, Deserialize)]
pub struct CardGame{
    
    //the value of the cards in the hands of the players
    playerscards: HashMap<u8, Vec<Card>>,
    
    //the cards in the middle of the board
    //is left as empty if there is no river (like in black jack)
    river: Vec<Card>,
    
    //if there is a river
    //the number of cards in the river that are revealed to the player
    //revealed from lowest to highest
    revealedriver: u8,
    
    
    //have either of these players passed
    hasplayerpassed: HashMap<u8, bool>,
    
    
    //true if this is a blackjack game
    //false if this is a poker game
    blackjackorpoker: bool,
    
    
    //if the game is over
    ended: bool,    
}




impl CardGame{
    
    
    
    //a new game started with these players
    pub fn new_blackjack() -> CardGame{
        
        let mut hasplayerpassedmap = HashMap::new();
        hasplayerpassedmap.insert(1, false);
        hasplayerpassedmap.insert(2, false);
        
        
        let mut playerscards = HashMap::new();
        playerscards.insert(1, Vec::new());
        playerscards.insert(2, Vec::new());
        
        
        CardGame{
            
            playerscards: playerscards,
            
            river: Vec::new(),
            
            revealedriver: 0,
            
            hasplayerpassed: hasplayerpassedmap,
            
            blackjackorpoker: true,
            
            ended: false,
            
        }
        
        
    }
    
    //get a river of 5 cards
    pub fn new_poker(river: Vec<Card>) -> CardGame{
        
        
        if river.len() != 5{
            panic!("the river is the wrong size");
        }
        
        
        let mut hasplayerpassedmap = HashMap::new();
        hasplayerpassedmap.insert(1, false);
        hasplayerpassedmap.insert(2, false);
        
        
        let mut playerscards = HashMap::new();
        playerscards.insert(1, Vec::new());
        playerscards.insert(2, Vec::new());
        
        
        CardGame{
            
            playerscards: playerscards,
            
            river: river,
            
            revealedriver: 3,
            
            hasplayerpassed: hasplayerpassedmap,
            
            blackjackorpoker: false,
            
            ended: false,
            
        }
        
        
    }
    
    //used for displaying the cards
    //get the list of the cards ina  players hand from a certain players perspective
    pub fn get_cards_in_hand(&self, playerperspective: u8, requestedplayer: u8) -> Vec<Card>{
        
        self.playerscards.get(&requestedplayer).unwrap().clone()
        
    }
    
    
    //get the cards in the river the open information about it
    pub fn get_cards_in_river_appearance(&self) -> Vec<Card>{
        
        let mut toreturn = Vec::new();
        
        let mut cardnumber = 1;
        
        //the cards that are higher in position in the river than the position revealed
        //are returned as an "unknown" card
        for card in &self.river{
            
            //if the current card is larger than the amount of cards added to the river
            //add a mystery card instead of the revealed card
            if cardnumber > self.revealedriver{
                
                toreturn.push( Card::new_unknown_card() );
            }
            else{
                toreturn.push(card.clone());
            }
            
            cardnumber += 1;
        };
        
        
        return(toreturn);
        
        
    }
    
    
    
    //can this player play any cards now?
    pub fn is_player_allowed_to_play_card(&self, playerid: u8) -> bool{
        
        //first of all, if the game is over, its false
        if self.is_game_over() == true{
            
            return false;
            
        }
        
        //then if this player has passed before, its false
        if self.hasplayerpassed.get(&playerid).unwrap() == &true{
            return false;
        }
        
        
        //if this is blackjack, if the player already has 4 cards in their hand
        if self.blackjackorpoker == true{
            
            let playerhand = self.playerscards.get(&playerid).unwrap();
            
            if playerhand.len() >= 4{
                return false;
            }
            
        }
        //if this is poker, if the player already has 2 cards in their hand
        else{
            
            let playerhand = self.playerscards.get(&playerid).unwrap();
            
            if playerhand.len() >= 2{
                return false;
            }
            
        }
        
        
        //otherwise its true
        return true;
        
        
    }
    
    //does this player NEED to play a card?
    pub fn must_player_play_card(&self, playerid: u8) -> bool{
        
        
        
        //if this is blackjack and the player has less than 2 cards
        if self.blackjackorpoker == true{
            
            let playerhand = self.playerscards.get(&playerid).unwrap();
            
            if playerhand.len() < 2{
                return true;
            }
            
        }
        //if this is poker and the player has less than 2 cards
        else{
            
            let playerhand = self.playerscards.get(&playerid).unwrap();
            
            if playerhand.len() < 2{
                return true;
            }
            
        }
        
        
        
        //otherwise its false
        return false;
        
    }
    
    
    
    pub fn is_game_over(&self) -> bool{
        
        self.ended        
        
    }
    
    
    
    //a player plays this card
    //return the winner of the game and the cards they win if they won
    //or return nothing if 
    pub fn play_card(&mut self, playerid: u8, card: Card){
        
        self.playerscards.get_mut(&playerid).unwrap().push(card);
        
        
        //this should be the "tick" function kinda
        
        
        //if this is a blackjack game
        if self.blackjackorpoker{
            
            let mut endedblackjack = true;
            
            //if there any players with less than 2 cards in their hand the blackjack game isnt over
            //otherwise it is
            for (playerid, hand) in &self.playerscards{
                
                if hand.len() < 2{
                    
                    endedblackjack = false;
                }
            }
            
            
            //set the game as ended
            self.ended = endedblackjack;
            
            
        }
        //if this is a poker game
        else {
            
            //get the minimum amount of cards in a players hand
            let mut minimumcardsinhand = 10;
            
            for (playerid, hand) in &self.playerscards{
                
                if hand.len() < minimumcardsinhand{
                    minimumcardsinhand = hand.len();
                }
                
            };
            
            
            //if the minimum amount of cards in both players hands is 0, set 3 cards as revealed
            if minimumcardsinhand == 0{
                
                self.revealedriver = 3;
            }
            //if the minimum amount of cards in both players hands is 1, set 4 cards as revealed
            else if minimumcardsinhand == 1{
                
                self.revealedriver = 4;
            }
            //if the minimum amount of cards in both players hands is 2, set 5 cards as revealed and the game as ended
            else if minimumcardsinhand == 2{
                
                self.revealedriver = 5;
                self.ended = true
                
            }
            else{
                
                //if its 10, panic
                panic!("why am i not matching the minimum cards in hand correctly?");
                
            }
            
            
        }
        
        
        
    }
    
    
    //a player passes their turn
    pub fn player_passes(&mut self, playerid: u8){
        
        self.hasplayerpassed.insert(playerid, true);
        
    }    
    
    
    //if the game is ended, get the winner, and the cards they won
    fn get_winner_rewards(&mut self)-> Option<(u8, Vec<Card>)> {
        
        
        //first, if the game is not completed yet, return none
        if self.is_game_over() == false{
            
            
            return (None);
        }
        
        
        //if this is a blackjack game
        if self.blackjackorpoker == true{
            
            let mut highestblackjackscore = 0;
            let mut highestblackjackscoreholder = 0;
            
            let mut allcardswon: Vec<Card> = Vec::new();
            
            //for each player
            for (playerid, hand) in &self.playerscards{
                
                let playerscore = CardGame::evaluate_blackjack_hand(hand);
                
                if playerscore > highestblackjackscore{
                    
                    highestblackjackscore = playerscore;
                    highestblackjackscoreholder = *playerid;
                    
                }
                
                
                allcardswon.extend( hand.clone() );
                
            }
            
            
            //if the "highestblackjackscoreholder" is zero, nobody won
            //so return that player 1 won, but they dont get any cards
            //the cards are just trashed
            if (highestblackjackscore == 0){
                
                return Some( ( 1, Vec::new() ) ) ;
            }
            else{
                
                //else return the player with the highest score
                //and every card from every players hand
                return Some( (highestblackjackscoreholder, allcardswon) )  ;
                
            }
            
            
            
            
        }
        //if this is a poker game
        else{
            
            
            
            let mut highestpokerscore = 0;
            let mut highestpokerscoreholder = 0;
            
            let mut allcardswon: Vec<Card> = Vec::new();
            
            let river = self.river.clone();
            
            
            //for each player
            for (playerid, hand) in &self.playerscards{
                
                let playerscore = CardGame::evaluate_poker_hand(hand, &river);
                
                if playerscore > highestpokerscore{
                    
                    highestpokerscore = playerscore;
                    highestpokerscoreholder = *playerid;
                    
                }
                
                
                allcardswon.extend( hand.clone() );
                
            }
            
            
            //if the "highestblackjackscoreholder" is zero, nobody won
            //so return that player 1 won, but they dont get any cards
            //the cards are just trashed
            if (highestpokerscore == 0){
                
                return Some( ( 1, Vec::new() ) ) ;
            }
            else{
                
                //else return the player with the highest score
                //and every card from every players hand
                return Some( (highestpokerscoreholder, allcardswon) )  ;
                
            }
            
            
            
            
            
            
        };
        
        
        panic!("i shouldnt get here");
        
        
    }
    
    
    //get the value of the blackjack hand
    //return 0 for bust
    //21 for blackjack
    //the best value of the hand otherwise
    fn evaluate_blackjack_hand(hand: &Vec<Card>) -> u16 {
        
        //get total value of the cards that arent aces
        let mut loweracevalue = 0; 
        
        //get the number of aces
        let mut numberofaces = 0;
        
        
        for currentcard in hand{
            
            if ( currentcard.is_ace() ){
                
                numberofaces += 1;
                
                loweracevalue += 1;
            }
            
            else{
                
                loweracevalue += currentcard.blackjackvalue();
            }
            
        };
        
        
        loop{
            
            //get the value with the aces as their 1 value
            
            //if the value is greater than 11, this is the best value that there can be
            if (loweracevalue > 11){
                
                return(loweracevalue);
                
            }
            
            
            //if the number of expandable aces is greater than 0
            if (numberofaces > 0){
                
                loweracevalue += 10;
                
                numberofaces = numberofaces -1;
                
            }
            else{
                
                return(loweracevalue);
            }
            
            
            
        }
        
        
    }
    
    
    //evaluate a poker hand with a given river
    //and get its best value
    fn evaluate_poker_hand(hand: &Vec<Card>, river: &Vec<Card>) -> u32{
        
        
        10
        
    }
    
    
    
}



