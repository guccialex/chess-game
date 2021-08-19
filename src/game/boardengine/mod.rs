
mod boardphysics;

mod boardstate;

mod visiblegameboardobject;


pub use visiblegameboardobject::VisibleGameBoardObject;
pub use boardstate::FullAction;
pub use boardstate::Piece;


use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};




use boardstate::Square;
use boardstate::BoardObject;

use boardstate::SquareCondition;

//use boardstate::RelativeSquare;
//use boardstate::SquarePos;

use boardstate::PieceType;

use boardstate::BoardObjects;


use boardphysics::BoardPhysics;



use super::CardEffect;
use super::EffectTrait;



impl EffectTrait for BoardEngine{

    fn apply_effect(&mut self, effect: CardEffect){

        match effect{

            CardEffect::Knight => {

                self.boardobjects.augment_all_knight();

            },
            CardEffect::RemoveSquares(x) => {


            },
            CardEffect::AddSquares(x) => {

                //get a boardsquare 

                //let pos = self.boardobjects.get_square_pos_to_add();



            },
            CardEffect::AddCheckersPieces => {


            },
            _ => {},
        }


    }

    fn get_effects(&self) -> Vec<CardEffect>{
        return Vec::new();
    }


}



#[derive(Serialize, Deserialize)]
pub struct BoardEngine{

    arekingsreplaced: bool,
    arepawnspromoted: bool,
    
    
    boardphysics: BoardPhysics,
    
    //information about the types of objects in the game
    boardobjects: BoardObjects,
}



use rapier3d::na;
use na::Point3;
use na::Vector3;


impl BoardEngine{
    
    
    pub fn get_object_intersection(& self,  ray: (Point3<f32>, Vector3<f32>) ) -> Option<u16>{    
        self.boardphysics.get_object_intersection(ray)
    }
    
    
    pub fn new(player1id: u8, player2id: u8) -> BoardEngine{

        let mut ownerdirection = HashMap::new();
        
        ownerdirection.insert(player1id, 0.0 );
        ownerdirection.insert(player2id, 0.5 );
        
        let mut gameengine = BoardEngine{

            arekingsreplaced: false,
            arepawnspromoted: true,

            boardphysics: BoardPhysics::new(), 
            boardobjects: BoardObjects::new(ownerdirection),
        };
        

        /*
        for pos in BoardObjects::default_square_physical_positions(){
            gameengine.create_boardsquare( pos );
        }
        */
        
        for x in 0..64{
            gameengine.add_next_boardsquare();
        }

        for x in gameengine.boardobjects.add_chess_pieces(){

            gameengine.boardphysics.create_piece_object( );

            gameengine.add_chess_pieces( );

        }

        
        gameengine
    }


    fn add_next_boardsquare(&mut self){



    }

    fn remove_random_boardsquare(&mut self){

        let squares = self.boardobjects.get_squares();

        let square = squares.iter().next().cloned().unwrap();

        self.remove_object( BoardObject::Square( square ) );
    }

    /*
    //create a piece at this position of this type
    fn create_piece(&mut self, pos: (f32,f32,f32), owner: u8, piecetype: PieceType){
        
        let objectid = Piece::new( self.boardphysics.create_piece_object( pos ) );
        
        self.boardobjects.add_piece(objectid, owner, piecetype);
    }
    */

    
    /*
    fn create_boardsquare(&mut self, pos: (f32,f32,f32) ){
        
        let objectid = self.boardphysics.create_boardsquare_object( pos );
        
        self.boardobjects.add_boardsquare( pos , Square::new(objectid) );
    }
    */
    

    
    fn remove_object(&mut self, object: BoardObject){
        self.boardphysics.remove_object(&object.id());
        self.boardobjects.remove_object(&object);
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
        
        return self.boardobjects.get_square_at_position(translation);
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
    
    
    /*
    //add the pieces to the game that a chess game would have
    pub fn add_chess_pieces(&mut self){
        
        //get the physical position, the type, and the owner
        //for the default pieces

        for (pos, owner, piecetype) in self.boardobjects.default_piece_pos_owner_type(){

            self.create_piece(pos, owner, piecetype);
        }
    }
    */
    
    


    
    pub fn tick(&mut self,){
        
        
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
        if self.arekingsreplaced{
            
            for playerid in 1..3{
                
                //if they dont
                if ! self.boardobjects.does_player_have_king(&playerid){
                    
                    let (pieceid, _) = self.boardobjects.get_owners_highest_value_piece(&playerid);
                    
                    self.boardobjects.get_mut_piecedata(&pieceid).set_piecetype(PieceType::King);
                }
            }
        }
        
        
        //promote the pawns to queens if theyre on the backrow of their opponent
        if self.arepawnspromoted{
            
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

    
    pub fn perform_action(&mut self, piece: &Piece, pieceaction: &FullAction ){
        
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
        self.boardobjects.get_mut_piecedata(&piece).is_moved();
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
    
    pub fn is_action_valid(&self, piece: &Piece, action: &FullAction  ) -> bool{

        let owner = self.boardobjects.get_owner_of_piece(piece).unwrap();
        let direction = self.boardobjects.get_direction_of_player(&owner);

        if let Some(square)= self.get_square_piece_is_on(piece){

            if self.boardobjects.get_piecedata(piece).is_action_allowed( action, &direction ){

                if self.are_square_conditions_met( &owner, &square, &action ){
                        
                    return true;
                }
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

    pub fn clicked_to_fullaction(&self, selected: Option<u16>, clicked: Option<u16>) -> Option<(Piece, FullAction)>{


        if let Some(piece) = self.boardobjects.maybe_id_to_piece( selected ){
            
            let actions = self.get_valid_actions(&piece);

            for action in actions{

                if let Some(targetsquare) = self.boardobjects.maybe_id_to_square( clicked ){

                    if Some(targetsquare) == self.get_square_target_of_action(&piece, &action){

                        return Some( ( piece,  action) );
                    }
                }
                else if let Some(targetpiece) = self.boardobjects.maybe_id_to_piece( clicked ){
    
                    if self.get_piece_targets_of_action(&piece, &action).contains( &targetpiece ){

                        return Some( ( piece,  action) );
                    }
                }

            }
        }

        return None;
    }
    

    pub fn get_visible_board_game_objects(&self, selected: Option<u16>) -> Vec<VisibleGameBoardObject>{

        let mut highlightedobjects: HashSet<BoardObject> = HashSet::new();
        let mut selectedobjects: HashSet<BoardObject> = HashSet::new();

        if let Some(piece) = self.boardobjects.maybe_id_to_piece( selected ){

            for action in self.get_valid_actions(&piece){

                for piecetarget in self.get_piece_targets_of_action(&piece, &action){

                    highlightedobjects.insert( BoardObject::Piece(piecetarget) );
                }

                if let Some(squaretarget) = self.get_square_target_of_action(&piece, &action){

                    highlightedobjects.insert(  BoardObject::Square(squaretarget)  );
                }
            }

            selectedobjects.insert( BoardObject::Piece(piece) );
        }
        else if let Some(square) = self.boardobjects.maybe_id_to_square( selected ){

            selectedobjects.insert( BoardObject::Square(square) );
        }

        let mut toreturn = Vec::new();

        for objectid in &self.boardobjects.get_boardobjects(){
            
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

            if highlightedobjects.contains(objectid){
                color = (1.,1.,0.);
            }
            
            if selectedobjects.contains(objectid){
                color = (1.,0.,1.);
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



//boardstate module
//gives the state of each object

//the objects and their physical state
//get the empty

//basically
//struct who's purpose is to give
//the empty 

/*

board objects 

create piece
create boardsquare
send piece or boardsquare on mission
get the boardsquare the pieces are on
get the square and the condition of the square
get the pieces


board engine





*/


