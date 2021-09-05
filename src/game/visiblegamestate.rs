use std::collections::HashMap;

use std::collections::HashSet;

use board::VisibleGameBoardObject;


pub struct VisibleGameState{
    
    //has either player won
    pub isgameover: Option<u8>,
    
    //the deck
    //whether the move is available
    //pub turnsuntildrawavailable: Option<u32>,
    
    pub player1totalticksleft: u32,
    pub player2totalticksleft: u32,
    
    //if they have a limit on ticks left for their turn
    //how many ticks do they have left for their turn
    pub player1ticksleft: Option<u32>,
    pub player2ticksleft: Option<u32>,
    
    
    //if its their turn
    pub playerswithactiveturns: HashSet<u8>,


    pub piles: [String; 4],
    

    //the list of game effects as locations of their texture
    pub gameeffects: Vec<String>,
    
    //the card effect as a location of its texture
    pub lastcardeffect: Option<String>,


    //only id and appearance
    //just click objects is the input
    pub boardobjects: Vec<VisibleGameBoardObject>,
}

