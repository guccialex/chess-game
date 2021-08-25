use crate::RelativeSquare;
use crate::SquarePos;
use crate::BoardObject;
use crate::Piece;
use crate::Square;
use crate::PieceType;
use crate::FullAction;
use crate::VisibleGameBoardObject;


mod piecedata;
mod boardstate;

use boardstate::BoardState;

use piecedata::PieceData;
use piecedata::SquareCondition;


//a mod that manages both
//functions
//create "knight" on square (3,4)
//get the actions of knight
//get the position it moves to


//create the piece with the owners position?
//or pass in the owners position when performing actions with the piece
//this question isn't answered with experience, it's answered by thinking about it


//each piece has its owners ID and facing direction
//facing direction is distinct from owner id




use std::collections::HashSet;
use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Board{

    //physical piece data
    physical: BoardState,

    //ability piecedata
    piecedata: HashMap<Piece, PieceData>,


    totalpieces: u16,
}


use rapier3d::na;
use na::Point3;
use na::Vector3;
use na::Isometry3;
use rapier3d::geometry::Shape;

impl Board{

    pub fn new() -> Board{

        return Board{
            physical: BoardState::new(),
            piecedata: HashMap::new(),
            totalpieces: 0
        };
    }

    pub fn get_visible_board_game_objects(&self, selected: &Option<BoardObject>) -> Vec<VisibleGameBoardObject>{

        let mut toreturn = Vec::new();


        let mut selectedobjects = HashSet::new();
        let mut highlightedobjects = HashSet::new();

        if let Some(selected) = selected{

            selectedobjects.insert( selected );

            if let BoardObject::Piece( selectedpiece ) = selected{

                for action in self.get_valid_actions( selectedpiece){

                    for capturedpiece in self.physical.get_action_captured_pieces(selectedpiece, &action){

                        highlightedobjects.insert( BoardObject::Piece(capturedpiece) );
                    }
                    if let Some(destsquare) = self.physical.get_action_destination(selectedpiece, &action){

                        highlightedobjects.insert( BoardObject::Square(destsquare) );
                    }
                }
            }
        }
        



        



        for object in self.physical.get_boardobjects(){

            let id = object.id();
            let (isometry, shape) = self.physical.get_isometry_and_shape( &object.id() );
            
            let texturelocation;

            if let BoardObject::Piece(piece) = &object{

                texturelocation = Some( self.piecedata.get( piece ).unwrap().get_image_location() );
            } 
            else{
                texturelocation = None;
            }


            let color;

            if selectedobjects.contains( &object ){
                color = (1.0, 0.3, 0.3);
            }
            else if highlightedobjects.contains( &object){
                color = (0.0, 1.0, 0.0);
            }
            else{

                if let BoardObject::Piece( piece ) = &object{
                    
                    let piecedata = self.piecedata.get( &piece ).unwrap();

                    if piecedata.get_owner() == 1{

                        color = (2., 2., 2.);
                    }
                    else{

                        color = (0., 0., 0.);
                    }

                }
                else if let BoardObject::Square(square) = &object{

                    let squarepos = self.physical.square_to_squarepos(square);

                    if squarepos.is_white(){
                        color = (1.5, 1.5, 1.5);
                    }
                    else{
                        color = (0.0, 0.0, 0.0);
                    }

                }
                else{

                    color = (0.5, 0.5, 0.5);
                }


            }

            toreturn.push(VisibleGameBoardObject{
                
                id: object.id(),
                isometry,
                shape, 
                color,
                texturelocation,
                rotation: None,
            });

        }




        toreturn

    }




    //get the piece or square it intersects with?
    pub fn get_object_intersection(&self, ray: (Point3<f32>, Vector3<f32>)) -> Option<BoardObject>{

        self.physical.get_object_intersection(ray)
    }


    pub fn create_square(&mut self){

        self.physical.create_next_boardsquare();
    }

    pub fn create_piece(&mut self, piecetype: &PieceType, pos: &SquarePos, owner: &u8, direction: &f32) -> Piece{

        let piece = Piece::new(self.totalpieces);

        self.physical.create_piece( &piece, pos);

        let mut data = PieceData::new(owner, direction);
        data.set_piecetype( piecetype );
        self.piecedata.insert( piece.clone(), data );
    
        self.totalpieces += 1;

        return piece;
    }


    pub fn remove_piece(&mut self, piece: &Piece){

        self.physical.remove_piece( piece );

        self.piecedata.remove( piece );
    }


    
    //turn objects into an action
    pub fn objects_to_action(&self, selected: &Piece, target: &BoardObject) -> Option<FullAction> {

        //get the valid actions of the selected piece
        for action in self.get_valid_actions(selected){


            //if it targets a piece
            if let BoardObject::Piece(piecetarget) = target {
                
                for capturedpiece in self.physical.get_action_captured_pieces(selected, &action){

                    if &capturedpiece == piecetarget{

                        return Some( action );
                    }
                }
            }
            //if it targets a square
            else if let BoardObject::Square(squaretarget) = target{

                //get the square destination
                if let Some(destination) = self.physical.get_action_destination(selected, &action){

                    if &destination == squaretarget{

                        return Some(action);
                    }
                }
            }
        }

        return None;
    }



    pub fn get_valid_actions(&self, piece: &Piece) -> Vec<FullAction>{

        let mut toreturn = Vec::new();

        if let Some(data) = self.piecedata.get(piece){

            for action in data.get_allowed_actions(){
            
                if self.is_action_valid(piece, &action){
                    toreturn.push( action );
                }
            }
        }

        toreturn
    }



    pub fn tick(&mut self){
        self.physical.tick();


        for piece in self.physical.get_pieces_below_border(){

            self.remove_piece(&piece);
        }
    }


    pub fn is_action_valid(&self, piece: &Piece, action: &FullAction) -> bool{

        if let Some(data) = self.piecedata.get(piece){

            //if the piece is on a square
            if let Some(square) = self.physical.square_piece_is_on( piece ){

                let squarepos = self.physical.square_to_squarepos(&square);

                if data.get_allowed_actions().contains( action ){

                    let conditions = data.get_action_conditions( &squarepos , action  );

                    if self.are_conditions_met( conditions ){

                        return true;
                    }
                }
                else{
                    return false;
                }
            }
        }

        return false;
    }


    fn are_conditions_met( &self, conditions: Vec<(SquarePos, SquareCondition)>) -> bool{

        for (pos, condition) in conditions{

            if let Some(square) = self.physical.squarepos_to_square( &pos ){

                //if the square is on a mission its conditions arent met
                if self.physical.is_square_on_mission(&square){
                    return false;
                }

                match condition{

                    SquareCondition::NeedsEmpty =>{

                        //if there are pieces on the square
                        if ! self.physical.pieces_on_square( &square ).is_empty(){
                            return false;
                        }
                    },
                    SquareCondition::NeedsNoPlayerPiece(playerid) =>{

                        //for each piece, if there is any 
                        for piece in self.physical.pieces_on_square( &square ){

                            let owner = self.piecedata.get(&piece).unwrap().get_owner();

                            if owner == playerid{
                                return false;
                            }
                        }
                    },
                    SquareCondition::NeedsPlayerPiece(playerid) =>{

                        //return false if there isn't a piece owned by this player on here
                        let mut piecesneeded = false;

                        for piece in self.physical.pieces_on_square( &square ){

                            let owner = self.piecedata.get(&piece).unwrap().get_owner();

                            if owner == playerid{
                                piecesneeded = true;
                            }
                        }

                        if piecesneeded == false{
                            return false;
                        }
                    },
                    //just needs the square to exist and not be on a mission
                    SquareCondition::NeedsNothing =>{


                    }
                }
            }
            //if the square doesnt exist
            else{
                return false;
            }
        }

        return true;
    }


    //also pass in a "speed" parameter for how fast the action should be performed
    pub fn perform_action(&mut self, piece: &Piece, fullaction: &FullAction){


        self.piecedata.get_mut(piece).unwrap().has_moved();


        self.physical.perform_action( piece , fullaction);
    }

    //does a king with this player id exist?
    pub fn does_king_exist(&self, playerid: &u8) -> bool{

        for (_, data) in &self.piecedata{

            if playerid == &data.get_owner(){

                if data.is_this_piecetype( &PieceType::King ){

                    return true;
                }
            }
        }

        false
    }

    
    

    /*
    //maybe a get pieces of this type function
    //and get if this piece is on the backrow function
    //if a pawn is on hte backrow is determined by its direction
    pub fn get_pawns_on_backrow(&self) -> Vec<Piece>{

        Vec::new()


    }


    pub fn augment_all_knight(&mut self){



    }
    */



}





/*
AddChessPieces,
AddCheckersPieces,
//create the default chess/checkers pieces and apply with the default distribution

SplitPieceIntoPawns,


Checkerify,
//turn pieces into checkers and apply to a checkers distribution

Chessify,


//since piecedata wont be exposed
//this has to be a method in board
Knight,


RemoveSquares(u32),
AddSquares(u32),



ChangeSpeed(u32),
//fullaction should have a speed parameter
//or it should be passed in when trying to perform a method, how fast it should be performed


LevelPieces,
//get each piece
//get its value
//or since piece data isnt exposed
//this might need to be a method in board interfaced with



AddRandomPieces(u32),


TiltActions(u32),
//actions should have a "tilt self" method
//that returns self, or modified self so that its targeting the square that it would be targeting if it was tilted that amount
//that is applied before an action is applied


SplitIntoPawns,




DelayAction(u32),
//the actions are held in a struct in the game engine
//and when that number of moves are tried to be performed, then perform this action


MakeBomb,
//give this a drop 9 adjacent squares option
//then each time an action is taken
//tick down
//when at zero
//drop the squares

MovesBecomeFlicks(u32),
//give the fullaction a method to turn itself into a flick
//when this is applied, apply that method before calling perform action on the board

KingsReplaced(bool),
//create a king in a random position
//when detected that there is no more king

LossWithoutKing(bool),
//the board engine should have its own determination of when its lost
//checking every tick if that is met

PawnsPromoted(bool),
//the game engine also manages this
//checks whats on the backsquare
//and turns it into a queen if true
*/