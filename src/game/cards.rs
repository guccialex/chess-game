use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cards{

    //the cards at the top of each pile
    piles: [CardEffect; 4],
    lastcardeffect: Option< String  >,

    usedeffects: Vec<CardEffect>,
}


impl Cards{


    pub fn new() -> Cards{

        let mut piles = [CardEffect::AddSquares(30), CardEffect::Knight, CardEffect::RemoveSquares(10) , CardEffect::ChangeSpeed(100) ];

        let mut usedeffects = Vec::new();

        usedeffects.push( piles[0].clone() );
        usedeffects.push( piles[1].clone() );
        usedeffects.push( piles[2].clone() );
        usedeffects.push( piles[3].clone() );

        Cards{

            piles,
            lastcardeffect: None,

            usedeffects
            
        }

    }


    


    pub fn get_last_effect_texture(&self) -> Option<String>{
        self.lastcardeffect.clone()
    }


    pub fn has_effect_been_used(&self, effect: &CardEffect) -> bool{

        if self.usedeffects.contains( effect ){
            return true
        }

        return false;

    }



    pub fn draw_card_from_pile(&mut self, pile: &u16, x: Vec<&mut dyn EffectTrait>) {

        if let Some(effect) = self.piles.get(*pile as usize).clone(){

            let effect = effect.clone();

            self.set_card_effect( effect.clone() , x );


            //set the effect to replace it


            
            self.usedeffects.push(effect.clone());


            use std::collections::HashSet;

            let mut effects = HashSet::new();
            effects.insert( CardEffect::ChangeSpeed(200) );
            effects.insert( CardEffect::HalveTimeLeft );
            effects.insert( CardEffect::IntoFlicks );
            effects.insert( CardEffect::Knight );
            effects.insert( CardEffect::TurnsTimed(200) );
            effects.insert( CardEffect::RemoveSquares(20) );
            effects.insert( CardEffect::AddSquares(20) );
            effects.insert( CardEffect::BackToBackTurns );




            for effect in effects{

                //if the effect hasnt been used before
                if ! self.has_effect_been_used( &effect ){

                    self.piles[*pile as usize] = effect;

                    return ();
                }
            }



        }
        else{
            panic!("that card pile doesnt exist");
        }
    }

            
    pub fn set_random_card_effect(&mut self, x: Vec<&mut dyn EffectTrait>) {

        //let effect = CardEffect::get_joker_card_effect();
        let effect = CardEffect::RemoveSquares(2);
        self.set_card_effect(effect, x);
    }


    //set the card effect onto these structs that implement effecttrait
    pub fn set_card_effect(&mut self, effect: CardEffect, x: Vec<&mut dyn EffectTrait>) {
        //log::info!("pebis");

        log::info!("applying effect {:?}", effect.get_card_texture_location());

        for y in x{
            y.apply_effect( effect.clone() );
        }

        self.lastcardeffect = Some( effect.get_card_texture_location() );
    }



    //get the card effects
    //that are in these structs that implement effect trait
    fn get_card_effects(x: Vec<& dyn EffectTrait>) -> Vec<CardEffect>{

        let mut toreturn = Vec::new();

        for y in x{
            toreturn.extend( y.get_effects() );
        }

        toreturn
    }


    pub fn get_active_card_effect_textures(x: Vec<& dyn EffectTrait>) -> Vec<String>{

        let mut toreturn = Vec::new();
    
        for effect in Cards::get_card_effects(x){
    
            toreturn.push( effect.get_card_texture_location() );
        }
    
        return toreturn;
    }


    pub fn get_card_pile_textures(&self) -> [String; 4]{

        [
            self.piles[0].get_card_texture_location(),
            self.piles[1].get_card_texture_location(),
            self.piles[2].get_card_texture_location(),
            self.piles[3].get_card_texture_location(),
        ]

    }


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


//the turn manager and the board engine implement these traits to apply and get the effects on them

//a list of card effects





pub trait EffectTrait{

    fn apply_effect(&mut self, effect: CardEffect);

    fn get_effects(&self) -> Vec<CardEffect>;
}



//a list of objects that represents 

//the game gets the list of cards active
//and applies cards to apply





//the effect of the card it can have
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardEffect{
    
    BackToBackTurns, 
    HalveTimeLeft,
    TurnsTimed(u32),    
    AddChessPieces,
    AddCheckersPieces,
    TurnsUntilDrawAvailable(u32),
    SplitPieceIntoPawns,
    Checkerify,
    Chessify,
    Knight,
    RemoveSquares(u32),
    AddSquares(u32),
    ChangeSpeed(u32),
    LevelPieces,
    AddRandomPieces(u32),
    TiltActions(u32),
    SplitIntoPawns,
    MakeCheckers,
    IntoFlicks,
    KingsReplaced(bool),
    LossWithoutKing(bool),
    PawnsPromoted(bool),    


    //delay actions by X moves
    //pieces actions are set and displayed but dont occur until the next move is placed
    //maybe implent this later, its not straightforward, or straightforward how interesting this would be
    //DelayAction(u32),
    //change how many ticks it takes for a piece to move 1 square
    //should speed be a component of "fullaction"?
    //no. that complicates, beyond simplicity the ability to check if a move is available by whether its in a list
    //(or i can change the EQ implementation on my fullaction to not consider speed if its the same)
    //increase the piece to the next level (pawn to knight to bishop to rook to queen)
    //give both player x new random pieces at different points on the board
    //tilt all actions by this amount
    //f32 is not serializable
    //so by 1/64 full rotations
    //split a random piece from both players pieces into pawns
    //9 random and adjacent squares on the board will drop in 3 turns (they're color coded for how long until dropping)
    //for X turns, all moves turn into flicks (lift and move upwards flick with power proportional to distance)
    //(slides, grounded flicks with power proportional to distance)
}


impl CardEffect{  
    
    /*
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
    */
    
    //card texture 
    fn get_card_texture_location(&self) -> String{
        

        
        match self{
            CardEffect::BackToBackTurns => format!("backtoback.png"), 
            CardEffect::HalveTimeLeft => format!("halvetimeleft.png"),
            CardEffect::TurnsTimed(u32) => format!("turnstimed.png"),
            CardEffect::AddChessPieces => format!("addchesspieces.png"),
            CardEffect::AddCheckersPieces  => format!("addcheckerspieces.png"),
            CardEffect::TurnsUntilDrawAvailable(turns) => format!("{:?}turnsuntildraw.png", turns),
            CardEffect::SplitPieceIntoPawns => format!("splitpieceintopawns.png"),
            CardEffect::Checkerify => format!("checkerify.png"),
            CardEffect::Chessify => format!("chessify.png"),
            CardEffect::Knight => format!("knight.png"),
            CardEffect::RemoveSquares(u32)=> format!("droppedsquares.png"),
            CardEffect::AddSquares(u32)=> format!("addsquares.png"),
            CardEffect::ChangeSpeed(u32)=> format!("slower.png"),
            CardEffect::LevelPieces=> format!("XX.png"),
            CardEffect::AddRandomPieces(u32)=> format!("XX.png"),
            CardEffect::TiltActions(u32)=> format!("XX.png"),
            CardEffect::SplitIntoPawns=> format!("XX.png"),
            CardEffect::MakeCheckers=> format!("XX.png"),
            CardEffect::IntoFlicks=> format!("intoflick.png"),
            CardEffect::KingsReplaced(bool) => format!("kingsreplaced.png"),
            CardEffect::LossWithoutKing(bool) => format!("losswithoutking.png"),
            CardEffect::PawnsPromoted(bool)=> format!("pawnspromoted.png"),

        }
        
        
    }
    
}

