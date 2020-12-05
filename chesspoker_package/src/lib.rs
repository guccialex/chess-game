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

use datastructs::GameSettings;


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
    
    gamesettings: GameSettings,
    
    
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
            gamesettings: GameSettings::new(),
            
        };
        
        
        //add two players
        toreturn.add_player();
        toreturn.add_player();
        
        toreturn.queuedinputs.insert(1, None);
        toreturn.queuedinputs.insert(2, None);
        
        
        toreturn.start_poker_game(1, 2);
        
        
        toreturn
        
    }
    
    pub fn get_game_information_string(&self) -> String{
        
        "somestring".to_string()
        
    }


    pub fn set_game_information_string(&mut self, gamestring: String){

        //i can serialize
        /*
        everything but the board game


        */
        


        
    }

    




    //what happens when a player wins?
    //just let the frontend know by having this method return true
    //i can either drop shit from the sky in a celebratory ass way
    //maybe have babylon js create confetti on its frontend thing
    //and the text "player X wins" over everything
    //or "YOU WON / LOST" depending on the player
    pub fn is_game_over(&self) -> bool{

        //win / lose conditions
        //no pieces left
        //king taken
        //no time left

        
        false
    }


    

    //get if it is the players turn, and if it is, how many ticks they have left in their turn
    //0 means it is not their turn
    pub fn get_players_turn_ticks_left(&self, playerid: u8) -> u32{

        if let Some(ticksleft) = self.turnmanager.get_ticks_left_for_players_turn(playerid){
            return ticksleft;
        }
        else{
            return 0;
        }

    }

    //get the total amount of time the player has lefts
    pub fn get_players_total_ticks_left(&self, playerid: u8) -> u32{

        self.turnmanager.get_players_total_ticks_left(playerid)

    }



    
    //get the id of the cards in the hands and the game
    pub fn get_card_ids(&self) -> Vec<u16>{
        self.cards.get_all_card_ids()
    }

    //get the information about the card
    pub fn get_card_by_id(&self, cardid: u16) -> Card{
        
        self.cards.get_card_unsafe(cardid)
    }


    //where is the card, what field is it in
    //what is its position in the field
    //what is the size of the field its in (hand size, river size)
    pub fn where_is_card(&self, cardid: u16) -> (u8, u8, u8){

        self.cards.where_is_card(cardid)
    }



    //if the cards is in the hand, get its owner
    pub fn get_card_owner(&self, cardid: u16) -> Option<u8>{
        
        if self.cards.does_player_own_card(1, cardid){
            return Some(1);
        }
        else if self.cards.does_player_own_card(2, cardid){
            return Some(2);
        }
        else{
            return None;
        }
    }


    pub fn get_size_of_hand(&self, playerid: u8) -> u8{

        self.cards.get_cards_in_hand(playerid).len() as u8
    }
    







    //get the objects on the board that that the card can interact with, and the associated input for it
    pub fn get_boardobject_actions_allowed_by_card(&self, playerid: u8, cardid: u16) -> HashMap<u16, PlayerInput> {
        
        //first, does the card exist
        if self.cards.does_card_exist(cardid){
            
            let card = self.cards.get_card_unsafe(cardid);
            
            let mut allowedinputs = HashMap::new();
            
            //if this card can drop or raise a square
            if card.effect == CardEffect::dropsquare || card.effect == CardEffect::raisesquare{
                
                //for every board square
                for boardsquareid in self.boardgame.get_empty_squares_not_on_mission(){

                    //if that square is empty
                    
                    let input = PlayerInput::playcardonsquare(cardid, boardsquareid);
                    
                    allowedinputs.insert( boardsquareid, input );
                }
                
            }
            
            return allowedinputs;
        }
        
        return HashMap::new();
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

    //get a string representing teh type of the piece
    pub fn get_piece_type_name(&self, pieceid: u16) -> String{
        self.boardgame.get_piece_type_name(pieceid)
    }
    
    
    pub fn get_board_game_object_owner(&self, objectid: u16) -> u8{
        
        self.boardgame.get_owner_of_piece(objectid)
        
    }

    //true if its white false if its black
    pub fn is_boardsquare_white(&self, boardsquareid: u16) -> bool{

        self.boardgame.is_boardsquare_white(boardsquareid)
    }

    
    
    
    
    
    //the actions allowed by the piece and the objects it captures or lands on
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec<(PieceAction, Vec<u16> )>){
        
        let mut toreturn = Vec::new();
        
        //get the actions allowed by the piece
        let (canflick, actions) = self.boardgame.get_actions_allowed_by_piece(pieceid);
        
        //get the pieces targeted by every action
        for action in actions{
            
            let objects = self.boardgame.get_objects_targeted_by_action(pieceid, action.clone());
            
            toreturn.push( (action, objects) );
        }
        
        (canflick, toreturn)
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
            //if this is the last tick before their turn ends
            else if self.turnmanager.is_it_this_players_turns_last_tick(playerid){
                
                //if they have to play a card
                if self.player_must_play_card_on_cardgame(playerid){
                    
                    //make the player draw a card
                    self.cards.draw_card(playerid);
                    
                    //and make them play a card from their hand
                    let handcardids = self.cards.get_cards_in_hand(playerid);
                    for cardid in handcardids{
                        self.cards.play_card(playerid, cardid);
                        break;
                    }
                
                }
                //if they dont, just make them take a draw action
                else {
                    self.cards.draw_card(playerid);
                    
                }
                actionwastaken = true;
            }
            
            
            
            //if an action was taken, let the turnmanager know that that player took their turn
            if actionwastaken{    
                self.turnmanager.player_took_action(playerid);
                
                //and clear queud inputs
                self.queuedinputs.insert(playerid, None);
            }

            
        }


        //check the card game if a player has won, and if they have, give them all the cards
        //do this by ticking the card interface
        self.cards.tick();
        
        //let the turn manager know that a tick has happeneds
        self.turnmanager.tick();
        
        //tick the physical game engine
        self.boardgame.tick();
    }
    
    
    /*
    both players start on the board with a chess background
    
    moving pieces back and forth
    
    they have a clock on the left of them with the total amount of time each player has left
    
    when one players total amount of time left dips below a minute
    the other player gets a card, or gets the ability to draw (2) cards at the cost of losing a turn
    
    
    maybe some cards in the deck, when drawn, get played automatically to change the game state
    the jokers
    
    
    and it is these cards that change the game state to make it different
    
    the point of drawing cards is to make the losing player introduce variance into the game to maybe not lose
    
    like, the ideal strategy would be like, if you think your current chance to win is like 30%, drawing a card
    you would hopefully increase your chances to win, but not above 50% on average
    tool for the losing player to introduce variant as i said
    
    you cant win just by drawing cards, they just help you win how you actually win
    
    like the ideal strategy isnt to be a mad man and keep drawing, because you dont win
    your opponent still wins, you can bs it away
    
    but if you just play well, and want to tryhard, you arent unaffected by the fact that the losing player can draw
    game state changing cards
    
    and a part of this, is that in the preffered game mode, the player cant draw cards before either some amount of turns pass
    or some amount of their or their opponents time passes
    
    since the player who is losing wants to introduce variance, its funny that they also get teh change to introduce variance as an
    added bonus if they draw a joker, that 
    
    i think the hands of both players should be shown
    just so theres more to think about
    and more to plan ahead about, without feeling like whatever you do is completely worthless when they pop out
    something you cant plan around
    
    
    cards:
    drop a board square for like a minutes long time
    get two extra turns after your opponents next next turn
    turn your opponents pieces into checkers pieces (you start winning, so your opponent draws to increase their favours)
    all pawns turn into queens
    other pieces can move any distance or capture from any distance
    if you capture your opponents king, you win
    start a blackjack game
    start a poker game
    discard the opponents hand
    
    
    when drawn:
    shuffle 10 more jokers into the deck
    current player takes an extra turn, opponent takes an extra, then curr again, then opp again
    both players now draw a card at the start of their turn
    turn each piece into a pool ball owned by their respective player
    turn each piece into a checkers piece
    both players amount of time per turn and total time is cut in half
    add 5 pieces to both players sides
    both players play with their hands revealed
    shake the board around a bit
    
    
    
    
    I think i figured it
    
    if you play a blackjack game and the opponent has no cards
    their next turn is spent drawing a card from their deck and putting it on the board
    both makes them take a turn at a possibly unideal time
    and makes them still put stuff up for stakes
    
    
    
    
    win conditions:
    no time
    no pieces
    (no cards)




    game begins
    both players
    */
    
    
    
    //if theres a cardgame going on , and this player has to play a card on it
    fn player_must_play_card_on_cardgame(&self, playerid: u8) -> bool{
        
        self.cards.is_player_forced_to_play_card(playerid)
    }
    
    
    
    
    
    //add a player
    fn add_player(&mut self){
        
        //the number of players starts counting at 1
        //so the first players id is 1 not 0
        let currentplayer = self.totalplayers + 1;
        
        self.players.insert(currentplayer);
        
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
                if &owner != &playerid{
                    return false;
                }
            }
            
        }
        
        
        //if the player has to play a card on the board
        //and this isnt an input to play a card on the board
        {
            
            if self.player_must_play_card_on_cardgame(playerid){
                
                if let PlayerInput::playcardonboard(cardid) = input {
                    return self.is_play_card_on_board_action_valid(&playerid, cardid) ;
                }
                else{
                    return false;
                };
                
                
            };
            
            
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
        //if its a draw action
        else if let PlayerInput::drawcard = input{
            return true
        }
        
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
        
    }
    
    //can this card be played alone
    fn is_play_card_on_board_action_valid(&self, playerid: &u8, cardid: &u16) -> bool{
        
        //first, does the card exist
        if self.cards.does_card_exist(*cardid){
            
            if self.cards.is_player_allowed_to_play_card(*playerid) {
                return true;
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

            if cardeffect == CardEffect::makepoolgame{
                return true;
            }
        }
        
        return false;
        
    }
    //if this card can be played on this piece 
    fn is_play_card_on_piece_action_valid(&self, playerid: &u8, cardid: &u16, pieceid: &u16) -> bool{
        
        
        return false;
        
    }   
    //if this card can be played on this boardsquare
    fn is_play_card_on_square_action_valid(&self, playerid: &u8, cardid: &u16, boardsquareid: &u16 ) -> bool{
        
        //first, does the card exist
        if self.cards.does_card_exist(*cardid){
            
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
            
            
        }
        
        
        
        
        false
        
    }
    //only called when the player is the one who owns the piece
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
    
    
    
    
    //perform an input that is valid, and it is the turn of the player
    fn perform_input(&mut self, playerid: &u8, playerinput: &PlayerInput) {
        
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = playerinput {
            self.boardgame.perform_action( *playerid, *pieceid, pieceaction.clone() );
        }
        
        //or if the input is a card action
        else if let PlayerInput::playcardonboard(cardid) = playerinput{

            let cardeffect = self.cards.get_card_unsafe(*cardid).effect;
            
            
            //if the player can play the card in the game
            if self.cards.is_player_allowed_to_play_card(*playerid){
                self.cards.play_card(*playerid, *cardid);
            }
            //if the cards effect is one that can be played on the board
            else if cardeffect == CardEffect::makepoolgame{

                self.boardgame.make_pool_game();

            }
            else{
                //otherwise panic, because this card should not have been allowed to be played
                //and it will fuck shit if i get here without actually having a valid action
            }

            
        }
        
        else if let PlayerInput::playcardonpiece(cardid, pieceid) = playerinput{
            
            
        }
        
        
        else if let PlayerInput::playcardonsquare(cardid, squareid) = playerinput{
            
            //get the effect of the card
            let effect = self.cards.get_card_unsafe(*cardid).effect;
            
            //if the effect is raise
            if let CardEffect::raisesquare = effect{
                
                self.boardgame.raise_square(*squareid);
            }
            //if the effect is drop
            else if let CardEffect::dropsquare = effect{
                
                //perform a long drop on the boardsquare
                self.boardgame.drop_square(*squareid);
            }
            
            
            //remove the card from the hand
            self.cards.remove_card_from_game(*playerid, *cardid);
            
        }
        else if let PlayerInput::drawcard = playerinput{
            
            self.cards.draw_card(*playerid);
        }
        
        
        
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









