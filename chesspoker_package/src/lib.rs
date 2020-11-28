use nalgebra::{Point3, RealField, Vector3};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;

use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};


mod maingame;





use maingame::MainGame;






use std::collections::HashMap;
use std::collections::HashSet;


//all these should be serializable
pub use maingame::PlayerInput;
pub use maingame::PieceAction;
pub use maingame::Card;
use maingame::PieceTypeData;
use maingame::TurnManager;
use maingame::GameEngineState;
use maingame::CardGame;


use serde::{Serialize, Deserialize};    

//complete representation of the state of the game
#[derive(Serialize, Deserialize)]
pub struct GameData{
    
    
    //if this struct is a complete representation of the data
    //or if this is an "update" version of the data
    iscompleterepresentation: bool,
    
    
    
    totalplayers: Option<u8>,
    totalpieces: Option<u32>,
    players: Option< HashSet<u8> >,
    playertopiece: Option< HashMap<u8, HashSet<u32> > >,
    piecetypedata: Option< HashMap<u32, PieceTypeData> >,
    cards: Option< HashMap< u16, Card> >,
    playertocards: Option< HashMap< u8, Vec<u16> > >,
    turnmanager: Option< TurnManager >,
    cardgame: Option< Option<CardGame> >,
    
    queuedinputs: Option< HashMap<u8, Option<PlayerInput>> >,
    
    
    
    
    
    //the physical game engine I cannot serialize automatically
    gameengine: Option< GameEngineState >,
    
    
    
}

impl GameData{
    
    fn new_empty() -> GameData{
        
        GameData{
            iscompleterepresentation: false,
            
            totalplayers: None,
            totalpieces: None,
            players: None,
            playertopiece: None,
            piecetypedata: None,
            cards: None,
            playertocards: None,
            turnmanager: None,
            cardgame: None,
            queuedinputs: None,
            
            //the physical game engine I cannot serialize automatically
            gameengine: None,
        }
        
    }
    
    
    
}



//the data to send and receive







pub struct GameInterface{
    
    
    thegame: MainGame,
    
    
    
}


impl GameInterface{

    
    pub fn new_2_player_game() -> GameInterface{

        //println!("HELLO I AM HERE");
        
        let mut toreturn = GameInterface{
            
            thegame: MainGame::new_two_player(),
        };


        //let gameinformationstring = toreturn.get_game_information_string(&1);
        //toreturn.set_game_information_string(gameinformationstring);

        toreturn


    }


    //tick
    pub fn tick(&mut self){
        
        self.thegame.tick();
        
    }



    //get the id of every piece
    pub fn get_piece_ids(&self) -> Vec<u32>{

        self.thegame.get_all_piece_ids()
        
    }
    //get the id of every board square
    pub fn get_board_square_ids(&self) -> Vec<(u32, u32)>{

        self.thegame.get_all_board_square_ids()

        
    }

    //get the id of every card
    pub fn get_card_ids(&self) -> Vec<u16>{

        self.thegame.get_all_card_ids()

    }

    //get the cards in the game
    //cards in player 1 hand (in game), river in game, player 2 hand (in game)
    pub fn get_cards_in_game(&self) -> Option< (Vec<Card>, Vec<Card>, Vec<Card>) >{

        self.thegame.get_cards_in_game()

    }
    
    

    //get the position of a piece by its id
    pub fn get_piece_translation(&self, pieceid: u32) -> (f32,f32,f32){

        self.thegame.get_piece_position(&pieceid)

    }
    pub fn get_piece_rotation(&self, pieceid: u32) -> (f32,f32,f32){

        self.thegame.get_piece_rotation(&pieceid)
    }




    //get the position of the board square by its id
    pub fn get_board_square_translation(&self, boardsquareid: &(u32,u32)) -> (f32,f32,f32){

        self.thegame.get_board_square_translation(boardsquareid)
    
    }
    pub fn get_board_square_rotation(&self, boardsquareid: &(u32,u32)) -> (f32,f32,f32){

        self.thegame.get_board_square_rotation(boardsquareid)


    }


    


    //get the data about the card
    //from the perspective of a player

    //get the data about the card
    pub fn get_card(&self, cardid: &u16, playerid: &u8) -> Card{

        //if i can get the information about the card
        if let Some(card) = self.thegame.get_card(cardid, playerid){

            return( card );
        }
        else{

            return( Card::new_unknown_card() );

        }


    }

    //get the cards owner
    pub fn get_card_owner(&self, cardid: &u16) -> u8{
        self.thegame.get_owner_of_card(cardid)
    }

    //get the position of the card in its owners hand
    pub fn get_card_position_in_hand(&self, cardid: &u16) -> u32{

        self.thegame.get_position_of_card_in_hand(cardid)
    }





    
    
    



    //give the game input by a player
    //get a return type on whether the input is valid
    // (or something)
    pub fn receive_input(&mut self, playerid: &u8, input: PlayerInput){

    
        self.thegame.receive_input(*playerid, input);        
        
    }



    


    //get the board squares that this piece is allowed to move to

    //and for each board square it can move to, whether it can move there by slide or lift and move
    //and alsow whether it can be flicked...
    pub fn get_squares_reachable_by_piece(&self, pieceid: &u32) ->  HashSet< (u32,u32) >  {

        self.thegame.get_board_squares_reachable_by_piece(pieceid)


    }


    //get  the -list- of squares this piece is on
    pub fn get_board_square_piece_is_on(&self, pieceid: &u32) -> Option< (u32,u32) >{

        return self.thegame.get_board_square_piece_is_on(pieceid);

    }



    
    //get a list of the actions allowed by a piece
    //the board square it lands on
    //and the pieces that it can capture
    pub fn get_actions_allowed_by_piece(&self, pieceid: &u32) -> (bool, Vec<(PieceAction, (u32,u32) , HashSet<u32> )>){

        //get the list of slide and lift actions it can perform
        let slideandliftactions = self.thegame.get_slide_and_lift_actions_allowed_for_piece(pieceid);

        let mut toreturn = (false, Vec::new());

        //for each of these slide and lift actions
        for curaction in slideandliftactions{
            //get the position of the board square that this piece, performing this action
            //will land on
            let squaretargeted = self.thegame.get_square_that_action_takes_piece(pieceid, curaction.clone());

            //get the list of pieces on the board square targeted
            let piecestargeted = self.thegame.get_pieces_on_board_square( &squaretargeted);            

            toreturn.1.push( (curaction, squaretargeted, piecestargeted)  );
        };



        //get if the piece can be flicked
        if ( self.thegame.get_if_piece_can_be_flicked(pieceid) ){
            toreturn.0 = true;
        }

        return toreturn;


    }



    pub fn get_piece_and_square_actions_allowed_by_card(&self, cardid: &u16) -> (Vec<((u8,u8), PlayerInput)>, Vec<(u32, PlayerInput)>){

        //get the pieces and square that the card can go to
        let (pieceids, boardsquareids) = self.get_pieces_and_squares_actable_by_card(cardid);

        let mut boardsquareinputs:Vec<((u8,u8), PlayerInput)> = Vec::new();
        let mut pieceinputs: Vec<(u32, PlayerInput)> = Vec::new();

        for pieceid in pieceids{

            let input = PlayerInput::playcardonpiece(*cardid, pieceid);

            pieceinputs.push( (pieceid, input)  );
        }

        for (boardsquareid) in boardsquareids{

            let input = PlayerInput::playcardonsquare( *cardid, boardsquareid);

            boardsquareinputs.push( (boardsquareid, input) );

        }



        return ( boardsquareinputs, pieceinputs );
        
    }
    


    //get the cards and pieces a card is allowed to perform an action on
    //and whether you can drag it out to play it on its own or not
    pub fn get_pieces_and_squares_actable_by_card(&self, cardid: &u16) -> ( Vec<u32>, Vec<(u8,u8)> ){

        self.thegame.get_pieces_and_squares_actable_by_card(cardid)

    }














    //get the game information as a string
    pub fn get_game_information_string(&self, playerid: &u8) -> String{


        let gamedata = self.get_game_information(playerid);

        //serialize it and return the serialized result

        serde_json::to_string(&gamedata).unwrap()


    }
    
    
    
    fn get_game_information(&self, playerid: &u8) -> GameData{
        
        
        let mut toreturn = GameData::new_empty();
        
        
        
        toreturn.totalplayers = Some ( self.thegame.get_totalplayers() );
        
        toreturn.totalpieces = Some ( self.thegame.get_totalpieces() );
        
        toreturn.playertopiece = Some ( self.thegame.get_playertopiece() );
        
        toreturn.players = Some ( self.thegame.get_players() );
        
        toreturn.piecetypedata = Some ( self.thegame.get_piecetypedata() );
        
        toreturn.cards = Some ( self.thegame.get_cards() );
        
        toreturn.playertocards = Some ( self.thegame.get_playertocards() );
        
        toreturn.turnmanager = Some ( self.thegame.get_turnmanager() );
        
        toreturn.cardgame = Some ( self.thegame.get_blackjackgame() );
        
        toreturn.queuedinputs = Some ( self.thegame.get_queuedinputs() );
        
        
        //fill the game engine
        {
            
            //get the missions
            toreturn.gameengine = Some ( self.thegame.get_game_engine_state() );
            
        }
        
        
        toreturn
        
    }
    
    

    pub fn set_game_information_string(&mut self, data: String){

        //serialize the string into its data
        //or should I be serializing and deserializing in the main game?
        //Or would I just need to return a NOTOK if it doesnt work...
        //and the error of something that cant serialize, or just doesnt serialize properly should be the same
        //and if theres ever an unrecoverable error in the client
        //the worst that can happen is that the server sends the client complete information about its game state and it restarts like that



        //if the data is OK to be parsed
        if let Ok(gamedata) = serde_json::from_str::<GameData>(&data){


            self.set_game_information(gamedata);

        };


    }
    
    
    //given information about the state of the game
    //update the game
    pub fn set_game_information(&mut self, data: GameData){
        
        
        /*
        totalplayers: Option<u8>,
        totalpieces: Option<u32>,
        players: Option< HashSet<u8> >,
        playertopiece: Option< HashMap<u8, HashSet<u32> > >,
        allowedactions: Option< HashMap<u32, AllowedActions> >,
        cards: Option< HashMap< u16, Card> >,
        playertocards: Option< HashMap< u8, Vec<u16> > >,
        turnmanager: Option< TurnManager >,
        blackjackgame: Option< Option<(u8,u8,BlackJackGame)> >,
        queuedinputs: Option< HashMap<u8, Option<PlayerInput>> >,
        
        //the physical game engine I cannot serialize automatically
        gameengine: Option< GameEngineState >,
        */
        

        //if each different struct exists, make the game set it
        if let Some(totalplayers) = data.totalplayers{

            self.thegame.set_totalplayers(totalplayers);
        }

        if let Some(totalpieces) = data.totalpieces{

            self.thegame.set_totalpieces(totalpieces);
        }

        if let Some(players) = data.players{

            self.thegame.set_players(players);
        }

        if let Some(playertopiece) = data.playertopiece{

            self.thegame.set_playertopiece(playertopiece);
        }

        if let Some(allowedactions) = data.piecetypedata{
            self.thegame.set_piecetypedata(allowedactions);
        }

        if let Some(cards) = data.cards{
            self.thegame.set_cards(cards);
        }

        if let Some(playertocards) = data.playertocards{
            self.thegame.set_playertocards(playertocards);
        }

        if let Some(turnmanager) = data.turnmanager{
            self.thegame.set_turnmanager(turnmanager);
        }

        if let Some(cardgame) = data.cardgame{
            self.thegame.set_blackjackgame(cardgame);
        }


        if let Some(queuedinputs) = data.queuedinputs{
            self.thegame.set_queuedinputs(queuedinputs);
        }


        if let Some(gameengine) = data.gameengine{

            self.thegame.set_game_engine_state(gameengine);
        }

        
    }
    
    
    
    
    
    
    
    
    
}





//a request for how the client wants to join a game
#[derive(Serialize, Deserialize)]
pub enum GameToConnectTo{


    joinpublicgame,

    joinprivategame(u32),

    createprivategame,


}


//the message sent when a client is connected to a game on the server
//and the game is active
#[derive(Serialize, Deserialize)]
pub struct ConnectedToGame{

    //what is your player id in the game
    playerid: u32,


}

impl ConnectedToGame{

    pub fn new(playerid: u32) -> ConnectedToGame{

        ConnectedToGame{

            playerid: playerid
        }


    }


}