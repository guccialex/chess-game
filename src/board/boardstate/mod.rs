
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



struct SquareManager{

    squares: HashMap<Square, SquarePos>,

}




use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BoardState{

    physics: BoardPhysics,
    
    pieces: HashSet<Piece>,

    squares: HashMap<Square, SquarePos>,

    totalsquares: u16,

    
    //set the future missions
    //the object id, to how many ticks in the future, and the physical effect
    queuedmissions: HashMap<BoardObject, (i32, PhysicalEffect)>,

}


use rapier3d::na;
use na::Point3;
use na::Vector3;


/*
    methods

    turn action into physical actiona and perform it

    get whats square each thing is on


    */

    /*

    board:
    is action allowed?
    what actions can piece perform?
    create piece at this board square

    (get the effects this action causes?)


    board state:
    no conception of piece type
    perform this physical effect on objects
    what pieces are on this baordsquare
    what boardsquare is this piece on

    what if it doesnt know The difference between pieces and squares?




    Manually drop squares? flick objects? end missions?



    //board physics
    apply different effects to different objects
    no conception of piece, square, anything

    //board state
    knows the pieces, squares, ids, square pos
    pieceid to position

    //board
    knows the piece data
    pieceid to piecedata





    //board physics
    apply a mission to an object


    //board state
    get what squares a piece is on
    get if a square is on a mission


    //board
    get valid fullactions of this piece
    create piece on this square



    where do I queue squares that are to be dropped in the future
    i dont think it can be on board, because board doesnt deal with how actions are performed
    unless i want to make it
*/

use rapier3d::geometry::Shape;
use rapier3d::na::Isometry3;


impl BoardState{

    pub fn tick(&mut self){
        self.physics.tick();


        
        //tick down
        for (_, (ticks, _)) in self.queuedmissions.iter_mut(){

            *ticks = *ticks - 1;
        }

        //apply effects
        for (object, (ticks, effect)) in self.queuedmissions.clone().iter(){

            if *ticks <= 0{

                self.perform_effect( &object, &effect );
            }
        }

        //remove queued missions with tick less than or equal to zero
        self.queuedmissions.drain_filter(|object, (ticks, effect)| *ticks <= 0 );



    }



    pub fn new() -> BoardState{


        BoardState{
            physics: BoardPhysics::new(),
            pieces: HashSet::new(),
            squares: HashMap::new(),
            totalsquares: 10000,
            queuedmissions: HashMap::new(),
        }
    }

    pub fn get_boardobjects(&self) -> Vec<BoardObject>{

        let mut toreturn = Vec::new();

        for piece in &self.pieces{
            toreturn.push(  BoardObject::Piece( piece.clone() ) );
        }

        for (square, _) in &self.squares{
            toreturn.push( BoardObject::Square( square.clone() )  );
        }

        toreturn
    }

    pub fn get_isometry_and_shape(& self, id: &u16) -> (Isometry3<f32>, Box<dyn Shape>){

        return self.physics.get_isometry_and_shape( id);
    }

    fn id_to_boardobject(&self, id: u16) -> Option<BoardObject>{

        if self.pieces.contains(  &Piece::new( id )  ){
            return Some( BoardObject::Piece( Piece::new( id ) ) );
        }

        if self.squares.contains_key( &Square::new( id )  ){
            return Some(  BoardObject::Square( Square::new( id ) ) );
        }

        return None;
    }

    pub fn get_object_intersection(& self, ray: (Point3<f32>, Vector3<f32>) ) -> Option<BoardObject>{
        
        if let Some(id) = self.physics.get_object_intersection( ray ){

            return self.id_to_boardobject( id );
        }

        return None;
    }

    //create a physical piece
    pub fn create_piece( &mut self, piece: &Piece, pos: &SquarePos  ){

        let mut physpos = pos.get_default_physical_pos();
        physpos.1 + 5.0;

        self.physics.create_piece_object( piece.id , physpos );

        self.pieces.insert( piece.clone() );
    }

    pub fn remove_piece(&mut self, piece: &Piece){

        self.physics.remove_object( &piece.id );

        self.pieces.remove( piece );
    }

    //create a board square at the next location
    pub fn create_next_boardsquare(  &mut self ){

        let mut existingpos = HashSet::new();
        
        for (_, pos) in self.squares.clone(){

            existingpos.insert( pos );
        }


        //go through the boardsquares order until one is missing, and then create that one
        for tomake in boardsquareorder(){

            if  !existingpos.contains( & tomake ){

                self.totalsquares += 1;
                let square = Square::new( self.totalsquares );
                let squarepos = tomake;
                self.physics.create_boardsquare_object( square.id, squarepos.get_default_physical_pos() );
                self.squares.insert( square, squarepos );

                return ();
            }
        }
    }

    pub fn remove_random_square(&mut self) {

        let squareid = self.squares.iter().next().unwrap().0.clone();

        self.physics.remove_object( &squareid.id );

        self.squares.remove( &squareid );
    }


    //get the pieces below the kill range
    pub fn get_pieces_below_border(&mut self) -> Vec<Piece>{

        let mut toreturn = Vec::new();

        for piece in self.pieces.iter(){

            let translation = self.physics.get_isometry(&piece.id);

            if translation.translation.y <= -3.{
                toreturn.push(piece.clone());
            }
        }

        return toreturn;
    }



    pub fn perform_action(&mut self, piece: &Piece, fullaction: &FullAction){


        let effects = PhysicalEffects::from_fullaction( fullaction, &10, &false );


        //apply the effect on self
        self.perform_effect( &BoardObject::Piece(piece.clone()), &effects.selfeffect );


        for (ticks, relsquare, effect) in effects.squareeffects{

            if let Some(square) = self.get_square_relative_to_piece(piece, &relsquare){

                self.queuedmissions.insert( BoardObject::Square(square), (ticks as i32, effect)  );
            }
        }
    }


    pub fn piece_to_relative_square_distance(&self, piece: &Piece, relsquare: &RelativeSquare) -> (f32,f32){


        let piecepos = self.physics.get_flat_pos( &piece.id);

        let squarepos = SquarePos::from_physical_pos( (piecepos.0, 1.0, piecepos.1) ).unwrap();

        let psquarepos = squarepos.get_default_physical_pos();


        let offset = (piecepos.0 - psquarepos.0, piecepos.1 - psquarepos.2);

        let movement = relsquare.to_relative_float();


        return ( movement.0 - offset.0 , movement.1 - offset.1    );

    }


    pub fn perform_effect(&mut self, object: &BoardObject,  effect: &PhysicalEffect){

        //let object = object.id();

        match effect{

            PhysicalEffect::Slide( pos, ticks) => {

                if let Some(piece) = object.as_piece(){
                    let pos = self.piece_to_relative_square_distance(&piece, pos);
                    
                    self.physics.slide_object( piece.id, pos, *ticks);
                }
            },
            PhysicalEffect::LiftAndMove( pos, ticks) => {

                if let Some(piece) = object.as_piece(){
                    let pos = self.piece_to_relative_square_distance(&piece, pos);
                    
                    self.physics.lift_and_move_object( piece.id, pos, *ticks);
                }
            },
            PhysicalEffect::Flick( dir, force) =>{

                self.physics.flick_object(object.id(), *dir, *force );
            },
            PhysicalEffect::LongDrop( lasts ) => {

                self.physics.set_long_drop(*lasts, object.id());
            },
            PhysicalEffect::Drop =>{

                self.physics.set_drop( object.id() );
            },
        }
    }




    //get the pieces this action targets
    pub fn get_action_captured_pieces(&self, piece: &Piece, action: &FullAction) -> Vec<Piece>{

        let mut toreturn = Vec::new();


        if let Some(reldest) = action.captures(){

            //if this piece is on a square
            if let Some(square) = self.square_piece_is_on(piece){

                //if the square it captures exists
                if let Some(capturedsquare) = self.square_and_relative_pos_to_square( &square, &reldest){

                    //add the pieces on the square
                    toreturn.extend(  self.pieces_on_square( &capturedsquare ) );
                }
            }
        }


        toreturn
    }

    //square plus relative square to new square
    fn square_and_relative_pos_to_square(&self, square: &Square, rel: &RelativeSquare) -> Option<Square>{

        let squarepos = self.square_to_squarepos(&square);

        let reldestpos = squarepos.new_from_added_relative_pos(rel.clone());

        return self.squarepos_to_square( &reldestpos );
    }


    pub fn get_square_relative_to_piece(&self, piece: &Piece, relsquare: &RelativeSquare) -> Option<Square>{

        //if this piece is on a square
        if let Some(square) = self.square_piece_is_on(piece){

            return self.square_and_relative_pos_to_square( &square, &relsquare);
        }

        return None;
    }


    //get the destionation
    pub fn get_action_destination(&self, piece: &Piece, action: &FullAction) -> Option<Square>{

        //if this action has a destionation
        if let Some(reldest) = action.destination(){

            //if this piece is on a square
            if let Some(square) = self.square_piece_is_on(piece){

                return self.square_and_relative_pos_to_square( &square, &reldest);
            }
        }

        return None;
    }





    //return if isnt on a mission
    pub fn is_square_on_mission(&self, square: &Square) -> bool{

        self.physics.is_object_on_mission( &square.id )

    }

    pub fn square_to_squarepos(&self, square: &Square) -> SquarePos{

        self.squares.get(square).unwrap().clone()
    }

    pub fn squarepos_to_square(&self, squarepos: &SquarePos) -> Option<Square>{

        for (id, pos) in &self.squares{

            if pos == squarepos{
                return Some( id.clone() );
            }
        }
        
        return None;
    }

    //get the squares this action targets
    pub fn square_piece_is_on(&self, piece: &Piece) -> Option<Square>{

        let pos = self.physics.get_isometry( &piece.id ).translation;

        let pos = (pos.x, pos.y, pos.z);

        if let Some(squarepos) = SquarePos::from_physical_pos( pos ){

            return self.squarepos_to_square( &squarepos );
        }

        return None;
    }


    //get what pieces are on the square
    pub fn pieces_on_square(&self, square: &Square) -> HashSet<Piece>{

        let mut toreturn = HashSet::new();

        for piece in &self.pieces{

            if Some(square.clone()) == self.square_piece_is_on(&piece){

                toreturn.insert( piece.clone() );
            }
        }

        return toreturn;
    }


}




fn boardsquareorder() -> Vec<SquarePos>{

    let mut toreturn = HashSet::new();


    //a random order of boardsquares
    for x in 0..8{

        for y in 0..8{

            toreturn.insert( SquarePos::new( (x,y) )  );
        }
    }


    let mut temp = HashSet::new();
    
    for x in -2..10{
        for y in -2..10{

            temp.insert( SquarePos::new( (x,y) ) );

        }
    }



    let mut x: Vec<SquarePos> = toreturn.into_iter().collect();
    
    //add the ring of outside pieces after the middle pieces
    x.extend( temp );

    return x;
}
