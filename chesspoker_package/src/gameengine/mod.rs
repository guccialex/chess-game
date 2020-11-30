mod boardgameinterface;
mod datastructs;
use datastructs::AllowedActions;
use boardgameinterface::BoardGame;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

use datastructs::slide_id_to_direction_change_from_objective_perspective;




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PieceAction{
    
    flick(f32, f32),
    
    liftandmove( (i32,i32) ),
    
    //what direction, and how many steps
    slide( u8, u8 ),
    
}






pub struct GameEngine{
    
    
    //the pieces that this player owns
    playertopiece: HashMap<u8, HashSet<u16> >,
    
    
    //the direction the player i facing, of the 8 cardinal directions
    playertodirection: HashMap<u8, u8>,
    
    
    //the pieces to their allowed actions
    pieceallowedactions: HashMap<u16, AllowedActions>,
    
    
    boardgame: BoardGame,
    
}


impl GameEngine{
    
    
    pub fn new(player1id: u8, player2id: u8) -> GameEngine{
        
        
        let mut gameengine = GameEngine{
            
            playertopiece: HashMap::new(),
            playertodirection: HashMap::new(),
            pieceallowedactions: HashMap::new(),
            
            boardgame: BoardGame::new_empty_board(),
            
        };
        
        
        gameengine.playertopiece.insert(player1id, HashSet::new());
        gameengine.playertopiece.insert(player2id, HashSet::new());
        
        
        gameengine.playertodirection.insert(player1id, 0 );
        gameengine.playertodirection.insert(player2id, 3 );
        
        
        gameengine.init_chess_game();
        
        
        gameengine
        
    }
    
    pub fn get_owner_of_piece(& self, pieceid: u16) -> u8{
        
        for (player, pieces) in self.playertopiece.clone(){
            
            for playerspieceid in pieces{
                
                if playerspieceid == pieceid{
                    
                    return player;
                }
                
            }
            
        }
        
        panic!("cant find the owner of the piece");
        
    }
    
    
    pub fn get_slide_and_lift_actions_allowed_for_piece(&self, pieceid: u16) -> Vec<PieceAction>{
        
        let actionsandpositionallowed = self.get_actions_allowed_by_piece(pieceid);
        
        let mut toreturn = Vec::new();
        
        for (action, boardpos, pieces) in actionsandpositionallowed.1{
            
            toreturn.push(action);
            
        };
        
        
        
        toreturn
    }
    
    
    
    //return if it can be flicked
    //then each action its allowed to do, where that action takes it, and what pieces will be captured
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> ( bool, Vec<(PieceAction, (u8,u8) , HashSet<u16> )>){
        
        
        let mut toreturn: Vec<(PieceAction, (u8,u8), HashSet<u16>)> = Vec::new();
        
        
        //get the allowed actions of the piece
        let allowedactions = self.pieceallowedactions.get(&pieceid).unwrap();
        
        
        //if the piece is on a board square
        if let Some(boardsquarepos) = self.boardgame.get_board_square_piece_is_on(pieceid){
            
            
            //get the owner of this piece
            let ownerofpiece = self.get_owner_of_piece(pieceid);
            
            //get the facing direction of the owner
            let ownerdirection = self.playertodirection.get(&ownerofpiece).unwrap();
            
            
            
            //get the slide actions
            let slide_actions = allowedactions.get_allowed_slide_actions(*ownerdirection);
            
            //get the lift and move actions
            let lift_and_move_actions = allowedactions.get_allowed_lift_and_move(*ownerdirection);
            
            
            
            //for each direction its allowed to slide
            for (direction, maxdistance, hastocapture, cancapture) in slide_actions.iter(){
                
                let mut stepnumber = 1;
                
                //the action representing this
                let mut action = PieceAction::slide(*direction, stepnumber);
                

                //if that board square exists
                while let Some(cursquarepos) = self.get_square_pos_that_action_takes_piece_at_pos(boardsquarepos, action.clone()){
                    
                    let piecesonboardsquare = self.boardgame.get_pieces_on_board_square(&cursquarepos);
                    
                    //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                    let mut onlyopponentspieces = true;
                    for otherpieceid in piecesonboardsquare.iter(){
                        
                        //if this piece is owned by the same player that owns the "pieceid" entered
                        //set "onlyopponentspieces" to false
                        let ownerofotherpiece = self.get_owner_of_piece(*otherpieceid);
                        
                        if ownerofotherpiece == ownerofpiece{   
                            onlyopponentspieces = false;
                        }
                    }
                    
                    //if this is an empty board square, and im not forced to capture to move, add this
                    if piecesonboardsquare.is_empty(){
                        if ! hastocapture{
                            toreturn.push( (action.clone(), cursquarepos, piecesonboardsquare.clone()) );
                        }
                    }
                    //if this board square has pieces
                    else{
                        //if it only has opponents pieces 
                        if onlyopponentspieces{
                            //if im allowed to capture, add this
                            if *cancapture{
                                
                                toreturn.push((action.clone(), cursquarepos, piecesonboardsquare.clone()));
                            }
                        }
                    }
                    

                    //if there is a piece on this board square break and end after this loop
                    if ! piecesonboardsquare.is_empty() {
                        break;   
                    }
                    

                    //increase the step
                    stepnumber += 1;
                    
                    //and update the action
                    action = PieceAction::slide(*direction, stepnumber);
                }
                
                
                
            }
            
            
            
            //for each position it can be lifted and moved to
            for (currelativeposition, hastocapture, cancapture ) in lift_and_move_actions.iter(){
                
                //the position of the piece + the direction this move wants to send it
                let currentboardsquare = boardsquarepos;
                
                let lift_action_to_get_here = PieceAction::liftandmove( (currelativeposition.0 as i32, currelativeposition.1 as i32) );
                let maybeendpos = self.get_square_pos_that_action_takes_piece_at_pos( currentboardsquare, lift_action_to_get_here.clone());
                
                if let Some(endpos) = maybeendpos{
                    
                    //if this board square does not have any of my pieces on it
                    let piecesonboardsquare = self.boardgame.get_pieces_on_board_square( & currentboardsquare );
                    
                    //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                    let mut onlyopponentspieces = true;
                    
                    for otherpieceid in piecesonboardsquare.iter(){
                        
                        let ownerofotherpiece = self.get_owner_of_piece( *otherpieceid);
                        if ownerofotherpiece == ownerofpiece{   
                            onlyopponentspieces = false;
                        }
                    }
                    
                    //if this is an empty board square, and im not forced to capture to move, add this
                    if piecesonboardsquare.is_empty(){
                        if ! hastocapture{
                            toreturn.push( (lift_action_to_get_here.clone(), currentboardsquare, piecesonboardsquare) );
                        }
                    }
                    else{
                        //if this square has a piece and only has opponents pieces, and im allowed to capture, add this
                        if onlyopponentspieces{
                            if *cancapture{
                                toreturn.push((lift_action_to_get_here.clone(), currentboardsquare, piecesonboardsquare));
                            }
                        }   
                    }
                }
            }

        };
        
        
        
        (true, toreturn)
        
    }
    
    
    
    
    //add the pieces to the game that a chess game would have
    fn init_chess_game(&mut self){
        
        self.create_pawn( (1,1), 1);
       
    }
    //create a pawn, at this position, with this owner
    fn create_pawn(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let pieceallowedactions = AllowedActions::get_unmoved_pawn();
        
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.pieceallowedactions.insert(pieceid, pieceallowedactions);
        
    }
    
    
    
    
    
    //if a piece is at a certain pos, and that action is performed (assume its valid)
    //if it is a position within the bounds, and none if its out of bounds
    pub fn get_square_pos_that_action_takes_piece_at_pos(&self, piecepos: (u8,u8), action: PieceAction) -> Option<(u8,u8)>{
        
        
        let intpiecepos = (piecepos.0 as i32, piecepos.1 as i32);
        
        
        if let PieceAction::liftandmove( relativepos ) = action{
            
            
            let newpos = (intpiecepos.0 + relativepos.0 , intpiecepos.1 + relativepos.1);
            
            //if the new pos is out of range, return none
            if newpos.0 <0 || newpos.0 > 7{
                return None;
            }
            if newpos.1 <0 || newpos.1 > 7{
                return None;
            }
            
            //otherwise, return it
            return Some( (newpos.0 as u8, newpos.1 as u8) );
            
        } 
        else if let PieceAction::slide( direction, distance ) = action{
            
            let (xstep, zstep) = slide_id_to_direction_change_from_objective_perspective(direction);
            
            let relativepos = (xstep * distance as i32, zstep * distance as i32);
            
            
            let newpos = (intpiecepos.0 + relativepos.0 , intpiecepos.1 + relativepos.1);
            
            //if the new pos is out of range, return none
            if newpos.0 <0 || newpos.0 > 7{
                return None;
            }
            if newpos.1 <0 || newpos.1 > 7{
                return None;
            }
            
            //otherwise, return it
            return Some( (newpos.0 as u8, newpos.1 as u8) );
            
        } 
        else{
            
            
            return None;
            
        }
        
        
        
    }
    
    
    
    
    
    
    
    
    //get the list of every object in the physical engine
    pub fn get_object_ids(&self) -> Vec<u16>{
        
        self.boardgame.get_object_ids()
        
    }    
    
    pub fn get_object_translation(&self, gameobjectid: u16) -> (f32,f32,f32){
        self.boardgame.get_translation(gameobjectid)
    }
    
    pub fn get_object_rotation(&self, gameobjectid: u16) -> (f32,f32,f32){
        self.boardgame.get_rotation(gameobjectid)
    }
    
    
    
    pub fn tick(&mut self){
        
        self.boardgame.tick();
        
    }
    
    
}

