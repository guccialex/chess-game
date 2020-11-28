
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use std::collections::HashSet;





//the different values of the card
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardValue{
    
    ace,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    ten,
    jack,
    queen,
    king
    
    
}

//the effect of the card it can have
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardEffect{
    
    //this card can initiate a blackjack game with it as the starting card
    blackjackgame,
    
    //this card can initiate a poker game with it as the starting card
    pokergame,
    
    //this card can remove a square from the board
    dropsquare,
    
    //this card can raise a square to not be able to be moved past by another piece
    raisesquare,
    
    
    
    
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CardSuit{

    diamonds,
    clubs,
    hearts,
    spades


}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Card{
    
    
    
    value: CardValue,

    suit: CardSuit,
    
    pub effect: CardEffect,


    //if the card is an unknown card
    isunknown: bool,
    
    
    
}

impl Card{
    
    //returns if this card is an ace or not
    pub fn is_ace(&self) -> bool{
        
        
        if self.value == CardValue::ace{
            
            return(true);
        }
        else{
            
            return(false);
        }
        
        
        
        
    }
    
    //get the blackjack value of the card
    pub fn blackjackvalue(&self) -> u16{
        
        
        if self.value == CardValue::two{
            return(2);
        }
        else if self.value == CardValue::three{
            return(3);
        }
        else if self.value == CardValue::four{
            return(4);
        }
        else if self.value == CardValue::five{
            return(5);
        }
        else if self.value == CardValue::six{
            return(6);
        }
        else if self.value == CardValue::seven{
            return(7);
        }
        else if self.value == CardValue::eight{
            return(8);
        }
        else if self.value == CardValue::nine{
            return(9);
        }
        else if self.value == CardValue::ten{
            return(10);
        }
        else if self.value == CardValue::jack{
            return(10);
        }
        else if self.value == CardValue::queen{
            return(10);
        }
        else if self.value == CardValue::king{
            return(10);
        }
        
        
        panic!("this is an ace, so i  dont know whether the value is 1 or 11");
        
        
    }


    //return the number representing the value
    //1 - 13
    pub fn numbervalue(&self) -> u16{


                
        if self.value == CardValue::two{
            return(2);
        }
        else if self.value == CardValue::three{
            return(3);
        }
        else if self.value == CardValue::four{
            return(4);
        }
        else if self.value == CardValue::five{
            return(5);
        }
        else if self.value == CardValue::six{
            return(6);
        }
        else if self.value == CardValue::seven{
            return(7);
        }
        else if self.value == CardValue::eight{
            return(8);
        }
        else if self.value == CardValue::nine{
            return(9);
        }
        else if self.value == CardValue::ten{
            return(10);
        }
        else if self.value == CardValue::jack{
            return(11);
        }
        else if self.value == CardValue::queen{
            return(12);
        }
        else if self.value == CardValue::king{
            return(13);
        }
        //if its an ace
        else{
            return(1);
        }

    }



    pub fn suitvalue(&self) -> u16{

        if self.suit == CardSuit::diamonds{
            return 1;
        }
        else if self.suit == CardSuit::clubs{
            return 2;
        }
        else if self.suit == CardSuit::hearts{
            return 3;
        }
        else{
            return 4;
        }


    }

    
    pub fn new_random_card() -> Card{

        Card{
            value: CardValue::ace,
            suit: CardSuit::spades,
            effect: CardEffect::dropsquare,
            isunknown: false
        }

    }

    pub fn new_unknown_card() -> Card{


        Card{

            value: CardValue::ace,
            suit: CardSuit::spades,
            effect: CardEffect::raisesquare,
            isunknown: true

        }



    }
    
}




//the other data associated with a piece aside from how it is allowed to move and capture
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PieceTypeData{
    
    isking: bool,
    
    //if this piece has taken an action to move yet
    takenaction: bool,


    cancastle: bool,
    
    
    allowedactions: AllowedActions,
}
impl PieceTypeData{

    pub fn new() -> PieceTypeData{

        PieceTypeData{
            isking: false,
            takenaction: false,
            cancastle: false,

            allowedactions: AllowedActions::get_unmoved_pawn(),
        }

    }
    
    //set the piece to be these types
    pub fn set_pawn(&mut self){

        if self.takenaction {
            self.allowedactions = AllowedActions::get_moved_pawn();
        }
        else{
            self.allowedactions = AllowedActions::get_unmoved_pawn();
        }

    }
    pub fn set_knight(&mut self){

        self.allowedactions = AllowedActions::get_knight();
    }
    pub fn set_bishop(&mut self){
        self.allowedactions = AllowedActions::get_bishop();
    }
    pub fn set_rook(&mut self){
        self.allowedactions = AllowedActions::get_rook();
    }
    pub fn set_queen(&mut self){
        self.allowedactions = AllowedActions::get_queen();
    }
    pub fn set_king(&mut self){
        self.allowedactions = AllowedActions::get_king();
    }
    
    pub fn get_allowed_slide_actions(&self, ownerdirection: &u8) -> HashSet<(u8,u8, bool, bool)>{

        //rotate each allowed action in the slide by its owners direction
        let temp = self.allowedactions.slidedirection.clone();
        
        let mut toreturn = HashSet::new();

        for (direction, a, b, c) in temp.iter(){

            let newdirection = players_perspective_to_objective_perspective_slide(ownerdirection, direction);

            toreturn.insert( (newdirection, *a, *b, *c)  );

        }

        toreturn


    }

    pub fn get_allowed_lift_and_move(&self, ownerdirection: &u8) -> HashSet<((i8,i8), bool, bool)> {

        //rotate each allowed action in the lift and move by its owners direction
        let temp = self.allowedactions.liftandmove.clone();
                        
        let mut toreturn = HashSet::new();


        for ( relativepos, a, b) in temp.iter(){

            if let Some(newrelativepos) = players_perspective_to_objective_perspective_lift(ownerdirection, relativepos){

                toreturn.insert( (newrelativepos, *a, *b)  );

            }

        }



        toreturn
    }

    //given an 
    pub fn get_if_this_flick_allowed(&self, direction:f32, force:f32) -> bool{


        true
    }


    pub fn get_if_castle_allowed(&self){
        
    }
    
    
}




#[derive(Serialize, Deserialize, Debug, Clone)]
struct AllowedActions{
    
    
    
    //allowed actions:
    //where it can it move?
    //does it slide to there or lift and move there?
    //can it capture an opponents piece with that movement
    //does it have to capture an opponents piece with that movement
    
    
    
    
    //what direction can it slide
    //what distance
    //does it have to capture an opponents piece to slide there
    //is it allowed to capture an opponents piece when sliding there
    slidedirection: HashSet<( u8, u8, bool, bool )>,
    
    
    
    //what relative positions can it move to
    //does it have to capture an opponents piece to move there
    //can it capture an opponents piece by moving there
    liftandmove: HashSet<( (i8, i8), bool, bool, )>,
    
}


impl AllowedActions{
    
    
    //get the allowed actions of a pawn that has not moved yet
    fn get_unmoved_pawn() -> AllowedActions{
        
        
        let mut slidedirection = HashSet::new();
        
        //moving forwards thing
        slidedirection.insert( (0, 2, false, false) );
        
        //capturing diagonally
        slidedirection.insert( (1, 1, true, true) );
        slidedirection.insert( (7, 1, true, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        
        
    }

    //get the allowed actions of a pawn that has been moved
    fn get_moved_pawn() -> AllowedActions{
        
        let mut slidedirection = HashSet::new();
        
        //moving forwards thing
        slidedirection.insert( (0, 1, false, false) );
        
        //capturing diagonally
        slidedirection.insert( (1, 1, true, true) );
        slidedirection.insert( (7, 1, true, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        
        
        
    }

    fn get_knight() -> AllowedActions{


        let mut slidedirection = HashSet::new();
        
        let mut liftandmove = HashSet::new();
        
        liftandmove.insert( ( (1,2), false, true   ) );
        liftandmove.insert( ( (2,1), false, true   ) );
        liftandmove.insert( ( (2,-1), false, true  ) );
        liftandmove.insert( ( (1,-2), false, true  ) );
        
        liftandmove.insert( ( (-1,-2), false, true ) );
        liftandmove.insert( ( (-2,-1), false, true ) );
        liftandmove.insert( ( (-2,1), false, true  ) );
        liftandmove.insert( ( (-1,2), false, true  ) );

        
        AllowedActions{
            
            liftandmove: liftandmove,
            
            slidedirection: slidedirection,
            
        }


    }

    fn get_bishop() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any diagonal direction
        slidedirection.insert( (1, 7, false, true) );
        slidedirection.insert( (3, 7, false, true) );
        slidedirection.insert( (5, 7, false, true) );
        slidedirection.insert( (7, 7, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }

    fn get_rook() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any diagonal direction
        slidedirection.insert( (0, 7, false, true) );
        slidedirection.insert( (2, 7, false, true) );
        slidedirection.insert( (4, 7, false, true) );
        slidedirection.insert( (6, 7, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }
    
    fn get_queen() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any orthogonal direction
        slidedirection.insert( (0, 7, false, true) );
        slidedirection.insert( (2, 7, false, true) );
        slidedirection.insert( (4, 7, false, true) );
        slidedirection.insert( (6, 7, false, true) );

        //move in any diagonal direction
        slidedirection.insert( (1, 7, false, true) );
        slidedirection.insert( (3, 7, false, true) );
        slidedirection.insert( (5, 7, false, true) );
        slidedirection.insert( (7, 7, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }

    fn get_king() -> AllowedActions{



        let mut slidedirection = HashSet::new();
        

        //move in any orthogonal direction
        slidedirection.insert( (0, 1, false, true) );
        slidedirection.insert( (2, 1, false, true) );
        slidedirection.insert( (4, 1, false, true) );
        slidedirection.insert( (6, 1, false, true) );

        //move in any diagonal direction
        slidedirection.insert( (1, 1, false, true) );
        slidedirection.insert( (3, 1, false, true) );
        slidedirection.insert( (5, 1, false, true) );
        slidedirection.insert( (7, 1, false, true) );
        
        
        
        AllowedActions{
            
            liftandmove: HashSet::new(),
            
            slidedirection: slidedirection,
            
        }
        


    }
    
    
}


fn players_perspective_to_objective_perspective_slide(playerdirection: &u8, slidedirection: &u8) -> u8{

    let slidedirection = *slidedirection as i32;

    let playerdirection = *playerdirection as i32;

    //add slide direction and player direction to get the new direction
    //and mod by 8 so it loops around if its too large
    let toreturn = (slidedirection + playerdirection) % 8;

    toreturn as u8

}


//if the object cant be rotated and still represented as an i8, return none
fn players_perspective_to_objective_perspective_lift(playerdirection: &u8, relativepos: &(i8,i8)) -> Option<(i8,i8)>{
    
    let angleasradians = *playerdirection as f32;
    let angleasradians = angleasradians / 8.0 ;
    let angleasradians = angleasradians * 2.0 * 3.14159;

    let relativeposx = relativepos.0 as f32;
    let relativeposy = relativepos.1 as f32;


    let roundedcosangle = angleasradians.cos().round();
    let roundedsinangle = angleasradians.sin().round();


    let newxfloat = (relativeposx * roundedcosangle - relativeposy * roundedsinangle) as i32;
    let newyfloat = (relativeposx * roundedsinangle + relativeposy * roundedcosangle) as i32;


    //if the new coordinates can be converted into an i8
    //which now that im thinking, should always be the case
    //but im not sure, and its already set up this way

    use std::convert::TryFrom;

    if let Some(resultingx) = i8::try_from(newxfloat).ok(){

        if let Some(resultingy) = i8::try_from(newyfloat).ok(){

            return  Some( (resultingx, resultingy) )  ;

        }

    }


    //else return None
    return( None );
    



}


//turn the direction step change into an id of the direction
//from the perspective of a certain player
pub fn direction_change_to_slide_id_from_objective_perspective(directionstepchange: (i32,i32) ) -> u8 {
    
    
    if directionstepchange == (0,1){
        return 0
    }
    if directionstepchange == (1,1){
        return 1
    }
    if directionstepchange == (1,0){
        return 2
    }
    if directionstepchange == (1,-1){
        return 3
    }
    if directionstepchange == (0,-1){
        return 4
    }
    if directionstepchange == (-1,-1){
        return 5
    }
    if directionstepchange == (-1,0){
        return 6
    }
    if directionstepchange == (-1,1){
        return 7
    }
    
    
    panic!("all the cases should've been dealt with");
    
}

pub fn slide_id_to_direction_change_from_objective_perspective(slideid: u8) -> (i32,i32){
    
    
    
    let mut toreturn = (0,0);
    
    //the 8 semi-cardinal directions
    //and the value for how much the piece moves going in each one        
    if slideid == 0{
        toreturn = (0,1);
    }
    else if slideid == 1{
        toreturn= (1,1) ;
    }
    else if slideid == 2{
        toreturn= (1,0 );
    }
    else if slideid == 3{
        toreturn =(1,-1 );
    }
    else if slideid == 4{
        toreturn = (0,-1 );
    }
    else if slideid == 5{
        toreturn = (-1,-1);
    }
    else if slideid == 6{
        toreturn = (-1,0 );
    }
    else if slideid == 7{
        toreturn = (-1,1);
    }
    
    
    toreturn
    
    
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerInput{
    
    
    //what types of input
    
    //playing a card on the active board
    playcardonboard(u16),

    //playing a card with a target of a piece
    playcardonpiece(u16, u32),

    //playing a card with a target of a board square
    playcardonsquare(u16, (u8,u8)),
    
    //perform an action on a piece
    pieceaction(u32, PieceAction),
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PieceAction{
    
    
    flick(f32, f32),
    
    liftandmove( (i32,i32) ),
    
    //what direction, and how many steps
    slide( u8, u8 ),
    
    
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
