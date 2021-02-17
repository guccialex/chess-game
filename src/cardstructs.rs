use std::collections::HashMap;
use serde::{Serialize, Deserialize};





//the manager for all card related data and structs in the game
#[derive(Clone, Serialize, Deserialize)]

pub struct CardsInterface{
}



use rand::Rng;


impl CardsInterface{
    
        
    //get a random card effect playable on the board
    pub fn get_joker_card_effect() -> CardEffect{

        let mut jokereffects = Vec::new();
        jokereffects.push(CardEffect::backtobackturns);
        jokereffects.push(CardEffect::halvetimeleft);
        //jokereffects.push(CardEffect::blackjackgame);
        //jokereffects.push(CardEffect::pokergame);
        jokereffects.push(CardEffect::makepoolgame);
        
        let mut rng = rand::thread_rng();
        let effectnumb = rng.gen_range(0, jokereffects.len() );
        let jokereffect = jokereffects[effectnumb].clone();
           
        jokereffect    
    }
    
}




//the effect of the card it can have
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardEffect{
    
    
    //joker effects
    
    backtobackturns, 
    
    halvetimeleft,
    
    makepoolgame,





    //non joker effects
    
    //this card can remove a square from the board
    dropsquare,
    
    //this card can raise a square to not be able to be moved past by another piece
    raisesquare,
    

    noeffect,

}

