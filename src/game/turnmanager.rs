use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};



//implement this for the turn amnager 
//or like a "effect interface"
//to apply cards, and then get the cards applied
//for both the turn manager and the board state

//apply a card effect
//get game effects

use super::gameeffect::CardEffect;
use super::gameeffect::EffectTrait;


impl EffectTrait for TurnManager{

    fn apply_effect(&mut self, effect: CardEffect){

        match effect{

            CardEffect::BackToBackTurns =>{
                self.turnsinarow = 2;
            },
            CardEffect::HalveTimeLeft =>{
                for (_, timeleft) in self.totaltimeleft.iter_mut(){
                    *timeleft = *timeleft / 2;
                }
            },
            CardEffect::TurnsTimed(ticks) =>{
                self.tickstotaketurn = Some(ticks);
            },
            CardEffect::TurnsUntilDrawAvailable(turns) =>{

                self.turnsuntildraw = Some(turns);


            },
            //half this players turn ticks
            _ => {
                log::info!("cant apply that effect");
                //panic!("card effect not settable");
            }


        }


    }

    fn get_effects(&self) -> Vec<CardEffect>{

        let mut toreturn = Vec::new();

        if let Some(x) = self.tickstotaketurn{

            toreturn.push( CardEffect::TurnsTimed(x) );
        };

        if self.turnsinarow > 1{

            toreturn.push( CardEffect::BackToBackTurns );
        };

        toreturn.push( CardEffect::PawnsPromoted );


        return toreturn;
    }


}

impl TurnManager{

    fn relevant_effects() -> Vec<CardEffect>{

        let mut toreturn = Vec::new();

        /*
        toreturn.push( CardEffect::TurnsUntilDrawAvailable );
        toreturn.push( CardEffect::TurnsTimed(10) );
        */


        toreturn

    }

}




//a struct to manage when it is every players turn
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TurnManager{
    
    turnsinarow: u16,

    //if none, can't draw ever, if option, can draw when at zero
    turnsuntildraw: Option<u32>,

    //if the time to take a turn is limited
    tickstotaketurn: Option<u32>,
    
    //the current player, and how many turns they've had before this one
    currentplayer: u8,
    turnsbefore: u16,

    //how many tick have been spent on the current turn
    currentturntick: u32,
    
    totaltimeleft: HashMap<u8, i32>,
}


impl TurnManager{


    pub fn new_two_player(player1: u8, player2: u8, totalticks: u32, turnsuntildraw: u32) -> TurnManager{
        

        let mut turns = HashMap::new();
        turns.insert(0, (player1, 100000, 1) );
        turns.insert(1, (player2, 100000, 0) );
        

        let mut totaltimeleft = HashMap::new();
        totaltimeleft.insert( player1, totalticks as i32 );
        totaltimeleft.insert( player2, totalticks as i32 );
        
        
        TurnManager{

            turnsinarow: 1,
            tickstotaketurn: Some(100),
            turnsuntildraw: Some(turnsuntildraw),

            currentplayer: 1,
            turnsbefore: 0,
            
            currentturntick: 0,
            
            totaltimeleft,
        }
        
    }


    fn next_turn(&mut self){

        if let Some(untildraw) = &mut self.turnsuntildraw{
            *untildraw = untildraw.saturating_sub(1);
        }

        
        self.turnsbefore += 1;
        

        if self.turnsbefore >= self.turnsinarow{

            self.turnsbefore = 0;
            
            if self.currentplayer == 1{
                self.currentplayer = 2;
            }
            else{
                self.currentplayer = 1;
            }
        }



        self.currentturntick = 0;

    }

    
    //progress timewards
    pub fn tick(&mut self) {

        self.currentturntick += 1;

        if let Some(tickstotaketurn) = self.tickstotaketurn {

            if self.currentturntick > tickstotaketurn{

                self.next_turn();
            }
        }

        *self.totaltimeleft.get_mut(&self.currentplayer).unwrap() += -1;
    }

    
    pub fn can_player_draw(&self, playerid: &u8) -> bool{
        
        if let Some(id) = self.turnsuntildraw{

            if id == 0{
                return true;
            };
        };

        return false;
    }


    pub fn player_drew(&mut self){

        self.turnsuntildraw = Some(3);
    }

    
    //this player took a turn action
    pub fn player_took_action(&mut self, playerid: u8){
        
        self.next_turn();
    }
    

    //get a set of players who currently have a turn
    pub fn get_current_players(&self) -> HashSet<u8>{
        
        let mut toreturn = HashSet::new();

        toreturn.insert( self.currentplayer );
        
        toreturn
    }

    
    //if it is this players turn, return how many ticks they have left in their turn
    pub fn get_ticks_left_for_players_turn(&self, playerid: u8) -> Option<u32>{

        
        if playerid == self.currentplayer{

            if let Some(tickstotaketurn) = self.tickstotaketurn{

                return  Some(  tickstotaketurn.saturating_sub( self.currentturntick ) );
            }
        }
        
        return None;
    }
    
    
    
    //the total amount of ticks this player has left
    pub fn get_players_total_ticks_left(&self, playerid: u8) -> u32{
        
        if let Some(ticksleft) = self.totaltimeleft.get(&playerid){
            
            if ticksleft.is_negative(){
                return 0;
            }
            else{
                return *ticksleft as u32;
            }
        }
        
        panic!("this player doesnt have a total ticks count");
    }


    pub fn turns_until_draw(&self) -> Option<u32>{

        self.turnsuntildraw
    }

    
}


