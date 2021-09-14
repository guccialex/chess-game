use serde::{Serialize, Deserialize};

use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cards{

    //the cards at the top of each pile
    piles: [CardEffect; 4],
    lastcardeffect: Option< String  >,

    queuedeffects: Vec<CardEffect>,
}


impl Cards{


    pub fn new() -> Cards{

        let mut piles = [CardEffect::AddSquares(30), CardEffect::Knight, CardEffect::RemoveSquares(15) , CardEffect::HalveTimeLeft ];


        let mut randomorder = HashSet::new();

        randomorder.insert( CardEffect::ChangeSpeed(200) );
        randomorder.insert( CardEffect::AddRandomPieces(15) );
        randomorder.insert( CardEffect::AddRandomPieces(15) );
        randomorder.insert( CardEffect::AddRandomPieces(15) );
        randomorder.insert( CardEffect::AddRandomPieces(15) );
        randomorder.insert( CardEffect::AddRandomPieces(15) );
        randomorder.insert( CardEffect::IntoFlicks );
        randomorder.insert( CardEffect::Knight );
        randomorder.insert( CardEffect::TurnsTimed(200) );
        randomorder.insert( CardEffect::RemoveSquares(15) );
        randomorder.insert( CardEffect::AddSquares(30) );
        randomorder.insert( CardEffect::BackToBackTurns );


        let queuedeffects = randomorder.into_iter().collect();

        Cards{

            piles,
            lastcardeffect: None,
            queuedeffects
        }

    }



    pub fn get_last_effect_texture(&self) -> Option<String>{
        self.lastcardeffect.clone()
    }



    pub fn draw_card_from_pile(&mut self, pile: &u16, x: Vec<&mut dyn EffectTrait>) {

        if let Some(effect) = self.piles.get(*pile as usize).clone(){

            let effect = effect.clone();

            self.set_card_effect( effect.clone() , x );


            if let Some(neweffect) = self.queuedeffects.pop(){
                self.piles[*pile as usize] = neweffect;
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

