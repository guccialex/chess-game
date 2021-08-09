
use serde::{Serialize, Deserialize};





//the effect of the card it can have
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardEffect{
    
    
    BackToBackTurns, 
    
    HalveTimeLeft,
    
    MakePoolGame,
    
    TurnsTimed(u32),
    
    //what other game effects?
    RemoveSquares(u32),
    
    RaiseSquares(u32),
    
    
    //add all the chess pieces to the game
    AddChessPieces,
    
    AddCheckersPieces,
    
    
    //how many turns until the deck can be drawn from again
    TurnsUntilDrawAvailable(u32),
    
    //split a piece into multiple pawns
    SplitPieceIntoPawns,
    
    
    Checkerify,


    Chessify,
    
    //give all non pieces with a value greater than 1 the abilities of a knight
    Knight,
    
    
    KingsReplaced,
    LossWithoutKing,
    PawnsPromoted,
    
}


impl CardEffect{  
    
    //get a random card effect playable on the board
    pub fn get_joker_card_effect() -> CardEffect{
        
        use rand::Rng;
        
        let mut jokereffects = Vec::new();
        
        
        jokereffects.push(CardEffect::BackToBackTurns);
        jokereffects.push(CardEffect::HalveTimeLeft);
        //jokereffects.push(CardEffect::MakePoolGame);
        jokereffects.push(CardEffect::TurnsTimed(60) );
        jokereffects.push(CardEffect::RaiseSquares(11));
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
    pub fn get_card_texture_location(&self) -> String{
        
        
        match self{
            
            CardEffect::MakePoolGame => format!("poolgame.png"),
            
            CardEffect::BackToBackTurns => format!("backtoback.png"),
            
            CardEffect::HalveTimeLeft =>format!("halvetimeleft.png"),
            
            CardEffect::RaiseSquares(_) =>format!("raisedsquares.png"),
            
            CardEffect::RemoveSquares(_) => format!("droppedsquares.png"),
            
            CardEffect::AddChessPieces => format!("addchesspieces.png"),
            
            CardEffect::AddCheckersPieces => format!("addcheckerspieces.png"),
            
            CardEffect::TurnsTimed(_) => format!("turnstimed.png"),
            
            CardEffect::TurnsUntilDrawAvailable(turns) => format!("{:?}turnsuntildraw.png", turns),
            
            CardEffect::SplitPieceIntoPawns => format!("splitpieceintopawns.png"),
            
            CardEffect::Checkerify => format!("checkerify.png"),
            
            CardEffect::Knight => format!("knight.png"),
            
            CardEffect::KingsReplaced => format!("kingsreplaced.png"),
            
            CardEffect::LossWithoutKing => format!("losswithoutking.png"),
            
            CardEffect::PawnsPromoted => format!("pawnspromoted.png"),

            CardEffect::Chessify => format!("chessify.png"),
        }
        
    }
    
}








#[derive(Serialize, Deserialize, Clone)]
pub struct GameEffects{
    
    //the list of card effects of this game
    cardeffects: Vec<CardEffect>,
}

impl GameEffects{
    
    pub fn new() -> GameEffects{
        
        GameEffects{
            cardeffects: Vec::new(),
        }
    }


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


    pub fn set_card_effect(&mut self, card: CardEffect){

        if self.cardeffects.contains(&card){
        }
        else{
            self.cardeffects.push( card );
        }

        self.combine_and_remove_redundant_effects();
    }


    pub fn remove_card_effect(&mut self, card: CardEffect){

        //keep every element that isnt one passed in
        self.cardeffects.retain(|x| x != &card);
    }
    
    
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
    
    
    
    pub fn get_effects_texture_locations(&self) -> Vec<String>{
        
        let mut toreturn = Vec::new();
        
        for effect in &self.cardeffects{
            toreturn.push( effect.get_card_texture_location() );
        }
        
        toreturn
    }
    


    //SETTERS
    
    pub fn decrement_raised_and_dropped_squares(&mut self){

        for effect in self.cardeffects.iter_mut(){

            if let CardEffect::RaiseSquares(value) = effect{
                *value = value.saturating_sub(1);
            }
            
            else if let CardEffect::RemoveSquares(value) = effect{
                *value = value.saturating_sub(1);
            }
        }

    }
    
    
    pub fn decrement_turns_until_draw_available(&mut self){
        
        for effect in self.cardeffects.iter_mut(){
            if let CardEffect::TurnsUntilDrawAvailable(value) = effect{
                *value = value.saturating_sub(1);
            }
        }

    }
    
    
    
    
    //GETTERS
    //assume only one variant of each card effect exists in the list
    
    pub fn is_draw_available(&self) -> bool{
        
        if let Some(turnstill) = self.get_turns_until_draw_available(){
            if turnstill == 0{
                return true;
            }
        }
        
        return false;
    }

    pub fn get_turns_until_draw_available(&self) -> Option<u32>{
        
        for effect in &self.cardeffects{
            if let CardEffect::TurnsUntilDrawAvailable(num) = effect{
                return Some(*num);
            }
        }
        return None;
    }
    
    pub fn get_are_pawns_promoted(&self) -> bool{
        self.cardeffects.contains(&CardEffect::PawnsPromoted)
    }
    
    pub fn get_are_kings_replaced(&self) -> bool{
        self.cardeffects.contains(&CardEffect::KingsReplaced)
    }

    pub fn get_loss_without_king(&self) -> bool{
        self.cardeffects.contains(&CardEffect::LossWithoutKing)
    }
    
    pub fn get_double_turns(&self) -> bool{
        self.cardeffects.contains(&CardEffect::BackToBackTurns)
    }
    
    pub fn get_knightified(&self) -> bool{
        self.cardeffects.contains(&CardEffect::Knight)
    }
    
    pub fn get_raised_squares(&self) -> u32{
        
        for effect in &self.cardeffects{
            if let CardEffect::RaiseSquares(num) = effect{
                return *num;
            }
        }
        return 0;
    }
    
    pub fn get_dropped_squares(&self) -> u32{
        
        for effect in &self.cardeffects{
            if let CardEffect::RemoveSquares(num) = effect{
                return *num;
            }
        }        
        return 0;
    }
    
    pub fn get_turn_length(&self) -> Option<u32>{
        
        for effect in &self.cardeffects{
            if let CardEffect::TurnsTimed(length) = effect{
                return Some( *length );
            }
        }

        return None;
    }
    
}

