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

pub use boardstate::BoardState;
pub use piecedata::PieceDatas;

use piecedata::PieceData;


use piecedata::SquareCondition;
use std::collections::HashSet;
use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;

use rapier3d::na;
use na::Point3;
use na::Vector3;
use na::Isometry3;
use rapier3d::geometry::Shape;



pub use boardstate::create_next_boardsquare;
pub use boardstate::remove_random_square;



pub fn does_player_own_piece(piecedatas: &PieceDatas, player: &u8, piece: &Piece) -> bool{


    if let Some(data) = piecedatas.get(&piece.id){

        return data.get_owner() == *player;
    }

    return false;

}



//get the best action for this player
pub fn get_player_best_action(piecedatas: &PieceDatas, boardstate: &BoardState, player: u8) -> (Piece, FullAction){


    //get all actions of this players pieces
    let pieces = piecedatas.get_players_pieces( player );

    let mut allactions = Vec::new();


    for piece in pieces{

        let piece = Piece::new(piece);

        for action in get_valid_actions( piecedatas, boardstate, &piece) {

            allactions.push( (piece.clone(), action.clone()) );


            if boardstate::get_action_captured_pieces( boardstate, &piece, &action ).len() > 0{
                return (piece.clone(), action);
            }
        }

    }



    return allactions.pop().unwrap();

}





pub fn get_visible_board_game_objects(piecedatas: &PieceDatas, boardstate: &BoardState, selected: &Option<BoardObject>) -> Vec<VisibleGameBoardObject>{

    let mut toreturn = Vec::new();


    let mut selectedobjects = HashSet::new();
    let mut highlightedobjects = HashSet::new();

    if let Some(selected) = selected{

        selectedobjects.insert( selected );

        if let BoardObject::Piece( selectedpiece ) = selected{

            for action in get_valid_actions( piecedatas, boardstate, selectedpiece){

                for capturedpiece in boardstate::get_action_captured_pieces(boardstate, selectedpiece, &action){

                    highlightedobjects.insert( BoardObject::Piece(capturedpiece) );
                }
                if let Some(destsquare) = boardstate::get_action_destination(boardstate, selectedpiece, &action){

                    highlightedobjects.insert( BoardObject::Square(destsquare) );
                }
            }
        }
    }
    



    for object in boardstate::get_boardobjects(boardstate){

        let id = object.id();
        let (isometry, shape) = boardstate::get_isometry_and_shape( boardstate, &object.id() );
        
        let texturelocation;

        if let BoardObject::Piece(piece) = &object{

            texturelocation = Some( piecedatas.get( &piece.id ).unwrap().get_image_location() );
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
                
                let piecedata = piecedatas.get( &piece.id ).unwrap();

                if piecedata.get_owner() == 1{

                    color = (2., 2., 2.);
                }
                else{

                    color = (0., 0., 0.);
                }
            }
            else if let BoardObject::Square(square) = &object{

                let squarepos = boardstate::square_to_squarepos(boardstate, square);

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
            
            id: object,
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
pub fn get_object_intersection(boardstate: & BoardState, ray: (Point3<f32>, Vector3<f32>)) -> Option<BoardObject>{

    boardstate::get_object_intersection(boardstate, ray)
}




pub fn create_piece(piecedatas: &mut PieceDatas, boardstate: &mut BoardState, piecetype: &PieceType, pos: &SquarePos, owner: &u8, direction: &f32){


    let mut data = PieceData::new(owner, direction);
    data.set_piecetype( piecetype );

    let id = piecedatas.add_piece(data);
    let piece = Piece::new(id);


    boardstate::create_piece( boardstate, &piece , pos);

    //return piece;
}


pub fn remove_piece(piecedatas: &mut PieceDatas, boardstate: &mut BoardState, piece: &Piece){

    boardstate::remove_piece( boardstate, piece );

    piecedatas.remove( &piece.id );
}



//turn objects into an action
pub fn objects_to_action(piecedatas: & PieceDatas, boardstate: &BoardState, selected: &Piece, target: &BoardObject) -> Option<FullAction> {

    //get the valid actions of the selected piece
    for action in get_valid_actions(piecedatas, boardstate, selected){


        //if it targets a piece
        if let BoardObject::Piece(piecetarget) = target {
            
            for capturedpiece in boardstate::get_action_captured_pieces(boardstate, selected, &action){

                if &capturedpiece == piecetarget{

                    return Some( action );
                }
            }
        }
        //if it targets a square
        else if let BoardObject::Square(squaretarget) = target{

            //get the square destination
            if let Some(destination) = boardstate::get_action_destination(boardstate, selected, &action){

                if &destination == squaretarget{

                    return Some(action);
                }
            }
        }
    }

    return None;
}



pub fn get_valid_actions(piecedatas: & PieceDatas, boardstate: & BoardState, piece: &Piece) -> Vec<FullAction>{

    let mut toreturn = Vec::new();

    if let Some(data) = piecedatas.get( & piece.id ){

        for action in data.get_allowed_actions(){
        
            if is_action_valid(piecedatas, boardstate, piece, &action){
                toreturn.push( action );
            }
        }
    }

    toreturn
}



pub fn tick( piecedatas: & mut PieceDatas, boardstate: &mut BoardState ){
    
    boardstate::tick(boardstate);

    for piece in boardstate::get_pieces_below_border( boardstate ){

        remove_piece(piecedatas, boardstate, &piece);
    }
}






pub fn is_action_valid(piecedatas: & PieceDatas, boardstate: &BoardState, piece: &Piece, action: &FullAction) -> bool{

    if let Some(data) = piecedatas.get(&piece.id){

        //if the piece is on a square
        if let Some(square) = boardstate::square_piece_is_on(boardstate, piece ){

            let squarepos = boardstate::square_to_squarepos(boardstate, &square);

            if data.get_allowed_actions().contains( action ){

                let conditions = data.get_action_conditions( &squarepos , action  );

                if are_conditions_met( piecedatas, boardstate, conditions ){

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






fn are_conditions_met( piecedatas: &PieceDatas, boardstate: &BoardState, conditions: Vec<(SquarePos, SquareCondition)>) -> bool{

    for (pos, condition) in conditions{

        if let Some(square) = boardstate::squarepos_to_square( boardstate, &pos ){

            //if the square is on a mission its conditions arent met
            if boardstate::is_square_on_mission(boardstate, &square){
                return false;
            }

            match condition{

                SquareCondition::NeedsEmpty =>{

                    //if there are pieces on the square
                    if ! boardstate::pieces_on_square( boardstate, &square ).is_empty(){
                        return false;
                    }
                },
                SquareCondition::NeedsNoPlayerPiece(playerid) =>{

                    //for each piece, if there is any 
                    for piece in boardstate::pieces_on_square(boardstate, &square ){

                        let owner = piecedatas.get(&piece.id).unwrap().get_owner();

                        if owner == playerid{
                            return false;
                        }
                    }
                },
                SquareCondition::NeedsPlayerPiece(playerid) =>{

                    //return false if there isn't a piece owned by this player on here
                    let mut piecesneeded = false;

                    for piece in boardstate::pieces_on_square( boardstate, &square ){

                        let owner = piecedatas.get( &piece.id ).unwrap().get_owner();

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
pub fn perform_action(piecedatas: &mut PieceDatas, boardstate: &mut BoardState , piece: &Piece, fullaction: &FullAction){

    piecedatas.set_moved(&piece.id);

    boardstate::perform_action( boardstate, piece , fullaction);
}



//does a king with this player id exist?
pub fn does_king_exist(piecedatas: &PieceDatas, playerid: &u8) -> bool{

    for data in &piecedatas.get_all(){

        if playerid == &data.get_owner(){

            if data.is_this_piecetype( &PieceType::King ){

                return true;
            }
        }
    }

    false
}

