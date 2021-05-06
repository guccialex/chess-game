
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use std::collections::HashSet;





//the actions that are allowed by this piece






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
    
    
    //how about a card that lets all pieces capture THROUGH all pieces
    //like all capture slides can go as far as wanted
    //a card that skews each movement like half of a 1/8th rotation


    
    //set the rules to 
    
    
    
    /*
    add the chess pieces to the game
    add checkers pieces to the game
    
    set the rules of the game to:
    
    loss without pieces
    */
    
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
    
    
    
    pub fn get_game_effect_names(&self) -> Vec<String>{
        
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














//a struct to manage when it is every players turn
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TurnManager{
    
    
    totalturnsid: u16,
    
    //the turn ID
    //the id of the player for that turn
    //the amount of ticks left for this turn
    //the turn ID that is going next after this on
    turns: HashMap<u16, (u8, u32, u16)>,
    
    //the id of the turn it currently is
    currentturn: u16,
    
    //the amount of ticks spent on this turn
    currentturntick: u32,
    
    
    //how many ticks its been since this player took an action
    tickssincelastaction: HashMap<u8, u32>,
    
    //how long until the player can take their next action after performing it
    cooldown: Option<HashMap<u8, u32>>,
    
    
    //the total amount of ticks a player has left
    playertimeleft: HashMap<u8, i32>,
    
    
    //how many turns there have been so far
    turncounter: u16,
    
    
    //if the turn changed this tick, if this is the first tick of a new turn
    turnchanged: bool,
}



impl TurnManager{
    
    pub fn new_two_player(player1: u8, player2: u8) -> TurnManager{
        
        
        let mut turns = HashMap::new();
        
        
        turns.insert(0, (player1, 100000, 1) );
        turns.insert(1, (player2, 100000, 0) );
        
        
        let mut tickssincelastaction = HashMap::new();
        
        tickssincelastaction.insert(player1, 0);
        tickssincelastaction.insert(player2, 0);
        
        
        let mut playertimeleft = HashMap::new();
        
        playertimeleft.insert( player1, 10000 );
        playertimeleft.insert( player2, 10000 );
        
        
        
        TurnManager{
            totalturnsid: 0,
            turns: turns,
            currentturn: 0,
            
            //turn 0 and 1 were used for the setup of the first two turns
            currentturntick: 2,
            
            tickssincelastaction: tickssincelastaction,
            
            cooldown: None,
            playertimeleft: playertimeleft,
            
            turncounter: 0,
            
            turnchanged: true,
        }
        
    }
    
    
    
    //progress timewards
    pub fn tick(&mut self, playerstake2inrow: bool, tickstotaketurn: Option<u32>) {
        
        
        //if players should take 2 turns in a row
        if playerstake2inrow == true{
            
            //if the players are only taking 1 turn in a row
            if self.turns.len() == 2{
                
                //get the length of the turns currently
                let (_, length, _) = self.turns.get(&0).unwrap();
                let length = *length;
                
                
                //clear the turns
                self.turns = HashMap::new();
                
                
                //player 1 goes, then goes again, 
                self.turns.insert(0, (1, length, 1) );
                self.turns.insert(1, (1, length, 2) );
                
                //then 2 goes then goes again
                self.turns.insert(2, (2, length, 3) );
                self.turns.insert(3, (2, length, 0) );
                
            }
            
            
        }
        //if the players shouldnt take more than 1 turn in a row
        else{
            
            
            //if the players are taking more than 1 turn in a row
            if self.turns.len() > 2{
                
                if self.currentturn < 2{
                    self.currentturn = 0;
                }
                if self.currentturn >= 2{
                    self.currentturn = 1;
                }
                
                
                //get the length of the turns currently
                let (_, length, _) = self.turns.get(&0).unwrap();
                let length = *length;
                
                //clear the turns
                self.turns = HashMap::new();
                
                //player 1 goes
                self.turns.insert(0, (1, length, 1) );
                
                //then 2 goes
                self.turns.insert(1, (2, length, 0) );                
            }
            
            
        }
        
        
        
        
        if let Some(newturnlen) = tickstotaketurn{
            //each turn has the length specifieds
            for (turnid, (playerid, turnlen, nextturn)) in self.turns.iter_mut(){
                
                *turnlen = newturnlen;
            }
        }
        else{
            //each turn has unlimited length
            for (turnid, (playerid, turnlen, nextturn)) in self.turns.iter_mut(){
                *turnlen = 100000;
            }
        }
        
        
        
        let (playerid, length, nextturn) = self.turns.get(&self.currentturn).unwrap();
        
        self.currentturntick += 1;
        
        if self.currentturntick > *length{
            
            self.currentturntick = 0;
            self.currentturn = *nextturn;
            
            //if a turn ends
            self.turncounter += 1;
            
            self.turnchanged = true;
        }
        else{
            
            self.turnchanged = false;
        }
        
        
        *self.playertimeleft.get_mut(playerid).unwrap() += -1;
        
        
    }
    
    
    //is it a new turn this tick?
    pub fn did_turn_change(&self) -> bool{
        
        self.turnchanged
    }
    
    pub fn get_turn_number(&self) -> u16{
        
        self.turncounter
    }
    
    //this player took a turn action
    pub fn player_took_action(&mut self, playerid: u8){
        
        self.currentturntick += 1000000;
        
    }
    
    //get a set of players who currently have a turn
    pub fn get_current_players(&self) -> HashSet<u8>{
        
        let turn = self.turns.get(&self.currentturn).unwrap();
        
        let mut toreturn = HashSet::new();
        
        toreturn.insert(turn.0);
        
        toreturn
    }
    
    //if it is this players turn, return how many ticks they have left in their turn
    pub fn get_ticks_left_for_players_turn(&self, playerid: u8) -> u32{
        
        let (curplayerid, turnlength, _) = self.turns.get(&self.currentturn).unwrap();
        
        if playerid == *curplayerid{
            
            return turnlength - self.currentturntick;
            
        };
        
        return 0;
    }
    
    
    
    //the total amount of ticks this player has left
    pub fn get_players_total_ticks_left(&self, playerid: u8) -> u32{
        
        if let Some(ticksleft) = self.playertimeleft.get(&playerid){
            
            if ticksleft.is_negative(){
                return 0;
            }
            else{
                return *ticksleft as u32;
            }
        }
        
        panic!("this player doesnt have a total ticks count");
    }


    pub fn halve_time_left(&mut self){
        
        for (player, timeleft) in self.playertimeleft.iter_mut(){
            
            *timeleft = *timeleft / 2;
        }
    }
    
}


