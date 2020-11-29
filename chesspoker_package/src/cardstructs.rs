

use std::collections::HashMap;
use serde::{Serialize, Deserialize};



//the manager for all card related data and structs in the game
#[derive(Clone, Serialize, Deserialize)]

pub struct CardsInterface{

    totalcards: u16,

    cards: HashMap<u16, Card>,
    
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

            deck: Vec::new(),

            cardgame: None,
        }



    }
    

    //get the card by ID and panic if it doesnt have it
    pub fn get_card_unsafe(&self, cardid: u16) -> Card{

        self.cards.get(&cardid).unwrap().clone()

    }

    
    pub fn get_cards_in_hand(&self, playerid: u8) -> Vec<Card>{

        let handcardids = self.hands.get(&playerid).unwrap();

        let mut hand = Vec::new();


        for cardid in handcardids{

            let card = self.get_card_unsafe(*cardid);

            hand.push(card);

        }



        return hand;

        
        
    }
    
    
    pub fn get_cards_in_game(&self) -> Option< (Vec<Card>, Vec<Card>, Vec<Card>) >{
        
        if let Some(cardgame) = &self.cardgame{
                
            let player1hand = cardgame.playerscards.get(&1).unwrap().clone();
            
            let river = cardgame.river.clone();
            
            let player2hand = cardgame.playerscards.get(&2).unwrap().clone();
            
            
            return  Some( ( player1hand, river, player2hand) ) ;
            
        }
        else{
            return None;
        }

    }


    //get the player to try to play a card in the game
    //return if we successful
    pub fn play_card(&mut self, playerid: u8, cardid: u16) -> bool{

        let card = self.get_card_unsafe(cardid).clone();

        //if there is a card game
        if let Some(cardgame) = &mut self.cardgame{

            //play the card in the game
            cardgame.play_card(playerid, card);


            //and remove it from the players hand
            //(and, unimplemented, remove it from the list of cards)
            let muthand = self.hands.get_mut(&playerid).unwrap();

            let mut removedcard = false;
        
            //remove the card from the players hand
            muthand.retain(|cardidinhand| {
                
                let delete = {
                    cardid == *cardidinhand
                };


                //if this card is being deleted from the hand
                if (delete == false){
                    removedcard = true;
                }
                
                !delete
            });


            //if an element was removed, a card was played, and so
            //return true, otherwise return false
            if removedcard{

                return(false);
            };
    

        };



        false

    }


    //remove this card from that players hand
    //if that player has that card in hand (should I panic if they dont?)
    pub fn remove_card_from_hand(&mut self, playerid: u8, cardid: u16){

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

    
        
    //start a poker game with the given players
    pub fn start_poker_game(&mut self, player1: u8, player2:u8){
        
        
        //create the river to be passed in
        let mut river = Vec::new();
        
        for x in 0..5{
            river.push( Card::new_random_card() );
        }
        
        
        self.cardgame = Some( CardGame::new_poker(river)  );
        
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


    //generate a new card and give it to this player
    //give this player a random card
    pub fn give_new_random_card(&mut self, playerid: u8){
        
        let cardid = self.totalcards;
        self.totalcards += 1;
        
        let thecard = Card::new_random_card();
        
        //put it into the list of cards
        self.cards.insert(cardid, thecard);
        
        self.hands.get_mut(&playerid).unwrap().push(cardid);
        
    }
    
}




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


    //if the card is an unknown card
    isunknown: bool,
    
    
    
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
            return 1;
        }
        else if self.suit == CardSuit::clubs{
            return 2;
        }
        else if self.suit == CardSuit::hearts{
            return 3;
        }
        else{
            return 4;
        }


    }

    
    pub fn new_random_card() -> Card{

        Card{
            value: CardValue::ace,
            suit: CardSuit::spades,
            effect: CardEffect::dropsquare,
            isunknown: false
        }

    }

    pub fn new_unknown_card() -> Card{


        Card{

            value: CardValue::ace,
            suit: CardSuit::spades,
            effect: CardEffect::raisesquare,
            isunknown: true

        }



    }
    
}
