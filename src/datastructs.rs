
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use std::collections::HashSet;


use super::PieceAction;




#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerInput{
    
    
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

        turns.insert(0, (player1, 40, 1) );
        turns.insert(1, (player2, 40, 0) );


        let mut tickssincelastaction = HashMap::new();

        tickssincelastaction.insert(player1, 0);
        tickssincelastaction.insert(player2, 0);


        let mut playertimeleft = HashMap::new();

        playertimeleft.insert( player1, 20000 );
        playertimeleft.insert( player2, 20000 );


        
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
    pub fn tick(&mut self) {
        
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

    //if it is this players turn, is it the last tick they have for their turn?
    pub fn is_it_this_players_turns_last_tick(&self, playerid: u8) -> bool{

        let (curplayerid, turnlength, _) = self.turns.get(&self.currentturn).unwrap();

        if playerid == *curplayerid{

            if self.currentturntick == *turnlength -1{

                return true;
            };
        };

        return false;
    }

    //if it is this players turn, return how many ticks they have left in their turn
    pub fn get_ticks_left_for_players_turn(&self, playerid: u8) -> Option<u32>{


        let (curplayerid, turnlength, _) = self.turns.get(&self.currentturn).unwrap();

        if playerid == *curplayerid{

            return Some( turnlength - self.currentturntick );

        };

        return None;

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


    //call this when the players should start taking 2 turns in a row
    pub fn players_take_2_turns_in_a_row(&mut self){

        //get the length of the turns currently
        let (_, length, _) = self.turns.get(&0).unwrap();
        let length = *length;


        //player 1 goes, then goes again, 
        self.turns.insert(0, (1, length, 1) );
        self.turns.insert(1, (1, length, 2) );

        //then 2 goes then goes again
        self.turns.insert(2, (2, length, 3) );
        self.turns.insert(3, (2, length, 0) );


    }


    pub fn halve_time_left(&mut self){


        for (player, timeleft) in self.playertimeleft.iter_mut(){

            *timeleft = *timeleft / 2;
        }


    }




}


