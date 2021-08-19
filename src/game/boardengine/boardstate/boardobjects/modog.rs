
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use super::squarepos::SquarePos;
//use super::relativesquare::RelativeSquare;
use super::piecetype::PieceType;
//use super::fullaction::FullAction;
use super::piecedata::PieceData;
//use super::squarecondition::SquareCondition;


mod boardobject;

pub use boardobject::Square;
pub use boardobject::Piece;
pub use boardobject::BoardObject;





#[derive(Serialize, Deserialize)]
pub struct BoardObjects{


    //boardsquare to its position
    boardsquarepos: HashMap<Square, SquarePos>,

    //the owner of the piece
    pieceowners: HashMap<Piece, u8>,
    
    ownerdirection: HashMap<u8, f32>,

    //the data of the piece
    piecedata: HashMap<Piece, PieceData>,
}

impl BoardObjects{

    pub fn new( ownerdirection: HashMap<u8, f32> ) -> BoardObjects{
        
        BoardObjects{

            boardsquarepos: HashMap::new(),
            pieceowners: HashMap::new(),
            piecedata: HashMap::new(),
            ownerdirection,
        }
    }

    
    /*
        //add a piece on a random square

        //when I want to add checkers pieces
        //do I add them here?
        //or do I ask the board for the position and type of the pieces
        //and add them according to that
        //and then also on the board

        //what if the board engine doesn't have any access to "boardsquare" or boardsquare id at all
        //so when I want to add chess pieces, I add them here and return the position I want them added in the physical engine
        //i Think thats a good idea
    */


    /*
    fn add_piece(&mut self, object: Piece, owner: u8, piecetype: PieceType){

        let mut piecedata = PieceData::new();
        piecedata.set_piecetype(piecetype);
        
        self.pieceowners.insert( object.clone(), owner );
        
        self.piecedata.insert( object, piecedata);
    }
    fn add_boardsquare(&mut self, pos: (f32,f32,f32), square: Square){

        let pos = SquarePos::from_physical_pos(pos).unwrap();

        self.boardsquarepos.insert( square,  pos );
    }
    */


    pub fn add_next_boardsquare(&mut self) -> Vec<(Square, (f32,f32,f32) )>{


        return Vec::new();
    }


    pub fn add_chess_pieces(&mut self) -> Vec<(Piece, (f32,f32,f32) )>{


        return Vec::new();
    }



    /*
    //the default pieces, their owner, and type
    pub fn default_piece_pos_owner_type(&self) -> Vec<((f32,f32,f32), u8, PieceType)>{

        let mut toreturn = Vec::new();


        //player 1 and 2, the 3 is not inclusive
        for playerx in 1..3{
            
            let perspective = self.get_direction_of_player( &playerx);
            
            for x in 0..8{

                toreturn.push(
                    (
                    SquarePos::new_from_perspective((x, 1), perspective).get_default_physical_pos(),
                    playerx,
                    PieceType::Pawn
                )
                );
            }
            
            
            toreturn.push((
                SquarePos::new_from_perspective((0, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Rook
            ));

            toreturn.push((
                SquarePos::new_from_perspective((1, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Knight
            ));
            
            toreturn.push((
                SquarePos::new_from_perspective((2, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Bishop
            ));
            
            
            //swap position of queen and king
            toreturn.push((
                SquarePos::new_from_perspective((3, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Queen
            ));


            toreturn.push((
                SquarePos::new_from_perspective((4, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::King
            ));
            
            
            toreturn.push((
                SquarePos::new_from_perspective((5, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Bishop
            ));
            
            toreturn.push((
                SquarePos::new_from_perspective((6, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Knight
            ));


            toreturn.push((
                SquarePos::new_from_perspective((7, 0), perspective).get_default_physical_pos(),
                playerx,
                PieceType::Rook
            ));
            
            
        };


        toreturn

    }
    */

    pub fn remove_object(&mut self, object: &BoardObject){

        if let Some(piece) = object.as_piece(){

            self.pieceowners.remove( &piece );

            self.piecedata.remove( &piece );
        }


        if let Some(square) = object.as_square(){

            self.boardsquarepos.remove( &square );
        }

    }

    pub fn augment_all_knight(&mut self){

        for (_, data) in &mut self.piecedata.iter_mut(){
            
            data.augment( &PieceType::Knight );

        };
    }
    
    pub fn get_mut_piecedata(&mut self, piece: &Piece) -> &mut PieceData{
        self.piecedata.get_mut(piece).unwrap()
    }
    






    pub fn get_piecedata(&self, piece: &Piece) -> PieceData{
        
        if let Some(pd) = self.piecedata.get(piece){
            
            return pd.clone();
        }
        else{
            return PieceData::new();
        }
    }
    
    pub fn get_players_pieces(&self, playerid: &u8) -> HashSet<Piece>{
        
        let mut toreturn = HashSet::new();
        
        for (pieceid, owner) in &self.pieceowners{
            
            if owner == playerid{
                toreturn.insert(pieceid.clone());
            }
        }
        
        toreturn
    }
    
    pub fn get_owner_of_piece(&self, piece: &Piece) -> Option<u8>{
        
        for (curpieceid, owner) in &self.pieceowners{
            
            if curpieceid == piece{
                return Some(*owner);
            }
        }
        
        return None;
    }
    
    pub fn does_player_have_king(&self, playerid: &u8) -> bool{
        
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

    pub fn does_player_have_pieces(&self, playerid: &u8) -> bool{
        
        //for every piece that player owns
        for (_, owner) in &self.pieceowners{
            
            if owner == playerid{
                
                return true;   
            }
        }
        
        return false;
    }

    pub fn get_pieces(&self) -> HashSet<Piece>{
    
        let mut toreturn = HashSet::new();

        for (obj, _) in &self.piecedata{

            toreturn.insert(obj.clone());
        }
        
        toreturn
    }

    pub fn get_squares(&self) -> HashSet<Square>{
        
        let mut toreturn = HashSet::new();
        
        for (obj, _) in &self.boardsquarepos{
            
            toreturn.insert(obj.clone());
        };
        
        toreturn
    }

    pub fn get_square_at_position(&self, pos: (f32,f32,f32) ) -> Option<Square>{

        if let Some(squarepos) = SquarePos::from_physical_pos(pos){

            for (square, cursquarepos) in &self.boardsquarepos{

                if &squarepos == cursquarepos{
    
                    return Some( square.clone() );
                }
            }
        }

        return None;
    }

    pub fn get_owners_highest_value_piece(&self, owner: &u8 ) -> (Piece, u8){
        
        let mut highestvalue: i8 = -1;
        let mut highestvalueid: Option<Piece> = None;
        
        for (curpieceid, curowner) in &self.pieceowners{
            
            if owner == curowner{
                
                let piecedata = self.piecedata.get( &curpieceid ).unwrap();
                let curvalue = piecedata.get_value() as i8;
                
                if  curvalue > highestvalue{
                    
                    highestvalueid = Some(curpieceid.clone());
                    highestvalue = curvalue;
                }
            }
        }
        
        
        if let Some(toreturn) = highestvalueid{
            return (toreturn, highestvalue as u8);
        }
        
        panic!("no pieces for this player");
    }
    
    pub fn get_boardobjects(&self) -> Vec<BoardObject>{

        let mut toreturn = Vec::new();

        for square in  self.get_squares(){

            toreturn.push( BoardObject::Square( square ) );
        }

        for piece in  self.get_pieces(){

            toreturn.push( BoardObject::Piece( piece ) );
        }

        toreturn

    }

    pub fn id_to_boardobject(&self, id: &u16) -> Option<BoardObject>{

        let pieces = self.get_pieces();

        let squares = self.get_squares();


        let ifpiece = Piece::new(*id);

        if pieces.contains( &ifpiece ){
            return Some( BoardObject::Piece( ifpiece ) );
        }


        let ifsquare = Square::new(*id);

        if squares.contains( &ifsquare ){
            return Some( BoardObject::Square( ifsquare ) );
        }


        return None;

    }

    pub fn maybe_id_to_piece(&self, id: Option<u16>) -> Option<Piece>{

        if let Some(id) = id{

            let pieces = self.get_pieces();

            let ifpiece = Piece::new(id);

            if pieces.contains( &ifpiece ){

                return Some( ifpiece );

            }

        }

        return None;
    }

    pub fn maybe_id_to_square(&self, id: Option<u16>) -> Option<Square>{

        if let Some(id) = id{

            let ifsquare = Square::new(id);

            if self.get_squares().contains( &ifsquare ){

                return Some( ifsquare );
            }
        }

        return None;
    }

    pub fn get_direction_of_owner(&self, piece: &Piece) -> f32{
        *self.ownerdirection.get( &self.pieceowners.get(piece).unwrap()  ).unwrap()
    }
    
    pub fn get_direction_of_player(&self, player: &u8) -> f32{
        *self.ownerdirection.get(player).unwrap()
    }

    pub fn get_square_pos(&self, square: &Square) -> SquarePos{
        self.boardsquarepos.get(square).unwrap().clone()
    }

    pub fn get_square_with_pos(&self, squarepos: &SquarePos) -> Option<Square>{
        
        for (square, cursquarepos) in &self.boardsquarepos{

            if cursquarepos == squarepos{

                return Some( square.clone() );
            }
        }

        return None;
    }


    /*
    pub fn get_value_of_pieces(&self, pieces: Vec<Piece>) -> u8{
        
        let mut totalvalue = 0;
        
        for pieceid in pieces{
                
            totalvalue += self.piecedata.get(&pieceid).unwrap().get_value();
        }

        return totalvalue;
    }
    */


}

