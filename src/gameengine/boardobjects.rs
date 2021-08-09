
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use crate::boardsquarestructs;

use boardsquarestructs::SquarePos;
use boardsquarestructs::RelativeSquare;



use crate::piecedata::PieceData;
use crate::piecedata::FullAction;
use crate::piecedata::SquareCondition;
use crate::piecedata::PieceType;


use crate::objects::Square;
use crate::objects::Piece;
use crate::objects::BoardObject;





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


    pub fn as_piece(&self, object: BoardObject) -> Option<Piece>{

        let aspiece = Piece::new( object.id() );

        if self.piecedata.contains_key(&aspiece){

            return Some(aspiece);
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

    
    pub fn get_value_of_pieces(&self, pieces: Vec<Piece>) -> u8{
        
        let mut totalvalue = 0;
        
        for pieceid in pieces{
                
            totalvalue += self.piecedata.get(&pieceid).unwrap().get_value();
        }

        return totalvalue;
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
    
    pub fn new( ownerdirection: HashMap<u8, f32> ) -> BoardObjects{
        
        BoardObjects{

            boardsquarepos: HashMap::new(),
            pieceowners: HashMap::new(),
            piecedata: HashMap::new(),
            ownerdirection,
        }
    }
    
    pub fn add_piece(&mut self, object: Piece, owner: u8, piecedata: PieceData){
        
        self.pieceowners.insert( object.clone(), owner );
        
        self.piecedata.insert( object, piecedata);
    }
    

    pub fn add_boardsquare(&mut self, pos: SquarePos, square: Square){

        self.boardsquarepos.insert( square,  pos );
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
    
    pub fn remove_object(&mut self, object: BoardObject){

        if let BoardObject::Piece(piece) = object{

            self.pieceowners.remove( &piece);

            self.piecedata.remove( &piece);

        }
        else if let BoardObject::Square(square) = object{

            self.boardsquarepos.remove( &square);
        }

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
    
    pub fn get_objects(&self) -> HashSet<BoardObject>{

        let mut toreturn = HashSet::new();

        for piece in self.get_pieces(){

            toreturn.insert( BoardObject::Piece(piece) );

        };


        for square in self.get_squares(){


            toreturn.insert( BoardObject::Square(square));
        };

        
        toreturn
    }    
}

