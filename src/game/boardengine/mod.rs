
//use board::Board;
use board::BoardObject;
use board::FullAction;
use board::Piece;
use board::PieceType;
use board::RelativeSquare;
use board::Square;
use board::SquarePos;
use board::VisibleGameBoardObject;


use board::boardstructs;

use board::BoardState;
use board::PieceDatas;




use std::collections::HashMap;
use std::collections::HashSet;







//PUBLIC METHODS

//is action valid
//perform action
//get valid actions of this piece
//perform card effect
//get card effects

//a function for each card effect
//and then another wrapper on top of this?

//there should be a level that deals with the card effects
//that doesnt need to know about squarepos
//like i should get the positions reccomended to create the object at by the boardstate
//create some piece by some condition


//how do I add chess pieces
//piece data manager creates the pieces
//and then tells me to create them physically at this specific position

//i dont think I can create them physically first
//and so I cant create tehm on the board and then add them to the piece manager

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BoardEngine{

    boardstate: BoardState,
    
    piecedatas: PieceDatas,



    ownerdirection: HashMap<u8, f32>,


    movesareflicks: bool,


    //delayed actions
}

//store all data about t
//a hashmap of pieceid to 

use rapier3d::na;
use na::Point3;
use na::Vector3;


impl BoardEngine{


    pub fn new() -> BoardEngine{

        let mut ownerdirection = HashMap::new();

        ownerdirection.insert( 1, 0.0);
        ownerdirection.insert( 2, 0.5);

        let mut toreturn = BoardEngine{
            boardstate: BoardState::new(),
            piecedatas: PieceDatas::new(),

            ownerdirection,
            movesareflicks: false,
        };



        for x in 0..100{

            board::create_next_boardsquare( &mut toreturn.boardstate );
        }


        toreturn.create_chess_pieces();


        toreturn
    }


    pub fn moves_become_flicks(&mut self){

        self.movesareflicks = true;
    }



    //the effects stored where?
    //what effects:
    /*

        //values store here:

        CardEffect::MovesBecomeFlicks(u32)=> {  },
        CardEffect::KingsReplaced(bool) => {  },
        CardEffect::LossWithoutKing(bool) => {  },
        CardEffect::PawnsPromoted(bool)=> {  },


        //effects stored in "piecedata"
        
        //or when I
        //"get allowed actions"
        //i have to pass "knight" and "tilt actions"
        //in
    
        CardEffect::TiltActions(u32)=> {  },
        CardEffect::ChangeSpeed(u32) => {  },
        CardEffect::Knight => { 

    */

    /*
    //get the move the player should make
    pub fn get_ideal_move_of_player(&self, playerid: &u8) -> FullAction{

        //get the pieces of this player
        
        //self.piecedatas




    }
    */

    pub fn get_players_ideal_action(&self, playerid: u8) -> (Piece, FullAction){

        return board::get_player_best_action(&self.piecedatas,&self.boardstate, playerid);

    }




    pub fn add_random_pieces(&mut self, x: u32){

        /*

        //add x pieces
        //on a distribution
        for _ in 0..x{

            self.board.create_piece( &PieceType::get_random() , pos: &SquarePos, owner: &u8, direction: &f32);
            
        }
        */

        //get the empty squares as close to the backrow of player x's backrow as possible (randomly on that backest row)

        //

        //delay actions
        //you can delay the actions
        //and I think this should be stored in here, this state, like the delayed actions
        //a vector, with the piece id and the action
        //this 

    }




    pub fn create_chess_pieces(&mut self){


        for playerx in 1..3{
            
            let rotation = self.ownerdirection.get( &playerx).unwrap();
            
            for x in 0..8{

                board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Pawn, &SquarePos::new_from_perspective( (x, 1), *rotation ) , &playerx, rotation );
            }

            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Rook, &SquarePos::new_from_perspective( (0, 0), *rotation ) , &playerx, rotation );

            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Knight, &SquarePos::new_from_perspective( (1, 0), *rotation ) , &playerx, rotation );
            
            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Bishop, &SquarePos::new_from_perspective( (2, 0), *rotation ) , &playerx, rotation );
            
            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Queen, &SquarePos::new_from_perspective( (3, 0), *rotation ) , &playerx, rotation );
            
            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::King, &SquarePos::new_from_perspective( (4, 0), *rotation ) , &playerx, rotation );
            
            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Bishop, &SquarePos::new_from_perspective( (5, 0), *rotation ) , &playerx, rotation );
            
            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Knight, &SquarePos::new_from_perspective( (6, 0), *rotation ) , &playerx, rotation );
            
            board::create_piece( &mut self.piecedatas, &mut self.boardstate, &PieceType::Rook, &SquarePos::new_from_perspective( (7, 0), *rotation ) , &playerx, rotation );
            
        };

    }


    pub fn get_object_intersection(&self, ray: (Point3<f32>, Vector3<f32>)) -> Option<BoardObject>{
        board::get_object_intersection(& self.boardstate, ray)
    }


    pub fn is_action_valid(& self, playerid: &u8, piece: &Piece, action: &FullAction ) -> bool{

        if board::does_player_own_piece( &self.piecedatas, playerid, piece ){

            return board::is_action_valid(&self.piecedatas, &self.boardstate, piece, action);
        }

        return false;
    }


    pub fn perform_action(&mut self, piece: &Piece, action: &FullAction ) {

        let mut action = action.clone();

        //if moves are flicks is true, turn this move into a flick first
        if self.movesareflicks{
            action.into_flick();
        }


        board::perform_action(&mut self.piecedatas, &mut self.boardstate, piece, &action);
    }


    pub fn tick(&mut self){

        board::tick(&mut self.piecedatas, &mut self.boardstate);
    }


    pub fn clicked_to_fullaction(&self, selected: Option<BoardObject>, clicked: Option<BoardObject>) -> Option<(Piece, FullAction)>{


        if let Some( BoardObject::Piece( piece ) ) = selected{

            if let Some(target) = clicked{

                if let Some(action) = board::objects_to_action(& self.piecedatas, &self.boardstate, &piece, &target){

                    return Some( (piece, action) );
                }
            }
        }

        return None;
    }

     

    pub fn get_visible_board_game_objects(&self, selected: &Option<BoardObject>) -> Vec<VisibleGameBoardObject>{

        return board::get_visible_board_game_objects( & self.piecedatas, &self.boardstate, selected );
    }


    //data about the state of the game
    //the length 





    //just those methods
    //and then all the methods for the effects



    /*
    //remove this piece
    //and create x pawns equal to its value
    pub fn split_into_pawns(&mut self, piece: &Piece){

        self.piecedatamanager.get_value(piece);
        
        self.remove_piece( piece );

        //get a distribution of pieces
        //create x pawns over that distribution
    }
    */


    /*
    //turn every piece on the board sideways
    pub fn turn_sideways(&mut self){

    }

    pub fn knightify(&mut self){

    }

    //tilt the all actions performed by all pieces
    pub fn tilt_actions(&mut self, amount: f32){

        //for every piece, get its piece data, tilt it if possible

        //piecedata should tilt the things before returning them
        //the tilt amount should be stored in and fetched from the piecedatamanager

    }


    //give this player a random piece
    pub fn add_random_piece(&mut self, playerid: u8){

    }


    //pawns are promoted
    //pawns arent promoted


    //pieces have cooldowns instead of players having timed turns or anything else
    //or players have cooldowns instead of timed turns
    //i think those are different things

    //combine pieces into a piece with their combined value
    pub fn combine_pieces(&mut self, piece1: Piece, piece2: Piece){

    }


    //increase the level of every piece
    pub fn level_every_piece(&mut self){

    }


    //the input should be how many ticks it takes to move 1 square
    //should this player have their actions slowed down?
    pub fn slow_down_actions(&mut self){

    }


    //delay actions of this piece
    //until its owner makes another move
    //this is different in function to "slow down actions" one lets you move when one of your pieces is targeted
    //the other lets you move knowing what move your opponent has made in advance, and will give the same opprotuntiy to your opponent
    pub fn delay_action_of_players_moves_by_x(&mut self){


    }


    pub fn move_piece_to_random_position(&mut self){


    }
    */





    //augment each piece with the abilities of this piece type


    //raise these squares with pieces on them
    //(amking the piece unable to move or get hit)


    //grow/ shrink board size

    //shake the board around

    //lift up this piece and put it in a random position on the board


    //reset the pieces on the board right now to their default positions (or a default distribution of them)


    //boardsquares that are dropped become dropped for 30 full seconds


    //what actually changes the nature of chess enough to make a person whos good at adapting better at the game than
    //someone who knows chess very well?


    //what are some default settings for the game? to like initiate as the game begins
    //
    //pieces are moved to random positions at the back rank (chess 960)




}





//make board effect public



use super::CardEffect;
use super::EffectTrait;


impl EffectTrait for BoardEngine{

    fn apply_effect(&mut self, effect: CardEffect){

        match effect{

            CardEffect::BackToBackTurns => {  }, 
            CardEffect::HalveTimeLeft => {  },
            CardEffect::TurnsTimed(turns) => {  },
            CardEffect::TurnsUntilDrawAvailable(turns) => {  },


            CardEffect::AddChessPieces => {


            },
            CardEffect::AddCheckersPieces  => { 


            },
            CardEffect::SplitPieceIntoPawns => { 


            },
            CardEffect::Checkerify => {  },
            CardEffect::Chessify => {  },

            CardEffect::Knight => { 
                self.piecedatas.augment_knight();

             },
            CardEffect::RemoveSquares(number) => { 
                for _ in 0..number{
                    board::remove_random_square(&mut self.boardstate);
                }
             },
            CardEffect::AddSquares(number) => { 
                log::info!("the number {:?}", number);
                for _ in 0..number{
                    board::create_next_boardsquare(&mut self.boardstate);
                }
             },

            CardEffect::ChangeSpeed(ticks) => { 
                self.boardstate.set_speed( ticks );
             },
            CardEffect::LevelPieces => {  },
            CardEffect::AddRandomPieces(u32)=> {  },
            CardEffect::TiltActions(u32)=> {  },
            CardEffect::SplitIntoPawns=> {  },
            CardEffect::MakeCheckers=> {

              },
            CardEffect::IntoFlicks=> {    
                self.boardstate.set_is_flicked( true )
            },
            CardEffect::KingsReplaced(bool) => {  },
            CardEffect::LossWithoutKing(bool) => {  },
            CardEffect::PawnsPromoted(bool)=> {  },
        };



    }

    fn get_effects(&self) -> Vec<CardEffect>{

        let mut toreturn = Vec::new();


        toreturn.push( CardEffect::HalveTimeLeft );

        return toreturn;
    }


}