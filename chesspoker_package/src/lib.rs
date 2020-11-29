mod gameengine;

use gameengine::GameEngine;


//export and make public the GameEngineState struct
pub use gameengine::GameEngineState;



use std::collections::HashSet;
use std::collections::HashMap;



mod datastructs;




//import the data structures needed

//make these public, and visible to the game interface
pub use datastructs::PlayerInput;
pub use datastructs::PieceAction;



pub use datastructs::TurnManager;

use datastructs::direction_change_to_slide_id_from_objective_perspective;
use datastructs::slide_id_to_direction_change_from_objective_perspective;


pub use datastructs::PieceTypeData;


mod cardstructs;
pub use cardstructs::Card;
use cardstructs::CardEffect;
use cardstructs::CardValue;
use cardstructs::CardSuit;
use cardstructs::CardsInterface;




//the maingame creates and returns these objects as its fuctions




pub struct MainGame{
    
    
    totalplayers: u8,
    
    //a map of objects to what type of object they are
    totalpieces: u32,
    
    
    //the list of players
    players: HashSet<u8>,
    
    
    //the pieces owned by each player
    playertopiece: HashMap<u8, HashSet<u32> >,
    
    //the direction the player i facing, of the 8 cardinal directions
    playertodirection: HashMap<u8, u8>,
    

    piecetypedata: HashMap<u32,PieceTypeData>,
    
    
    //what players the turn currently is
    turnmanager: TurnManager,
    
    
    //the games running, and the players of that game
    physicalgameengine: GameEngine,
    
    



    //the card interface
    cards: CardsInterface,

    /*
    cards: HashMap<u16, Card>,


    //the cards each player has in their hand as a list
    playertocards: HashMap<u8, Vec<u16> >,

    //the total number of cards ever, used for setting ID
    totalcards: u16,
    

    //the games and the players of the games
    //with the id of player 1 and 2
    cardgame: Option<CardGame>,
    */

    
    
    //the last input of each player
    queuedinputs: HashMap<u8, Option<PlayerInput>>,

    
    
    
    
}

impl MainGame{

    //create a game with two players
    pub fn new_two_player() -> MainGame{
        
        
        //create a new 2 player game        
        let mut toreturn = MainGame{
            
            cards: CardsInterface::new_two_player(),
            totalplayers: 0,
            totalpieces: 0,
            players: HashSet::new(),
            playertodirection: HashMap::new(),
            playertopiece: HashMap::new(),
            piecetypedata: HashMap::new(),
            turnmanager: TurnManager::new_two_player(1, 2),            
            physicalgameengine: GameEngine::new(),
            queuedinputs: HashMap::new(),
            
            
        };
        
        
        //add two players
        toreturn.add_player();
        toreturn.add_player();
        
        toreturn.queuedinputs.insert(1, None);
        toreturn.queuedinputs.insert(2, None);
        
        
        
        //add the standard configuration of chess pieces
        //owned by each player
        toreturn.initialize_pieces();
        
        
        toreturn.start_poker_game(1, 2);
        
        
        toreturn
        
    }
    
    
    //given information about the state of the game
    //update the game
    pub fn set_game_information(&mut self, data: GameData){
        


        //if each different struct exists, make the game set it
        if let Some(totalplayers) = data.totalplayers{

            self.set_totalplayers(totalplayers);
        }

        if let Some(totalpieces) = data.totalpieces{

            self.set_totalpieces(totalpieces);
        }

        if let Some(players) = data.players{

            self.set_players(players);
        }

        if let Some(playertopiece) = data.playertopiece{

            self.set_playertopiece(playertopiece);
        }

        if let Some(allowedactions) = data.piecetypedata{
            self.set_piecetypedata(allowedactions);
        }

        if let Some(turnmanager) = data.turnmanager{
            self.set_turnmanager(turnmanager);
        }

        if let Some(cardgame) = data.cards{
            self.set_cards(cardgame);
        }


        if let Some(queuedinputs) = data.queuedinputs{
            self.set_queuedinputs(queuedinputs);
        }


        if let Some(gameengine) = data.gameengine{

            self.set_game_engine_state(gameengine);
        }

        
    }

    pub fn get_game_information(&self, playerid: &u8) -> GameData{
        
        
        let mut toreturn = GameData::new_empty();
        
        
        
        toreturn.totalplayers = Some ( self.get_totalplayers() );
        
        toreturn.totalpieces = Some ( self.get_totalpieces() );
        
        toreturn.playertopiece = Some ( self.get_playertopiece() );
        
        toreturn.players = Some ( self.get_players() );
        
        toreturn.piecetypedata = Some ( self.get_piecetypedata() );
        
        toreturn.cards = Some ( self.get_cards() );
        
        toreturn.turnmanager = Some ( self.get_turnmanager() );
        
        toreturn.queuedinputs = Some ( self.get_queuedinputs() );
        
        
        //fill the game engine
        {
            
            //get the missions
            toreturn.gameengine = Some ( self.get_game_engine_state() );
            
        }
        
        
        toreturn
        
    }




    //the functions needed by the wasm
    pub fn get_all_piece_ids(&self) -> Vec<u32>{
        
        let mut toreturn = Vec::new();
        
        for (playerid, pieceidmap) in self.playertopiece.iter() {
            
            for pieceid in pieceidmap{
                
                toreturn.push(*pieceid);
                
            };
            
            
        };
        
        toreturn
        
        
        
    }
    pub fn get_all_board_square_ids(&self) -> Vec<(u32,u32)>{
        
        //assume, for important reasons
        //(because I shouldnt have any other or different board squares and i dont want to make)
        //(the function call a function, call a function, call a function, call a function)
        //that the board squares are just the normal 64 board squares of a normal chess board
        //and id'd the normal way
        
        let  mut toreturn = Vec::new();
        
        for x in 0..8{
            for y in 0..8{
                
                toreturn.push( (x,y) );
                
            };
        };
        
        
        toreturn
        
        
    }

    
    pub fn get_piece_position(&self, pieceid: &u32) -> (f32,f32,f32){
        
        self.physicalgameengine.get_piece_translation(pieceid)
        
    } 
    pub fn get_piece_rotation(&self, pieceid: &u32) -> (f32,f32,f32){
        
        self.physicalgameengine.get_piece_rotation(pieceid)
    }
    pub fn get_board_square_translation(&self, boardsquareid:& (u8,u8)) -> (f32,f32,f32){
        
        self.physicalgameengine.get_board_square_translation( boardsquareid)
        
        
    }
    pub fn get_board_square_rotation(&self, boardsquareid:& (u8,u8)) -> (f32,f32,f32){
        
        self.physicalgameengine.get_board_square_rotation( boardsquareid)
        
        
    }







    //card getter functions
    //get player 1 and 2s hand as a list of cards
    pub fn get_cards_in_hands(&self) ->  (Vec<Card>, Vec<Card>) {
        
        let player1hand = self.cards.get_cards_in_hand(1);
        let player2hand = self.cards.get_cards_in_hand(2);

        (player1hand, player2hand)
        
    }
    
    pub fn get_cards_in_game(&self) -> Option< (Vec<Card>, Vec<Card>, Vec<Card>) >{
        
        self.cards.get_cards_in_game()
    }


    //given the card and player id, get the actions allowed to be performed on what pieces and board squares
    pub fn get_piece_and_square_actions_allowed_by_card(&self, cardid: u16 , playerid: u8) -> ( Vec<(u32, PlayerInput)>, Vec<((u8,u8), PlayerInput)> ){

        let card = self.cards.get_card_unsafe(cardid);


        //get every possible piece and card input
        //if its allowed, push it to the list of cards and squares to return
        let mut allboardinputs = Vec::new();
        let mut allpieceinputs: Vec<(u32, PlayerInput)> = Vec::new();


        let mut allowedboardinputs = Vec::new();
        let mut allowedpieceinputs = Vec::new();

        //if this card can drop or raise a square
        if card.effect == CardEffect::dropsquare || card.effect == CardEffect::raisesquare{

            //push every board square and input into the list of all board inputs
            for x in 0..8{
                for y in 0..8{

                    let boardsquareid = (x,y);

                    let playerinput = PlayerInput::playcardonsquare( cardid, boardsquareid );

                    allboardinputs.push( (boardsquareid, playerinput) );

                }
            }


            for (boardsquareid, playerinput) in allboardinputs{

                let isvalid = self.is_input_valid(playerid, &playerinput);

                if isvalid{
                    allowedboardinputs.push( (boardsquareid, playerinput) );
                }


            }


        }



        return ( allowedpieceinputs , allowedboardinputs );
        
    }
    
    pub fn get_pieces_and_squares_actable_by_card(&self, cardid: u16, playerid: u8) -> ( Vec<u32>, Vec<(u8,u8)> ){
        
        let mut toreturn = (Vec::new(), Vec::new());

        let (pieceinput, bsinput) = self.get_piece_and_square_actions_allowed_by_card(cardid, playerid);
        
        for (pieceid, _) in pieceinput{
            toreturn.0.push(pieceid);
        };

        for (bsid, _) in bsinput{
            toreturn.1.push(bsid);
        };

        toreturn
        
    }


    




    //piece getter functions
    fn get_board_square_piece_is_on(&self, pieceid: &u32) -> Option<(u8,u8)>{
        
        return self.physicalgameengine.get_board_square_piece_is_on(pieceid);
        
    }
    
    pub fn get_pieces_and_squares_reachable_by_piece(&self, pieceid: &u32) -> (HashSet<(u32)>, HashSet<(u8,u8)>){
        

        let mut pieceids = HashSet::new();
        let mut boardsquareids = HashSet::new();
        
        //get  the slide and lift actions allowed by the piece
        let actionsallowed = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);
        
        //for each action allowed, get the board square it would take the piece, and add it to return
        for action in actionsallowed{
            
            let boardsquareid = self.get_square_that_action_takes_piece(pieceid, action);
            
            boardsquareids.insert(boardsquareid);
        }
        
        
        //if its not on any board square, its handled by the "get_slide_and_lift_actions" function by returnign empty vec
        (pieceids, boardsquareids)
        
    }
    
    pub fn get_actions_allowed_by_piece(&self, pieceid: &u32) -> (bool, Vec<(PieceAction, (u8,u8) , HashSet<u32> )>){

        //get the list of slide and lift actions it can perform
        let slideandliftactions = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);

        let mut toreturn = (false, Vec::new());

        //for each of these slide and lift actions
        for curaction in slideandliftactions{
            //get the position of the board square that this piece, performing this action
            //will land on
            let squaretargeted = self.get_square_that_action_takes_piece(pieceid, curaction.clone());

            //get the list of pieces on the board square targeted
            let piecestargeted = self.get_pieces_on_board_square( &squaretargeted);            

            toreturn.1.push( (curaction, squaretargeted, piecestargeted)  );
        };



        //get if the piece can be flicked
        if ( self.get_if_piece_can_be_flicked(pieceid) ){
            toreturn.0 = true;
        }

        return toreturn;


    }
    
    //get the list of pieceactions allowed by the specified piece (not just what the struct says are allowed, but all the positions to move)
    pub fn get_slide_and_lift_actions_allowed_for_piece(&self, pieceid: &u32) -> Vec<PieceAction>{
        
        let mut toreturn: Vec<PieceAction> = Vec::new();
        
        
        
        //get the piece data of this piece
        let piecetypedata = self.piecetypedata.get(pieceid).unwrap();
        
        
        
        
        //get the board square the piece is on
        let maybeboardsquareid = self.get_board_square_piece_is_on(pieceid);
        
        //if its not on a board square its not allowed to take any actions
        if maybeboardsquareid == None{
            return Vec::new();
        };
        
        let boardsquareid = maybeboardsquareid.unwrap();
        
        //get the owner of this piece
        let ownerofpiece = self.get_owner_of_piece(pieceid);
        
        //get the facing direction of the owner
        let ownerdirection = self.playertodirection.get(&ownerofpiece).unwrap();
        
        
        //get the slide actions
        let slide_actions = piecetypedata.get_allowed_slide_actions(ownerdirection);
        
        //get the lift and move actions
        let lift_and_move_actions = piecetypedata.get_allowed_lift_and_move(ownerdirection);
        
        
        
        //for each direction its allowed to slide
        for (direction, maxdistance, hastocapture, cancapture) in slide_actions.iter(){
            
            let mut currentboardsquare = (boardsquareid.0 as i32, boardsquareid.1 as i32);
            
            //get the change in position every step from the direction
            let (xstep, zstep) = slide_id_to_direction_change_from_objective_perspective(*direction);
            
            
            for stepnumber in 1..*maxdistance+1{
                
                //step in the direction from the current position
                currentboardsquare.0 += xstep;
                currentboardsquare.1 += zstep;
                
                
                //if the board square gets out of range, break immediately
                if currentboardsquare.0 < 0 || currentboardsquare.0 > 7{
                    break;
                }
                if currentboardsquare.1 < 0 || currentboardsquare.1 > 7{
                    break;
                }
                
                
                let currentboardsquareid = (currentboardsquare.0 as u8, currentboardsquare.1 as u8);
                
                let piecesonboardsquare = self.physicalgameengine.get_pieces_on_board_square(&currentboardsquareid);
                
                //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                let mut onlyopponentspieces = true;
                for otherpieceid in piecesonboardsquare.iter(){
                    
                    //if this piece is owned by the same player that owns the "pieceid" entered
                    //set "onlyopponentspieces" to false
                    let ownerofotherpiece = self.get_owner_of_piece(&otherpieceid);
                    
                    if ownerofotherpiece == ownerofpiece{   
                        onlyopponentspieces = false;
                    }
                    
                }
                
                
                
                //if this is an empty board square, and im not forced to capture to move, add this
                if piecesonboardsquare.is_empty(){
                    
                    if ! hastocapture{
                        
                        let action_to_slide_here = PieceAction::slide(*direction, stepnumber);
                        
                        toreturn.push(action_to_slide_here);
                        
                    }
                    
                    
                }
                else{
                    
                    //if this square has a piece and only has opponents pieces, and im allowed to capture, add this
                    if onlyopponentspieces{
                        
                        if *cancapture{
                            let action_to_slide_here = PieceAction::slide(*direction, stepnumber);
                            
                            toreturn.push(action_to_slide_here);
                            
                            
                        }
                    }
                }
                
                
                
                
                //if there is a piece on this board square break and end after this loop
                
                if ! piecesonboardsquare.is_empty() {
                    
                    break;
                    
                }
                
                
                
                
            }
            
            
            
        }
        
        
        
        //for each position it can be lifted and moved to
        for (currelativeposition, hastocapture, cancapture ) in lift_and_move_actions.iter(){
            
            //the position of the piece + the direction this move wants to send it
            let currentboardsquare = (currelativeposition.0 + boardsquareid.0 as i8, currelativeposition.1 + boardsquareid.1 as i8);
            
            //if the board square considered is out of range, dont add it
            if currentboardsquare.0 < 0 || currentboardsquare.0 > 7{
                
            }
            else if currentboardsquare.1 < 0 || currentboardsquare.1 > 7{
                
            }
            else{
                
                
                let currentboardsquareid = (currentboardsquare.0 as u8, currentboardsquare.1 as u8);
                
                
                //if this board square does not have any of my pieces on it
                let piecesonboardsquare = self.physicalgameengine.get_pieces_on_board_square(& currentboardsquareid);
                
                //for each piece on the board square, get if it only has opponents pieces on it (includes being empty)
                let mut onlyopponentspieces = true;
                
                for otherpieceid in piecesonboardsquare.iter(){
                    
                    let ownerofotherpiece = self.get_owner_of_piece(&otherpieceid);
                    if ownerofotherpiece == ownerofpiece{   
                        onlyopponentspieces = false;
                    }
                    
                }
                
                
                //if this is an empty board square, and im not forced to capture to move, add this
                if piecesonboardsquare.is_empty(){
                    
                    if ! hastocapture{
                        
                        let lift_action_to_get_here = PieceAction::liftandmove( (currelativeposition.0 as i32, currelativeposition.1 as i32) );
                        
                        toreturn.push(lift_action_to_get_here);
                        
                    }
                    
                    
                }
                else{
                    
                    //if this square has a piece and only has opponents pieces, and im allowed to capture, add this
                    if onlyopponentspieces{
                        
                        if *cancapture{
                            
                            let lift_action_to_get_here = PieceAction::liftandmove( (currelativeposition.0 as i32, currelativeposition.1 as i32) );
                            
                            toreturn.push(lift_action_to_get_here);
                            
                        }
                        
                    }
                    
                }
                
            }
            
            
            
        }
        
        
        //if it can castle
        
        
        
        
        
        toreturn
        
        
        
    }
    



    //assume the action is valid, and the piece is on a board square
    //try not to call this method any tick after the "get actions allowed for piece" is called
    //get the id of the board square that this action will take this piece
    fn get_square_that_action_takes_piece(&self, pieceid: &u32, pieceaction: PieceAction) -> (u8,u8){
        
        //get the board square id this piece is on
        let boardsquareid = self.get_board_square_piece_is_on(pieceid).unwrap();
        
        let mut boardsquarepos = (boardsquareid.0 as i32, boardsquareid.1 as i32);
        
        
        //if its a slide action
        if let PieceAction::slide(direction, distance) = pieceaction{
            
            let (xdiff, zdiff) = slide_id_to_direction_change_from_objective_perspective(direction);
            
            let distance = distance as i32;
            
            let xdiff = xdiff * distance;
            let zdiff = zdiff * distance;
            
            boardsquarepos.0 += xdiff;
            boardsquarepos.1 += zdiff;
            
        };
        
        
        //if its a lift and move action
        if let PieceAction::liftandmove( (xdiff, zdiff) ) = pieceaction{
            
            boardsquarepos.0 += xdiff;
            boardsquarepos.1 += zdiff;
            
        }
        
        
        //make sure its within range
        //which it SHOULD BE if this method is called appropriately (and I dont have errors)
        //(and i cant get panic messages when running as wasm.... fuck)
        
        if boardsquarepos.0 < 0 || boardsquarepos.0 > 7{
            panic!("board square not within range");
        }
        if boardsquarepos.1 < 0 || boardsquarepos.1 > 7{
            panic!("board square not within range");
        }
        
        
        
        return  ( boardsquarepos.0 as u8, boardsquarepos.1 as u8 )   ;
        
    }
    
    fn get_pieces_on_board_square(&self, boardsquareid: &(u8, u8)) -> HashSet<u32>{
        
        self.physicalgameengine.get_pieces_on_board_square(boardsquareid)
        
    }
    
    fn get_if_piece_can_be_flicked(&self, pieceid: &u32) -> bool{
        
        true
        
    }
    
    
    
    
    
    
    
    
    
    

    //get the input that a player sends and set it to be performed next tick
    //return whether this input is valid for this player to have queued
    pub fn receive_input(&mut self, playerid: u8, input: PlayerInput) -> bool{        

        
        
        //get if the input is valid for this player
        if  self.is_input_valid(playerid, &input ) {
            
            self.queuedinputs.insert(playerid, Some(input));
            
            return true ;
            
        }
        else{
            
            
            return false ;
        };
        
        
    }


    //get what pieces are captures in the game engine and remove them from here
    pub fn tick(&mut self){
        
        
        //get each player whos turn it currently is
        let currentturnplayers = self.turnmanager.get_current_players();
        
        
        
        
        for playerid in currentturnplayers.clone(){
            
            
            //if an action was taken
            let mut actionwastaken = false;
            
            
            
            //if this player has a queued input
            if let Some(playerinput) = self.queuedinputs.get(&playerid).unwrap(){
                
                
                //if its valid to perform it
                if self.is_input_valid(playerid, &playerinput){
                    
                    self.perform_input(&playerid, &playerinput.clone());
                    
                    actionwastaken = true;
                    
                }
                else{
                    actionwastaken = false;
                }
                
                
            }
            
            
            
            //if an action was taken, let the turnmanager know that that player took their turn
            if (actionwastaken){
                self.turnmanager.player_took_action(playerid);
                
                //and clear queud inputs
                self.queuedinputs.insert(playerid, None);
            }
            
            
            
        }
        
        
        
        //let the turn manager know that a tick has happeneds
        self.turnmanager.tick();
        
        //tick the physical game engine
        self.physicalgameengine.tick();
        
    }
    















    
    //set the type of piece a piece is
    //what actions it can perform
    //what it looks like
    fn set_piece_type(&mut self, pieceid: &u32, typeid: &u32){
        
        /*
        //1 pawn
        //2 knight
        //3 bishop
        //4 rook
        //5 queen
        //6 king
        OTHER: "all"
        */
        
        let mut piecetypedata = self.piecetypedata.get_mut(pieceid).expect("why doesnt htis piece ID have a piecetype associated?");
        
        
        //set the piece type data according to this 
        
        if typeid == &1{
            piecetypedata.set_pawn();
        }
        else if typeid == &2{
            piecetypedata.set_knight();
        }
        else if typeid == &3{
            piecetypedata.set_bishop();
        }
        else if typeid == &4{
            piecetypedata.set_rook();
        }
        else if typeid == &5{
            
            piecetypedata.set_queen();
        }
        else if typeid == &6{
            piecetypedata.set_king();
        }
        
        
    }
    
    //create all the pieces for both players
    fn initialize_pieces(&mut self){
        
        
        //add all the pawns
        for x in 0..8{
            
            for y in 1..2{
                
                let player1pawnid = self.add_piece(1,  (x,y) );
                self.set_piece_type(&player1pawnid, &1);
                
                let player2pawnid = self.add_piece(2,  (x,y+5) );
                self.set_piece_type(&player2pawnid, &1);
            }
            
            
        }
        
        //add the rooks
        let rook = self.add_piece(1, (0,0) );
        self.set_piece_type(&rook, &4);
        let rook = self.add_piece(1, (7,0) );
        self.set_piece_type(&rook, &4);
        let rook = self.add_piece(2, (0,7) );
        self.set_piece_type(&rook, &4);
        let rook = self.add_piece(2, (7,7) );
        self.set_piece_type(&rook, &4);
        
        
        //add the knights
        let piece = self.add_piece(1, (1,0) );
        self.set_piece_type(&piece, &2);
        let piece = self.add_piece(1, (6,0) );
        self.set_piece_type(&piece, &2);
        let piece = self.add_piece(2, (1,7) );
        self.set_piece_type(&piece, &2);
        let piece = self.add_piece(2, (6,7) );
        self.set_piece_type(&piece, &2);
        
        
        //add the bishops
        let piece = self.add_piece(1, (2,0) );
        self.set_piece_type(&piece, &3);
        let piece = self.add_piece(1, (5,0) );
        self.set_piece_type(&piece, &3);
        let piece = self.add_piece(2, (2,7) );
        self.set_piece_type(&piece, &3);
        let piece = self.add_piece(2, (5,7) );
        self.set_piece_type(&piece, &3);
        
        
        //add the queens
        let piece = self.add_piece(1, (3,0) );
        self.set_piece_type(&piece, &5);
        let piece = self.add_piece(2, (3,7) );
        self.set_piece_type(&piece, &5);
        
        
        
        //add the kings
        let piece = self.add_piece(1, (4,0) );
        self.set_piece_type(&piece, &6);
        let piece = self.add_piece(2, (4,7) );
        self.set_piece_type(&piece, &6);
        
        
    }
    
    //add a player
    fn add_player(&mut self){
        
        //the number of players starts counting at 1
        //so the first players id is 1 not 0
        let currentplayer = self.totalplayers + 1;
        
        self.players.insert(currentplayer);
        
        if currentplayer == 1{
            
            //player 1 faces direction "0"
            self.playertodirection.insert(1, 0);
            
        }
        else if currentplayer == 2{
            
            //player 2 faces direction "4"
            self.playertodirection.insert(2, 4);
            
        }
        else{
            panic!("not implemented for anything other than 2 players and probably never will be");
        }
        
        
        
        self.playertopiece.insert(currentplayer, HashSet::new());
        
        
        //give that player a random card
        self.cards.give_new_random_card(currentplayer);
        self.cards.give_new_random_card(currentplayer);
        self.cards.give_new_random_card(currentplayer);
        self.cards.give_new_random_card(currentplayer);

        
        
        self.totalplayers += 1;
        
    }
    
    //add a piece
    //specify which player
    fn add_piece(&mut self, playerid: u8, position:(u8,u8)) -> u32{
        
        //println!("I ADDED PIECE, TESTING LOGGIN");
        let pieceid = self.totalpieces;
        
        
        //set this piece as a Queen
        self.piecetypedata.insert( pieceid, PieceTypeData::new() );
        
        
        //set this piece to be owned by that player
        
        self.playertopiece.get_mut( &playerid).unwrap().insert(pieceid);
        //self.playertopiece.insert( playerid, pieceid);
        
        //add the piece in the game engine
        //at square (3,3), and with shapeid  of 0
        self.physicalgameengine.add_piece( pieceid, (position.0, position.1), 0);
        
        
        self.totalpieces += 1;
        
        return(pieceid);
        
    }
    

    
    





    //check if input is valid rather than just if the action is
    //if the player is the one sending the request or some shit like that i guess
    fn is_input_valid(&self, playerid: u8, input: &PlayerInput) -> bool{
        
        
        //if the player doesnt own the piece or the card its not valid
        //return false before proceeding
        {
            
            
            //if its a card action, what the id of it is
            let mut maybecardid: Option<u16> = None;
            
            //if its a play card alone action
            if let PlayerInput::playcardonboard(cardid) = input {
                maybecardid = Some(*cardid);
            }
            //if its a play card alone action
            else if let PlayerInput::playcardonpiece(cardid, pieceid) = input {
                maybecardid = Some(*cardid);
            }
            //if its a play card alone action
            else if let PlayerInput::playcardonsquare(cardid, boardsquareid) = input {
                maybecardid = Some(*cardid);
            }
            
            if let Some(cardid) = maybecardid {
                
                let owns = self.cards.does_player_own_card(playerid, cardid);

                if owns == false{
                    return false;
                }
            }
            
            if let PlayerInput::pieceaction(pieceid, _ ) = input.clone(){
                
                let owns = self.playertopiece.get(&playerid).unwrap().contains(&pieceid);
                if owns == false{
                    return false;
                }
            }
            
        }
        
        
        
        
        
        //if its a play card alone action
        if let PlayerInput::playcardonboard(cardid) = input {
            return self.is_play_card_on_board_action_valid(&playerid, cardid) ;
            
        }
        
        //if its a play card alone action
        else if let PlayerInput::playcardonpiece(cardid, pieceid) = input {
            return self.is_play_card_on_piece_action_valid(&playerid, cardid, pieceid)
        }
        
        //if its a play card alone action
        else if let PlayerInput::playcardonsquare(cardid, boardsquareid) = input {
            return self.is_play_card_on_square_action_valid(&playerid, cardid, boardsquareid)
        }
        
        //if its a piece action
        //get if its valid        
        else if let PlayerInput::pieceaction(pieceid, pieceaction) = input.clone(){
            return self.is_piece_action_valid( &playerid, &pieceid, &pieceaction);
        }
        
        
        //if any of the cases are missed
        panic!(" why isnt this case dealt with? ");
        
    }
    
    //can this card be played alone
    fn is_play_card_on_board_action_valid(&self, playerid: &u8, cardid: &u16) -> bool{
        
        
        if self.cards.is_player_allowed_to_play_card(*playerid) {

            return(true);
        }

        let cardeffect = self.cards.get_card_unsafe( *cardid).effect;

            
        //if the card effect is to make a blackjack game
        if cardeffect == CardEffect::blackjackgame{
            return true;
        }

        //if the card effect is to make a poker game
        if cardeffect == CardEffect::pokergame{
            return true;
        }


        return false;
            
    }
    
    //if this card can be played on this piece 
    fn is_play_card_on_piece_action_valid(&self, playerid: &u8, cardid: &u16, pieceid: &u32) -> bool{
        
        
        return false;
        
    }
    
    //if this card can be played on this boardsquare
    fn is_play_card_on_square_action_valid(&self, playerid: &u8, cardid: &u16, boardsquareid: &(u8,u8) ) -> bool{
        
        
        //get if this card has an effect that can be played on a board square
        let cardeffect = self.cards.get_card_unsafe(*cardid).effect.clone();
        
        
        //if its a drops a board square
        if cardeffect == CardEffect::dropsquare{
            
            //see if thats a board square valid to be dropped
            return(true);

        }
        
        
        //if its trying to lift a board square
        if cardeffect == CardEffect::raisesquare{
            
            //see if thats a board square valid to be raised
            return(true);
            
        }
        
        
        
        true
        
    }
    
    //only called when the player is the one who owns the piece
    fn is_piece_action_valid(&self, playerid: &u8, pieceid: &u32,  pieceaction: &PieceAction) -> bool{
        
        
        //if the piece action is a slide or lift action
        if  let PieceAction::slide(_,_) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return(true);                
            }
            else{
                return(false);
            }
            
            
            
        }
        else if let PieceAction::liftandmove( _ ) = pieceaction{
            
            //get the slide and lift actions allowed for the piece
            let allowedactions = self.get_slide_and_lift_actions_allowed_for_piece(pieceid);
            
            //if the action is one of the allowed actions, then, yea, its good
            if allowedactions.contains(pieceaction){
                return(true);                
            }
            else{
                return(false);
            }
            
        }
        else if let PieceAction::flick(direction, force) = pieceaction{
            
            //get its piece data
            let piecetypedata = self.piecetypedata.get(pieceid).unwrap();
            
            
            //get if this piece can perform this flick
            let canperformflick = piecetypedata.get_if_this_flick_allowed(*direction, *force);
            
            
            //and return it
            return(canperformflick);
            
        }
        
        panic!(" dont know what kind of mission this is..");
        
        
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    //perform an input that is valid, and it is the turn of the player
    fn perform_input(&mut self, playerid: &u8 ,playerinput: &PlayerInput) {
        
        
        if let PlayerInput::pieceaction(pieceid, pieceaction) = playerinput {
            
            
            if let PieceAction::liftandmove(relativeposition) = pieceaction{
                
                let relativeposition = (relativeposition.0 as f32, relativeposition.1 as f32);
                
                self.physicalgameengine.lift_and_move_piece_to(pieceid, relativeposition);
                
                
            }
            if let PieceAction::slide(slidedirection, slidedistance) = pieceaction{
                
                
                self.physicalgameengine.slide_piece(pieceid, slide_id_to_direction_change_from_objective_perspective(*slidedirection), *slidedistance );
                
                
            }
            if let PieceAction::flick(direction, force) = pieceaction{
                
                self.physicalgameengine.flick_piece(*pieceid, *direction, *force);
                
                
            }
            
            
        };
        
        
        //if the input is a card action
        if let PlayerInput::playcardonboard(cardid) = playerinput{

            //play that card
            self.cards.play_card(*playerid, *cardid);
            
        };
        
        
        if let PlayerInput::playcardonpiece(cardid, pieceid) = playerinput{
            
            
        };
        
        
        if let PlayerInput::playcardonsquare(cardid, squareid) = playerinput{
            
            panic!("im playing a card on da square");

            self.physicalgameengine.set_long_boardsquare_drop(50, *squareid);
            
            //and remove that card from that players hand
            self.cards.remove_card_from_hand(*playerid, *cardid);
            
        };
        
        
        
    }
    
    
    fn get_owner_of_piece(&self, pieceid: &u32) -> u8{
        
        //go over every player and get if they own this piece
        //if none do, panic
        
        for (playerid, piecemap) in self.playertopiece.iter(){
            
            if piecemap.contains(pieceid){
                
                return *playerid ;
            }
            
        }
        
        panic!(" piece not found to be owned by any player");
        
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //start a blackjack game with the given players
    fn start_blackjack_game(&mut self, player1: u8, player2:u8){
        
        self.cards.start_blackjack_game(player1, player2);
    }
    
    //start a poker game with the given players
    fn start_poker_game(&mut self, player1: u8, player2:u8){
        
        self.cards.start_poker_game(player1, player2);
        
    }
    


    
        
    
    //the fuctions for reading & writing the state used by the game interface to turn this game into
    fn get_queuedinputs(&self) ->  HashMap<u8, Option<PlayerInput>>{
        self.queuedinputs.clone()
    }
    fn set_queuedinputs(&mut self, queuedinputs: HashMap<u8, Option<PlayerInput>>){
        
        self.queuedinputs = queuedinputs;
    }
    fn get_totalplayers(&self) -> u8{
        self.totalplayers.clone()
    }
    fn set_totalplayers(&mut self, totalplayers: u8){
        self.totalplayers = totalplayers;
    }
    fn get_totalpieces(&self) -> u32{
        self.totalpieces.clone()
    }
    fn set_totalpieces(&mut self, totalpieces: u32){
        self.totalpieces = totalpieces;
    }
    fn get_players(&self)-> HashSet<u8>{
        self.players.clone()
    }
    fn set_players(&mut self, players: HashSet<u8>){
        self.players = players;
    }
    fn get_playertopiece(&self) -> HashMap<u8, HashSet<u32> >{
        
        self.playertopiece.clone()
    }
    fn set_playertopiece(&mut self, playertopiece: HashMap<u8, HashSet<u32> >){
        
        self.playertopiece = playertopiece;
        
        
    }
    fn get_piecetypedata(&self) -> HashMap<u32, PieceTypeData>{
        self.piecetypedata.clone()
    }
    fn set_piecetypedata(&mut self, piecetypedata: HashMap<u32, PieceTypeData>){
        self.piecetypedata = piecetypedata;
    }
    fn get_turnmanager(&self) -> TurnManager{
        self.turnmanager.clone()
    }
    fn set_turnmanager(&mut self, turnmanager: TurnManager){
        self.turnmanager = turnmanager;
    }
    fn set_cards(&mut self, cards: CardsInterface){
        self.cards = cards;
    }
    fn get_cards(&self) -> CardsInterface{
        self.cards.clone()
    }

    //get the physics engine state as a "GameEngineState"
    fn get_game_engine_state(&self) -> GameEngineState{
        
        self.physicalgameengine.get_game_engine_state()
    }
    fn set_game_engine_state(&mut self, data: GameEngineState){
        
        self.physicalgameengine.set_game_engine_state(data);
    }
    
}


















use serde::{Serialize, Deserialize};



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
    turnmanager: Option< TurnManager >,

    cards: Option< CardsInterface >,

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
            turnmanager: None,
            cards: None,
            queuedinputs: None,
            
            //the physical game engine I cannot serialize automatically
            gameengine: None,
        }
        
    }
    
    
    
}
