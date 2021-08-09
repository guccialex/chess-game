
mod boardphysics;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;



use crate::piecedata::PieceData;
use crate::piecedata::FullAction;
use crate::piecedata::SquareCondition;
use crate::piecedata::PieceType;


use crate::boardsquarestructs;


use boardsquarestructs::SquarePos;
use boardsquarestructs::RelativeSquare;


use boardphysics::BoardPhysics;


mod boardobjects;
use boardobjects::BoardObjects;



use crate::objects::BoardObject;

use crate::inputs::BoardInput;

use crate::objects::Square;
use crate::objects::Piece;



//manages who owns each piece
//what each piece can do

//returns what piece each other piece can capture
//thats what a full action is
//dont expose "piece action"



#[derive(Serialize, Deserialize)]
pub struct GameEngine{
    
    boardphysics: BoardPhysics,
    
    //information about the types of objects in the game
    boardobjects: BoardObjects,
    
}



use rapier3d::na;
use na::Point3;
use na::Vector3;


impl GameEngine{
    
    
    //get the id of the object selected
    
    pub fn get_object_intersection(& self,  ray: (Point3<f32>, Vector3<f32>) ) -> Option<u16>{
        
        self.boardphysics.get_object_intersection(ray)
    }
    
    
    
    
    
    
    pub fn new(player1id: u8, player2id: u8) -> GameEngine{

        let mut ownerdirection = HashMap::new();
        
        ownerdirection.insert(player1id, 0.0 );
        ownerdirection.insert(player2id, 0.5 );
        
        let mut gameengine = GameEngine{
            boardphysics: BoardPhysics::new(), 
            boardobjects: BoardObjects::new(ownerdirection),
        };
        
        //make the boardsquares
        for squarepos in SquarePos::get_all_default_square_pos(){
            gameengine.create_boardsquare( squarepos );
        }
        
        
        gameengine
    }
    
    fn create_boardsquare(&mut self, bs: SquarePos){
        
        let bsidphyspos = bs.get_default_physical_pos();
        
        let objectid = self.boardphysics.create_boardsquare_object( bsidphyspos );
        
        self.boardobjects.add_boardsquare(bs , Square::new(objectid) );
    }
    
    //create a piece at this position of this type
    fn create_piece(&mut self, pos: SquarePos, owner: u8, piecetype: PieceType){
        
        let mut piecepos = pos.get_default_physical_pos();
        piecepos.1 += 3.0;
        
        let objectid = Piece::new( self.boardphysics.create_piece_object( piecepos ) );
        
        let mut piecedata = PieceData::new();
        piecedata.set_piecetype( piecetype);
        
        self.boardobjects.add_piece(objectid, owner, piecedata);
    }
    
    
    fn remove_object(&mut self, object: BoardObject){
        
        self.boardphysics.remove_object(&object.id());
        self.boardobjects.remove_object(object);
    }
    
    
    fn get_pieces_on_board_square(&self, square: &Square ) -> HashSet<Piece>{
        
        let mut toreturn = HashSet::new();
        
        //for all pieces
        for pieceid in self.boardobjects.get_pieces(){
            
            //get the piece its on
            if let Some(bsiditson) = self.get_square_piece_is_on(&pieceid){
                
                if &bsiditson == square{
                    
                    toreturn.insert(pieceid);
                }
            }
        };
        
        return toreturn;
    }
    
    pub fn get_square_piece_is_on(&self, pieceid: &Piece) -> Option<Square>{
        
        let translation = self.boardphysics.get_isometry(&pieceid.id).translation;
        
        let translation = (translation.x, translation.y, translation.z);
        
        if let Some(pos) = SquarePos::from_physical_pos( translation ){
            
            return self.boardobjects.get_square_with_pos( &pos);
        }
        
        return None;
    }
    
    
    
    
    //get the id of every board square without a piece on it
    //and that arent on a mission currently
    fn get_empty_squares_not_on_mission(&self) -> Vec<Square>{
        
        let squares = self.boardobjects.get_squares();
        
        let mut toreturn = Vec::new();
        
        for square in squares{
            
            let piecesonboardsquare = self.get_pieces_on_board_square( &square );
            
            //if it doesnt have anything on it
            if piecesonboardsquare.is_empty(){
                
                //if its not on a mission
                if ! self.boardphysics.is_object_on_mission( &square.id ) {
                    
                    //then push it into the list of empty squares not on a mission
                    toreturn.push( square );
                }
            }
        }
        
        return toreturn;
    }
    
    
    //add the pieces to the game that a chess game would have
    pub fn add_chess_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = self.boardobjects.get_direction_of_player( &playerx);
            
            for x in 0..8{
                self.create_piece(
                    SquarePos::new_from_perspective((x, 1), perspective),
                    playerx,
                    PieceType::Pawn
                );
            }
            
            
            self.create_piece(
                SquarePos::new_from_perspective((0, 0), perspective),
                playerx,
                PieceType::Rook
            );
            self.create_piece(
                SquarePos::new_from_perspective((1, 0), perspective),
                playerx,
                PieceType::Knight
            );
            
            self.create_piece(
                SquarePos::new_from_perspective((2, 0), perspective),
                playerx,
                PieceType::Bishop
            );
            
            
            //swap position of queen and king
            self.create_piece(
                SquarePos::new_from_perspective((3, 0), perspective),
                playerx,
                PieceType::Queen
            );
            self.create_piece(
                SquarePos::new_from_perspective((4, 0), perspective),
                playerx,
                PieceType::King
            );
            
            
            self.create_piece(
                SquarePos::new_from_perspective((5, 0), perspective),
                playerx,
                PieceType::Bishop
            );
            
            self.create_piece(
                SquarePos::new_from_perspective((6, 0), perspective),
                playerx,
                PieceType::Knight
            );
            self.create_piece(
                SquarePos::new_from_perspective((7, 0), perspective),
                playerx,
                PieceType::Rook
            );
            
            
        };
    }
    
    //add the pieces to the game that a chess game would havef
    pub fn add_checkers_pieces(&mut self){
        
        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = self.boardobjects.get_direction_of_player( &playerx);
            
            for x in 0..8{
                
                for z in 0..3{
                    
                    if (x + z) % 2 == 1{
                        
                        self.create_piece(
                            SquarePos::new_from_perspective((x, z), perspective),
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
                
                if let Some(square) = potentialsquares.pop(){
                    
                    self.boardphysics.set_long_raise(10000, square.id);
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
            self.remove_object( BoardObject::Piece( highestpieceid ) );
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();
            
            
            //create as many pawn pieces as that highest value pieces value is
            for x in 0..highestpiecevalue{
                
                if let Some(square) = emptysquares.pop(){
                    
                    let squarepos = self.boardobjects.get_square_pos(&square);
                    
                    self.create_piece( squarepos, playerid, PieceType::Pawn );
                }
            }
        }
    }
    
    //give all pieces with a value greater than 1 the ability of knights
    pub fn knightify(&mut self){
        
        for pieceid in self.boardobjects.get_pieces(){
            
            self.boardobjects.get_mut_piecedata( &pieceid ).augment( &PieceType::Knight );
        }
    }
    
    pub fn unaugment_abilities(&mut self){
        
        for pieceid in self.boardobjects.get_pieces(){
            
            self.boardobjects.get_mut_piecedata( &pieceid ).remove_augmentations();
        }
    }
    
    pub fn checkerify(&mut self){
        
        
        for playerid in 1..3{
            
            //get the sum of the value of the players pieces and remove them
            let mut valuesum = 0;
            
            for pieceid in self.boardobjects.get_players_pieces(&playerid){
                
                let curvalue = self.boardobjects.get_mut_piecedata(&pieceid).get_value();
                valuesum += curvalue;
                
                self.remove_object( BoardObject::Piece(pieceid) );
            };
            
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();
            
            //create half as many checkers pieces as that players total value of pieces
            for x in 0.. valuesum/2 +1 {
                
                if let Some(square) = emptysquares.pop(){
                    
                    let squarepos = self.boardobjects.get_square_pos(&square);
                    
                    self.create_piece( squarepos , playerid, PieceType::Checker);
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
                
                self.remove_object( BoardObject::Piece(pieceid) );
            };
            
            
            
            let mut emptysquares = self.get_empty_squares_not_on_mission();
            
            //create a king first
            if let Some(square) = emptysquares.pop(){
                
                let squarepos = self.boardobjects.get_square_pos(&square);
                
                self.create_piece(squarepos, playerid, PieceType::King);
            }
            
            for x in 0.. valuesum/2 +1 {                
                if let Some(square) = emptysquares.pop(){
                    
                    let squarepos = self.boardobjects.get_square_pos(&square);
                    
                    self.create_piece(squarepos, playerid, PieceType::Pawn);
                }
            };
        };
    }
    
    pub fn tick(&mut self, arekingsreplaced: bool, arepawnspromoted: bool){
        
        
        //remove the pieces that are lower than -5 in pos
        for pieceid in &self.boardobjects.get_pieces().clone(){
            
            //if its not in the valid range for pieces to exist
            if ! self.boardphysics.is_object_in_position_range(
                pieceid.id, (-10.0,10.0), (-4.0,100.0), (-10.0,10.0)
            ){
                self.remove_object( BoardObject::Piece(pieceid.clone()) );
            }
            
            
        }
        
        
        
        //if the kings are replaced, the piece with the highest score becomes a king
        if arekingsreplaced{
            
            for playerid in 1..3{
                
                //if they dont
                if ! self.boardobjects.does_player_have_king(&playerid){
                    
                    let (pieceid, _) = self.boardobjects.get_owners_highest_value_piece(&playerid);
                    
                    self.boardobjects.get_mut_piecedata(&pieceid).set_piecetype(PieceType::King);
                }
            }
        }
        
        
        //promote the pawns to queens if theyre on the backrow of their opponent
        if arepawnspromoted{
            
            for pieceid in self.boardobjects.get_pieces(){
                
                if let Some( square ) = self.get_square_piece_is_on( &pieceid){
                    
                    let squarepos = self.boardobjects.get_square_pos(&square);
                    
                    if squarepos.is_backrow(){
                        
                        self.boardobjects.get_piecedata(&pieceid).set_piecetype( PieceType::Queen);
                    }                    
                }
            }
        }
        
        self.boardphysics.tick();
    }
    
    
    
    
    pub fn does_player_have_king(&self, playerid: &u8) -> bool{
        self.boardobjects.does_player_have_king(playerid)
    }
    
    pub fn does_player_have_pieces(&self, playerid: &u8) -> bool{
        self.boardobjects.does_player_have_pieces(playerid)
    }
    
    //is this object a piece and do I own it?
    pub fn does_player_own_object(&self, playerid: &u8, objectid: &u16) -> bool{
        
        //assume its a piece, and it should return none if its not a piece
        let piece = Piece::new(*objectid);
        
        if let Some(ownerid) = self.boardobjects.get_owner_of_piece(&piece){
            
            if playerid == &ownerid{
                
                return true;
            }
        }
        
        return false;
    }
    
    

    


    
    fn perform_action(&mut self, piece: &Piece, pieceaction: &FullAction ){
        
        //move piece
        if let Some(destinationsquare) = pieceaction.get_destination_square(){
            
            if pieceaction.is_lifted(){
                self.boardphysics.lift_and_move_object(10, piece.id, destinationsquare.to_relative_float() );
            }
            else{
                self.boardphysics.slide_object(30, piece.id, destinationsquare.to_relative_float() );
            }
        }
        
        
        let square = self.get_square_piece_is_on(&piece).unwrap();
        let squarepos = self.boardobjects.get_square_pos(&square);
        
        //drop the boardsquares that should be dropped when they should be dropped
        for (squareposrelative, tick) in pieceaction.get_squares_dropped(){
            
            let newsquarepos = squarepos.new_from_added_relative_pos(squareposrelative);
            
            if let Some(newsquare) = self.boardobjects.get_square_with_pos(&newsquarepos){
                
                self.boardphysics.set_future_drop(tick, newsquare.id);
            };
        };
        
        //set the piece as having moved
        self.boardobjects.get_piecedata(&piece).is_moved();
    }
    
    fn are_square_conditions_met(&self, owner: &u8, square: &Square, action: &FullAction) -> bool{
        
        let conditions = action.get_conditions();
        
        let squareitsonpos = self.boardobjects.get_square_pos(&square);
        
        //for every square and condition for that square
        for (relativesquare, squarecondition) in conditions{
            
            let cursquarepos = squareitsonpos.new_from_added_relative_pos( relativesquare.clone() );
            
            //if that square exists
            if let Some(cursquare) = self.boardobjects.get_square_with_pos(&cursquarepos){
                
                //get whats on the square
                let piecesonsquare = self.get_pieces_on_board_square(&cursquare);
                
                //return false if the square is on a mission
                if self.boardphysics.is_object_on_mission(&cursquare.id){
                    return false;
                }
                
                
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
                            
                            if Some(*owner) == self.boardobjects.get_owner_of_piece(&otherpieceid){
                                return false;
                            };    
                        };
                    },
                    //if there needs to be at least one opponents piece on this square
                    SquareCondition::OpponentRequired =>{
                        
                        let mut opponentspiece = false;
                        
                        //for every piece on the square
                        for otherpieceid in piecesonsquare{
                            
                            if Some(*owner) != self.boardobjects.get_owner_of_piece(&otherpieceid){
                                opponentspiece = true;
                            };
                        };
                        
                        if opponentspiece == false{
                            return false;
                        };
                    },
                };
            }
            //if that square doesnt exist
            else{
                return false;
            };
        };
        
        
        //if all the conditions are met
        return true;
    }
    
    fn is_action_valid(&self, piece: &Piece, action: &FullAction  ) -> bool{

        let owner = self.boardobjects.get_owner_of_piece(piece).unwrap();
        let direction = self.boardobjects.get_direction_of_player(&owner);
        let square = self.get_square_piece_is_on(piece).unwrap();

        if self.boardobjects.get_piecedata(piece).is_action_allowed( action, &direction ){

            if self.are_square_conditions_met( &owner, &square, &action ){
                    
                return true;
            }
        }
        
        return false;
    }
    
    fn get_valid_actions(&self, piece: &Piece) -> Vec<FullAction>{
        
        let mut toreturn = Vec::new();
        
        let piecedata = self.boardobjects.get_piecedata(piece);
        let direction = self.boardobjects.get_direction_of_owner(piece);
        
        for action in piecedata.get_allowed_actions(&direction){
            
            if self.is_action_valid( &piece, &action){
                
                toreturn.push(action);
            };
        };
        
        return toreturn;
    }

    fn get_piece_targets_of_action(&self, piece: &Piece, action: &FullAction) -> Vec<Piece>{
        
        let mut toreturn = Vec::new();
        
        if let Some(square) = self.get_square_piece_is_on(piece){
            
            let squarepos = self.boardobjects.get_square_pos(&square);
            
            for relativeposid in action.get_squares_captured(){
                
                let newsquarepos = squarepos.new_from_added_relative_pos( relativeposid);
                
                if let Some(square) = self.boardobjects.get_square_with_pos( &newsquarepos){
                    
                    toreturn.extend( self.get_pieces_on_board_square( &square )  );
                };
            };
        };
        
        toreturn
    }
    
    fn get_square_target_of_action(&self, piece:&Piece, action: &FullAction) -> Option<Square>{
        
        if let Some(square) = self.get_square_piece_is_on(piece){
            
            let squarepos = self.boardobjects.get_square_pos(&square);

            if let Some(relsquarepos) = action.get_destination_square(){

                let newsquarepos = squarepos.new_from_added_relative_pos(relsquarepos);

                if let Some(square) = self.boardobjects.get_square_with_pos(&newsquarepos){

                    return Some(square);
                }
            }
        }

        return None;
    }
    
    fn get_all_targets_of_action(&self, piece: &Piece, action: &FullAction) -> Vec<u16>{

        let mut toreturn = Vec::new();

        for piece in self.get_piece_targets_of_action(piece, action){
            toreturn.push( piece.id );
        }

        if let Some(square) = self.get_square_target_of_action(piece, action){
            toreturn.push( square.id );
        }

        toreturn
    }
    
    pub fn action_to_input(&self, piece: &Piece, action: &FullAction) -> Option<BoardInput>{

        if self.is_action_valid(piece, action){

            let square = self.get_square_piece_is_on(piece).unwrap();

            if let Some(target) = self.get_piece_targets_of_action(piece, action).get(0){

                return Some( BoardInput::new( Some(square.id), target.id) );
            }


            let squarepos = self.boardobjects.get_square_pos(&square);
            let destsquarepos = squarepos.new_from_added_relative_pos( action.get_destination_square().unwrap() );
            let destsquare = self.boardobjects.get_square_with_pos(&destsquarepos).unwrap();
            return Some( BoardInput::new( Some(square.id), destsquare.id) );
        }

        None
    }
    
    pub fn get_valid_targets(&self, playerid: &u8,  object: &u16) ->  Vec<u16>{
        
        let mut toreturn: Vec<u16> = Vec::new();
        
        //if the object is a piece
        if let Some( piece ) = self.boardobjects.as_piece( BoardObject::Piece( Piece::new(*object) ) ){

            //if the player owns that piece
            if Some( *playerid) == self.boardobjects.get_owner_of_piece(&piece){

                let validactions = self.get_valid_actions( &piece  );
    
                for action in validactions{
                    
                    for target in self.get_piece_targets_of_action(&piece, &action){
                        toreturn.push( target.id );
                    }
        
                    toreturn.push( self.get_square_target_of_action(&piece, &action).unwrap().id );
                };
            }
        }

                
        return toreturn;
    }    
    
    pub fn is_boardinput_valid(&self, playerid: &u8, boardinput: &BoardInput ) -> bool{

        let ownedpieces = self.boardobjects.get_players_pieces(playerid);

        if let Some( selected ) = boardinput.selected{

            if ownedpieces.contains( &Piece::new(selected) ){

                let targeted = boardinput.clicked;
            
                let targets = self.get_valid_targets(&playerid, &selected);
                
                if targets.contains( &targeted ){
                    
                    return true;
                };
            };
        };
        
        return false;
    }
    
    pub fn perform_boardinput(&mut self, boardinput: &BoardInput) {

        
        if let Some(piece) = boardinput.selected{
            let piece = Piece::new( piece );
            
            let target = boardinput.clicked;
            
            //for each valid action
            for action in self.get_valid_actions( &piece ){
                
                //if it targets the piece, or a the square
                if self.get_all_targets_of_action(&piece, &action).contains(&target){
                    
                    //perform that action
                    self.perform_action(&piece, &action);
                }
            }
        }
    }
    
    pub fn get_visible_board_game_objects(&self) -> Vec<VisibleGameBoardObject>{
        
        let objects = self.boardobjects.get_objects();
        
        let mut toreturn = Vec::new();
        
        for objectid in &objects{
            
            let mut texturelocation = None;
            let mut color = (1., 0., 0.);
            
            
            if let BoardObject::Square(square) = objectid{

                if self.boardobjects.get_square_pos(square).is_white(){
                    color = (5., 5., 5.);
                }
                else{
                    color = (-1., -1., -1.);
                }
                
            }
            else if let BoardObject::Piece(piece) = objectid{

                texturelocation = Some( self.boardobjects.get_piecedata(&piece).get_image_location() );

                if Some(1) == self.boardobjects.get_owner_of_piece(piece){

                    color = (4.5, 4.5, 4.5);
                }
                else if Some(2) == self.boardobjects.get_owner_of_piece(piece){

                    color = (0.2, 0.2, 0.2);

                    texturelocation = Some("b_".to_string() + &texturelocation.unwrap());
                }
                
            }
            
            
            
            let (isometry, shape) = self.boardphysics.get_isometry_and_shape( &objectid.id() );
            
            toreturn.push(VisibleGameBoardObject{
                
                id: objectid.id(),
                isometry,
                shape, 
                color,
                texturelocation,
            });
        }
        
        toreturn
    }


    
    
    
}



//AI methods
impl GameEngine{
    
    
    
    //get_valid_targets should also return the square ids
    
    fn get_squares_capturable_by_players_pieces(&self, playerid: &u8) -> HashSet<Square>{
        
        let mut toreturn = HashSet::new();
        
        //for each piece they own
        for pieceid in self.boardobjects.get_players_pieces(playerid){
            
            let actions = self.get_valid_actions(&pieceid);
            
            for action in actions{
                
                if let Some(square) = self.get_square_target_of_action(&pieceid, &action){

                    toreturn.insert(square);
                }
            }
        }
        
        toreturn
    }
    
    
    //get the piece action that captures the highest value of piece
    pub fn get_best_fullaction_for_player(&self, playerid: &u8) -> BoardInput{
        
        //get the players pieces
        let ownedpieces = self.boardobjects.get_players_pieces(playerid);
        
        //the piece, the action, and the value of the action
        let mut bestaction: Option<((Piece, FullAction), i8)> = None;
        
        
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
            let curbsid = self.get_square_piece_is_on(&pieceid).unwrap();
            
            if targetedbyopponent.contains(&curbsid){
                pieceleavevalue = selfvalue;
            }
            else{
                pieceleavevalue = 0;
            }
            
            
            //for each target
            for action in self.get_valid_actions(&pieceid){

                
                //plus half this pieces value if it is currently on a square being captured by an opponents piece
                let mut totalvalue: i8 = pieceleavevalue;


                if let Some(destination) = self.get_square_target_of_action(&pieceid, &action){

                    
                    //minus its value if its going to a piece targeted by opponent
                    if targetedbyopponent.contains(&destination){
                    
                        totalvalue += -selfvalue;
                    }    
                }

                

                //plus the opponents pieces value if it captures opponents pieces
                totalvalue += self.boardobjects.get_value_of_pieces(  self.get_piece_targets_of_action(&pieceid, &action)  ) as i8;
                
                
                if let Some( (_, returnhighestvalue )  ) = bestaction.clone(){
                    
                    if totalvalue >= returnhighestvalue {
                        
                        bestaction = Some( ((pieceid.clone(), action), totalvalue) );
                    }
                }
                else{
                    bestaction = Some( ((pieceid.clone(), action), totalvalue) );
                }
            }
        }
        
        
        if let Some( (toreturn, _) ) = bestaction{
            
            return self.action_to_input(&toreturn.0, &toreturn.1).unwrap();
        }
        else{
            panic!("no action found for player");
        }
        
    }
    
    
    
}





use rapier3d::geometry::Shape;
use rapier3d::na::Isometry3;


//the struct to return to the frontend to render the state of the game
pub struct VisibleGameBoardObject{
    
    pub isometry: Isometry3<f32>,
    
    pub id: u16,
    
    pub shape: Box<dyn Shape>,
    
    pub color: (f32,f32,f32),
    
    pub texturelocation: Option<String>,
}
