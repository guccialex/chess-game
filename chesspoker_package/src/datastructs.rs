
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use std::collections::HashSet;


use super::PieceAction;











#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerInput{
    
    //playing a card on the active board
    playcardonboard(u16),

    //playing a card with a target of a piece
    playcardonpiece(u16, u16),

    //playing a card with a target of a board square
    playcardonsquare(u16, u16),
    
    //perform an action on a piece
    pieceaction(u16, PieceAction),
    
}





//given a value, and a list of ranges
//returns whether the value is in this range or not
fn is_in_range(ranges: Vec< (u32,u32) >, value: u32) -> bool{
    
    
    for (currangestart, currangeend) in ranges{
        
        if value >= currangestart{
            
            if value < currangeend{
                
                return(true);
            }
        }
    }
    
    return(false);
    
    
}


//the same, but for f32 values
fn is_in_range_f32(ranges: Vec< (f32,f32) >, value: f32) -> bool{
    
    for (currangestart, currangeend) in ranges{
        
        if value >= currangestart{
            
            if value < currangeend{
                
                return(true);
                
            }   
        }
    }
    

    return(false);
}







//the actions that are allowed by this piece



//test the turn manager
pub fn test_turn_manager(){
    
    let mut turnmanager = TurnManager::new_two_player(1, 2);
    
    
    //make sure there is a player with an active turn
    assert_eq!(turnmanager.is_active_turns(), true);
    
    //make sure the only active turn is that of player 1
    {
        
        let currentplayers = turnmanager.get_current_players();
        
        let mut only1 = 0;
        for activeplayer in currentplayers{
            assert_eq!(1, activeplayer);
            
            only1 += 1;
        }
        
        assert_eq!(only1, 1);
    }
    
    
    //tick for 30 ticks
    for x in 0..30{
        turnmanager.tick();
    }
    
    //make sure there is a player with
    //make sure the only active turn is that of player 2
    {
        
        let currentplayers = turnmanager.get_current_players();
        
        let mut only1 = 0;
        for activeplayer in currentplayers{
            assert_eq!(2, activeplayer);
            
            only1 += 1;
        }
        
        assert_eq!(only1, 1);
    }
    
    
    //have player 2 perform an action
    //make sure that its just player 1s turn now
    {
        turnmanager.player_took_action(2);
        
        let currentplayers = turnmanager.get_current_players();
        
        let mut only1 = 0;
        for activeplayer in currentplayers{
            assert_eq!(1, activeplayer);
            
            only1 += 1;
        }
        
        assert_eq!(only1, 1);
    }
    
    
    
    
    
    
    
    
}




//a struct to manage when it is every players turn
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TurnManager{
    
    //a list of the turns queued
    //the playerid, the ticks until their turn, and then how long that turn is
    //0 ticks until their turn means that it is currently their turn
    queuedturns: Vec< (u8, u32, u32) >,
    
    
    //the playerid, the ticks until their turn, and then how long that turn is
    //becomes the queuedturns when queuedturns is empty
    basequeue: Vec< (u8, u32, u32) >,
    
    
}

impl TurnManager{
    
    pub fn new_two_player(player1: u8, player2: u8) -> TurnManager{
        
        let mut toreturn = TurnManager{
            
            queuedturns: Vec::new(),
            
            basequeue: Vec::new(),
            
        };
        
        
        //set up the basequeue
        //with player 1 and player 2
        toreturn.basequeue.push(  (player1, 0, 10)  );
        toreturn.basequeue.push(  (player2, 10, 20)  );
        
        toreturn.tick();
        
        
        toreturn
        
        
    }
    
    
    //upkeep the struct
    //should be called after every tick
    //and after every change
    pub fn timeless_upkeep(&mut self){
        
        //if the queued turns is empty, make base queue the queued turns
        if self.queuedturns.is_empty(){
            self.queuedturns = self.basequeue.clone();
        }
        
        
        //remove all queued turns that have a zero length of turn left
        self.queuedturns.retain(|&x| x.2 > 0);
        
        
        //if there are no players with an active turn, tick until there is one
        if !self.is_active_turns(){
            
            self.tick();
            
        }
        
        
    }
    
    
    //progress timewards
    pub fn tick(&mut self) {        
        
        
        //tick each item in queued turns
        for (_, ticksuntil, turnlength) in self.queuedturns.iter_mut(){
            
            //decrease ticks until, unless its zero
            //then tick down turnlength
            if *ticksuntil > 0{
                
                *ticksuntil = *ticksuntil -1;
            }
            else{
                
                *turnlength = *turnlength -1;
                
            }
            
            
        }
        
        //perform a timeless upkeep
        self.timeless_upkeep();
        
        //panic!("this is whos turn it is")
        
        
        
    }
    
    //return whether there are any players with active turns
    pub fn is_active_turns(&self)-> bool{
        
        //return whether there are active turns in this turnmanager
        
        let mut isactiveturn = false;
        
        
        for (_, timeuntil, timeleft) in self.queuedturns.clone(){
            
            if timeuntil <= 0{
                
                if timeleft > 0{
                    
                    isactiveturn = true;
                    
                }
            }
            
        }
        
        
        
        isactiveturn
        
        
    }
    
    
    //this player took a turn action
    pub fn player_took_action(&mut self, playerid: u8){
        
        //tick until it is no longer that players turn
        
        
        while self.get_current_players().contains(&playerid) {
            
            self.tick();
            
        }
        
        
    }
    
    
    //get a set of players who currently have a turn
    pub fn get_current_players(&self) -> HashSet<u8>{
        
        
        let mut activeplayers = HashSet::new();
        
        
        for (playerid, timeuntil, timeleft) in &self.queuedturns{
            
            
            if *timeuntil <= 0 && *timeleft > 0{
                
                activeplayers.insert(*playerid);
                
            }
            
            
        }
        
        
        activeplayers
        
        
        
    }

}
