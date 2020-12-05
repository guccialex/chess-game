
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

    //draw card from the deck
    drawcard,
    
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


    //the total amount of time a player has left
    playertimeleft: HashMap<u8, i32>,
}



impl TurnManager{
    
    pub fn new_two_player(player1: u8, player2: u8) -> TurnManager{
        
        let mut toreturn = TurnManager{
            
            queuedturns: Vec::new(),
            basequeue: Vec::new(),
            playertimeleft: HashMap::new(),
        };

        
        //set up the basequeue
        //with player 1 and player 2
        //a max of 60 seconds per turn
        toreturn.basequeue.push(  (player1, 0, 1800)  );
        toreturn.basequeue.push(  (player2, 1800, 1800)  );
        
        toreturn.playertimeleft.insert( player1, 1800 * 5);
        toreturn.playertimeleft.insert( player2, 1800 * 5);


        toreturn.tick();
        
        toreturn
    }
    
    
    //upkeep the struct
    //should be called after every tick
    //and after every change
    fn timeless_upkeep(&mut self){
        
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


        let activeplayers = self.get_current_players();

        //if it is currently the players turn
        //tick the amount of time they have left down
        for (player, timeleft) in self.playertimeleft.iter_mut(){

            //if the player is active, tick his timeleft down
            if activeplayers.contains(player){
                
                *timeleft = *timeleft - 1;
            }
        }

        
        //perform a timeless upkeep
        self.timeless_upkeep();        
        
        
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
        
        //tick until it is no longer that players turn and dont
        while self.get_current_players().contains(&playerid) {
            
            self.tick();
            
            //and add to that players total time to offset the tick that shouldnt be
            //taking their total time down
            *self.playertimeleft.get_mut(&playerid).unwrap() += 1;
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

    //if it is this players turn, is it the last tick they have for their turn?
    pub fn is_it_this_players_turns_last_tick(&self, playerid: u8) -> bool{

        for (queuedplayer, ticksuntil, turnlength) in self.queuedturns.iter(){

            if queuedplayer == &playerid{

                if *turnlength <= 1{
                    return true;
                }
            }
        };

        false    
    }


    //if it is this players turn, return how many ticks they have left in their turn
    pub fn get_ticks_left_for_players_turn(&self, playerid: u8) -> Option<u32>{

        for (queuedplayer, ticksuntil, turnlength) in self.queuedturns.iter(){

            if queuedplayer == &playerid{

                if ticksuntil == &0{
                    return Some(*turnlength);
                }
            }
        };

        None
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
    

}




pub struct GameSettings{


    //the amount of cards players draw at the start of their turn
    cardsdrawnatstartofturn: u8,

    //how many cards are drawn by 
    //if 0 it means the player cant draw
    cardsdrawnbyaction: u8,



}


impl GameSettings{

    pub fn new() -> GameSettings{

        GameSettings{
            cardsdrawnatstartofturn: 0,
            cardsdrawnbyaction: 0,
        }

    }

}