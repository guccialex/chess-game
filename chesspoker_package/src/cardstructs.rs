

use std::collections::HashMap;
use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PokerAction{
    
    //raise
    //the list of pieces offered
    raise( Vec<u16>),
    
    //folding action
    fold,
    
    //checking action
    //the list of pieces offered
    //if the player needs to 
    check( Vec<u16> ),
}




#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BlackJackAction{
    
    //hitting action
    hit,
    
    //standing action
    stand,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CardAction{
    
    //playing a card on the active board
    playcardonboard,
    
    //playing a card with a target of a piece
    playcardonpiece(u16),
    
    //playing a card with a target of a board square
    playcardonsquare(u16),
    
}









//the manager for all card related data and structs in the game
#[derive(Clone, Serialize, Deserialize)]

pub struct CardsInterface{
    
    totalcards: u16,
    
    //the cards in the hands, deck and 
    cards: HashMap<u16, Card>,
    
    maxhandsize: usize,
    
    //the cards in the hand of each player
    hands: HashMap<u8, Vec<u16>>,
    
    //the cards in the deck
    deck: Vec<u16>,
    
    //the game of poker if it exists
    pokergame: Option<PokerGame>,
    
}



impl CardsInterface{
    
    pub fn new_two_player() -> CardsInterface{
        
        let mut hands = HashMap::new();
        
        hands.insert(1, Vec::new());
        hands.insert(2, Vec::new());
        
        CardsInterface{
            totalcards: 0,
            
            cards: HashMap::new(),
            
            hands: hands,
            
            maxhandsize: 5,
            
            deck: Vec::new(),
            
            pokergame: None,
        }
        
        
        
    }
    
    
    fn add_random_card_to_game(&mut self) -> u16{
        let cardid = self.totalcards;
        self.totalcards += 1;
        let newcard = Card::new_random_card();
        
        self.cards.insert( cardid, newcard );
        
        cardid
    }
    
    
    fn add_random_card_to_deck(&mut self){
        
        let cardid = self.totalcards;
        self.totalcards += 1;
        let newcard = Card::new_random_card();
        
        self.cards.insert( cardid, newcard );
        self.deck.push(cardid);
        
    }
    
    //and how much debt does this player need to settle
    pub fn pool_debt_of_player(&self, playerid: &u8) -> u8{
        
        //if there is a pokergame
        //get the amount of money that player has in their pool in the pokergame
        //so that the money they can put up the money they owe
        if let Some(pokergame) = &self.pokergame{
            
            
            if playerid == &1{
                
                return pokergame.player1pool;
            }
            else{
                
                return pokergame.player2pool;
            }
            
        };
        
        return 0;
        
    }
    


    //is there a pokergame ongoing
    pub fn is_pokergame_ongoing(&self) -> bool{

        return self.pokergame.is_some();
    }

    
    
    
    //if theres a poker game going on
    //and the player can perform an action in it RIGHT NOW (aka, its not the opponents turn)
    //and how much it costs for the player to match the opponents raise
    pub fn pokergame_options(&self, playerid: u8) -> Option<u8>{

        if let Some(pokergame) = &self.pokergame{
            return pokergame.can_player_act(playerid);
        }
        
        return None;
    }
    
    //this player checks and pays the amount owed to check
    pub fn player_checks(&mut self){

        if let Some(pokergame) = &mut self.pokergame{

            pokergame.player_checks();
        }
        else{
            panic!("aaaaa");
        }
        
        
    }

    //the player raises by this amount
    pub fn player_raises(&mut self, amount: u8){


        if let Some(pokergame) = &mut self.pokergame{

            pokergame.player_raises(amount);
        }
        else{
            panic!("aaaaa");
        }
        
        
    }

    //the player folds
    pub fn player_folds(&mut self){


        if let Some(pokergame) = &mut self.pokergame{

            pokergame.player_folds();
        }
        else{
            panic!("aaaaa");
        }
        
    }
    
    
    
    
    //the player draws a card and return the id of the card drawn
    pub fn draw_card(&mut self, playerid: u8) -> u16{
        
        //draw a card from the deck, if there is no card in the deck
        //create a random card in the deck then draw
        if self.deck.is_empty(){
            self.add_random_card_to_deck();
        }
        
        let cardid = self.deck.pop().unwrap();
        self.hands.get_mut(&playerid).unwrap().push(cardid);
        return cardid;
    }
    
    
    pub fn does_card_exist(&self, cardid: u16 ) -> bool{
        
        return  self.cards.contains_key(&cardid)  ;
        
    }
    
    //get the card by ID and panic if it doesnt have it
    pub fn get_card_unsafe(&self, cardid: u16) -> Card{
        
        self.cards.get(&cardid).unwrap().clone()
    }
    
    //get all cards that 
    pub fn get_all_card_ids(&self) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        for (cardid, card) in &self.cards{
            toreturn.push(*cardid);
        };
        
        
        toreturn
        
    }
    
    
    pub fn get_cards_in_hand(&self, playerid: u8) -> Vec<u16>{
        
        let handcardids = self.hands.get(&playerid).unwrap();
        
        let mut hand = Vec::new();
        
        
        for cardid in handcardids{
            
            hand.push(*cardid);
            
        }
        
        
        return hand;
        
    }
    
    //get a list of the cards in the game by ID
    pub fn get_cards_in_game(&self) -> Option< Vec<u16> >{
        
        if let Some(pokergame) = &self.pokergame{
            
            
            return  Some( pokergame.get_card_ids() ) ;
            
        }
        else{
            return None;
        }
        
    }
    
    
    pub fn does_player_own_card(&self, playerid: u8, cardid: u16) ->  bool{
        
        //for each card in the players hand
        for playerscardid in self.hands.get(&playerid).unwrap(){
            
            //if that card in the hand matches the card passed in
            if playerscardid == &cardid{
                  
                return true;
            };  
        };
        
        
        return false;        
    }
    
    
    //where is the card, what field is it in
    //what is its position in the field
    //what is the size of the field its in (hand size, river size)
    pub fn where_is_card(&self, cardid: u16) -> (u8, u8, u8){
        
        /*
        the positions where the card can be
        
        player 1s hand                  1
        player 2s hand                  2
        player 1s hand in the game      3
        player 2s hand in the game      4
        the river in the game           5
        */
        
        let mut field = 0;
        
        let mut positioninfield = 0;
        
        
        for (playerid, hand) in &self.hands{


            let fieldsize = hand.len() as u8;
            
            let mut curcardpos = 0;
            
            for cardidinhand in hand{
                
                curcardpos += 1;
                
                if cardidinhand == &cardid{
                    
                    return (field,positioninfield,fieldsize);
                }
            }
        }
        
        
        if let Some(pokergame) = &self.pokergame{
            
            //if the card is in the game
            if pokergame.get_card_ids().contains(&cardid){
                
                return pokergame.where_is_card(cardid);
            }
        }
        

        panic!("cant find card");
    }
    
    
    
    
    //remove this card from that players hand
    //if that player has that card in hand (should I panic if they dont?)
    pub fn remove_card_from_game(&mut self, playerid: u8, cardid: u16){
        
        let muthand = self.hands.get_mut(&playerid).unwrap();
        
        let mut removedcard: Option<&u16> = None;
        
        //remove the card from the players hand
        muthand.retain(|cardidinhand| {
            
            let delete = {
                cardid == *cardidinhand
            };
            
            !delete
        });
        
        self.cards.remove(&cardid);
    }
    
    
    
    //start a poker game with the given players
    pub fn start_poker_game(&mut self, player1: u8, player2:u8){
        
        //create the river to be passed in
        let mut river = Vec::new();
        for x in 0..5{
            let cardid = self.add_random_card_to_game();
            let card = self.cards.get(&cardid).unwrap();
            
            river.push( (cardid, card.clone()) );
        }
        
        
        let mut player1hand = Vec::new();
        for x in 0..2{
            let cardid = self.add_random_card_to_game();
            let card = self.cards.get(&cardid).unwrap();
            
            player1hand.push( (cardid, card.clone()) );
        }
        
        
        let mut player2hand = Vec::new();
        for x in 0..2{
            let cardid = self.add_random_card_to_game();
            let card = self.cards.get(&cardid).unwrap();
            
            player2hand.push( (cardid, card.clone()) );
        }
        
        
        
        
        self.pokergame = Some( PokerGame::new(river, player1hand, player2hand)  );
    }
    
    
    //if a player has won the card game, end the game
    //and return the winner of the game
    pub fn tick(&mut self) -> Option<u8>{
        
        let mut toreturn = None;
        let mut cardgameended = false;
        
        //if theres a cardgame going on
        if let Some(pokergame) = &mut self.pokergame{
            
            //if the cardgame is finished
            if let Some(winnerid) = pokergame.get_winner(){
                
                toreturn = Some(winnerid);
                
                cardgameended = true;
            }
        }
        
        if cardgameended{
            self.pokergame = None;
        }
        
        //the "garbage collecting" step
        //remove all cards that arent in the players hand or deck or poker game
        self.remove_nonexisting_cards();
        
        
        
        return toreturn;
    }
    
    
    //remove the cards from the list of cards that no longer exist in the
    //poker game, hands or deck
    fn remove_nonexisting_cards(&mut self){
        
        
        
        use std::collections::HashSet;
        
        
        let mut existingcards = HashSet::new();
        
        
        
        if let Some(pokergame) = &self.pokergame{
            
            for cardid in pokergame.get_card_ids(){
                
                existingcards.insert(cardid);
            }
        }
        
        
        
        for (_, cards) in &self.hands{
            
            for cardid in cards{
                existingcards.insert(*cardid);
            }
        }
        
        
        for cardid in &self.deck{
            
            existingcards.insert(*cardid);
        }
        
        
        
        //for every card, remove it if its not a card that exists anymore
        for (cardid, card) in self.cards.clone(){
            
            if existingcards.contains(&cardid){
                //do nothing
            }
            else{
                self.cards.remove(&cardid);
            }
        }
        
        
        
    }
    
    
}







use rust_poker;



#[derive(Clone, Serialize, Deserialize)]
//a 2 player holdem poker game
pub struct PokerGame{
    
    
    //the money player 1 has in the pool
    player1pool: u8,
    //the money player 2 has in the pool
    player2pool: u8,
    
    
    
    
    
    //the hands of the players
    player1hand: Vec<(u16, Card)>,
    player2hand: Vec<(u16, Card)>,
    
    
    //the cards in the middle of the board
    //is left as empty if there is no river (like in black jack)
    river: Vec<(u16, Card)>,
    
    
    //the player that is the dealer
    //the dealer is the small blind
    //and the dealer goes first
    dealer: u8,
    
    
    
    //if the dealer has taken their turn yet in this round
    hasdealergone: bool,
    
    
    //what round is it?
    //round 0 is whether the small blind wants to buy in
    //round 1 is the players deciding what to do after the flop
    //round 2 is the players deciding what to do after the turn
    //round 3 is the players deciding what to do after the river
    //round 4 is the game being over
    
    roundnumber: u32,
    
    
}

impl PokerGame{
    
    fn new(river: Vec<(u16, Card)>, player1hand: Vec<(u16, Card)>, player2hand: Vec<(u16, Card)>) -> PokerGame{
        
        PokerGame{
            
            //player 1 is the dealer and needs to put in 1,
            //player 2 is the non dealer and needs to put in 2
            player1pool: 1,
            player2pool: 2,
            
            player1hand: player1hand,
            player2hand: player2hand,
            
            river: river,
            
            //player 1 is the dealer
            dealer: 1,
            
            hasdealergone: false,
            
            roundnumber: 0,
        }


    }
    
    
    //if it is this players turn
    //return some if it can act, and how much debt it costs for this player to check
    fn can_player_act(& self, playerid: u8) -> Option<u8>{
        
        if self.roundnumber < 4{
            
            //if this player is the dealer
            if self.dealer == playerid {
                
                //and the dealer hasnt gone this round
                if self.hasdealergone == false{
                    
                    //the cost to check is equal to the difference
                    //between the money curplayer has in the pool
                    //and the money other player has in the pool
                    let costtocheck = (self.player1pool as i32 - self.player2pool as i32).abs();
                    
                    return Some( costtocheck as u8);
                }
                else{
                    return None;
                }
            }
            //if this player isnt the dealer
            else{
                
                //and the dealer has already gone this round
                if self.hasdealergone == true{
                    
                    //the cost to check is equal to the difference
                    //between the money curplayer has in the pool
                    //and the money other player has in the pool
                    let costtocheck = (self.player1pool as i32 - self.player2pool as i32).abs();
                    
                    return Some( costtocheck as u8);
                    
                }
                else{                    
                    return None;
                }
            }
        }
        
        
        return None;
    }
    
    
    
    //the player raises by this amount
    fn player_raises(&mut self, amount: u8){
        
        //assume the player acting is the player whos turn it is
        let playerid = self.get_current_player();
        
        
        if playerid == 1{
            self.player1pool += amount;
        }
        else{
            self.player2pool += amount;
        }
        
        
        
        if playerid == self.dealer{
            self.hasdealergone = true;
        }
        else{
            self.hasdealergone = false;
            self.roundnumber += 1;
        }
        
    }
    
    
    
    fn player_checks(&mut self){
        
        //assume the player acting is the player whos turn it is
        let playerid = self.get_current_player();
        
        
        
        //the player has checked and made the value in their pool equal to the other player
        if playerid == 1{
            self.player1pool = self.player2pool;
        }
        else{
            self.player2pool = self.player1pool;
        }
        
        
        
        if playerid == self.dealer{
            self.hasdealergone = true;
        }
        else{
            self.hasdealergone = false;
            self.roundnumber += 1;
        }
        
        
    }
    
    
    
    fn player_folds(&mut self){
        
        
        //assume the player acting is the player whos turn it is
        let playerid = self.get_current_player();
        
        
        //remove the cards from their hand so they lose
        //and skip the game to the end
        
        
        if playerid == 1{
            self.player1hand = Vec::new();
        }
        else{
            self.player2hand = Vec::new();
        }
        
        self.roundnumber = 4;
    }
    
    
    
    
    //get the ID of the player whos turn we're waiting on
    fn get_current_player(&self) -> u8{
        
        
        //assume the player acting is the player whos turn it is
        let playerid;
        
        //if the dealer has gone, the player the non dealer
        if self.hasdealergone{
            
            if self.dealer == 1{
                playerid = 2;
            }
            else{
                playerid = 1;
            }
            
        }
        else
        //if the dealer hasnt gone, the player is the dealer
        {
            playerid = self.dealer;
        }
        
        
        return playerid;
        
    }
    
    //if the game is finished, get the winner of the pool
    fn get_winner(&self) -> Option<u8>{
        
        use rust_poker::hand_evaluator::{Hand, CARDS, evaluate};
        
        
        //if the game is over
        if self.roundnumber >= 4{
            
            let mut player1hand = Hand::empty();
            let mut player2hand = Hand::empty();
            
            
            for (_, card) in &self.player1hand{
                player1hand += CARDS[card.get_rust_poker_cardid()];
            }
            
            for (_, card) in &self.player2hand{
                player2hand += CARDS[card.get_rust_poker_cardid()];
            }
            
            
            //add the cards in the river to both players hands
            for (_, card) in &self.river{
                player1hand += CARDS[card.get_rust_poker_cardid()];
                player2hand += CARDS[card.get_rust_poker_cardid()];
            }
            
            
            
            let player1handscore = evaluate(&player1hand);
            let player2handscore = evaluate(&player2hand);
            
            if player1handscore >= player2handscore{
                return Some(1);
            }
            else{
                return Some(2);
            }
            
        }
        
        return None;
    }
    
    
    
    fn get_card_ids(&self) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        for (cardid, card) in &self.player1hand{
            
            toreturn.push(*cardid);
        }
        
        for (cardid, card) in &self.player2hand{
            
            toreturn.push(*cardid);
        }
        
        for (cardid, card) in &self.river{
            
            toreturn.push(*cardid);
        }
        
        
        toreturn
    }
    
    
    
    
    
    
    
    //get the field the card is in
    //its position in the field
    //and the size of the field
    fn where_is_card(&self, cardid: u16) -> (u8,u8,u8){
        
        /*
        player 1s hand in the game      3
        player 2s hand in the game      4
        the river in the game           5
        */

        let mut fieldposition = 0;
        for (playercardid, _) in &self.player1hand{

            if &cardid == playercardid{

                return (3, fieldposition, self.player1hand.len() as u8);
            }
            fieldposition += 1;
        }



        let mut fieldposition = 0;
        for (playercardid, _) in &self.player2hand{

            if &cardid == playercardid{

                return (4, fieldposition, self.player2hand.len() as u8);
            }
            fieldposition += 1;
        }



        let mut fieldposition = 0;
        for (rivercardid, _) in &self.river{

            if &cardid == rivercardid{

                return (5, fieldposition, self.river.len() as u8);
            }
            fieldposition += 1;
        }
        
        


        panic!("this card is not in the game");
    }
    
    
}





//the traits that all card games implement
trait CardGame{
    
    
    /*
    get the list of the cards in the game by ID
    
    
    get the position of a card id in the game (in player 1s hand, player 2s hand, or the river
        
        
        get who the winner is if the games over now
        */
        
        
        
        
    }
    
    
    
    /*
    //a card game
    #[derive(Clone, Serialize, Deserialize)]
    pub struct CardGame{
        
        //if the game is over
        ended: bool,
    }
    
    impl CardGame{
        
        
        //a new game started with these players
        fn new_blackjack() -> CardGame{
            
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
        fn new_poker(river: Vec<(u16, Card)>) -> CardGame{
            
            
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
        
        
        
        fn get_winner(&self) -> Option<u8>{
            
            Some(1)
        }
        
        //used for displaying the cards
        //get the list of the cards ina  players hand from a certain players perspective
        fn get_card_ids_in_hand(&self, requestedplayer: u8) -> Vec<u16>{
            
            let mut toreturn = Vec::new();
            
            for (cardid, card) in self.playerscards.get(&requestedplayer).unwrap().clone(){
                
                toreturn.push(cardid);
            }
            
            toreturn
            
        }
        
        fn get_card_ids_in_river(&self) -> Vec<u16>{
            
            
            let mut toreturn = Vec::new();
            
            for (cardid, card) in &self.river{
                toreturn.push(*cardid);
            }
            
            
            toreturn
            
        }
        
        
        fn get_card_ids_in_game(&self) -> (Vec<u16>, Vec<u16>, Vec<u16>){
            
            (self.get_card_ids_in_hand(1), self.get_card_ids_in_river(), self.get_card_ids_in_hand(2))
            
        }
        
        
        
        
        //can this player play any cards now?
        fn is_player_allowed_to_play_card(&self, playerid: u8) -> bool{
            
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
        fn must_player_play_card(&self, playerid: u8) -> bool{
            
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
        
        fn is_game_over(&self) -> bool{
            
            self.ended        
            
        }
        
        //a player plays this card
        //return the winner of the game and the cards they win if they won
        //or return nothing if 
        fn play_card(&mut self, playerid: u8, card: (u16, Card)){
            
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
        fn player_passes(&mut self, playerid: u8){
            
            self.hasplayerpassed.insert(playerid, true);
            
        }    
        
        
        //if the game is ended, get the winner, and the cards they won, then the cards to remove from the game
        fn get_winner_rewards(&mut self)-> Option<(u8, Vec<(u16,Card)>, Vec<(u16, Card)>)> {
            
            
            //first, if the game is not completed yet, return none
            if self.is_game_over() == false{
                
                
                return (None);
            }
            
            
            //if this is a blackjack game
            if self.blackjackorpoker == true{
                
                let mut highestblackjackscore = 0;
                let mut highestblackjackscoreholder = 0;
                
                let mut allcardswon: Vec<(u16,Card)> = Vec::new();
                
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
                if (highestblackjackscore == 0){
                    
                    return Some( ( 1, Vec::new(), allcardswon ) ) ;
                }
                else{
                    
                    //else return the player with the highest score
                    //and every card from every players hand
                    return Some( (highestblackjackscoreholder, allcardswon, Vec::new()) )  ;
                    
                }
                
                
                
                
            }
            //if this is a poker game
            else{
                
                
                
                let mut highestpokerscore = 0;
                let mut highestpokerscoreholder = 0;
                
                let mut allcardswon: Vec<(u16,Card)> = Vec::new();
                
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
                    
                    allcardswon.extend(river);
                    
                    return Some( ( 1, Vec::new(), allcardswon ) ) ;
                }
                else{
                    
                    //else return the player with the highest score
                    //and every card from every players hand
                    return Some( (highestpokerscoreholder, allcardswon, river) ) ;
                    
                }
                
                
            };
            
            
            panic!("i shouldnt get here");
        }
        
        
        //get the value of the blackjack hand
        //return 0 for bust
        //21 for blackjack
        //the best value of the hand otherwise
        fn evaluate_blackjack_hand(hand: &Vec<(u16,Card)>) -> u16 {
            
            //get total value of the cards that arent aces
            let mut loweracevalue = 0; 
            
            //get the number of aces
            let mut numberofaces = 0;
            
            
            for currentcard in hand{
                
                if ( currentcard.1.is_ace() ){
                    
                    numberofaces += 1;
                    
                    loweracevalue += 1;
                }
                
                else{
                    
                    loweracevalue += currentcard.1.blackjackvalue();
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
        fn evaluate_poker_hand(hand: &Vec<(u16,Card)>, river: &Vec<(u16,Card)>) -> u32{
            
            
            10
            
        }
        
        
        
    }
    
    
    
    
    */
    
    
    
    
    //the different values of the card
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum CardValue{
        
        ace,
        two,
        three,
        four,
        five,
        six,
        seven,
        eight,
        nine,
        ten,
        jack,
        queen,
        king
    }
    
    //the effect of the card it can have
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum CardEffect{
        
        //this card can initiate a blackjack game with it as the starting card
        blackjackgame,
        
        //this card can initiate a poker game with it as the starting card
        pokergame,
        
        //this card can remove a square from the board
        dropsquare,
        
        //this card can raise a square to not be able to be moved past by another piece
        raisesquare,
        
        
        //make the game have pool settings
        makepoolgame,
        
        
        backtobackturns, 
        
        
        halvetimeleft,
        
        
    }
    
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum CardSuit{
        
        diamonds,
        clubs,
        hearts,
        spades
        
        
    }
    
    
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
    pub struct Card{
        
        
        
        value: CardValue,
        
        suit: CardSuit,
        
        pub effect: CardEffect,
        
        
        
    }
    
    impl Card{
        
        //returns if this card is an ace or not
        pub fn is_ace(&self) -> bool{
            
            
            if self.value == CardValue::ace{
                
                return(true);
            }
            else{
                
                return(false);
            }
            
            
            
            
        }
        
        //get the blackjack value of the card
        pub fn blackjackvalue(&self) -> u16{
            
            
            if self.value == CardValue::two{
                return(2);
            }
            else if self.value == CardValue::three{
                return(3);
            }
            else if self.value == CardValue::four{
                return(4);
            }
            else if self.value == CardValue::five{
                return(5);
            }
            else if self.value == CardValue::six{
                return(6);
            }
            else if self.value == CardValue::seven{
                return(7);
            }
            else if self.value == CardValue::eight{
                return(8);
            }
            else if self.value == CardValue::nine{
                return(9);
            }
            else if self.value == CardValue::ten{
                return(10);
            }
            else if self.value == CardValue::jack{
                return(10);
            }
            else if self.value == CardValue::queen{
                return(10);
            }
            else if self.value == CardValue::king{
                return(10);
            }
            
            
            panic!("this is an ace, so i  dont know whether the value is 1 or 11");
            
            
        }
        
        
        //return the number representing the value
        //1 - 13
        pub fn numbervalue(&self) -> u16{
            
            
            
            if self.value == CardValue::two{
                return(2);
            }
            else if self.value == CardValue::three{
                return(3);
            }
            else if self.value == CardValue::four{
                return(4);
            }
            else if self.value == CardValue::five{
                return(5);
            }
            else if self.value == CardValue::six{
                return(6);
            }
            else if self.value == CardValue::seven{
                return(7);
            }
            else if self.value == CardValue::eight{
                return(8);
            }
            else if self.value == CardValue::nine{
                return(9);
            }
            else if self.value == CardValue::ten{
                return(10);
            }
            else if self.value == CardValue::jack{
                return(11);
            }
            else if self.value == CardValue::queen{
                return(12);
            }
            else if self.value == CardValue::king{
                return(13);
            }
            //if its an ace
            else{
                return(1);
            }
            
        }
        
        
        
        //get the cardid of the card in the form the rust_poker crate wants it
        // cards are indexed 0->51 where index is 4 * rank + suit
        fn get_rust_poker_cardid(&self) -> usize{
            
            //i need ace to be highest, not the lowest
            let numbervalue;
            
            {
                if CardValue::ace == self.value{
                    numbervalue = 0;
                }
                else if CardValue::king == self.value{
                    numbervalue = 1;
                }
                else if CardValue::queen == self.value{
                    numbervalue = 2;
                }
                else if CardValue::jack == self.value{
                    numbervalue = 3;
                }
                else if CardValue::ten == self.value{
                    numbervalue = 4;
                }
                else if CardValue::nine == self.value{
                    numbervalue = 5;
                }
                else if CardValue::eight == self.value{
                    numbervalue = 6;
                }
                else if CardValue::seven == self.value{
                    numbervalue = 7;
                }
                else if CardValue::six == self.value{
                    numbervalue = 8;
                }
                else if CardValue::five == self.value{
                    numbervalue = 9;
                }
                else if CardValue::four == self.value{
                    numbervalue = 10;
                }
                else if CardValue::three == self.value{
                    numbervalue = 11;
                }
                else if CardValue::two == self.value{
                    numbervalue = 12;
                }
                else{
                    panic!("aaa");
                }
                
            }
            
            
            numbervalue * 4 + self.suitvalue() as usize
            
        }
        
        
        pub fn suitvalue(&self) -> u16{
            
            if self.suit == CardSuit::clubs{
                return 0;
            }
            else if self.suit == CardSuit::diamonds{
                return 1;
            }
            else if self.suit == CardSuit::hearts{
                return 2;
            }
            else{
                return 3;
            }
            
            
        }
        
        
        pub fn new_random_card() -> Card{
            
            use rand::Rng;
            
            let mut rng = rand::thread_rng();
            
            let effectnumb = rng.gen_range(0, 7);
            let effect;
            {
                if effectnumb == 0{
                    effect = CardEffect::blackjackgame;
                }
                else if effectnumb == 1{
                    effect = CardEffect::pokergame;
                }
                else if effectnumb == 2{
                    effect = CardEffect::dropsquare;
                }
                else if effectnumb == 3{
                    effect = CardEffect::raisesquare;
                }
                else if effectnumb == 4{
                    effect = CardEffect::makepoolgame;
                }
                else if effectnumb == 5{
                    effect = CardEffect::backtobackturns;
                }
                else if effectnumb == 6{
                    effect = CardEffect::halvetimeleft;
                }
                else{
                    panic!("not in the range generated");
                }
            }
            
            //let effect = CardEffect::makepoolgame;
            
            
            let valuenumb = rng.gen_range(0, 13);
            let value;
            {
                
                if valuenumb == 0{
                    value = CardValue::ace;
                }
                else if valuenumb == 1{
                    value = CardValue::two;
                }
                else if valuenumb == 2{
                    value = CardValue::three;
                }
                else if valuenumb == 3{
                    value = CardValue::four;
                }
                else if valuenumb == 4{
                    value = CardValue::five;
                }
                else if valuenumb == 5{
                    value = CardValue::six;
                }
                else if valuenumb == 6{
                    value = CardValue::seven;
                }
                else if valuenumb == 7{
                    value = CardValue::eight;
                }
                else if valuenumb == 8{
                    value = CardValue::nine;
                }
                else if valuenumb == 9{
                    value = CardValue::ten;
                }
                else if valuenumb == 10{
                    value = CardValue::jack;
                }
                else if valuenumb == 11{
                    value = CardValue::queen;
                }
                else if valuenumb == 12{
                    value = CardValue::king;
                }
                else{
                    panic!("not in the generated range");
                }
                
            }
            
            
            let suitvalue = rng.gen_range(0,4);
            let suit;
            
            if suitvalue == 0{
                suit = CardSuit::diamonds;
            }
            else if suitvalue == 1{
                suit = CardSuit::clubs;
            }
            else if suitvalue == 2{
                suit = CardSuit::hearts;
            }
            else if suitvalue == 3{
                suit = CardSuit::spades;
            }
            else{
                panic!("not in teh genereated range");
            }
            
            Card{
                value: value,
                suit: suit,
                effect: effect,
            }
            
        }
        
    }
    