
mod boardphysics;
mod physicaleffects;

use physicaleffects::PhysicalEffects;
use physicaleffects::PhysicalEffect;


use boardphysics::BoardPhysics;

use crate::squarestructs::RelativeSquare;
use crate::squarestructs::SquarePos;
use crate::boardobject::BoardObject;
use crate::boardobject::Piece;
use crate::boardobject::Square;
use crate::FullAction;



use std::collections::HashSet;
use std::collections::HashMap;

use rapier3d::na;
use na::Point3;
use na::Vector3;
use rapier3d::geometry::Shape;
use rapier3d::na::Isometry3;







use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BoardState{

    physics: BoardPhysics,
    
    pieces: HashSet<Piece>,

    squares: HashMap<Square, SquarePos>,

    totalsquares: u16,

    queuedmissions: HashMap<BoardObject, (i32, PhysicalEffect)>,




    speed: u32,
    
    isflicked: bool,

}

impl BoardState{

    pub fn new() -> BoardState{


        BoardState{
            physics: BoardPhysics::new(),
            pieces: HashSet::new(),
            squares: HashMap::new(),
            totalsquares: 10000,
            queuedmissions: HashMap::new(),


            speed: 10,
            isflicked: false,
        }
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn is_speed_slower(& self) -> bool{

        if self.speed != BoardState::default_speed(){
            return true;
        }
        else{
            return false;
        }
    }

    fn default_speed() -> u32{
        return 10;
    }


    pub fn set_is_flicked(&mut self, isflicked: bool) {

        self.isflicked = isflicked;
    }


    pub fn get_is_flicked(&self) -> bool{

        return self.isflicked ;
    }




     
    

}



pub fn tick(boardstate: &mut BoardState){

    boardstate.physics.tick();


    //the physics engine takes up 99% of the data when serialized
    //let serialized = bincode::serialize( &boardstate.physics ).unwrap();
    //println!( " serialized bytes{:?}", serialized.len() );



    
    //tick down
    for (_, (ticks, _)) in boardstate.queuedmissions.iter_mut(){

        *ticks = *ticks - 1;
    }

    //apply effects
    for (object, (ticks, effect)) in boardstate.queuedmissions.clone().iter(){

        if *ticks <= 0{

            perform_effect( boardstate, &object, &effect );
        }
    }

    //remove queued missions with tick less than or equal to zero
    boardstate.queuedmissions.drain_filter(|object, (ticks, effect)| *ticks <= 0 );



}



pub fn get_boardobjects(boardstate: &BoardState) -> Vec<BoardObject>{

    let mut toreturn = Vec::new();

    for piece in &boardstate.pieces{
        toreturn.push(  BoardObject::Piece( piece.clone() ) );
    }

    for (square, _) in &boardstate.squares{
        toreturn.push( BoardObject::Square( square.clone() )  );
    }

    toreturn
}


pub fn get_isometry_and_shape(boardstate: & BoardState, id: &u16) -> (Isometry3<f32>, Box<dyn Shape>){

    return boardstate.physics.get_isometry_and_shape( id);
}

fn id_to_boardobject(boardstate: & BoardState, id: u16) -> Option<BoardObject>{

    if boardstate.pieces.contains(  &Piece::new( id )  ){
        return Some( BoardObject::Piece( Piece::new( id ) ) );
    }

    if boardstate.squares.contains_key( &Square::new( id )  ){
        return Some(  BoardObject::Square( Square::new( id ) ) );
    }

    return None;
}

pub fn get_object_intersection(boardstate: & BoardState, ray: (Point3<f32>, Vector3<f32>) ) -> Option<BoardObject>{
    
    if let Some(id) = boardstate.physics.get_object_intersection( ray ){

        return id_to_boardobject(boardstate,  id );
    }

    return None;
}

//create a physical piece
pub fn create_piece( boardstate: &mut BoardState, piece: &Piece, pos: &SquarePos  ){

    let mut physpos = pos.get_default_physical_pos();
    physpos.1 + 5.0;

    boardstate.physics.create_piece_object( piece.id , physpos );

    boardstate.pieces.insert( piece.clone() );
}

pub fn remove_piece(boardstate: &mut BoardState, piece: &Piece){

    boardstate.physics.remove_object( &piece.id );

    boardstate.pieces.remove( piece );
}

//create a board square at the next location
pub fn create_next_boardsquare( boardstate: &mut BoardState ){

    let mut existingpos = HashSet::new();
    
    for (_, pos) in boardstate.squares.clone(){

        existingpos.insert( pos );
    }


    //go through the boardsquares order until one is missing, and then create that one
    for tomake in boardsquareorder(){

        if  !existingpos.contains( & tomake ){

            boardstate.totalsquares += 1;
            let square = Square::new( boardstate.totalsquares );
            let squarepos = tomake;
            boardstate.physics.create_boardsquare_object( square.id, squarepos.get_default_physical_pos() );
            boardstate.squares.insert( square, squarepos );

            return ();
        }
    }
}

//remove a square not on mission or with pieces on it
pub fn remove_random_square( boardstate: &mut BoardState) {

    for (square, _) in boardstate.squares.clone().iter(){

        if is_square_empty_and_not_on_mission(boardstate, square){

            boardstate.physics.remove_object( &square.id );

            boardstate.squares.remove( &square );
            
            break;
        }
    }

}


//get the pieces below the kill range
pub fn get_pieces_below_border( boardstate: &mut BoardState) -> Vec<Piece>{

    let mut toreturn = Vec::new();

    for piece in boardstate.pieces.iter(){

        let translation = boardstate.physics.get_isometry(&piece.id);

        if translation.translation.y <= -0.0{
            toreturn.push(piece.clone());
        }
    }

    return toreturn;
}



pub fn perform_action(boardstate: & mut BoardState, piece: &Piece, fullaction: &FullAction){


    let effects = PhysicalEffects::from_fullaction( fullaction, &boardstate.speed, &boardstate.isflicked );

    perform_effect( boardstate, &BoardObject::Piece(piece.clone()), &effects.selfeffect );


    for (ticks, relsquare, effect) in effects.squareeffects{

        if let Some(square) = get_square_relative_to_piece(boardstate, piece, &relsquare){

            boardstate.queuedmissions.insert( BoardObject::Square(square), (ticks as i32, effect)  );
        }
    }
}


pub fn piece_to_relative_square_distance(boardstate: & BoardState, piece: &Piece, relsquare: &RelativeSquare) -> (f32,f32){



    let piecepos = boardstate.physics.get_flat_pos( &piece.id);

    let squarepos = SquarePos::from_physical_pos( (piecepos.0, 1.0, piecepos.1) ).unwrap();

    let psquarepos = squarepos.get_default_physical_pos();


    let offset = (piecepos.0 - psquarepos.0, piecepos.1 - psquarepos.2);

    let movement = relsquare.to_relative_float();


    return ( movement.0 - offset.0 , movement.1 - offset.1    );

}


pub fn perform_effect(boardstate: & mut BoardState, object: &BoardObject,  effect: &PhysicalEffect){

    //let object = object.id();

    match effect{

        PhysicalEffect::Slide( pos, ticks) => {

            if let Some(piece) = object.as_piece(){
                let pos = piece_to_relative_square_distance(boardstate, &piece, pos);
                
                boardstate.physics.slide_object( piece.id, pos, *ticks);
            }
        },
        PhysicalEffect::LiftAndMove( pos, ticks) => {

            if let Some(piece) = object.as_piece(){
                let pos = piece_to_relative_square_distance(boardstate, &piece, pos);
                
                boardstate.physics.lift_and_move_object( piece.id, pos, *ticks);
            }
        },
        PhysicalEffect::Flick( dir, force) =>{

            boardstate.physics.flick_object(object.id(), *dir, *force );
        },
        PhysicalEffect::LongDrop( lasts ) => {

            boardstate.physics.set_long_drop(*lasts, object.id());
        },
        PhysicalEffect::Drop =>{

            boardstate.physics.set_drop( object.id() );
        },
    }
}




//get the pieces this action targets
pub fn get_action_captured_pieces(boardstate: & BoardState, piece: &Piece, action: &FullAction) -> Vec<Piece>{

    let mut toreturn = Vec::new();


    if let Some(reldest) = action.captures(){

        //if this piece is on a square
        if let Some(square) = square_piece_is_on(boardstate, piece){

            //if the square it captures exists
            if let Some(capturedsquare) = square_and_relative_pos_to_square( boardstate, &square, &reldest){

                //add the pieces on the square
                toreturn.extend(  pieces_on_square(boardstate, &capturedsquare ) );
            }
        }
    }


    toreturn
}

//square plus relative square to new square
fn square_and_relative_pos_to_square(boardstate: & BoardState, square: &Square, rel: &RelativeSquare) -> Option<Square>{

    let squarepos = square_to_squarepos(boardstate, &square);

    let reldestpos = squarepos.new_from_added_relative_pos(rel.clone());

    return squarepos_to_square( boardstate, &reldestpos );
}


pub fn get_square_relative_to_piece(boardstate: & BoardState, piece: &Piece, relsquare: &RelativeSquare) -> Option<Square>{

    //if this piece is on a square
    if let Some(square) = square_piece_is_on( boardstate, piece){

        return square_and_relative_pos_to_square( boardstate, &square, &relsquare);
    }

    return None;
}


//get the destionation
pub fn get_action_destination(boardstate: & BoardState, piece: &Piece, action: &FullAction) -> Option<Square>{

    //if this action has a destionation
    if let Some(reldest) = action.destination(){

        //if this piece is on a square
        if let Some(square) = square_piece_is_on(boardstate, piece){

            return square_and_relative_pos_to_square(boardstate, &square, &reldest);
        }
    }

    return None;
}





//return if isnt on a mission
pub fn is_square_on_mission(boardstate: & BoardState, square: &Square) -> bool{

    boardstate.physics.is_object_on_mission( &square.id )
}

pub fn square_to_squarepos(boardstate: & BoardState, square: &Square) -> SquarePos{

    boardstate.squares.get(square).unwrap().clone()
}

pub fn squarepos_to_square(boardstate: & BoardState, squarepos: &SquarePos) -> Option<Square>{

    for (id, pos) in &boardstate.squares{

        if pos == squarepos{
            return Some( id.clone() );
        }
    }
    
    return None;
}

//get the squares this action targets
pub fn square_piece_is_on(boardstate: & BoardState, piece: &Piece) -> Option<Square>{

    let pos = boardstate.physics.get_isometry( &piece.id ).translation;

    let pos = (pos.x, pos.y, pos.z);

    if let Some(squarepos) = SquarePos::from_physical_pos( pos ){

        return squarepos_to_square(boardstate, &squarepos );
    }

    return None;
}


//get what pieces are on the square
pub fn pieces_on_square(boardstate: & BoardState, square: &Square) -> HashSet<Piece>{

    let mut toreturn = HashSet::new();

    for piece in &boardstate.pieces{

        if Some(square.clone()) == square_piece_is_on(boardstate, &piece){

            toreturn.insert( piece.clone() );
        }
    }

    return toreturn;
}


pub fn is_square_empty_and_not_on_mission(boardstate: & BoardState, square: &Square) -> bool{

    if pieces_on_square(boardstate, square).is_empty(){

        if ! is_square_on_mission(boardstate, square){

            return true;
        };
    };

    return false;
}




fn boardsquareorder() -> Vec<SquarePos>{

    let mut toreturn = HashSet::new();


    //a random order of boardsquares
    for x in 0..8{

        for y in 0..8{

            toreturn.insert( SquarePos::new( (x,y) )  );
        }
    }


    let mut x: Vec<SquarePos> = toreturn.into_iter().collect();


    let mut temp = HashSet::new();
    for x in -1..9{
        for y in -1..9{
            temp.insert( SquarePos::new( (x,y) ) );
        }
    }
    x.extend( temp );


    let mut temp = HashSet::new();
    for x in -2..10{
        for y in -2..10{
            temp.insert( SquarePos::new( (x,y) ) );
        }
    }
    x.extend( temp );

    let mut temp = HashSet::new();
    for x in -4..12{
        for y in -4..12{
            temp.insert( SquarePos::new( (x,y) ) );
        }
    }
    x.extend( temp );

    let mut temp = HashSet::new();
    for x in -8..16{
        for y in -8..16{
            temp.insert( SquarePos::new( (x,y) ) );
        }
    }
    x.extend( temp );
    

    return x;
}
