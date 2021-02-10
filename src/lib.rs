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

pub use cardstructs::CardAction;
pub use cardstructs::BlackJackAction;
pub use cardstructs::PokerAction;


use serde::{Serialize, Deserialize};



//the maingame creates and returns these objects as its fuctions
#[derive(Serialize, Deserialize)]
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
    
    
    //if the game is finished, and who the winner is
    gameover: Option<u8>,
    
    
    //the players that have drawn cards
    playerdrewcard: HashSet<u8>,
    
    
    //how many ticks the game has been ended for
    //if its been 3000 ticks, panic, to stop running
    ticksgamehasbeenoverfor: u32,


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
            gameover: None,
            playerdrewcard: HashSet::new(),
            ticksgamehasbeenoverfor: 0,
        };
        
        //add two players
        toreturn.add_player();
        toreturn.add_player();
        
        
        toreturn        
    }
    
    pub fn can_piece_be_offered(&self, playerid: u8, pieceid: u16) -> bool{
        
        let mut piecelist = Vec::new();
        piecelist.push(pieceid);
        
        self.boardgame.are_pieces_offered_valid(playerid, piecelist)
    }
    
    pub fn is_game_over(&self) -> Option<u8>{
        
        //win / lose conditions
        //no pieces left
        //king taken
        //no time left
        
        
        self.gameover
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
    pub fn get_card_by_id(&self, cardid: &u16) -> Option<Card>{
        
        self.cards.get_card_by_id(cardid)
    }
    
    
    //where is the card, what field is it in
    //what is its position in the field
    //what is the size of the field its in (hand size, river size)
    pub fn where_is_card(&self, cardid: u16) -> Option<(u8, u8, u8)>{
        
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
    
    //get the objects on the board that that the card can interact with, and the associated input for it
    pub fn get_boardobject_actions_allowed_by_card(&self, playerid: u8, cardid: &u16) -> HashMap<u16, PlayerInput> {
        
        
        if let Some(card) = self.cards.get_card_by_id(cardid){
            
            
            let mut allowedinputs = HashMap::new();
            
            //if this card can drop or raise a square
            if card.effect == CardEffect::dropsquare || card.effect == CardEffect::raisesquare{
                
                //for every board square
                for boardsquareid in self.boardgame.get_empty_squares_not_on_mission(){
                    
                    let cardaction = CardAction::playcardonsquare(boardsquareid);
                    let input = PlayerInput::cardaction(*cardid, cardaction);
                    
                    allowedinputs.insert( boardsquareid, input );
                }
                
            }
            
            return allowedinputs;
        }
        
        return HashMap::new();
    }
    
    
    //get every player with an active turn
    pub fn get_active_players(&self) -> HashSet<u8>{
        
        self.turnmanager.get_current_players()
    }
    
    
    
    //return if theres an active pokergame or not
    pub fn is_pokergame_ongoing(&self) -> bool{
        
        self.cards.is_pokergame_ongoing()
    }
    pub fn get_value_of_offered_pieces(&self, playerid: u8, piecesoffered: Vec<u16>) -> Option<u8>{
        
        self.boardgame.get_value_of_offered_pieces(playerid, piecesoffered)
    }
    pub fn get_debt_of_player(&self, playerid: &u8) -> u8{
        
        //get the debt according to the card games
        if let Some(totaldebtofplayer) = self.cards.pool_debt_of_player(playerid){
            
            //get the value of this players pool
            let valueinpoolcurrently = self.boardgame.get_value_of_players_pool(*playerid);
            
            let debtofplayer = totaldebtofplayer - valueinpoolcurrently;
            
            if debtofplayer > 100{
                return 0;
                //panic!("ive done something wrong");
            }
            
            return debtofplayer;
            
        }
        else{
            
            return 0;
        }
        
    }
    pub fn get_cost_to_check(&self, playerid: &u8) -> Option<u8>{
        self.cards.pokergame_options(*playerid)
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
    
    //the things that would prevent a card or piece action    
    fn is_cardgame_ongoing_or_debt_unsettled(&self, playerid: &u8) -> bool{
        
        
        if ! self.is_debt_settled(playerid){

            return true;
        }
        
        //return false if there is any card game ongoing
        if self.cards.is_pokergame_ongoing(){

            return true;
        }
        
        
        false
    }
    
    
    //get a string representing teh type of the piece
    pub fn get_piece_type_name(&self, pieceid: u16) -> Option<String>{
        
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
    pub fn is_boardsquare_white(&self, boardsquareid: u16) -> bool{
        
        self.boardgame.is_boardsquare_white(boardsquareid)
    }
    
    //the actions allowed by the piece and the objects it captures or lands on
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec<(PieceAction, Vec<u16> )>){
        
        let mut toreturn = Vec::new();
        
        let owner = self.get_board_game_object_owner(pieceid).unwrap();
        
        if self.is_cardgame_ongoing_or_debt_unsettled(&owner){
            return (false, Vec::new());
        }
        
        
        
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
        
        
        
        
        
        
        //check the card game if a player has won, and if they have, give them all the cards
        //do this by ticking the card interface
        let maybewon = self.cards.tick();
        
        //if the game is over, give the pieces in the pool to the winner
        if let Some(winner) = maybewon{
            self.boardgame.give_pool_to_player(winner);
        }
        
        
        //if a player doesnt confirm to settle their ante
        //an opponent gets ownership of their lowest cost piece
        
        //if an opponent doesnt make a move in poker without owning any debt
        //its considered a fold
        //a player folds if they have to
        
        
        



        



        let mut arekingsreplaced = false;

        //if a player has drawn, then the kings get replaced
        if self.playerdrewcard.is_empty() == false{

            arekingsreplaced = true;
        }



        //tick the physical game engine
        self.boardgame.tick(arekingsreplaced);


                
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
    pub fn can_player_draw(& self, playerid: &u8) -> bool{


        if self.is_cardgame_ongoing_or_debt_unsettled(playerid) {

            return false;
        };

        //if its past turn 10
        if self.turnmanager.get_turn_number() > 10{
            return true;
        }


        return false;
    }
    
    
    
    
    //add a player
    fn add_player(&mut self){
        
        //the number of players starts counting at 1
        //so the first players id is 1 not 0
        let currentplayer = self.totalplayers + 1;
        
        self.players.insert(currentplayer);
        
        self.queuedinputs.insert(currentplayer, None);
        
        self.totalplayers += 1;
    }
    
    //check if input is valid rather than just if the action is
    //if the player is the one sending the request or some shit like that i guess
    fn is_input_valid(&self, playerid: &u8, input: &PlayerInput) -> bool{
        
        
        if let PlayerInput::cardaction(cardid, action) = input{
            return self.is_card_action_valid(playerid, cardid, action);
        }
        
        else if let PlayerInput::pokeraction(action) = input{
            return self.is_poker_action_valid(playerid, action);
        }
        
        else if let PlayerInput::blackjackaction(action) = input{
            panic!("blackjack not implemented");
            //return self.is_blackjack_action_valid(playerid, action);
        }
        
        else if let PlayerInput::pieceaction(pieceid, pieceaction) = input.clone(){
            return self.is_piece_action_valid( &playerid, &(pieceid as u16), &pieceaction);
        }
        
        else if let PlayerInput::drawcard = input{
            return self.can_player_draw(playerid);
        }
        
        else if let PlayerInput::settledebt(pieces) = input{
            return self.is_settle_debt_action_valid(playerid, pieces);
        }
        
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
        
    }
    
    
    
    //does this player not have any debt. is this player debt free
    fn is_debt_settled(&self, playerid: &u8) -> bool{
        
        let debt = self.get_debt_of_player(playerid);
        
        if debt != 0{
            return false;
        };
        
        return true;
        
    }
    
    
    fn is_card_action_valid(&self, playerid: &u8, cardid: &u16, action: &CardAction) -> bool{
        
        
        
        
        if self.is_cardgame_ongoing_or_debt_unsettled(playerid){
            return false;
        }
        
        
        
        if let Some(card) = self.cards.get_card_by_id(cardid){
            
            
            //RETURN TRUE ALWAYS FOR TESTING RIGHT NOW
            return true;
            
            
            let cardeffect = card.effect;
            
            if let CardAction::playcardonpiece(pieceid) = action{
                return true;
                
            }
            else if let CardAction::playcardonsquare(squareid) = action{
                
                if cardeffect == CardEffect::dropsquare{
                    return true;
                }
                if cardeffect == CardEffect::raisesquare{
                    return true;
                }
                
            }
            else if CardAction::playcardonboard == *action{
                
                if cardeffect == CardEffect::blackjackgame{
                    return true;
                }
                if cardeffect == CardEffect::pokergame{
                    return true;
                }
                if cardeffect == CardEffect::makepoolgame{
                    return true;
                }
                if cardeffect == CardEffect::backtobackturns{   
                    return true;
                }
                if cardeffect == CardEffect::halvetimeleft{
                    return true;
                }
            }
            
            
        }
        
        
        panic!("card doesnt exist, or uncaught action some other how {:?}", action);
        
        false
    }
    
    fn is_piece_action_valid(&self, playerid: &u8, pieceid: &u16,  pieceaction: &PieceAction) -> bool{
        
        if self.is_cardgame_ongoing_or_debt_unsettled(playerid){
            return false;
        }
        
        
        
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
    
    fn is_poker_action_valid(&self, playerid: &u8, pokeraction: &PokerAction) ->  bool{
        
        //if the debt is settled, you have to do a settle debt action first
        if self.is_debt_settled(playerid){
            
            
            //is there a poker game going on?
            //and is it this players turn currently
            if let Some(checkvalue) = self.cards.pokergame_options(*playerid){
                
                
                if let PokerAction::check(piecesandvalue) = pokeraction{
                    
                    //are the pieces and values being offered valid
                    if let Some(totalvalueoffered) = self.boardgame.get_value_of_offered_pieces(*playerid, piecesandvalue.clone()){
                        
                        //is it offering exactly the amount needed to check?
                        if totalvalueoffered == checkvalue{
                            return true;
                        }
                    }
                }
                else if let PokerAction::raise(piecesandvalue) = pokeraction{
                    
                    //are the pieces and values being offered valid
                    if let Some(totalvalueoffered) = self.boardgame.get_value_of_offered_pieces(*playerid, piecesandvalue.clone()){
                        
                        //is it offering more than the amount needed to check?
                        if totalvalueoffered > checkvalue{
                            return true;
                        }
                    }
                }
                else if let PokerAction::fold = pokeraction{
                    return true;
                }
            }
        }
        
        false
    }
    
    fn is_settle_debt_action_valid(&self, playerid: &u8, pieces: &Vec<u16>) -> bool{
        
        //if the pieces are valid
        if let Some(value) = self.boardgame.get_value_of_offered_pieces(*playerid, pieces.clone()){
            
            // and equal to the amount of debt owed by the player
            if value == self.get_debt_of_player(playerid){
                
                //and also make sure that its not settling 0 value (maybe?)
                if value != 0{
                    return true;
                }
            }
        }
        
        
        return false;
        
    }
    
    
    fn apply_card_effect_to_board(&mut self, playerid: &u8, cardeffect: CardEffect){
        
        //dont play cards 
        /*
        else if let CardAction::playcardonpiece(pieceid) = action{
            
            
        }
        else if let CardAction::playcardonsquare(squareid) = action{
            
            if let CardEffect::raisesquare = cardeffect{    
                
                self.boardgame.raise_square(*squareid);
            }
            else if let CardEffect::dropsquare = cardeffect{
                
                self.boardgame.drop_square(*squareid);
            }
        }
        */
        
        if cardeffect == CardEffect::makepoolgame{
            self.boardgame.make_pool_game();
        }
        else if cardeffect == CardEffect::backtobackturns{
            self.turnmanager.players_take_2_turns_in_a_row();
        }
        else if cardeffect == CardEffect::halvetimeleft{
            self.turnmanager.halve_time_left();
        }
        else if cardeffect == CardEffect::pokergame{
            self.cards.start_poker_game(1, 2);
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
        else if let PlayerInput::cardaction(cardid, action) = playerinput{
            
            
            if let Some(card) = self.cards.get_card_by_id(cardid){
                
                if let CardAction::playcardonboard = action{
                    
                    self.apply_card_effect_to_board(playerid, card.effect);
                    
                }
                
                //remove the card from the game
                self.cards.remove_card_from_game(*cardid);
                
            }
            
            
            
        }
        else if let PlayerInput::blackjackaction(action) = playerinput{
            
            panic!("blackjack not implemeneted");
            
        }
        else if let PlayerInput::pokeraction(action) = playerinput{
            
            
            if let PokerAction::check(piecesandvalue) = action{
                
                //break up the pieces offered into the parts needed to only give the value of each
                //and tell the boardgame to put them in this players pool, to make their owners this players pool
                self.cards.player_checks();
                
                
                self.boardgame.put_pieces_in_pool(piecesandvalue.clone());
            }
            
            else if let PokerAction::fold = action{
                self.cards.player_folds();
            }
            
            else if let PokerAction::raise(piecesandvalue) = action{
                
                let amountoffered = self.boardgame.get_value_of_offered_pieces(*playerid, piecesandvalue.clone()).unwrap();
                let amountneededtocheck = self.cards.pokergame_options(*playerid).unwrap();
                
                self.cards.player_raises( amountoffered - amountneededtocheck );
                
                self.boardgame.put_pieces_in_pool(piecesandvalue.clone());
            }
            
            
        }
        else if let PlayerInput::drawcard = playerinput{
            
            self.playerdrewcard.insert(*playerid);
            
            self.apply_card_effect_to_board(playerid, CardsInterface::get_joker_card_effect());
            
        }
        else if let PlayerInput::settledebt(piecesandvalue) = playerinput{
            
            self.boardgame.put_pieces_in_pool(piecesandvalue.clone());
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
    
    
}



