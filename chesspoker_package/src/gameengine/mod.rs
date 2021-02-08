mod boardgameinterface;
mod datastructs;
use datastructs::PieceData;
use boardgameinterface::BoardGame;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

//use datastructs::slide_id_to_direction_change_from_objective_perspective;

pub use datastructs::PieceAction;








#[derive(Serialize, Deserialize)]
pub struct GameEngine{
    
    
    //the pieces that this player owns
    playertopiece: HashMap<u8, HashSet<u16> >,
    
    //player 11 is the pool of player 1
    //player 12 is the pool of player 2
    
    
    
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


        //the pools for the players
        gameengine.playertopiece.insert(11, HashSet::new());
        gameengine.playertopiece.insert(12, HashSet::new());

        
        gameengine.playertodirection.insert(player1id, 0 );
        gameengine.playertodirection.insert(player2id, 4 );
        
        gameengine.init_chess_game();
        
        gameengine
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
    
    
    pub fn are_pieces_offered_valid(&self, playerid: u8, piecesoffered: Vec<u16>) -> bool{
        
        let mut allpiecesvalid = true;
        
        
        //for every piece, get if its owned by the player
        //and is worth at least as much as the value suggested, if true
        for pieceid in piecesoffered{
            
            //if its owned by the player
            if self.playertopiece.get(&playerid).unwrap().contains(&pieceid){
                
                let piecedata = self.piecetypedata.get(&pieceid).unwrap();
                
                //if the piece has a zero value
                if piecedata.get_value() == 0{

                    allpiecesvalid = false;
                }
            }
            else{
                allpiecesvalid = false;
            }
        }
        
        
        allpiecesvalid
    }
    
    
    //if the pieces offered are valid
    //and if so, what their total value is
    pub fn get_value_of_offered_pieces(&self, playerid: u8, piecesoffered: Vec<u16>) -> Option<u8>{
        
        //if its not valid, return None
        if ! self.are_pieces_offered_valid(playerid, piecesoffered.clone()){
            return None;
        };
                
        let mut totalvaluerequested = 0;
        

        for pieceid in piecesoffered{

            let piecedata = self.piecetypedata.get(&pieceid).unwrap();
                
            totalvaluerequested += piecedata.get_value();
        }
        
        
        Some( totalvaluerequested )
    }
    
    
    //give every piece in the pool to this player
    pub fn give_pool_to_player(&mut self, playerid: u8){
        
        //give the pieces in player 1s pool
        for pieceid in self.playertopiece.get(&11).unwrap().clone(){
            
            self.transfer_ownership(pieceid, playerid);
        }
        
        
        //give the pieces in player 2s pool
        for pieceid in self.playertopiece.get(&12).unwrap().clone(){
            
            self.transfer_ownership(pieceid, playerid);
        }
        
    }
    
    
    
    
    //get the value of the pieces in the pool of this player
    pub fn get_value_of_players_pool(&self, playerid: u8) -> u8{
        
        let mut totalvalue = 0;
        
        let poolid = playerid + 10;
        
        //get every piece owned by this players pool and add their values
        for pieceid in self.playertopiece.get(&poolid).unwrap(){
            
            let piecedata = self.piecetypedata.get(pieceid).unwrap();
            
            totalvalue += piecedata.get_value();
        }
        
        
        return totalvalue;
        
    }
    
    
    //put the amount of value from these pieces into the pool of player X
    pub fn put_pieces_in_pool(&mut self, pieces: Vec<u16>){
        
        
        //for each piece and value for it
        for pieceid in pieces{

            //transfer the ownership of the piece to put in the pool to the pool of the player who owns it
            self.transfer_piece_to_pool(pieceid);
        };
        
    }
    
    
    //split a piece in two, ensuring that one of the pieces split's value is equal to the amount passed in
    //return the ID of both pieces that were made when this one split
    //and have the first ID be of the piece who's value is equal to the amount
    fn split_piece(&mut self, pieceid: u16, amount: u8) -> (u16, u16){
        
        
        
        panic!("no");
        
    }
    
    
    
    
    fn transfer_piece_to_pool(&mut self, pieceid: u16){
        
        
        let pieceowner = self.get_owner_of_piece(pieceid);
        
        let thisplayerspoolid = pieceowner + 10;
        
        self.transfer_ownership(pieceid, thisplayerspoolid);
        
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
    fn init_checkers_game(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let firstrow = 0;
            
            
            let id = self.create_piece( (0, 1), playerx);
            self.set_checkers(id);
            
            
        };
        
    }
    
    
    
    
    
    
    
    
    
    //make this a pool game
    //make every piece a pool ball shape
    //and remove all their actions aside from flick
    //drop 6 random empty squares
    pub fn make_pool_game(&mut self){
        
        //for every piece with a type
        for (pieceid, _) in self.piecetypedata.clone().iter(){
            
            self.set_pool_ball(*pieceid);            
        }
        
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
            if self.boardgame.is_object_on_mission(boardsquareid){
                return false;
            }
            //if the current piece is on a mission, return false
            if self.boardgame.is_object_on_mission(pieceid){
                return false;
            }
            
            //the position of the boardsquare its on
            let boardsquarepos = self.boardgame.get_pos_id_of_boardsquare(boardsquareid).unwrap();
            
            
            //if its a lift and move action
            if let PieceAction::liftandmove( relativepos ) = action{
                
                //the position of the boardsquare this action takes the piece
                let endpos = action.get_square_pos_that_action_takes_piece_at_pos( boardsquarepos );
                
                //the id of this board square it lands on if it exists
                if let Some(endid) = self.boardgame.get_id_of_boardsquare_i8_pos(endpos){
                    
                    //if the boardsquare is on a mission, return false
                    if self.boardgame.is_object_on_mission(endid){
                        return false;
                    }
                    
                    
                    //get if this lift and move action has to capture, and if it can capture
                    let (hastocapture, cancapture) = piecedata.get_capture_type_of_lift_and_move( relativepos, *ownerdirection );
                    
                    
                    //does the boardsquare have any enemy pieces on it
                    let mut opposingpiecesonsquare: bool = false; 
                    //does the boardsquare have any ally pieces on it
                    let mut alliedpiecesonsquare: bool = false;
                    
                    {
                        //get the pieces on the boardsquare
                        let piecesonboardsquare = self.boardgame.get_pieces_on_board_square( endid );
                        
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
                    
                    
                    
                    //if this square has any of my pieces on it, return false
                    if alliedpiecesonsquare{
                        return false;
                    }
                    //if it doesnt have any of my pieces on it
                    else{
                        
                        //if this square has (an) opponents piece(s) on it
                        if opposingpiecesonsquare{
                            if cancapture{
                                return true;
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
                }
                
                
            }
            //if this is a slide action
            else if let PieceAction::slide( direction, distance ) = action{
                
                //get if this lift and move action has to capture, and if it can capture
                let (hastocapture, cancapture) = piecedata.get_capture_type_of_slide(direction, distance, *ownerdirection);
                
                
                let mut stepnumber = 1;
                let mut actiontogethere = PieceAction::slide(direction, stepnumber);
                let mut cursquarepos = actiontogethere.get_square_pos_that_action_takes_piece_at_pos(boardsquarepos);
                
                
                //if the board square exists
                while let Some(cursquareid) = self.boardgame.get_id_of_boardsquare_i8_pos(cursquarepos){
                    
                    
                    //if the current square is on a mission, break
                    if self.boardgame.is_object_on_mission(cursquareid){
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
                    
                    
                    //if this is final boardsquare the action will arrive at
                    if action == actiontogethere{
                        
                        //if this square has any of my pieces on it, return false
                        if alliedpiecesonsquare{
                            return false;
                        }
                        //if it doesnt have any of my pieces on it
                        else{
                            
                            //if this square has (an) opponents piece(s) on it
                            if opposingpiecesonsquare{
                                if cancapture{
                                    return true;
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
                        
                    }
                    //if this is a boardsquare on the way to the final position
                    else{
                        
                        //if its not empty, return false
                        //if this square has any pieces on it break
                        if opposingpiecesonsquare || alliedpiecesonsquare{
                            return false;
                        };
                        
                    }
                    
                    
                    //increase the step
                    stepnumber += 1;
                    //update the action
                    actiontogethere = PieceAction::slide(direction, stepnumber);
                    //update the boardsquare pos
                    cursquarepos = actiontogethere.get_square_pos_that_action_takes_piece_at_pos(boardsquarepos);
                }
            }
            //if this is a flick action
            else if let PieceAction::flick(direction, force) = action{
                
                //if the piece data allowd it, as long as its on a boardsquare
                //a flick is allowed
                return true;
            }
        }
        
        
        return false;
    }
    
    
    
    
    
    
    
    
    
    //if a piece can perform this action, what objects will it target
    pub fn get_objects_targeted_by_action(&self, pieceid: u16, action: PieceAction) -> Vec<u16>{        
        
        //if the piece is on a boardsquare
        if let Some(bsid) = self.boardgame.get_board_square_piece_is_on(pieceid){
            
            //the position of the boardsquare its on
            let startpos = self.boardgame.get_pos_id_of_boardsquare(bsid).unwrap();
            
            //the board square this action takes the piece
            let pos = action.get_square_pos_that_action_takes_piece_at_pos(startpos);
            
            
            if let Some(endid) = self.boardgame.get_id_of_boardsquare_i8_pos(pos){
                
                let mut toreturn = Vec::new();
                
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
        
        self.boardgame.make_object_pool_ball(&pieceid);
    }
    
    fn set_checkers(&mut self, pieceid: u16){
        
        let typedata = self.piecetypedata.get_mut(&pieceid).unwrap();       
        typedata.set_checkers();
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

        //its if the players opponent hasnt drawn a card yet
        //create a new king if it doesnt have one
        //otherwise the main struct ticking will end the game and make this player lose

        //if either player doesnt have a king
        //turn their highest valued piece into a king
        for playerid in 1..3{

            let playerhasking = self.does_player_have_king(playerid);


            //if they dont
            if ! playerhasking{

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


        //remove the pieces that are lower than -5 in pos
        for (pieceid, _) in &self.piecetypedata.clone(){

            let pos = self.boardgame.get_translation(*pieceid);

            if pos.1 < -3.0{

                self.remove_piece(*pieceid);
            }
        }
        



        self.boardgame.tick();
    }
    


    pub fn remove_piece(&mut self, pieceid: u16){

        //panic!("removed piece");


        let playerid = self.get_owner_of_piece(pieceid);

        self.playertopiece.get_mut(&playerid).unwrap().remove(&pieceid);

        self.piecetypedata.remove(&pieceid);

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
        
        let bsids = self.get_squares();
        
        let mut toreturn = Vec::new();
        
        
        for bsid in bsids{
            
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
    
    pub fn perform_action(&mut self, piece: u16, pieceaction: PieceAction ){
        
        //set that piece to having moved
        if let PieceAction::liftandmove(relativeposition) = pieceaction{
            
            let relativeposition = pieceaction.get_relative_position_action_takes_piece();
            let floatrelpos = (relativeposition.0 as f32, relativeposition.1 as f32);
            
            self.boardgame.lift_and_move_piece_to(piece, floatrelpos);
            
        }
        else if let PieceAction::slide(slidedirection, slidedistance) = pieceaction{
            
            let relativeposition = pieceaction.get_relative_position_action_takes_piece();
            let floatrelpos = (relativeposition.0 as f32, relativeposition.1 as f32);
            
            //panic!("{:?}", floatrelpos);

            self.boardgame.slide_piece(piece, floatrelpos);
            
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
        
        let bspos = self.boardgame.get_pos_id_of_boardsquare(bsid).unwrap();
        
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



    //only for testing
    pub fn does_piece_have_owner(&self, pieceid: u16) -> bool{


        for (player, pieces) in self.playertopiece.clone(){
            
            for playerspieceid in pieces{
                
                if playerspieceid == pieceid{
                    
                    return true;
                }
            }
        }

        return false;
    }
}

