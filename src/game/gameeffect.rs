use serde::{Serialize, Deserialize};


/*
use turnmanager::TurnManager;
use boardengine::BoardEngine;
*/


//get the card effects
//that are in these structs that implement effect trait
pub fn get_card_effects(x: Vec<& dyn EffectTrait>) -> Vec<CardEffect>{

    let mut toreturn = Vec::new();

    for y in x{
        toreturn.extend( y.get_effects() );
    }

    toreturn
}


pub fn get_card_effect_textures(x: Vec<& dyn EffectTrait>) -> Vec<String>{

    let mut toreturn = Vec::new();

    for effect in get_card_effects(x){

        toreturn.push( effect.get_card_texture_location() );
    }

    return toreturn;
}


//set the card effect onto these structs that implement effecttrait
pub fn set_card_effect( effect: CardEffect, x: Vec<&mut dyn EffectTrait>) -> String{

    log::info!("applying effect {:?}", effect.get_card_texture_location());

    for y in x{
        y.apply_effect( effect.clone() );
    }

    return effect.get_card_texture_location();
}



pub fn draw( x: Vec<&mut dyn EffectTrait>) -> String{

    let effect = CardEffect::get_joker_card_effect();

    let effect = CardEffect::RemoveSquares(2);

    return set_card_effect(effect, x);
}



/*
fn combine_and_remove_redundant_effects(&mut self){

    //the variants are the only ones that could have multiple versions added to this struct
    //so combine them and remove multiple ones

    let mut oldraisesquare : Option<(usize, u32)> = None;
    let mut olddropsquare : Option<(usize, u32)> = None;
    let mut oldturnstimed : Option<(usize, u32)> = None;
    let mut oldturnsuntildraw : Option<(usize, u32)> = None;

    let mut curindex = 0;

    let mut indextoremove: Option<usize> = None;


    //I think I should learn closures. It was hard to ever see a use for them before this
    //because I didnt know how to use them
    //but it seems like to make this cleaner, those might be important
    for effect in self.cardeffects.iter_mut(){

        match effect{

            CardEffect::RaiseSquares(num) =>{

                if let Some( (oldindex, oldvalue) ) = oldraisesquare {

                    *num += oldvalue;
                    indextoremove = Some(oldindex);
                }
                else{
                    oldraisesquare = Some( (curindex, num.clone()) );
                }
            },
            CardEffect::RemoveSquares(num) =>{

                if let Some( (oldindex, oldvalue) ) = olddropsquare {
                    
                    *num += oldvalue;
                    indextoremove = Some(oldindex);
                }
                else{
                    olddropsquare = Some( (curindex, num.clone()) );
                }

            },
            CardEffect::TurnsTimed(num) =>{

                if let Some( (oldindex, oldvalue) ) = oldturnstimed {
                    
                    *num = std::cmp::min(oldvalue, *num);
                    indextoremove = Some(oldindex);
                }
                else{
                    oldturnstimed = Some( (curindex, num.clone()) );
                }

            },
            CardEffect::TurnsUntilDrawAvailable(value) =>{

                if let Some( (oldindex, oldvalue) ) = oldturnsuntildraw {
                    
                    *value += oldvalue;
                    indextoremove = Some(oldindex);
                }
                else{
                    oldturnsuntildraw = Some( (curindex, value.clone()) );
                }
            },
            _ => {},
        };

        curindex += 1;
        
    };


    if let Some(indextoremove) = indextoremove{
        self.cardeffects.remove(indextoremove);
    }


}
*/


/*
*/




/*
pub fn remove_card_effect(&mut self, card: CardEffect){

    //keep every element that isnt one passed in
    self.cardeffects.retain(|x| x != &card);
}
*/


//apply a random card effect
//need a mutable reference to the turn manager
//and the game engine

/*
pub fn get_random_card_effect(&self) -> CardEffect{
    
    for x in 0..10{
        let mut toreturn = CardEffect::get_joker_card_effect();
        
        if self.cardeffects.contains( &toreturn ){
            
            continue;
        };
        
        return toreturn;
    }
    
    //default if no action available
    return CardEffect::RaiseSquares(2);
}
*/


/*
pub fn get_effects_texture_locations(&self) -> Vec<String>{
    
    let mut toreturn = Vec::new();
    
    for effect in &self.get_card_effects(){
        toreturn.push( effect.get_card_texture_location() );
    }
    
    toreturn
}
*/






pub trait EffectTrait{

    fn apply_effect(&mut self, effect: CardEffect);

    fn get_effects(&self) -> Vec<CardEffect>;
}








//the effect of the card it can have
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardEffect{
    
    BackToBackTurns, 
    HalveTimeLeft,
    
    TurnsTimed(u32),

    
    
    AddChessPieces,
    AddCheckersPieces,
    
    
    TurnsUntilDrawAvailable(u32),
    

    //split a piece into multiple pawns
    SplitPieceIntoPawns,
    Checkerify,
    Chessify,
    Knight,
    



    RemoveSquares(u32),

    //increase the size of the board by this many pieces
    AddSquares(u32),

    //delay actions by X moves
    //pieces actions are set and displayed but dont occur until the next move is placed
    //maybe implent this later, its not straightforward, or straightforward how interesting this would be
    //DelayAction(u32),

    //change how many ticks it takes for a piece to move 1 square
    //should speed be a component of "fullaction"?
    //no. that complicates, beyond simplicity the ability to check if a move is available by whether its in a list
    //(or i can change the EQ implementation on my fullaction to not consider speed if its the same)
    ChangeSpeed(u32),


    //increase the piece to the next level (pawn to knight to bishop to rook to queen)
    LevelPieces,


    //give both player x new random pieces at different points on the board
    AddRandomPieces(u32),


    //tilt all actions by this amount
    //f32 is not serializable
    //so by 1/64 full rotations
    TiltActions(u32),


    //split a random piece from both players pieces into pawns
    SplitIntoPawns,


    MakeCheckers,


    //9 random and adjacent squares on the board will drop in 3 turns (they're color coded for how long until dropping)
    MakeBomb,
    

    //for X turns, all moves turn into flicks (lift and move upwards flick with power proportional to distance)
    //(slides, grounded flicks with power proportional to distance)
    MovesBecomeFlicks(u32),


    //chess 960 distribution






    KingsReplaced(bool),

    LossWithoutKing(bool),

    PawnsPromoted(bool),    
}


impl CardEffect{  
    
    //get a random card effect playable on the board
    fn get_joker_card_effect() -> CardEffect{
        
        use rand::Rng;
        
        let mut jokereffects = Vec::new();
        
        
        jokereffects.push(CardEffect::BackToBackTurns);
        jokereffects.push(CardEffect::HalveTimeLeft);
        //jokereffects.push(CardEffect::MakePoolGame);
        jokereffects.push(CardEffect::TurnsTimed(60) );
        jokereffects.push(CardEffect::RemoveSquares(11));
        jokereffects.push(CardEffect::SplitPieceIntoPawns);
        jokereffects.push(CardEffect::Checkerify);
        jokereffects.push(CardEffect::Chessify );
        jokereffects.push(CardEffect::Knight);

        //jokereffects.push(CardEffect::SwapPawns);
        
        let mut rng = rand::thread_rng();
        let effectnumb = rng.gen_range(0, jokereffects.len() );
        let jokereffect = jokereffects[effectnumb].clone();
        
        jokereffect    
    }
    
    //card texture 
    fn get_card_texture_location(&self) -> String{
        
        
        return format!("halvetimeleft.png");
        /*
        match self{
            
            CardEffect::BackToBackTurns => format!("backtoback.png"),
            
            CardEffect::HalveTimeLeft =>format!("halvetimeleft.png"),
            
            CardEffect::RemoveSquares(_) => format!("droppedsquares.png"),
            
            CardEffect::AddChessPieces => format!("addchesspieces.png"),
            
            CardEffect::AddCheckersPieces => format!("addcheckerspieces.png"),
            
            CardEffect::TurnsTimed(_) => format!("turnstimed.png"),
            
            CardEffect::TurnsUntilDrawAvailable(turns) => format!("{:?}turnsuntildraw.png", turns),
            
            CardEffect::SplitPieceIntoPawns => format!("splitpieceintopawns.png"),
            
            CardEffect::Checkerify => format!("checkerify.png"),
            
            CardEffect::Knight => format!("knight.png"),
            
            CardEffect::KingsReplaced(x) => format!("kingsreplaced.png"),
            
            CardEffect::LossWithoutKing(x) => format!("losswithoutking.png"),
            
            CardEffect::PawnsPromoted(x) => format!("pawnspromoted.png"),

            CardEffect::Chessify => format!("chessify.png"),
        }
        */
        
    }
    
}