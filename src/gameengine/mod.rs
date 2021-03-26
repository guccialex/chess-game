mod boardgameinterface;
mod datastructs;
use datastructs::PieceData;
use boardgameinterface::BoardGame;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;

//use datastructs::slide_id_to_direction_change_from_objective_perspective;

pub use datastructs::PieceAction;


pub use boardgameinterface::BoardSquarePosID;


//use boardgameinterface::convert_physical_pos_to_board_square_pos;
//use boardgameinterface::convert_board_square_pos_to_physical_pos;
//use datastructs::is_square_posid_valid;
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
    
    //add the pieces to the game that a chess game would havef
    pub fn add_checkers_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            for column in 0..8{
                
                let firstrow;
                
                if playerx == 1{
                    firstrow = 2;
                }
                else{
                    firstrow = 6;
                }
                
                
                let id = self.create_piece( (column, firstrow), playerx);
                self.set_checker(id);
                
            }
        };
        
    }
    
    
    //set the number of squares raised
    pub fn set_randomly_raised_squares(&mut self, numbertoraise: u32){
        
        //get the number of raised squares
        let curraisedsquares = self.boardgame.get_raised_squares();
        
        //how many more raised squares I have than I need
        let raiseddifference = curraisedsquares.len() as i32 - numbertoraise as i32;
        
        
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
                
                let squareid = self.boardgame.boardsquare_posid_to_id(potentialsquares[x].clone()).unwrap();
                
                self.boardgame.set_long_boardsquare_raise(10000, squareid);
            }
            
        }
        
    }

    //set the number of squares that should be randomly dropped
    pub fn set_randomly_dropped_squares(&mut self, numbertodrop: u32){
        
        
        
        //get the number of dropped squares
        let curdroppedsquares = self.boardgame.get_dropped_squares();
        
        //how many more raised squares I have than I need
        let droppeddifference = curdroppedsquares.len() as i32 - numbertodrop as i32;
        
        
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
                
                let squareid = self.boardgame.boardsquare_posid_to_id(potentialsquares[x].clone()).unwrap();
                
                self.boardgame.set_long_boardsquare_raise(10000, squareid);
            }
            
        }
        
        
        
    }
    

    pub fn split_piece_into_pawns(&mut self){


        //get each players highest valued piece
        //turn it into as many pawns as that piece was valued
        
        
        for playerid in 1..3{

            //get the players pieces

            let mut highestpiecevalueandid = (0,0);

            for pieceid in self.playertopiece.get(&playerid).unwrap(){

                let curvalue = self.piecetypedata.get(&pieceid).unwrap().get_value();

                if curvalue >= highestpiecevalueandid.0{

                    highestpiecevalueandid = (curvalue, *pieceid);
                };
            };

            //remove the highest valued piece
            self.remove_piece(highestpiecevalueandid.1);


            //create as many pawn pieces as that highest value pieces value is
            for x in 0..highestpiecevalueandid.0{

                let pos = self.get_empty_squares_not_on_mission()[0].get_pos();
            
    
                let id = self.create_piece( pos, playerid);
                self.set_pawn(id);        
    
            }
        }


    }


    //give all pieces with a value greater than 1 the ability of knights
    pub fn knightify(&mut self){

        for (_, piecedata) in &mut self.piecetypedata{
            piecedata.augment_knight_abilities();
        }
    }

    pub fn unaugment_abilities(&mut self){

        for (_, piecedata) in &mut self.piecetypedata{
            piecedata.remove_ability_augmentations();
        }

    }



    pub fn checkerify(&mut self){


        
        for playerid in 1..3{

            //get the sum of the value of the players pieces and remove them

            let mut valuesum = 0;

            for pieceid in self.playertopiece.get(&playerid).unwrap().clone(){

                let curvalue = self.piecetypedata.get(&pieceid).unwrap().get_value();

                valuesum += curvalue;

                self.remove_piece(pieceid);

            };



            //create half as many checkers pieces as that players total value of pieces
            for x in 0.. valuesum/2 +1 {

                let pos = self.get_empty_squares_not_on_mission()[0].get_pos();

                let id = self.create_piece( pos, playerid);
                self.set_checker(id);        
            };
        };

        
    }


    
    //tick, with true if kings are replaced and false if theyre not
    pub fn tick(&mut self, arekingsreplaced: bool, arepawnspromoted: bool){
        
        
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
                
                if let Some( bsposid ) = self.boardgame.get_board_square_piece_is_on(pieceid){
                    
                    //if that pawn is on the backrow
                    if bsposid.get_row() == backrow{
                        
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
    
    
    
    
    //get if this action is allowed by this piece
    pub fn is_action_allowed(&self, action: &PieceAction, pieceid: &u16) -> bool{
        
        //get the owner of this piece
        let owner = self.get_owner_of_piece(*pieceid);
        
        //the direction of the owner
        let ownerdirection = self.playertodirection.get(&owner).unwrap();
        
        //if this is is one of the actions the piece is allowed to perform
        let piecedata = self.piecetypedata.get(&pieceid).unwrap();
        
        
        //get the square that its on
        if let Some(squareposid) = self.boardgame.get_board_square_piece_is_on(*pieceid){
            
            //if the action is allowed by the piecedata
            if let Some(squareconditions) = piecedata.is_action_valid(action, ownerdirection){
                
                
                //for every square and condition for that square
                for (relativesquare, squarecondition) in squareconditions{
                    
                    //if that square exists
                    if let Some(cursquarepos) = squareposid.new_add_relative_pos( &relativesquare){
                        
                        
                        use datastructs::SquareCondition;
                        
                        
                        //get whats on the square
                        let piecesonsquare = self.boardgame.get_pieces_on_board_square(&cursquarepos);
                        
                        
                        
                        match squarecondition{
                            
                            //if the square needs to be empty
                            SquareCondition::EmptyRequired => { 
                                
                                if ! piecesonsquare.is_empty(){
                                    return false;
                                };
                            },
                            //if the square cant have a friendly piece on it
                            SquareCondition::NoneFriendlyRequired =>{
                                
                                //for every piece on the square
                                for otherpieceid in piecesonsquare{
                                    
                                    if self.get_owner_of_piece(*pieceid) == self.get_owner_of_piece(otherpieceid){
                                        return false;
                                    };    
                                };
                            },
                            //if there needs to be at least one opponents piece on this square
                            SquareCondition::OpponentRequired =>{
                                
                                let mut opponentspiece = false;
                                
                                //for every piece on the square
                                for otherpieceid in piecesonsquare{
                                    
                                    if self.get_owner_of_piece(*pieceid) != self.get_owner_of_piece(otherpieceid){
                                        opponentspiece = true;
                                    };
                                };
                                
                                if opponentspiece == false{
                                    return false;
                                };
                            },
                        };
                        
                        
                    }
                    //if the boardsquare targeted isnt valids
                    else{
                        return false;
                    };
                };
            }
            //if the action isnt allowed by the piecedata
            else{
                return false;
            };
        }
        //if its not on a square
        else{
            return false;
        };
        
        
        //if all the actions conditions were met, or none of them werent met
        return true;
    }
    
    
    pub fn perform_action(&mut self, piece: u16, pieceaction: PieceAction ){
        
        
        if let Some(liftandmoveforces) = pieceaction.get_lift_and_move_forces(){
            let floatrelpos = (liftandmoveforces.0 as f32, liftandmoveforces.1 as f32);
            self.boardgame.lift_and_move_piece_to(piece, floatrelpos);
        };
        
        if let Some(slideforces) = pieceaction.get_slide_forces(){
            let floatrelpos = (slideforces.0 as f32, slideforces.1 as f32);
            self.boardgame.slide_piece(piece, floatrelpos);
        };
        
        if let Some( (direction, force) ) = pieceaction.get_flick_forces(){
            self.boardgame.flick_piece(piece, direction, force);
        };
        
        
        
        //drop the boardsquares that should be dropped when they should be dropped
        for (squareposrelative, tick) in pieceaction.get_squares_dropped_relative(){
            
            let squareposid = self.boardgame.get_board_square_piece_is_on(piece).unwrap();
            
            if let Some(relativesquareposid) = squareposid.new_add_relative_pos( &squareposrelative ){
                
                let relativesquareid = self.boardgame.boardsquare_posid_to_id(relativesquareposid).unwrap();
                self.boardgame.set_future_boardsquare_drop(tick, relativesquareid);
                
            };
        };
        
        
        //set the piece has having moved
        self.piecetypedata.get_mut(&piece).unwrap().moved_piece();
    }
    
    
    
    
    
    
    //create a piece
    fn create_piece(&mut self, pos: (i8,i8), owner: u8) -> u16{
        
        let boardsquareposid = BoardSquarePosID::new(pos).unwrap();
        
        let pieceid = self.boardgame.new_piece(boardsquareposid);
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
    fn set_checker(&mut self, pieceid: u16){
        
        let typedata = self.piecetypedata.get_mut(&pieceid).unwrap();       
        typedata.set_checker();
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
    fn subjective_row_to_objective_row(playerid: &u8, subjectiverow: &i8) -> i8{
        
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
    fn get_square_ids(&self) -> Vec<u16>{
        self.boardgame.get_square_ids()
    }
    
    
    //get the id of every board square without a piece on it
    //and that arent on a mission currently
    fn get_empty_squares_not_on_mission(&self) -> Vec<BoardSquarePosID>{
        
        let bsids = self.get_square_ids();
        
        let mut toreturn = Vec::new();
        
        
        for bsid in bsids{
            
            let bsposid = self.boardgame.boardsquare_id_to_posid(bsid).unwrap();
            let piecesonboardsquare = self.boardgame.get_pieces_on_board_square(&bsposid);
            
            //if it doesnt have anything on it
            if piecesonboardsquare.is_empty(){
                
                //if its not on a mission
                if ! self.boardgame.is_object_on_mission(&bsid){
                    
                    //then push it into the list of empty squares not on a mission
                    toreturn.push( bsposid );
                }
            }
        }
        
        return toreturn;
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
    
    
    //get the pieces that are targeted by a piece performing an action
    fn get_piece_targets_of_action(&self, pieceid: &u16, action: &PieceAction) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        //get the boardsquares dropped by this action
        
        //get the pieces on those boardsquares
        
        if let Some(boardsquareid) = self.boardgame.get_board_square_piece_is_on(*pieceid){
            
            for (relativeposid, _) in action.get_squares_dropped_relative(){
                
                if let Some(newboardsquare) = boardsquareid.new_add_relative_pos( &relativeposid){
                    
                    toreturn.extend( self.boardgame.get_pieces_on_board_square( &newboardsquare )  );
                    
                };
            };
        };
        
        
        toreturn        
    }
    
    //get the action that this piece can perform now, and the objects it targets
    pub fn get_piece_valid_actions_and_targets(&self, pieceid: u16) -> (bool, Vec< (PieceAction, Vec<u16>) >){
        
        
        //get the piece data
        let piecedata = self.piecetypedata.get(&pieceid).unwrap();
        //the owner of the piece
        let owner = self.get_owner_of_piece(pieceid);
        //the direction of the owner of the piece
        let ownerdirection = self.playertodirection.get(&owner).unwrap();
        
        //get all the actions this piece can potentially perform
        let allactions = piecedata.get_numberable_piece_actions(ownerdirection);
        
        
        let mut actionsandtargets: Vec<(PieceAction, Vec<u16>)> = Vec::new();
        
        
        //for every action, get if it is allowed
        for action in allactions{
            
            if self.is_action_allowed(&action.clone(), &pieceid){
                
                let mut targets = self.get_piece_targets_of_action(&pieceid, &action);
                
                
                let curbsposid = self.boardgame.get_board_square_piece_is_on(pieceid).unwrap();
                
                if let Some(targetbsposid) = curbsposid.new_add_relative_pos( &action.get_relative_position_action_takes_piece() ){
                    
                    targets.push( self.boardgame.boardsquare_posid_to_id(targetbsposid).unwrap() );                    
                }                
                
                actionsandtargets.push( (action, targets) );
            };
        };
        
        
        let flickable = self.is_action_allowed( &PieceAction::flick(1.0, 1.0) , &pieceid);
        
        return (flickable, actionsandtargets);        
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
        
        let bsposid = self.boardgame.boardsquare_id_to_posid(bsid).unwrap();
        
        let bspos = bsposid.get_pos();
        
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