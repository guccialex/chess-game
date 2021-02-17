

use std::collections::HashMap;
use serde::{Serialize, Deserialize};






//THE DECK CAN ONLY HAVE CARDS WITH JOKER EFFECTS
//THE HAND CAN HAVE EITHER CARDS WITH JOKER EFFECTS OR NON JOKER EFFECTS







//the manager for all card related data and structs in the game
#[derive(Clone, Serialize, Deserialize)]

pub struct CardsInterface{
    
    
}



impl CardsInterface{
    
    
    
    //get a random card effect playable on the board
    pub fn get_joker_card_effect() -> CardEffect{
        
        Card::new_random_joker_card().effect
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
    
    
    //joker effects
    
    backtobackturns, 
    
    halvetimeleft,
    
    //this card can initiate a blackjack game with it as the starting card
    //blackjackgame,
    
    //this card can initiate a poker game with it as the starting card
    //pokergame,
    
    //make the game have pool settings
    makepoolgame,


    //effects
    
    
    



    //non joker effects
    
    //this card can remove a square from the board
    dropsquare,
    
    //this card can raise a square to not be able to be moved past by another piece
    raisesquare,
    
        
    

    noeffect,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardSuit{
    
    diamonds,
    clubs,
    hearts,
    spades
    
    
}



use rand::Rng;


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
    
    
    
    
    fn new_effectless_card() -> Card{
        
        let mut rng = rand::thread_rng();
        
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
        
        {
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
        }
        
        
        Card{
            value: value,
            suit: suit,
            effect: CardEffect::noeffect,
        }
        
    }
    
    
    //get a new joker card, card that can be played on board
    pub fn new_random_joker_card() -> Card{
        
        let mut effectlesscard = Card::new_effectless_card();
        
        
        let mut jokereffects = Vec::new();
        jokereffects.push(CardEffect::backtobackturns);
        jokereffects.push(CardEffect::halvetimeleft);
        //jokereffects.push(CardEffect::blackjackgame);
        //jokereffects.push(CardEffect::pokergame);
        jokereffects.push(CardEffect::makepoolgame);
        
        
        let mut rng = rand::thread_rng();
        let effectnumb = rng.gen_range(0, jokereffects.len() );
        let jokereffect = jokereffects[effectnumb].clone();
        
        
        effectlesscard.effect = jokereffect;
        
        
        effectlesscard
        
    }
    
    
    //get a random card that can be played from the hand
    pub fn new_random_card() -> Card{
        
        let mut effectlesscard = Card::new_effectless_card();
        
        let mut effects = Vec::new();
        effects.push(CardEffect::backtobackturns);
        effects.push(CardEffect::halvetimeleft);
        //effects.push(CardEffect::blackjackgame);
        //effects.push(CardEffect::pokergame);
        effects.push(CardEffect::makepoolgame);

        effects.push(CardEffect::raisesquare);
        effects.push(CardEffect::dropsquare);
        
        
        let mut rng = rand::thread_rng();
        let effectnumb = rng.gen_range(0, effects.len() );
        let effect = effects[effectnumb].clone();
        
        
        effectlesscard.effect = effect;
        
        effectlesscard
        
    }
    
}
