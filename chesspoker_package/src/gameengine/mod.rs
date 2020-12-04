mod boardgameinterface;
mod datastructs;
use datastructs::PieceData;
use boardgameinterface::BoardGame;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

use datastructs::slide_id_to_direction_change_from_objective_perspective;

pub use datastructs::PieceAction;









pub struct GameEngine{
    
    
    //the pieces that this player owns
    playertopiece: HashMap<u8, HashSet<u16> >,
    
    
    //the direction the player i facing, of the 8 cardinal directions
    playertodirection: HashMap<u8, u8>,
    
    
    //the pieces to their allowed actions
    piecetypedata: HashMap<u16, PieceData>,
    
    
    boardgame: BoardGame,
    
}


impl GameEngine{
    
    
    pub fn new(player1id: u8, player2id: u8) -> GameEngine{
        
        
        let mut gameengine = GameEngine{
            
            playertopiece: HashMap::new(),
            playertodirection: HashMap::new(),
            piecetypedata: HashMap::new(),
            boardgame: BoardGame::new_empty_board(),
            
        };
        
        
        gameengine.playertopiece.insert(player1id, HashSet::new());
        gameengine.playertopiece.insert(player2id, HashSet::new());
        
        
        gameengine.playertodirection.insert(player1id, 0 );
        gameengine.playertodirection.insert(player2id, 4 );
        
        
        gameengine.init_chess_game();
        
        
        gameengine
        
    }
    
    
    
    //is this board game object a square
    pub fn is_board_game_object_square(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_square(objectid)
    }
    //is this board game object a piece
    pub fn is_board_game_object_piece(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_piece(objectid)
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
    
    
    
    
    
    
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec< PieceAction >){
        
        //if its on a board square
        if let Some(boardsquareid) = self.boardgame.get_board_square_piece_is_on(pieceid){
            
            let boardsquarepos = self.boardgame.get_pos_of_boardsquare(boardsquareid).unwrap();
            
            //get the actions of the piece
            let piecedata = self.piecetypedata.get(&pieceid).unwrap();
            
            //the owner of teh piece
            let owner = self.get_owner_of_piece(pieceid);
            let ownerdirection = self.playertodirection.get(&owner).unwrap();
            
            //the direction of the owner of the piece
            let allactions = piecedata.get_piece_actions(*ownerdirection);
            
            //add to this if an action is allowed
            let mut allowedactions: Vec<PieceAction> = Vec::new();
            
            
            //for every action, get if it is allowed
            for (action, hastocapture, cancapture) in allactions{
                
                //if its a lift and move
                if let PieceAction::liftandmove( relativepos ) = action{
                    
                    //get the square that it is taken to if it exists
                    if let Some(endpos) = GameEngine::get_square_pos_that_action_takes_piece_at_pos(boardsquarepos, action.clone()){
                        
                        //the id of this board square it lands on
                        let endid = self.boardgame.get_id_of_boardsquare_pos(endpos).unwrap();
                        
                        //if this board square does not have any of my pieces on it
                        let piecesonboardsquare = self.boardgame.get_pieces_on_board_square( endid );
                        
                        //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                        let mut onlyopponentspieces = true;
                        for otherpieceid in piecesonboardsquare.iter(){
                            let ownerofotherpiece = self.get_owner_of_piece( *otherpieceid);
                            if ownerofotherpiece == owner{   
                                onlyopponentspieces = false;
                            }
                        }
                        
                        let mut objectsinteractedwith = Vec::new();
                        objectsinteractedwith.push(endid);
                        
                        
                        //if this is an empty board square, and im not forced to capture to move, add this
                        if piecesonboardsquare.is_empty(){
                            if ! hastocapture{
                                allowedactions.push(action);
                            }
                        }
                        else{
                            //if this square has a piece and only has opponents pieces, and im allowed to capture, add this
                            if onlyopponentspieces{
                                if cancapture{
                                    allowedactions.push(action);
                                }
                            }   
                        }
                    }
                }
                //if its a slide action
                else if let PieceAction::slide( direction, distance ) = action{
                    
                    let mut stepnumber = 1;
                    
                    let mut actiontogethere = PieceAction::slide(direction, stepnumber);
                    
                    //if that board square exists
                    while let Some(cursquarepos) = GameEngine::get_square_pos_that_action_takes_piece_at_pos( boardsquarepos, actiontogethere.clone() ){
                        
                        //the current square id
                        let cursquareid = self.boardgame.get_id_of_boardsquare_pos(cursquarepos).unwrap();
                        
                        let piecesonboardsquare = self.boardgame.get_pieces_on_board_square(cursquareid);
                        
                        //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                        let mut onlyopponentspieces = true;
                        for otherpieceid in piecesonboardsquare.iter(){
                            //if this piece is owned by the same player that owns the "pieceid" entered
                            //set "onlyopponentspieces" to false
                            let ownerofotherpiece = self.get_owner_of_piece(*otherpieceid);
                            
                            if ownerofotherpiece == owner{   
                                onlyopponentspieces = false;
                            }
                        }
                        
                        
                        
                        //if the action to get here is the same as the action
                        if actiontogethere == action{
                            
                            //if this is an empty board square, and im not forced to capture to move, add this
                            if piecesonboardsquare.is_empty(){
                                if ! hastocapture{
                                    //add it to the list of allowed actions
                                    allowedactions.push(action.clone());
                                }
                            }
                            
                            //if this board square has pieces
                            else{
                                //if it only has opponents pieces 
                                if onlyopponentspieces{
                                    //if im allowed to capture, add this
                                    if cancapture{
                                        //add it to the list of allowed actions
                                        allowedactions.push(action.clone());
                                    }
                                }
                            }
                            
                        }
                        
                        
                        //if there is a piece on this board square break and end after this loop
                        if ! piecesonboardsquare.is_empty() {
                            break;   
                        }
                        
                        
                        //increase the step
                        stepnumber += 1;
                        
                        //update the action
                        actiontogethere = PieceAction::slide(direction, stepnumber);
                        
                    }
                }
            }
            
            
            
            return (piecedata.canflick() , allowedactions);
        }
        
        
        
        //if its not on a board square, it cant do anything
        (false, Vec::new())
        
        
    }
    
    
    //if a piece can perform this action, what objects will it target
    pub fn get_objects_targeted_by_action(&self, pieceid: u16, action: PieceAction) -> Vec<u16>{
        
        //get the boardsquare this action will land on
        //and the pieces on that boardsquare
        
        
        //if the piece is on a boardsquare
        if let Some(bsid) = self.boardgame.get_board_square_piece_is_on(pieceid){
            
            //the position of the boardsquare its on
            let pos = self.boardgame.get_pos_of_boardsquare(bsid).unwrap();
            
            if let Some(endpos) = GameEngine::get_square_pos_that_action_takes_piece_at_pos(pos, action){
                
                let mut toreturn = Vec::new();
                
                //push the id of the boardsquare it ends up on
                //and every piece on that boardsquare
                let endid = self.boardgame.get_id_of_boardsquare_pos(endpos).unwrap();
                
                let pieces = self.boardgame.get_pieces_on_board_square(endid);
                
                toreturn.push(endid);
                
                for pieceid in pieces{
                    
                    toreturn.push( pieceid );
                }
                
                
                return toreturn;
                
            }
        }
        
        
        return Vec::new() ;
        
    }
    
    
    
    
    
    
    //add the pieces to the game that a chess game would have
    fn init_chess_game(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let pawnrow;
            let backrow;
            
            if playerx == 1{
                pawnrow = 1;
                backrow = 0;
            }
            else{
                pawnrow = 6;
                backrow = 7;
            }
            
            
            self.create_pawn( (0,pawnrow), playerx);
            self.create_pawn( (1,pawnrow), playerx);
            self.create_pawn( (2,pawnrow), playerx);
            self.create_pawn( (3,pawnrow), playerx);
            self.create_pawn( (4,pawnrow), playerx);
            self.create_pawn( (5,pawnrow), playerx);
            self.create_pawn( (6,pawnrow), playerx);
            self.create_pawn( (7,pawnrow), playerx);
            
            
            
            self.create_rook  ( (0,backrow), playerx);
            self.create_knight( (1,backrow), playerx);
            self.create_bishop( (2,backrow), playerx);
            self.create_queen ( (3,backrow), playerx);
            self.create_king  ( (4,backrow), playerx);
            self.create_bishop( (5,backrow), playerx);
            self.create_knight( (6,backrow), playerx);
            self.create_rook  ( (7,backrow), playerx);
            
        };
        
    }
    
    
    //create a pawn, at this position, with this owner
    fn create_pawn(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        piecedata.set_pawn();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
    }
    
    fn create_knight(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        piecedata.set_knight();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
    }
    
    fn create_bishop(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        piecedata.set_bishop();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
    }
    
    fn create_rook(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        piecedata.set_rook();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
    }
    
    fn create_king(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        piecedata.set_king();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
    }
    
    fn create_queen(&mut self, pos: (u8,u8), owner: u8){
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        piecedata.set_queen();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
    }
    
    
    
    //if a piece is at a certain pos, and that action is performed (assume its valid)
    //if it is a position within the bounds, and none if its out of bounds
    pub fn get_square_pos_that_action_takes_piece_at_pos(piecepos: (u8,u8), action: PieceAction) -> Option<(u8,u8)>{
        
        let intpiecepos = (piecepos.0 as i8, piecepos.1 as i8);
        
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
            
            let relativepos = (xstep * distance as i8, zstep * distance as i8);
            
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



    
    //get the id of every board square in the game
    pub fn get_squares(&self) -> Vec<u16>{
        
        //get the position of every board square in the game
        let mut boardsquareposs: Vec<(u8,u8)> = Vec::new();
        for x in 0..8{
            for y in 0..8{
                boardsquareposs.push( (x,y) );
            }
        }
        
        let mut toreturn = Vec::new();
        
        //for every board square pos get its id
        for boardsquarepos in boardsquareposs{
            let bsid = self.boardgame.get_id_of_boardsquare_pos(boardsquarepos).unwrap();
            toreturn.push( bsid );
        }
        
        
        return toreturn;
    }
    
    
    //get the id of every board square without a piece on it
    //and that arent on a mission currently
    pub fn get_empty_squares_not_on_mission(&self) -> Vec<u16>{
        
        //get the position of every board square in the game
        let mut boardsquareposs: Vec<(u8,u8)> = Vec::new();
        for x in 0..8{
            for y in 0..8{
                boardsquareposs.push( (x,y) );
            }
        }
        
        let mut toreturn = Vec::new();
        
        //for every board square pos get its id
        for boardsquarepos in boardsquareposs{
            let bsid = self.boardgame.get_id_of_boardsquare_pos(boardsquarepos).unwrap();
            
            let piecesonboardsquare = self.boardgame.get_pieces_on_board_square(bsid);
            //if it doesnt have anything on it
            if piecesonboardsquare.is_empty(){
                
                //if its not on a mission
                if ! self.boardgame.is_object_on_mission(bsid){
                    
                    //then push it into the list of empty squares not on a mission
                    toreturn.push( bsid );
                    
                }
            }
        }
        
        
        return toreturn;
        
        
    }
    
    pub fn perform_action(&mut self, player: u8, piece: u16, pieceaction: PieceAction ){
        
        let playerdirection = self.playertodirection.get(&player).unwrap();
        
        
        //set that piece to having moved
        if let PieceAction::liftandmove(relativeposition) = pieceaction{
            
            let relativeposition = (relativeposition.0 as i8, relativeposition.1 as i8);
            let rotatedrelpos = datastructs::players_perspective_to_objective_perspective_lift(playerdirection, &relativeposition).unwrap();
            let floatrelpos = (rotatedrelpos.0 as f32, rotatedrelpos.1 as f32);
            self.boardgame.lift_and_move_piece_to(piece, floatrelpos);
            
        }
        else if let PieceAction::slide(slidedirection, slidedistance) = pieceaction{
            
            let objslidedir = datastructs::players_perspective_to_objective_perspective_slide(playerdirection, &slidedirection);
            let slidechange = datastructs::slide_id_to_direction_change_from_objective_perspective(objslidedir);
            self.boardgame.slide_piece(piece, slidechange, slidedistance);
            
        }
        else if let PieceAction::flick(direction, force) = pieceaction{
            
            self.boardgame.flick_piece(piece, direction, force);
            
        }
        
        
        //set the piece has having moved
        self.piecetypedata.get_mut(&piece).unwrap().moved_piece();
        
    }
    
    pub fn drop_square(&mut self, bsid: u16){
        self.boardgame.set_long_boardsquare_drop(500, bsid);
    }
    
    pub fn raise_square(&mut self, bsid: u16){
        self.boardgame.set_long_boardsquare_raise(500, bsid);
    }
    
    pub fn is_boardsquare_white(&self, bsid: u16 ) -> bool{
        
        let bspos = self.boardgame.get_pos_of_boardsquare(bsid).unwrap();
        
        let bstotal = bspos.0 + bspos.1;
        
        let evenness = bstotal % 2;
        
        
        if evenness == 0{
            return true;
        }
        else{
            return false;
        }
        
        
    }
    
    //get the name of the type of the piece
    pub fn get_piece_type_name(&self, pieceid: u16) -> String{
        
        let piecetypedata = self.piecetypedata.get(&pieceid).unwrap();
        
        piecetypedata.get_type_name()
        
    }
    
}

