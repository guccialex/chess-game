

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


    //get a card that is to be played when drawn
    pub fn get_joker_card(&mut self) -> u16{

        //the card should be cleared from existing 
        //when the cardinterface is ticked
        let cardid = self.add_random_card_to_game();

        cardid
        //self.cards.get(&cardid).unwrap().clone()
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
        
        
        for (playerid, hand) in &self.hands{
            
            
            let fieldsize = hand.len() as u8;
            
            let mut curcardpos = 0;
            
            for cardidinhand in hand{
                
                curcardpos += 1;
                
                if cardidinhand == &cardid{
                    
                    return (*playerid,curcardpos,fieldsize);
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







use rs_poker;



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
        
        use rs_poker::core::{Hand, Rank, Rankable};

        
        //if the game is over
        if self.roundnumber >= 4{
            
            let mut player1hand = Hand::default();
            let mut player2hand = Hand::default();
            
            
            for (_, card) in &self.player1hand{
                
                player1hand.push( card.get_rs_poker_card() );
            }
            
            for (_, card) in &self.player2hand{
                
                player2hand.push( card.get_rs_poker_card() );
            }
            
            
            //add the cards in the river to both players hands
            for (_, card) in &self.river{
                
                player1hand.push( card.get_rs_poker_card() );
                player2hand.push( card.get_rs_poker_card() );
            }
            
            
            
            let player1handscore = player1hand.rank();
            let player2handscore = player2hand.rank();
            
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
    fn get_rs_poker_card(&self) -> rs_poker::core::Card{
        
        
        let value;// = rs_poker::core::Value::Two;
        let suit;// = rs_poker::core::Suit::Spade;
        
        
        {
            if CardValue::ace == self.value{
                value = rs_poker::core::Value::Ace;
            }
            else if CardValue::king == self.value{
                value = rs_poker::core::Value::King;
            }
            else if CardValue::queen == self.value{
                value = rs_poker::core::Value::Queen;
            }
            else if CardValue::jack == self.value{
                value = rs_poker::core::Value::Jack;
            }
            else if CardValue::ten == self.value{
                value = rs_poker::core::Value::Ten;
            }
            else if CardValue::nine == self.value{
                value = rs_poker::core::Value::Nine;
            }
            else if CardValue::eight == self.value{
                value = rs_poker::core::Value::Eight;
            }
            else if CardValue::seven == self.value{
                value = rs_poker::core::Value::Seven;
            }
            else if CardValue::six == self.value{
                value = rs_poker::core::Value::Six;
            }
            else if CardValue::five == self.value{
                value = rs_poker::core::Value::Five;
            }
            else if CardValue::four == self.value{
                value = rs_poker::core::Value::Four;
            }
            else if CardValue::three == self.value{
                value = rs_poker::core::Value::Three;
            }
            else if CardValue::two == self.value{
                value = rs_poker::core::Value::Two;
            }
            else{
                panic!("aaa");
            }
        }

        {

            if CardSuit::clubs == self.suit{

                suit = rs_poker::core::Suit::Club;
            }
            else if CardSuit::diamonds == self.suit{

                suit = rs_poker::core::Suit::Diamond;
            }
            else if CardSuit::spades == self.suit{

                suit = rs_poker::core::Suit::Spade;
            }
            else if CardSuit::hearts == self.suit{

                suit = rs_poker::core::Suit::Heart;
            }
            else{
                panic!("doesnt have a valid suit");
            }

        }
        
        
        let toreturn = rs_poker::core::Card{
            suit: suit,
            value: value,
        };
        

        toreturn
        
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
        let mut effect;
        {
            if effectnumb == 0{
                effect = CardEffect::pokergame;
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
        
        effect = CardEffect::pokergame;
        
        
        
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
