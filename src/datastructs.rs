
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use std::collections::HashSet;







//the actions that are allowed by this piece






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
        }
        
        
        *self.playertimeleft.get_mut(playerid).unwrap() += -1;
        
        
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


