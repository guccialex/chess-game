

use std::collections::HashMap;
use serde::{Serialize, Deserialize};



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
    
    //the card game if it exists
    cardgame: Option<CardGame>,
    
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
            
            cardgame: None,
        }
        
        
        
    }
    
    //this player draws a card
    pub fn draw_card(&mut self, playerid: u8) -> Option<u16>{
        
        
        //if this player still has room in their hand to draw cards
        if self.get_cards_in_hand(playerid).len() < self.maxhandsize{
            
            //draw a card from the deck, if there is no card in the deck
            //create a random card in the deck then draw
            if self.deck.is_empty(){
                
                let cardid = self.totalcards;
                self.totalcards += 1;
                let newcard = Card::new_random_card();
                
                self.cards.insert( cardid, newcard );
                self.deck.push(cardid);
            }
            
            if let Some(cardid) = self.deck.pop(){
                
                self.hands.get_mut(&playerid).unwrap().push(cardid);
                return Some(cardid);
            }
            
            panic!("why no card ppopped>?");
        }
        else{
            
            return None;
        }
        
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
        
        if let Some(cardgame) = &self.cardgame{
            
            let mut toreturn = Vec::new();
            
            for cardid in cardgame.get_card_ids_in_hand(1){
                toreturn.push(cardid);
            }
            for cardid in cardgame.get_card_ids_in_hand(2){
                toreturn.push(cardid);
            }
            for cardid in cardgame.get_card_ids_in_river(){
                toreturn.push(cardid);
            }
            
            return  Some( toreturn ) ;
            
        }
        else{
            return None;
        }
        
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
        
        let mut fieldsize = 0;
        
        for (playerid, hand) in &self.hands{
            
            let mut curcardpos = 0;
            
            for cardidinhand in hand{
                
                curcardpos += 1;
                
                if cardidinhand == &cardid{
                    
                    field = *playerid;
                    positioninfield = curcardpos;
                    fieldsize = hand.len();
                }
            }
        }
        
        if let Some(cardgame) = &self.cardgame{
            
            
            let cardsinplayer1hand = cardgame.get_card_ids_in_hand(1);
            let cardsinplayer2hand = cardgame.get_card_ids_in_hand(2);
            let cardsinriver = cardgame.get_card_ids_in_river();
            
            
            
            let mut curcardpos = 0;
            for curcardid in cardsinplayer1hand.clone(){
                curcardpos += 1;
                if cardid == curcardid{
                    field = 3;
                    positioninfield = curcardpos;
                    fieldsize = cardsinplayer1hand.len();
                }
            }
            
            
            let mut curcardpos = 0;
            for curcardid in cardsinplayer2hand.clone(){
                curcardpos += 1;
                if cardid == curcardid{
                    field = 4;
                    positioninfield = curcardpos;
                    fieldsize = cardsinplayer2hand.len();
                }
            }
            
            
            let mut curcardpos = 0;
            for curcardid in cardsinriver.clone(){
                curcardpos += 1;
                if cardid == curcardid{
                    field = 5;
                    positioninfield = curcardpos;
                    fieldsize = cardsinriver.len();
                }
            }
            
        }
        
        
        (field,positioninfield,fieldsize as u8)
        
        
    }
    
    
    
    
    //get the player to try to play a card in the game
    //return if we successful
    pub fn play_card(&mut self, playerid: u8, cardid: u16) -> bool{
        
        let card = self.get_card_unsafe(cardid).clone();
        
        //if there is a card game
        if let Some(cardgame) = &mut self.cardgame{
            
            //play the card in the game
            cardgame.play_card(playerid, (cardid,card));
            
            self.remove_card_from_hand(playerid, cardid);
            
            return true;
            
        };
        
        false
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
    
    fn remove_card_from_hand(&mut self, playerid: u8, cardid: u16){
        
        let muthand = self.hands.get_mut(&playerid).unwrap();
        
        let mut removedcard: Option<&u16> = None;
        
        //remove the card from the players hand
        muthand.retain(|cardidinhand| {
            
            let delete = {
                cardid == *cardidinhand
            };
            
            !delete
        });
        
    }
    
    
    //get a card from the deck
    fn get_card_from_deck(&mut self) -> (u16, Card){
        
        //draw a card from the deck, if there is no card in the deck
        //create a random card in the deck then draw
        if self.deck.is_empty(){
            
            let cardid = self.totalcards;
            self.totalcards += 1;
            let newcard = Card::new_random_card();
            
            self.cards.insert( cardid, newcard );
            self.deck.push(cardid);
        }
        
        if let Some(cardid) = self.deck.pop(){
            
            let card = self.get_card_unsafe(cardid);
            
            return (cardid, card) ;
        }
        
        
        panic!("why no card in the deck after adding to it");
        
    }
    
    
    
    //start a poker game with the given players
    pub fn start_poker_game(&mut self, player1: u8, player2:u8){
        
        //create the river to be passed in
        let mut river = Vec::new();
        
        for x in 0..5{
            river.push( self.get_card_from_deck() );
        }
        
        self.cardgame = Some( CardGame::new_poker(river) );
    }
    
    
    
    pub fn start_blackjack_game(&mut self, player1: u8, player2:u8){
        self.cardgame = Some( CardGame::new_blackjack() );
    }
    
    //does a game exist, and is this player allowed to play a card in it?
    pub fn is_player_allowed_to_play_card(&self, playerid: u8) -> bool{
        
        if let Some(cardgame) = & self.cardgame{
            return cardgame.is_player_allowed_to_play_card(playerid) ;
        };
        
        return false;
    }
    
    pub fn is_player_forced_to_play_card(&self, playerid: u8) -> bool{
        
        if let Some(cardgame) = & self.cardgame{
            return cardgame.must_player_play_card(playerid);
        };
        
        return false;
    }
    
    
    
    pub fn does_player_own_card(&self, playerid: u8, cardid: u16) ->  bool{
        
        //for each player
        for (cardownerid, cardidlist) in &self.hands{
            
            //for each card in that players hand
            for playerscardid in cardidlist{
                
                //if that card in the hand matches the card passed in
                if playerscardid == &cardid{
                    
                    //if the owner of that card matches the player inputted
                    if cardownerid == &playerid{
                        
                        return true;
                        
                    };
                    
                };
                
            };
            
        };
        
        
        return false;
        
        
        
    }
    
    
    
    //if a player has won the card game, end the game
    //get the cards, and give them to the winning player
    pub fn tick(&mut self){
        
        let mut cardgameended = false;
        
        //if theres a cardgame going on
        if let Some(cardgame) = &mut self.cardgame{
            
            //if the cardgame is finished
            if cardgame.is_game_over(){
                
                //get the winner and the rewards
                let (winnerid, rewards, toremovefromgame) = cardgame.get_winner_rewards().unwrap();
                

                //for every card won, push its id into the winners hand
                for (cardid, _) in rewards{
                    self.hands.get_mut(&winnerid).unwrap().push(cardid);
                }

                for (cardid, _) in toremovefromgame{
                    self.cards.remove(&cardid);
                }


                cardgameended = true;
            }
        }

        if cardgameended{
            self.cardgame = None;
        }


        
    }
    
    
    
}




//a card game
#[derive(Clone, Serialize, Deserialize)]
pub struct CardGame{
    
    //the value of the cards in the hands of the players
    playerscards: HashMap<u8, Vec<(u16, Card)>>,
    
    //the cards in the middle of the board
    //is left as empty if there is no river (like in black jack)
    river: Vec<(u16, Card)>,
    
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
    
    
    
    pub fn suitvalue(&self) -> u16{
        
        if self.suit == CardSuit::diamonds{
            return 0;
        }
        else if self.suit == CardSuit::clubs{
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
        
        let effectnumb = rng.gen_range(0, 5);
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
            else{
                panic!("not in the range generated");
            }
        }
        
        let effect = CardEffect::makepoolgame;
        
        
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
