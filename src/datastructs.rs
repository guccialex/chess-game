
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


    //how many turns until the deck can be drawn from again
    TurnsUntilDrawAvailable(u32),



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
        jokereffects.push(CardEffect::TurnsTimed(30) );
        jokereffects.push(CardEffect::RaiseSquares(7));
        jokereffects.push(CardEffect::RemoveSquares(7));
        
        
        
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
            
            CardEffect::RemoveSquares(_) => format!("dropedsquares.png"),

            CardEffect::AddChessPieces => format!("addchesspieces.png"),

            CardEffect::TurnsTimed(_) => format!("turnstimed.png"),

            CardEffect::TurnsUntilDrawAvailable(turns) => format!("{:?}turnsuntildraw.png", turns),
        }
        
    }
    
    
    
}








#[derive(Serialize, Deserialize, Clone)]
pub struct GameEffects{
    
    //if players lose when they dont have any pieces left
    losswithoutpieces: bool,
    
    //if players lose when they dont have a king
    losswithoutking: bool,
    
    kingsreplaced: bool,
    
    pawnspromoted: bool,
    
    doubleturns: bool,
    
    poolgame: bool,
    
    totalraisedsquares: u32,
    
    totalremovedsquares: u32,
    
    turnlength: Option<u32>,


    turnsuntildrawavailable: Option<u32>,
    
}

impl GameEffects{
    
    pub fn new() -> GameEffects{
        
        GameEffects{
            losswithoutpieces: false,
            
            losswithoutking: false,
            
            kingsreplaced: false,
            
            pawnspromoted: false,
            
            doubleturns: false,
            
            poolgame: false,
            
            totalraisedsquares: 0,
            
            totalremovedsquares: 0,
            
            turnlength: None,

            turnsuntildrawavailable: None,
        }
    }


    pub fn get_random_card_effect(&self) -> CardEffect{
        
        loop{

            let mut toreturn = CardEffect::get_joker_card_effect();


            if let CardEffect::BackToBackTurns = toreturn{
                if self.get_double_turns() == true{
                    continue;
                }
            }


            if let CardEffect::RaiseSquares(toraise) = toreturn{

                if self.get_raised_squares() + toraise > 12{
                    continue;
                }
            }


            if let CardEffect::RemoveSquares(todrop) = toreturn{

                if self.get_removed_squares() + todrop > 12{
                    continue;
                }
            }

            if let CardEffect::TurnsTimed(_) = toreturn{

                if let Some(oldticks) = self.get_turn_length(){

                    if oldticks > 15{

                        toreturn = CardEffect::TurnsTimed(oldticks - 10);
                    }
                }
            }




            return toreturn;
        }



    }



    pub fn set_turns_until_draw_available(&mut self, turns: u32){

        self.turnsuntildrawavailable = Some(turns);
    }

    pub fn decrement_turns_until_draw_available(&mut self){

        if let Some(value) = &mut self.turnsuntildrawavailable{

            *value = value.saturating_sub(1);
        }
    }

    pub fn is_draw_available(&self) -> bool{


        if let Some(value) = self.turnsuntildrawavailable{

            if value == 0{
                return true;
            }
        }

        return false;
    }
    
    
    
    
    
    pub fn get_game_effect_names(&self) -> Vec<String>{
        
        //what effects are returned
        
        let mut toreturn = Vec::new();
        
        if self.losswithoutpieces == true{
            toreturn.push("losswithoutpieces.png".to_string());
        }
        if self.losswithoutking == true{
            toreturn.push("losswithoutking.png".to_string());
        }
        if self.kingsreplaced == true{
            toreturn.push("kingsreplaced.png".to_string());
        }
        if self.pawnspromoted == true{
            toreturn.push("pawnspromoted.png".to_string());
        }
        if self.doubleturns == true{
            toreturn.push("backtoback.png".to_string());
        }
        if self.poolgame == true{
            toreturn.push("poolgame.png".to_string());
        }
        if self.totalraisedsquares > 0{
            toreturn.push( format!("raisedsquares.png") );
        }
        if self.totalremovedsquares > 0{
            toreturn.push( format!("droppedsquares.png") );
        }
        if let Some(turnlength) = self.turnlength{
            toreturn.push( format!("turnstimed.png") );
        }
        if let Some(turnsleft) = self.turnsuntildrawavailable{

            if turnsleft != 0{
                toreturn.push( format!("{:?}turnsuntildraw.png", turnsleft) );
            }


        }

        
        toreturn
    }
    
    
    pub fn reset_effects(&mut self){
        *self = GameEffects::new();
    }
    
    
    pub fn add_chess_game_rules(&mut self) {
        self.losswithoutking = true;
        self.pawnspromoted = true;
    }
    
    pub fn add_checkers_game_rules(&mut self){
        self.losswithoutpieces = true;
    }
    
    
    
    pub fn get_loss_without_king(&self) -> bool{
        self.losswithoutking
    }
    
    pub fn get_loss_without_pieces(&self) -> bool{
        self.losswithoutpieces
    }
    
    
    
    //pool game rules
    //(an 8 ball piece should be added to the game)
    //theres an "8 ball" piece
    //if the 8 ball is captured
    //if the player who captured it's opponent has no pieces, the player win
    //if the player who captured it's opponent has pieces, the player loses
    //the player's pieces become the colour of the last piece that hit them (and carry the colour, aka the impulse)
    //and when the 8 ball is captured, get the impulse passed onto it, and the greatest impulse on it, that player
    //is considered to be the one to have sinked it
    /*
    pub fn add_pool_game_rules(&mut self){
        
        
        
    }
    */
    
    
    
    pub fn set_pawns_are_promoted(&mut self){
        self.pawnspromoted = true;
    }
    
    pub fn get_pawns_are_promoted(&self) -> bool{
        self.pawnspromoted
    }
    
    pub fn set_kings_replaced(&mut self){
        self.kingsreplaced = true;
    }
    pub fn get_kings_replaced(&self) -> bool{
        self.kingsreplaced
    }
    
    
    
    pub fn set_double_turns(&mut self){
        self.doubleturns = true;
    }
    pub fn get_double_turns(&self) -> bool{
        self.doubleturns
    }
    
    
    
    
    //set the total raised squares
    pub fn set_raised_squares(&mut self, number: u32){
        self.totalraisedsquares = number;
    }
    pub fn add_raised_squares(&mut self, number: u32){
        self.totalraisedsquares += number;
    }
    pub fn subtract_raised_squares(&mut self, tosubtract: u32){
        self.totalraisedsquares = self.totalraisedsquares.saturating_sub(tosubtract);
    }
    //get the total number of raised squares
    pub fn get_raised_squares(&self) -> u32{
        self.totalraisedsquares    
    }
    
    
    pub fn add_removed_squares(&mut self, number: u32){
        self.totalremovedsquares += number;
    }
    
    pub fn subtract_removed_squares(&mut self, number: u32){
        self.totalremovedsquares = self.totalremovedsquares.saturating_sub(number);
    }
    pub fn set_removed_squares(&mut self, number: u32){
        self.totalremovedsquares = number;
    }
    
    //get the total number of raised squares
    pub fn get_removed_squares(&self) -> u32{
        self.totalremovedsquares
    }
    
    
    
    //a few ways I can have the invarants I want satisfied
    //the setter establishes the invariant
    //the getter makes sure the invariant is delivered
    //the tick function reestablishes the invariants
    
    
    //set the ticks that a player has for their turn
    pub fn set_turn_length(&mut self, ticks: u32){
        self.turnlength = Some(ticks);
    }
    
    
    pub fn get_turn_length(& self) -> Option<u32>{
        
        self.turnlength
    }
    
    
    pub fn card_drawn(&mut self){
        
        self.kingsreplaced = true;
        
        self.pawnspromoted = false;
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


