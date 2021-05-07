mod datastructs;
mod boardphysics;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use datastructs::PieceData;
pub use datastructs::FullAction;
use datastructs::SquareCondition;
use datastructs::PieceType;

use datastructs::BoardSquarePosID;
use datastructs::RelativeSquare;



use boardphysics::BoardPhysics;







#[derive(Serialize, Deserialize)]
pub struct BoardObjects{
    
    boardsquares: HashMap<BoardSquarePosID, u16>,
    
    pieces: HashSet<u16>,
    
    pieceowners: HashMap<u16, u8>,
    
    piecedata: HashMap<u16, PieceData>,
}

impl BoardObjects{
    
    
    fn get_value_of_pieces(&self, pieces: Vec<u16>) -> u8{
        
        let mut totalvalue = 0;
        
        for pieceid in pieces{
            
            //if its not a boardsquare
            if ! self.is_object_boardsquare(&pieceid){
                
                totalvalue += self.get_piecedata(&pieceid).get_value();
                
            }
        }
        
        return totalvalue;
    }
    
    fn get_owners_highest_value_piece(&self, owner: &u8 ) -> (u16, u8){
        
        let mut highestvalue: i8 = -1;
        let mut highestvalueid: Option<u16> = None;
        
        for (curpieceid, curowner) in &self.pieceowners{
            
            if owner == curowner{
                
                let piecedata = self.piecedata.get( &curpieceid ).unwrap();
                let curvalue = piecedata.get_value() as i8;
                
                if  curvalue > highestvalue{
                    
                    highestvalueid = Some(*curpieceid);
                    highestvalue = curvalue;
                }
            }
        }
        
        
        if let Some(toreturn) = highestvalueid{
            return (toreturn, highestvalue as u8);
        }
        
        panic!("no pieces for this player");
    }
    
    
    fn new() -> BoardObjects{
        
        BoardObjects{
            boardsquares: HashMap::new(),
            pieces: HashSet::new(),
            pieceowners: HashMap::new(),
            piecedata: HashMap::new(),
        }
    }
    
    fn add_piece(&mut self, objectid: u16, owner: u8, piecedata: PieceData){
        
        self.pieces.insert( objectid);
        
        self.pieceowners.insert( objectid, owner );
        
        self.piecedata.insert( objectid, piecedata);
    }
    
    
    fn add_boardsquare(&mut self, pos: BoardSquarePosID, objectid: u16){
        self.boardsquares.insert( pos, objectid);
    }
    
    fn get_boardsquares(&self) -> HashSet<BoardSquarePosID>{
        
        let mut toreturn = HashSet::new();
        
        for (bsid, _) in &self.boardsquares{
            toreturn.insert(bsid.clone());
        }
        return toreturn;
    }
    
    fn get_pieces(&self) -> HashSet<u16>{
        
        self.pieces.clone()
    }
    
    fn get_boardsquare_object_id(&self, bsid: &BoardSquarePosID) -> Option<u16>{
        
        self.boardsquares.get( bsid ).copied()
    }
    
    fn get_boardsquare_by_object_id(&self, objectid: &u16) -> Option<BoardSquarePosID>{
        
        for (curbsid, curobjectid) in &self.boardsquares{
            
            if curobjectid == objectid{
                
                return Some(curbsid.clone());
            }
        }
        
        return None;
    }
    
    fn get_mut_piecedata(&mut self, pieceid: &u16) -> &mut PieceData{
        self.piecedata.get_mut(pieceid).unwrap()
    }
    
    fn get_piecedata(&self, pieceid: &u16) -> PieceData{
        
        if let Some(pd) = self.piecedata.get(pieceid){
            
            return pd.clone();
        }
        else{
            return PieceData::new();
        }
    }
    
    fn get_players_pieces(&self, playerid: &u8) -> HashSet<u16>{
        
        let mut toreturn = HashSet::new();
        
        for (pieceid, owner) in &self.pieceowners{
            
            if owner == playerid{
                toreturn.insert(*pieceid);
            }
        }
        
        toreturn
    }
    
    fn get_owner_of_piece(&self, pieceid: &u16) -> Option<u8>{
        
        for (curpieceid, owner) in &self.pieceowners{
            
            if curpieceid == pieceid{
                return Some(*owner);
            }
        }
        
        return None;
        //panic!("piece doesnt have owner or piece not found {:?}", pieceid);
    }
    
    fn remove_piece(&mut self, pieceid: &u16){
        
        self.pieces.remove(pieceid);
        
        self.pieceowners.remove(pieceid);
        
        self.piecedata.remove(pieceid);
    }
    
    fn does_player_have_king(&self, playerid: &u8) -> bool{
        
        //for every piece that player owns
        for (pieceid, owner) in &self.pieceowners{
            
            if owner == playerid{
                
                let piecedata = self.piecedata.get(&pieceid).unwrap();
                
                if piecedata.is_this_piecetype(&PieceType::King){
                    return true;
                }   
            }
        }
        
        return false;
    }
    
    fn does_player_have_pieces(&self, playerid: &u8) -> bool{
        
        
        //for every piece that player owns
        for (_, owner) in &self.pieceowners{
            
            if owner == playerid{
                
                return true;   
            }
        }
        
        return false;
        
    }
    
    fn is_object_boardsquare(&self, objectid: &u16) -> bool{
        
        for (_, curobjectid) in self.boardsquares.iter(){
            if objectid == curobjectid{
                return true;
            }
        }
        
        return false;
    }
    
    fn get_boardsquare_object_ids(&self) -> HashSet<u16>{
        
        let mut toreturn = HashSet::new();
        
        for (_, objectid) in &self.boardsquares{
            
            toreturn.insert(*objectid);
        };
        
        toreturn
    }
    
    
    fn get_object_ids(&self) -> HashSet<u16>{
        
        let mut toreturn = self.get_pieces();
        
        toreturn.extend( self.get_boardsquare_object_ids() );
        
        toreturn
    }    
    
}





#[derive(Serialize, Deserialize)]
pub struct GameEngine{
    
    //information about the types of objects in the game
    boardobjects: BoardObjects,
    
    //the direction the player is facing
    playertodirection: HashMap<u8, f32>,
    
    boardphysics: BoardPhysics,
}




impl GameEngine{
    
    
    pub fn new(player1id: u8, player2id: u8) -> GameEngine{
        
        let mut gameengine = GameEngine{
            playertodirection: HashMap::new(),
            boardobjects: BoardObjects::new(),
            boardphysics: BoardPhysics::new(), 
        };
        
        //make the boardsquares
        for x in 0..8{
            for y in 0..8{
                gameengine.create_boardsquare( BoardSquarePosID::new((x,y)).unwrap() );
            }
        }
        
        gameengine.playertodirection.insert(player1id, 0.0 );
        gameengine.playertodirection.insert(player2id, 0.5 );
        
        gameengine
    }
    
    
    //create a piece at this position of this type
    fn create_piece(&mut self, pos: BoardSquarePosID, owner: u8, piecetype: PieceType){
        
        let piecepos = (pos.to_physical_pos().0,  3.0  ,pos.to_physical_pos().1 );
        
        let objectid = self.boardphysics.create_piece_object( piecepos );
        
        let mut piecedata = PieceData::new();
        piecedata.set_piecetype( piecetype);
        
        self.boardobjects.add_piece(objectid, owner, piecedata);
    }
    
    
    fn remove_piece(&mut self, pieceid: u16){
        
        self.boardobjects.remove_piece(&pieceid);
        self.boardphysics.remove_object(&pieceid);
    }
    
    
    fn create_boardsquare(&mut self, bsid: BoardSquarePosID){
        
        let bsidphyspos = (bsid.to_physical_pos().0, 0.0, bsid.to_physical_pos().1 );
        let objectid = self.boardphysics.create_boardsquare_object( bsidphyspos );
        
        self.boardobjects.add_boardsquare(bsid , objectid);
    }
    
    
    fn get_pieces_on_board_square(&self, bsid: &BoardSquarePosID) -> HashSet<u16>{
        
        let mut toreturn = HashSet::new();
        
        //for all pieces
        for pieceid in self.boardobjects.get_pieces(){
            
            //get the piece its on
            if let Some(bsiditson) = self.get_board_square_piece_is_on(&pieceid){
                
                if &bsiditson == bsid{
                    
                    toreturn.insert(pieceid);
                }
            }
        };
        
        return toreturn;
    }
    
    pub fn get_board_square_piece_is_on(&self, pieceid: &u16) -> Option<BoardSquarePosID>{
        
        let piecepos = (self.boardphysics.get_translation(pieceid).0, self.boardphysics.get_translation(pieceid).2);
        
        BoardSquarePosID::from_physical_pos( piecepos )
    }
    
    
    fn is_square_empty(&self, bsid: BoardSquarePosID) -> bool{
        
        let piecesonboardsquare = self.get_pieces_on_board_square( &bsid);
        
        return piecesonboardsquare.is_empty();
    }
    
    
    //get the id of every board square without a piece on it
    //and that arent on a mission currently
    fn get_empty_squares_not_on_mission(&self) -> Vec<BoardSquarePosID>{
        
        let bsids = self.boardobjects.get_boardsquares();
        
        let mut toreturn = Vec::new();
        
        for bsid in bsids{
            
            let bsobjectid = self.boardobjects.get_boardsquare_object_id(&bsid).unwrap();
            
            let piecesonboardsquare = self.get_pieces_on_board_square( &bsid);
            
            //if it doesnt have anything on it
            if piecesonboardsquare.is_empty(){
                
                //if its not on a mission
                if ! self.boardphysics.is_object_on_mission(&bsobjectid) {
                    
                    //then push it into the list of empty squares not on a mission
                    toreturn.push( bsid );
                }
            }
        }
        
        return toreturn;
    }
    
    
    
    
    
    
    //add the pieces to the game that a chess game would have
    pub fn add_chess_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = *self.playertodirection.get(&playerx).unwrap();
            
            
            
            for x in 0..8{
                self.create_piece(
                    BoardSquarePosID::new_from_perspective((x, 1), perspective).unwrap(),
                    playerx,
                    PieceType::Pawn
                );
            }
            
            
            
            
            self.create_piece(
                BoardSquarePosID::new_from_perspective((0, 0), perspective).unwrap(),
                playerx,
                PieceType::Rook
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((1, 0), perspective).unwrap(),
                playerx,
                PieceType::Knight
            );
            
            self.create_piece(
                BoardSquarePosID::new_from_perspective((2, 0), perspective).unwrap(),
                playerx,
                PieceType::Bishop
            );
            
            
            //swap position of queen and king
            self.create_piece(
                BoardSquarePosID::new_from_perspective((3, 0), perspective).unwrap(),
                playerx,
                PieceType::Queen
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((4, 0), perspective).unwrap(),
                playerx,
                PieceType::King
            );
            
            
            self.create_piece(
                BoardSquarePosID::new_from_perspective((5, 0), perspective).unwrap(),
                playerx,
                PieceType::Bishop
            );
            
            self.create_piece(
                BoardSquarePosID::new_from_perspective((6, 0), perspective).unwrap(),
                playerx,
                PieceType::Knight
            );
            self.create_piece(
                BoardSquarePosID::new_from_perspective((7, 0), perspective).unwrap(),
                playerx,
                PieceType::Rook
            );
            
            
        };
    }
    
    //add the pieces to the game that a chess game would havef
    pub fn add_checkers_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = *self.playertodirection.get(&playerx).unwrap();
            
            for x in 0..8{
                
                for z in 0..3{
                    
                    if (x + z) % 2 == 1{
                        
                        self.create_piece(
                            BoardSquarePosID::new_from_perspective((x, z), perspective).unwrap(),
                            playerx,
                            PieceType::Checker
                        );
                    }
                }
            }
        };
    }
    
    
    
    
    
    
    //set the number of squares raised
    pub fn set_randomly_raised_squares(&mut self, numbertoraise: u32){
        
        
        //get the number of raised squares
        let mut curraisedsquares = self.boardphysics.get_objects_on_long_raise_mission();
        
        //how many more raised squares I have than I need
        let difference = curraisedsquares.len() as i32 - numbertoraise as i32;
        
        let absdifference = difference.abs() as usize;
        
        
        if difference > 0{
            
            for x in 0..absdifference{
                
                if let Some(objectid) = curraisedsquares.pop(){
                    
                    self.boardphysics.end_mission( &objectid );
                }
            }
        }
        else if difference < 0{
            
            let mut potentialsquares = self.get_empty_squares_not_on_mission();
            
            for x in 0..absdifference{
                
                if let Some(bsposid) = potentialsquares.pop(){
                    
                    let objectid = self.boardobjects.get_boardsquare_object_id( &bsposid ).unwrap();
                    
                    self.boardphysics.set_long_raise(10000, objectid);
                }
            }
        }
    }
    
    //set the number of squares that should be randomly dropped
    pub fn set_randomly_dropped_squares(&mut self, numbertodrop: u32){
    }
    
    
    //get each players highest valued piece
    //turn it into as many pawns as that piece was valued
    pub fn split_highest_piece_into_pawns(&mut self){
        
        for playerid in 1..3{
            
            let (highestpieceid, mut highestpiecevalue) = self.boardobjects.get_owners_highest_value_piece(&playerid);
            
            //remove that highest valued piece
            self.remove_piece( highestpieceid );
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();
            
            
            //create as many pawn pieces as that highest value pieces value is
            for x in 0..highestpiecevalue{
                
                if let Some(bsid) = emptysquares.pop(){
                    
                    self.create_piece( bsid, playerid, PieceType::Pawn );
                }
            }
        }
    }
    
    //give all pieces with a value greater than 1 the ability of knights
    pub fn knightify(&mut self){
        
        for pieceid in self.boardobjects.get_pieces(){
            
            self.boardobjects.get_mut_piecedata( &pieceid ).augment_knight_abilities();
        }
    }
    
    pub fn unaugment_abilities(&mut self){
        
        for pieceid in self.boardobjects.get_pieces(){
            
            self.boardobjects.get_mut_piecedata( &pieceid ).remove_ability_augmentations();
        }
    }
    
    pub fn checkerify(&mut self){
        
        
        for playerid in 1..3{
            
            //get the sum of the value of the players pieces and remove them
            
            let mut valuesum = 0;
            
            for pieceid in self.boardobjects.get_players_pieces(&playerid){
                
                let curvalue = self.boardobjects.get_mut_piecedata(&pieceid).get_value();
                valuesum += curvalue;
                self.remove_piece(pieceid);
            };
            
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();
            
            //create half as many checkers pieces as that players total value of pieces
            for x in 0.. valuesum/2 +1 {
                
                if let Some(bsposid) = emptysquares.pop(){
                    
                    self.create_piece(bsposid, playerid, PieceType::Checker);
                }
            };
        };
    }
    
    pub fn chessify(&mut self) {
        
        for playerid in 1..3{
            
            //get the sum of the value of the players pieces and remove them
            
            let mut valuesum = 0;
            
            for pieceid in self.boardobjects.get_players_pieces(&playerid){
                
                let curvalue = self.boardobjects.get_mut_piecedata(&pieceid).get_value();
                valuesum += curvalue;
                self.remove_piece(pieceid);
            };
            
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();
            
            //create a king first
            if let Some(bsposid) = emptysquares.pop(){
                self.create_piece(bsposid, playerid, PieceType::King);
            }
            
            for x in 0.. valuesum/2 +1 {                
                if let Some(bsposid) = emptysquares.pop(){
                    self.create_piece(bsposid, playerid, PieceType::Pawn);
                }
            };
        };
    }
    
    //tick, with true if kings are replaced and false if theyre not
    pub fn tick(&mut self, arekingsreplaced: bool, arepawnspromoted: bool){
        
        
        //remove the pieces that are lower than -5 in pos
        for pieceid in &self.boardobjects.get_pieces().clone(){
            
            //if its not in the valid range for pieces to exist
            if ! self.boardphysics.is_object_in_position_range(
                *pieceid, (-10.0,10.0), (-4.0,100.0), (-10.0,10.0)
            ){
                self.remove_piece(*pieceid);
            }
        }
        
        
        
        //if the kings are replaced, the piece with the highest score becomes a king
        if arekingsreplaced{
            
            for playerid in 1..3{
                
                //if they dont
                if ! self.does_player_have_king(&playerid){
                    
                    let (pieceid, _) = self.boardobjects.get_owners_highest_value_piece(&playerid);
                    
                    self.boardobjects.get_mut_piecedata(&pieceid).set_piecetype(PieceType::King);
                }
            }
        }
        
        
        //promote the pawns to queens if theyre on the backrow of their opponent
        if arepawnspromoted{
            
            for pieceid in self.boardobjects.get_pieces(){
                
                //get the owner
                let ownerid = self.boardobjects.get_owner_of_piece( &pieceid);
                
                //MAKE THE BACKROW A SET OF POINTS AND THEN CHECK IF A PAWN IS ON ANY OF THEM
                //get the "objective back row" from that players perspective
                let backrow = 1;
                
                if let Some( bsposid ) = self.get_board_square_piece_is_on( &pieceid){
                    
                    //if that pawn is on the backrow
                    if bsposid.get_row() == backrow{
                        self.boardobjects.get_piecedata(&pieceid).set_piecetype( PieceType::Queen);
                    }
                }
            }
        }
        
        
        self.boardphysics.tick();
    }
    
    
    
    fn are_square_conditions_met(&self, pieceid: &u16, conditions: &HashSet<( RelativeSquare, SquareCondition )>) -> bool{
        
        
        //get the square that its on
        if let Some(squareposid) = self.get_board_square_piece_is_on(pieceid){
            
            
            //for every square and condition for that square
            for (relativesquare, squarecondition) in conditions{
                
                //if that square exists
                if let Some(cursquarepos) = squareposid.new_from_added_relative_pos( relativesquare.clone() ){
                    
                    //get whats on the square
                    let piecesonsquare = self.get_pieces_on_board_square(&cursquarepos);
                    
                    match squarecondition{
                        
                        //if the square needs to be empty and not on a mission
                        SquareCondition::EmptyRequired => { 

                            let cursquareobjectid = self.boardobjects.get_boardsquare_object_id(&cursquarepos).unwrap();

                            if self.boardphysics.is_object_on_mission(&cursquareobjectid){
                                return false;
                            }
                            
                            if ! piecesonsquare.is_empty(){
                                return false;
                            };
                        },
                        //if the square cant have a friendly piece on it
                        SquareCondition::NoneFriendlyRequired =>{
                            
                            //for every piece on the square
                            for otherpieceid in piecesonsquare{
                                
                                if self.boardobjects.get_owner_of_piece(pieceid) ==
                                self.boardobjects.get_owner_of_piece(&otherpieceid){
                                    return false;
                                };    
                            };
                        },
                        //if there needs to be at least one opponents piece on this square
                        SquareCondition::OpponentRequired =>{
                            
                            let mut opponentspiece = false;
                            
                            //for every piece on the square
                            for otherpieceid in piecesonsquare{
                                
                                if self.boardobjects.get_owner_of_piece(pieceid) !=
                                self.boardobjects.get_owner_of_piece(&otherpieceid){
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
        
        
        //if all teh conditions are met
        return true;
    }
    
    
    
    //get if this action is allowed by this piece
    pub fn is_action_allowed(&self, action: &FullAction, pieceid: &u16) -> bool{
        
        //get the owner of this piece
        if let Some(owner) = self.boardobjects.get_owner_of_piece(pieceid){
            
            //the direction of the owner
            let ownerdirection = self.playertodirection.get(&owner).unwrap();
            //if this is is one of the actions the piece is allowed to perform
            let piecedata = self.boardobjects.get_piecedata(&pieceid);
            
            //if the action is allowed by the piecedata
            if piecedata.is_action_valid(action, ownerdirection){
                
                if self.are_square_conditions_met( pieceid, &action.get_conditions() ){
                    
                    return true;
                }
            }
        }
        
        return false;
    }
    
    
    pub fn perform_action(&mut self, piece: u16, pieceaction: FullAction ){
        
        
        if let Some(destinationsquare) = pieceaction.get_destination_square(){
            
            if pieceaction.is_lifted(){
                
                self.boardphysics.lift_and_move_object(10, piece, destinationsquare.to_relative_float() );
            }
            else{
                
                self.boardphysics.slide_object(30, piece, destinationsquare.to_relative_float() );
            }
            
        }
        
        
        if let Some( (direction, force) ) = pieceaction.get_flick_forces(){
            self.boardphysics.flick_object(piece, direction, force);
        };
        
        let squareposid = self.get_board_square_piece_is_on(&piece).unwrap();
        
        
        //drop the boardsquares that should be dropped when they should be dropped
        for (squareposrelative, tick) in pieceaction.get_squares_dropped(){
            
            if let Some(bsid) = squareposid.new_from_added_relative_pos( squareposrelative ){
                
                let relativesquareid = self.boardobjects.get_boardsquare_object_id( &bsid ).unwrap();
                
                self.boardphysics.set_future_drop(tick, relativesquareid);
            };
        };
        
        //set the piece as having moved
        self.boardobjects.get_piecedata(&piece).moved_piece();
    }
    
    
    pub fn does_player_have_king(&self, playerid: &u8) -> bool{
        self.boardobjects.does_player_have_king(playerid)
    }
    
    pub fn does_player_have_pieces(&self, playerid: &u8) -> bool{
        self.boardobjects.does_player_have_pieces(playerid)
    }
    
    //is this a piece and does this player own it?
    pub fn does_player_own_object(&self, playerid: &u8, objectid: &u16) -> bool{
        
        if let Some(ownerid) = self.boardobjects.get_owner_of_piece(objectid){
            
            if playerid == &ownerid{
                
                return true;
            }
        }
        
        return false;
    }
    
}




//used for getting the visual state of the game
impl GameEngine{
    
    
    pub fn get_visible_board_game(&self) -> Vec<VisibleGameBoardObject>{
        
        let objects = self.boardobjects.get_object_ids();
        
        let mut toreturn = Vec::new();
        
        for objectid in objects{
            
            
            
            let objecttype;
            let owner = self.boardobjects.get_owner_of_piece(&objectid);
            let mut texturelocation = None;
            
            if let Some(bsid) = self.boardobjects.get_boardsquare_by_object_id(&objectid){
                objecttype = VisibleGameObjectType::Square( bsid.is_white() );
            }
            else{
                objecttype = VisibleGameObjectType::Piece;
                texturelocation = Some( self.boardobjects.get_piecedata(&objectid).get_image_location() );
            }
            
            toreturn.push(VisibleGameBoardObject{
                position: self.boardphysics.get_translation(&objectid),
                rotation: self.boardphysics.get_rotation(&objectid),
                id: objectid,
                isonmission: self.boardphysics.is_object_on_mission(&objectid),
                objecttype: objecttype,
                owner: owner,
                texturelocation: texturelocation,
            });
        }
        
        toreturn
    }
    
    
    //get the pieces that are targeted by a piece performing an action
    fn get_piece_targets_of_action(&self, pieceid: &u16, action: &FullAction) -> Vec<u16>{
        
        let mut toreturn = Vec::new();
        
        
        if let Some(boardsquareid) = self.get_board_square_piece_is_on(pieceid){
            
            for relativeposid in action.get_squares_captured(){
                
                if let Some(newboardsquare) = boardsquareid.new_from_added_relative_pos( relativeposid){
                    
                    toreturn.extend( self.get_pieces_on_board_square( &newboardsquare )  );
                };
            };
        };
        
        toreturn        
    }
    
    
    pub fn get_actions_allowed_for_piece(&self, pieceid: &u16) -> Vec<FullAction>{
        
        let mut toreturn = Vec::new();
            


        //get the piece data
        let piecedata = self.boardobjects.get_piecedata(pieceid);
        
        //the owner of the piece
        if let Some(owner) = self.boardobjects.get_owner_of_piece(pieceid){
            
            //the direction of the owner of the piece
            let ownerdirection = self.playertodirection.get(&owner).unwrap();
            
            //get all the actions this piece can potentially perform
            let allactions = piecedata.get_numberable_actions(ownerdirection);
            
            //for every action, get if it is allowed
            for action in allactions{
                
                if self.is_action_allowed(&action.clone(), &pieceid){
                    
                    toreturn.push(action);
                };
            };
            
        }
        
        
        
        return toreturn;
    }
    
    //get the action that this piece can perform now, and the objects it targets
    pub fn get_piece_valid_actions_and_targets(&self, pieceid: &u16) -> (bool, Vec< (FullAction, Vec<u16>) >){
        
        
        //get all the actions this piece can potentially perform
        let allactions = self.get_actions_allowed_for_piece(pieceid);
        
        let mut actionsandtargets: Vec<(FullAction, Vec<u16>)> = Vec::new();
        
        
        //for every action, get if it is allowed
        for action in allactions{
            
            let mut targets = self.get_piece_targets_of_action(&pieceid, &action);
            
            if let Some(curbsposid) = self.get_board_square_piece_is_on(pieceid){
                
                if let Some(destination) =  action.get_destination_square(){
                    
                    if let Some(targetbsposid) = curbsposid.new_from_added_relative_pos( destination ){
                        
                        targets.push( self.boardobjects.get_boardsquare_object_id(&targetbsposid).unwrap() );                    
                    }                    
                }
            }
            
            actionsandtargets.push( (action, targets) );
        };
        
        
        
        return (false, actionsandtargets);        
    }
    
    
    
    pub fn get_squares_capturable_by_players_pieces(&self, playerid: &u8) -> HashSet<BoardSquarePosID>{
        
        let mut toreturn = HashSet::new();
        
        //for each piece they own
        for pieceid in self.boardobjects.get_players_pieces(playerid){
            
            let actions = self.get_actions_allowed_for_piece(&pieceid);
            
            for action in actions{
                
                let bsid = self.get_board_square_piece_is_on(&pieceid).unwrap();
                
                for relativepos in action.get_squares_captured(){
                    
                    let destsquare = bsid.new_from_added_relative_pos(relativepos).unwrap();
                    
                    toreturn.insert(destsquare);
                }            
            }
        }
        
        
        toreturn
    }
    
    
    
    //get the piece action that captures the highest value of piece
    pub fn get_best_fullaction_for_player(&self, playerid: &u8) -> (u16, FullAction){
        
        //get the players pieces
        let ownedpieces = self.boardobjects.get_players_pieces(playerid);
        
        //the piece, the action, and the value of the action
        let mut bestaction: Option<((u16, FullAction), i8)> = None;
        
        
        //order of moves to make
        //1. moves that capture opponents piece and get this piece out of capture
        //2. moves that capture opponents pieces with the lowest valued piece prioritized
        //3. moves that move your piece out of the way of an opponents capture
        //4. any move
        
        
        //get the squares that an opponents piece could capture
        
        let mut opponent = 2;
        
        if playerid == &2{
            opponent = 1;
        }
        
        
        let targetedbyopponent = self.get_squares_capturable_by_players_pieces(&opponent);
        
        
        //for each piece I own
        for pieceid in ownedpieces{
            
            let pieceleavevalue: i8;
            let selfvalue = self.boardobjects.get_piecedata(&pieceid).get_value() as i8;
            
            //get the boardsquare its on
            let curbsid = self.get_board_square_piece_is_on(&pieceid).unwrap();
            
            if targetedbyopponent.contains(&curbsid){
                
                pieceleavevalue = selfvalue;
            }
            else{
                pieceleavevalue = 0;
            }
            
            
            //for each action of each piece
            for (action, capturedpieces ) in self.get_piece_valid_actions_and_targets(&pieceid).1{
                
                //plus half this pieces value if it is currently on a square being captured by an opponents piece
                let mut totalvalue: i8 = pieceleavevalue;
                
                //minus half this pieces value if it enters a spot being captured by an opponents piece
                let destination = curbsid.new_from_added_relative_pos( action.get_destination_square().unwrap() ).unwrap();
                
                if targetedbyopponent.contains(&destination){
                    
                    totalvalue += -selfvalue;
                    
                    //panic!("targeted by an opponent squares value of thing {:?}", targetedbyopponent);
                }
                
                
                //plus the opponents pieces value if it captures opponents pieces
                totalvalue += self.boardobjects.get_value_of_pieces(capturedpieces) as i8;
                
                
                if let Some( (_, returnhighestvalue )  ) = bestaction.clone(){
                    
                    if totalvalue >= returnhighestvalue {
                        
                        bestaction = Some( ((pieceid, action), totalvalue) );
                    }
                }
                else{
                    bestaction = Some( ((pieceid, action), totalvalue) );
                }
            }
        }
        
        
        if let Some( (toreturn, _) ) = bestaction{
            
            return toreturn;
        }
        else{
            
            panic!("no action found for player");
            
        }
        
    }
    
}



//the struct to return to the frontend to render the state of the game

pub struct VisibleGameBoardObject{
    
    pub position: (f32,f32,f32),
    
    pub rotation: (f32,f32,f32),
    
    pub id: u16,
    
    pub isonmission: bool,
    
    pub objecttype: VisibleGameObjectType,
    
    pub owner: Option<u8>,
    
    pub texturelocation: Option<String>,
}




#[derive(Eq, PartialEq)]
pub enum VisibleGameObjectType{
    
    Piece,
    
    //a square and if its white or not
    Square(bool),
}