

mod maingame;





use maingame::MainGame;






use std::collections::HashMap;
use std::collections::HashSet;


//all these should be serializable
pub use maingame::PlayerInput;
pub use maingame::PieceAction;
pub use maingame::Card;


pub use maingame::GameData;
pub use maingame::GameToConnectTo;
pub use maingame::ConnectedToGame;

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
    
        self.thegame.get_card(cardid, playerid)

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

        self.thegame.get_board_square_piece_is_on(pieceid)

    }



    
    //get a list of the actions allowed by a piece
    //the board square it lands on
    //and the pieces that it can capture
    pub fn get_actions_allowed_by_piece(&self, pieceid: &u32) -> (bool, Vec<(PieceAction, (u32,u32) , HashSet<u32> )>){

        self.thegame.get_actions_allowed_by_piece(pieceid)

    }



    pub fn get_piece_and_square_actions_allowed_by_card(&self, cardid: &u16) -> (Vec<((u8,u8), PlayerInput)>, Vec<(u32, PlayerInput)>){
    
        self.thegame.get_piece_and_square_actions_allowed_by_card(cardid)
        
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
        
        self.thegame.get_game_information(playerid)
        
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
        
        self.thegame.set_game_information(data);
        
    }
    
    
    
    
    
    
    
    
    
}




