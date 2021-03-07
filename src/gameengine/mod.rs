mod boardgameinterface;
mod datastructs;
use datastructs::PieceData;
use boardgameinterface::BoardGame;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

//use datastructs::slide_id_to_direction_change_from_objective_perspective;

pub use datastructs::PieceAction;



//use boardgameinterface::convert_physical_pos_to_board_square_pos;
//use boardgameinterface::convert_board_square_pos_to_physical_pos;

use datastructs::is_square_posid_valid;

//use datastructs::players_perspective_to_objective_perspective_lift;








#[derive(Serialize, Deserialize)]
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
        
        gameengine
    }
    
    
    //add the pieces to the game that a chess game would have
    pub fn add_chess_pieces(&mut self){
        
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
            
            
            let id = self.create_piece( (0,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (1,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (2,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (3,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (4,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (5,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (6,pawnrow), playerx);
            self.set_pawn(id);
            let id = self.create_piece( (7,pawnrow), playerx);
            self.set_pawn(id);
            
            
            
            let id = self.create_piece( (0,backrow), playerx);
            self.set_rook(id);
            let id = self.create_piece( (1,backrow), playerx);
            self.set_knight(id);
            let id = self.create_piece( (2,backrow), playerx);
            self.set_bishop(id);
            let id = self.create_piece( (3,backrow), playerx);
            self.set_queen(id);
            let id = self.create_piece( (4,backrow), playerx);
            self.set_king(id);
            let id = self.create_piece( (5,backrow), playerx);
            self.set_bishop(id);
            let id = self.create_piece( (6,backrow), playerx);
            self.set_knight(id);
            let id = self.create_piece( (7,backrow), playerx);
            self.set_rook(id);
            
        };
        
    }
    
    //add the pieces to the game that a chess game would have
    fn make_checkers_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            for column in 0..8{
                
                let firstrow = 0;
                
                let id = self.create_piece( (column, 1), playerx);
                self.set_checkers(id);    
                
                
            }
        };
        
    }
    
    
    
    
    
    //tick, with true if kings are replaced and false if theyre not
    pub fn tick(&mut self, arekingsreplaced: bool, arepawnspromoted: bool,  raisedsquares: u32, removedsquares: u32){
        
        
        
        //get the number of raised squares
        let curraisedsquares = self.boardgame.get_raised_squares();
        
        //how many more raised squares I have than I need
        let raiseddifference = curraisedsquares.len() as i32 - raisedsquares as i32;
        
        
        //if i have more squares raised than I need
        if raiseddifference > 0{
            
            for x  in 0..(raiseddifference as usize){
                
                self.boardgame.end_mission( &curraisedsquares[x] );
            }
        }
        //if I dont have enough squares raised
        else if raiseddifference < 0{
            
            let potentialsquares = self.get_empty_squares_not_on_mission();
            
            for x in 0..(-raiseddifference as usize){
                self.boardgame.set_long_boardsquare_raise(10000, potentialsquares[x]);
            }
            
        }
        
        
        
        
        
        
        //get the number of dropped squares
        let curdroppedsquares = self.boardgame.get_dropped_squares();
        
        //how many more raised squares I have than I need
        let droppeddifference = curdroppedsquares.len() as i32 - removedsquares as i32;
        
        
        //if i have more squares raised than I need
        if droppeddifference > 0{
            
            for x  in 0..(droppeddifference as usize){
                
                self.boardgame.end_mission( &curdroppedsquares[x] );
            }
        }
        //if I dont have enough squares raised
        else if droppeddifference < 0{
            
            let potentialsquares = self.get_empty_squares_not_on_mission();
            
            for x in 0..(-droppeddifference as usize){
                self.boardgame.set_long_boardsquare_drop(10000, potentialsquares[x]);
            }
            
        }
        
        
        
        
        
        
        
        //remove the pieces that are lower than -5 in pos
        for (pieceid, _) in &self.piecetypedata.clone(){
            
            let pos = self.boardgame.get_translation(*pieceid);
            
            if pos.1 < -3.0{
                
                self.remove_piece(*pieceid);
            }
        }
        
        
        
        
        
        //if the kings are replaced, the piece with the highest score becomes a king
        if arekingsreplaced{
            
            for playerid in 1..3{
                
                //if they dont
                if ! self.does_player_have_king(playerid){
                    
                    let mut highestvaluepieceid = 0;
                    let mut highestvaluepiecevalue = 0;
                    
                    //find their highest valued piece, and turn it into a king
                    for pieceid in self.playertopiece.get(&playerid).unwrap(){
                        
                        let piecedata = self.piecetypedata.get(pieceid).unwrap();
                        let piecevalue = piecedata.get_value();
                        
                        if piecevalue > highestvaluepiecevalue{
                            highestvaluepiecevalue = piecevalue;
                            highestvaluepieceid = *pieceid;
                        }
                        
                    }
                    
                    
                    let mut piecedata = self.piecetypedata.get_mut(&highestvaluepieceid).unwrap();
                    
                    piecedata.set_king();
                }
            }
            
        }
        
        
        //promote the pawns to queens on the opponents backrow if allowed
        if arepawnspromoted{
            
            //if theres a pawn on its opponents back row, promote it
            for (pieceid, piecedata) in self.piecetypedata.clone(){
                
                //get the owner
                let ownerid = self.get_owner_of_piece(pieceid);
                
                //get the "objective back row" from that players perspective
                let backrow = GameEngine::subjective_row_to_objective_row(&ownerid, &7);
                
                if let Some( (curpiececolumn, curpiecerow) ) = self.boardgame.get_square_pos_piece_is_on(pieceid){
                    
                    //if that pawn is on the backrow
                    if curpiecerow == backrow{
                        
                        self.set_queen(pieceid);
                    }
                }
            }
        }
        
        
        self.boardgame.tick();
        
    }    
    
    //transfer the ownership fo this piece to this player
    fn transfer_ownership(&mut self, pieceid: u16, newplayerid: u8){
        
        //get the old owner of the piece
        let oldowner = self.get_owner_of_piece(pieceid);
        
        //remove that piece from that players list of pieces
        self.playertopiece.get_mut(&oldowner).unwrap().remove(&pieceid);
        
        //and give that piece to the new player
        self.playertopiece.get_mut(&newplayerid).unwrap().insert(pieceid);
        
    }
    
    
    //given a piece, and an action
    //get the list of squares it passes over
    //the time that it passes over them
    //and the action at the point which it passes over them
    fn get_passed_over_squares(&self, action: PieceAction, pieceid: u16) -> Vec< (u32, PieceAction, u16) >{
        
        let mut toreturn = Vec::new();
        
        if let PieceAction::liftandmove(relativepos) = action{
            
            let startsquareid = self.boardgame.get_board_square_piece_is_on(pieceid).unwrap();
            
            let startsquareposid = self.boardgame.boardsquare_id_to_posid(startsquareid).unwrap();
            
            
            let endsquarepos = (startsquareposid.0 as i8 + relativepos.0, startsquareposid.1 as i8 + relativepos.1);
            
            
            if let Some(endposid) = is_square_posid_valid(endsquarepos){
                
                let endsquareid = self.boardgame.boardsquare_posid_to_id(endposid).unwrap();
                
                toreturn.push( (1, action, endsquareid) );
                
            };
            
        }
        else if let PieceAction::slide(direction, distance) = action{
            
            
            let startsquareid = self.boardgame.get_board_square_piece_is_on(pieceid).unwrap();
            let startsquareposid = self.boardgame.boardsquare_id_to_posid(startsquareid).unwrap();
            let startsquarei8pos = (startsquareposid.0 as i8, startsquareposid.1 as i8);
            
            
            for step in 1..distance+1{
                
                let relativeposstep = action.get_single_step_pos_change();
                
                
                let cursquarepos = (startsquarei8pos.0 + (relativeposstep.0 * step as i8), startsquarei8pos.1 + (relativeposstep.1 * step as i8));                
                
                
                if let Some(curposid) = is_square_posid_valid(cursquarepos){
                    
                    let cursquareid = self.boardgame.boardsquare_posid_to_id(curposid).unwrap();
                    
                    toreturn.push( (step as u32 * 5, PieceAction::slide(direction, step), cursquareid) );
                    
                };
                
            }
            
        }
        
        
        toreturn
    }
    
    
    //get if this action is allowed by this piece
    pub fn is_action_allowed(&self, action: PieceAction, pieceid: u16) -> bool{
        
        //get the owner of this piece
        let owner = self.get_owner_of_piece(pieceid);
        
        //the direction of the owner
        let ownerdirection = self.playertodirection.get(&owner).unwrap();
        
        //if this is is one of the actions the piece is allowed to perform
        let piecedata = self.piecetypedata.get(&pieceid).unwrap();
        
        //if the action is not allowed by the piecedata, return false before proceeding
        if ! piecedata.is_action_allowed(action.clone(), *ownerdirection){
            return false;
        }
        
        
        //if its on a board square
        if let Some(boardsquareid) = self.boardgame.get_board_square_piece_is_on(pieceid){
            
            //if the current boardsquare is on a mission, return false
            if self.boardgame.is_object_on_mission(&boardsquareid){
                return false;
            }
            //if the current piece is on a mission, return false
            if self.boardgame.is_object_on_mission(&pieceid){
                return false;
            }
            
            //the position of the boardsquare its on
            let boardsquarepos = self.boardgame.boardsquare_id_to_posid(boardsquareid).unwrap();
            
            
            let mut liftorslide = false;
            let mut hastocapture = false;
            let mut cancapture = true;
            
            
            if let PieceAction::liftandmove(relativepos) = action{
                
                let (temphastocapture, tempcancapture) = piecedata.get_capture_type_of_lift_and_move( relativepos, *ownerdirection);
                
                hastocapture = temphastocapture;
                cancapture = tempcancapture;
                liftorslide = true;
                
            }
            //if this is a slide action
            else if let PieceAction::slide( direction, distance ) = action{
                
                let (temphastocapture, tempcancapture) = piecedata.get_capture_type_of_slide(direction, distance, *ownerdirection);
                
                hastocapture = temphastocapture;
                cancapture = tempcancapture;
                liftorslide = true;
                
            }
            //if this is a flick action
            else if let PieceAction::flick(direction, force) = action{
                
                //if the piece data allowd it, as long as its on a boardsquare
                //a flick is allowed
                return true;
            }
            
            
            
            
            
            
            
            if liftorslide{
                
                //get the list of board squares its moving over and at what time it moves over each
                //then check in order if it is allowed to move there
                let squarespassedover = self.get_passed_over_squares(action.clone(), pieceid);
                
                
                for (_tick, curaction, cursquareid) in squarespassedover{
                    
                    //if the cur square isnt the starting square
                    if cursquareid != boardsquareid{
                        
                        if self.boardgame.is_object_on_mission(&cursquareid){
                            
                            break;
                        }
                        
                        
                        //does the boardsquare have any enemy pieces on it
                        let mut opposingpiecesonsquare: bool = false; 
                        //does the boardsquare have any ally pieces on it
                        let mut alliedpiecesonsquare: bool = false;
                        
                        {
                            //get the pieces on the boardsquare
                            let piecesonboardsquare = self.boardgame.get_pieces_on_board_square( cursquareid );
                            
                            //for each piece on the boardsquare
                            for otherpieceid in piecesonboardsquare.iter(){
                                
                                let ownerofotherpiece = self.get_owner_of_piece( *otherpieceid);
                                
                                //if its owned by the owner of the piece performing the action
                                if ownerofotherpiece == owner{
                                    alliedpiecesonsquare = true;
                                }
                                //if its owned by a different player
                                else{
                                    opposingpiecesonsquare = true;
                                }
                            }
                            
                        }
                        
                        
                        
                        if action == curaction{
                            
                            //if this square has any of my pieces on it, return false
                            if alliedpiecesonsquare{
                                return false;
                            }
                            //if it doesnt have any of my pieces on it
                            //if this square has (an) opponents piece(s) on it
                            else if opposingpiecesonsquare{
                                if cancapture{
                                    return true;
                                }
                                else{
                                    return false;
                                }
                            }
                            //if this square doesnt have any of my, or my opponents pieces on it
                            else{
                                
                                //if it doesnt have to capture a piece, return true
                                if ! hastocapture{
                                    return true;
                                }
                                //if it has to capture a piece, then return false
                                else{
                                    return false;
                                }
                            }
                        }
                        
                        //if this is a boardsquare on the way to the final position
                        else{
                            
                            //if its not empty, return false
                            //if this square has any pieces on it break
                            if opposingpiecesonsquare || alliedpiecesonsquare{
                                return false;
                            };
                        }
                        
                        
                    }
                    
                    
                }
            }
        }
        
        
        return false;
        //panic!("returning base case for whatever reason");
        
    }
    
    
    
    //if a piece can perform this action, what objects will it target
    pub fn get_objects_targeted_by_action(&self, pieceid: u16, action: PieceAction) -> Vec<u16>{        
        
        //if the piece is on a boardsquare
        if let Some(bsid) = self.boardgame.get_board_square_piece_is_on(pieceid){
            
            //the position of the boardsquare its on
            let startpos = self.boardgame.boardsquare_id_to_posid(bsid).unwrap();
            
            //the board square this action takes the piece
            if let Some(posid) = action.get_square_posid_that_action_takes_piece_at_posid(startpos){
                
                if let Some(endid) = self.boardgame.boardsquare_posid_to_id(posid){
                    
                    let mut toreturn = Vec::new();
                    
                    let pieces = self.boardgame.get_pieces_on_board_square(endid);
                    
                    toreturn.push(endid);
                    
                    for pieceid in pieces{
                        toreturn.push( pieceid );
                    }
                    
                    return toreturn;
                }
            }
        }
        
        
        return Vec::new() ;
        
    }
    
    
    
    //create a piece
    fn create_piece(&mut self, pos: (u8,u8), owner: u8) -> u16{
        
        let pieceid = self.boardgame.new_piece(pos);
        let mut piecedata = PieceData::new();
        
        self.playertopiece.get_mut(&owner).unwrap().insert(pieceid);
        self.piecetypedata.insert(pieceid, piecedata);
        
        pieceid
    }
    
    
    //make an existing piece a certain type of piece
    fn set_pawn(&mut self, pieceid: u16){
        
        let piecedata = self.piecetypedata.get_mut(&pieceid).unwrap();
        piecedata.set_pawn();
        
    }
    fn set_knight(&mut self, pieceid: u16){
        
        let piecedata = self.piecetypedata.get_mut(&pieceid).unwrap();
        piecedata.set_knight();
        
    }
    fn set_rook(&mut self, pieceid: u16){
        
        let piecedata = self.piecetypedata.get_mut(&pieceid).unwrap();
        piecedata.set_rook();
        
    }
    fn set_bishop(&mut self, pieceid: u16){
        
        let piecedata = self.piecetypedata.get_mut(&pieceid).unwrap();
        piecedata.set_bishop();
        
    }
    fn set_queen(&mut self, pieceid: u16){
        
        let piecedata = self.piecetypedata.get_mut(&pieceid).unwrap();
        piecedata.set_queen();
        
    }
    fn set_king(&mut self, pieceid: u16){
        
        let piecedata = self.piecetypedata.get_mut(&pieceid).unwrap();
        piecedata.set_king();
        
    }
    fn set_pool_ball(&mut self, pieceid: u16){
        
        let typedata = self.piecetypedata.get_mut(&pieceid).unwrap();       
        typedata.set_pool_ball();

        if self.piecetypedata.get(&pieceid).unwrap().get_type_name() != "poolball"{

            self.boardgame.make_object_pool_ball_shape(&pieceid);
        }

    }

    fn set_chess_piece(&mut self, pieceid: u16){
        
        let typedata = self.piecetypedata.get_mut(&pieceid).unwrap();       
        typedata.set_chess_piece();
        
        
        self.boardgame.make_object_piece_shape(&pieceid);
        
    }

    fn set_checkers(&mut self, pieceid: u16){
        
        let typedata = self.piecetypedata.get_mut(&pieceid).unwrap();       
        typedata.set_checkers();
    }
    
    
    
    
    
    //get the list of every object in the physical engine
    pub fn get_object_ids(&self) -> Vec<u16>{
        
        let mut toreturn = self.boardgame.get_piece_ids();
        
        toreturn.extend( self.boardgame.get_square_ids() );
        
        toreturn
        
    }    
    
    
    pub fn does_player_have_king(&self, playerid: u8) -> bool{
        
        let mut kingexists = false;
        
        //for every piece that player owns
        for pieceid in self.playertopiece.get(&playerid).unwrap(){
            
            //if the player has a piece with the name king
            let piecedata = self.piecetypedata.get(pieceid).unwrap();
            if piecedata.get_type_name() == "king"{
                kingexists = true;
            }
        }
        
        
        kingexists
    }

    pub fn does_player_have_pieces(&self, playerid: &u8) -> bool{


        if self.playertopiece.get(playerid).unwrap().len() == 0{
            return false;
        }
        else{
            return true;
        }

    }
    
    
    
    
    
    //get the row from a players perspective (0 is closest row to player, 7 is farther row from player)
    //and returns what row that is
    fn subjective_row_to_objective_row(playerid: &u8, subjectiverow: &u8) -> u8{
        
        if playerid == &1{
            
            return *subjectiverow;
        }
        else if playerid == &2{
            
            return 7 - subjectiverow;
        }
        else{
            panic!("no player other than 1 and 2");
        }
    }
    
    
    
    fn remove_piece(&mut self, pieceid: u16){
        
        let playerid = self.get_owner_of_piece(pieceid);
        
        self.playertopiece.get_mut(&playerid).unwrap().remove(&pieceid);
        
        self.piecetypedata.remove(&pieceid);
        
        self.boardgame.remove_piece(&pieceid);
    }
    
    
    //get the id of every board square in the game
    fn get_squares(&self) -> Vec<u16>{
        
        self.boardgame.get_square_ids()
    }
    
    
    //get the id of every board square without a piece on it
    //and that arent on a mission currently
    fn get_empty_squares_not_on_mission(&self) -> Vec<u16>{
        
        let bsids = self.get_squares();
        
        let mut toreturn = Vec::new();
        
        
        for bsid in bsids{
            
            let piecesonboardsquare = self.boardgame.get_pieces_on_board_square(bsid);
            
            //if it doesnt have anything on it
            if piecesonboardsquare.is_empty(){
                
                //if its not on a mission
                if ! self.boardgame.is_object_on_mission(&bsid){
                    
                    //then push it into the list of empty squares not on a mission
                    toreturn.push( bsid );
                }
            }
        }
        
        return toreturn;
    }
    
    
    pub fn perform_action(&mut self, piece: u16, pieceaction: PieceAction ){
        
        //set that piece to having moved
        if let PieceAction::liftandmove(relativeposition) = pieceaction{
            
            let relativeposition = pieceaction.get_relative_position_action_takes_piece();
            let floatrelpos = (relativeposition.0 as f32, relativeposition.1 as f32);
            
            self.boardgame.lift_and_move_piece_to(piece, floatrelpos);
            
            
            for (tick, _, squareid) in self.get_passed_over_squares(pieceaction, piece){
                
                self.boardgame.set_future_boardsquare_drop(tick, squareid);
            };
            
        }
        else if let PieceAction::slide(slidedirection, slidedistance) = pieceaction{
            
            let relativeposition = pieceaction.get_relative_position_action_takes_piece();
            let floatrelpos = (relativeposition.0 as f32, relativeposition.1 as f32);
            
            self.boardgame.slide_piece(piece, floatrelpos);
            
            for (tick, _, squareid) in self.get_passed_over_squares(pieceaction, piece){
                
                self.boardgame.set_future_boardsquare_drop(tick, squareid);
            };
        }
        else if let PieceAction::flick(direction, force) = pieceaction{
            
            self.boardgame.flick_piece(piece, direction, force);
        }
        
        
        //set the piece has having moved
        self.piecetypedata.get_mut(&piece).unwrap().moved_piece();
    }
    
    
    pub fn get_owner_of_piece(& self, pieceid: u16) -> u8{
        
        for (player, pieces) in self.playertopiece.clone(){
            
            for playerspieceid in pieces{
                
                if playerspieceid == pieceid{
                    
                    return player;
                }
            }
        }
        
        panic!("cant find the piece, doesnt seem to exist");
    }
    
}








//getters used only outside of this module
impl GameEngine{
    
    
    pub fn is_object_on_mission(&self, id: u16) -> bool{
        
        self.boardgame.is_object_on_mission(&id)
    }
    
    
    pub fn get_actions_allowed_by_piece(&self, pieceid: u16) -> (bool, Vec<PieceAction>){
        
        
        //get the piece data
        let piecedata = self.piecetypedata.get(&pieceid).unwrap();
        //the owner of the piece
        let owner = self.get_owner_of_piece(pieceid);
        //the direction of the owner of the piece
        let ownerdirection = self.playertodirection.get(&owner).unwrap();
        
        
        //get all the actions this piece can potentially perform
        let allactions = piecedata.get_piece_actions(*ownerdirection);
        
        //the list of allowed actions to return
        let mut allowedactions: Vec<PieceAction> = Vec::new();
        
        
        //for every action, get if it is allowed
        for action in allactions{
            
            if self.is_action_allowed(action.clone(), pieceid){
                
                allowedactions.push( action );
            };
        };
        
        
        
        let flickable;
        
        //if its on a boardsquare
        if let Some(_) = self.boardgame.get_board_square_piece_is_on(pieceid){
            flickable = piecedata.canflick();
        }
        else{
            flickable = false;
        }
        
        
        return (flickable, allowedactions);
    }
    
    //is this board game object a square
    pub fn is_board_game_object_square(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_square(objectid)
    }
    
    //is this board game object a piece
    pub fn is_board_game_object_piece(&self, objectid: u16) -> bool{
        self.boardgame.is_board_game_object_piece(objectid)
    }
    
    //get the name of the type of the piece
    pub fn get_piece_type_name(&self, pieceid: u16) -> String{
        
        let piecetypedata = self.piecetypedata.get(&pieceid).unwrap();
        
        piecetypedata.get_type_name()
        
    }
    
    pub fn is_boardsquare_white(&self, bsid: u16 ) -> bool{
        
        let bspos = self.boardgame.boardsquare_id_to_posid(bsid).unwrap();
        
        let bstotal = bspos.0 + bspos.1;
        
        let evenness = bstotal % 2;
        
        
        if evenness == 0{
            return true;
        }
        else{
            return false;
        }
        
    }
    
    pub fn get_object_translation(&self, gameobjectid: u16) -> (f32,f32,f32){
        self.boardgame.get_translation(gameobjectid)
    }
    
    pub fn get_object_rotation(&self, gameobjectid: u16) -> (f32,f32,f32){
        self.boardgame.get_rotation(gameobjectid)
    }
    
}