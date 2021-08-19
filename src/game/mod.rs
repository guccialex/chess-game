
mod boardengine;
use boardengine::BoardEngine;
//use boardengine::FullAction;
pub use boardengine::VisibleGameBoardObject;

mod turnmanager;
use turnmanager::TurnManager;

mod gameinput;
pub use gameinput::GameInput;

mod gameobject;
pub use gameobject::GameObject;

mod visiblegamestate;
pub use visiblegamestate::VisibleGameState;


mod gameeffect;

use gameeffect::CardEffect;
use gameeffect::EffectTrait;
use std::any::Any;



use rapier3d::na::Point3;
use rapier3d::na::Vector3;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Game{

    boardengine: BoardEngine,

    turnmanager: TurnManager,


    gameover: Option<u8>,

    queuedinputs: HashMap<u8, GameInput>,

    lastcardeffect: Option< ( i32, String ) >,
}


impl Game{

    pub fn new()  -> Game{

        let mut toreturn = Game{

            boardengine: BoardEngine::new(1, 2),
            turnmanager: TurnManager::new_two_player(1, 2, 10000, 2),
            gameover: None,
            queuedinputs: HashMap::new(),
            lastcardeffect: None,
        };

        toreturn.lastcardeffect = Some( (50, gameeffect::set_card_effect(CardEffect::HalveTimeLeft , toreturn.get_mut_effect_x())) );

        toreturn.lastcardeffect = Some( (50, gameeffect::set_card_effect(CardEffect::TurnsTimed(10) , toreturn.get_mut_effect_x())) );

        toreturn
    }



    pub fn get_gameobject_targeted(&self, ray: (Point3<f32>, Vector3<f32>)) -> Option<GameObject>{

        if let Some( objectid) = self.boardengine.get_object_intersection(ray){

            return Some( GameObject::BoardObject(objectid) );
        }

        return None;
    }
    

    pub fn objects_to_gameinput(&self, playerid: &u8, selected: &Option<GameObject>, clicked: &Option<GameObject>) -> Option<GameInput>{


        if let Some(GameObject::BoardObject(selectedid)) = selected {

            if let Some(GameObject::BoardObject(clickedid)) = clicked {

                if let Some(fullaction) = self.boardengine.clicked_to_fullaction( Some(*selectedid), Some(*clickedid) ){

                    return Some( GameInput::FullAction(fullaction.0, fullaction.1) );
                }
            }
        }

        if let Some(GameObject::Deck) = clicked{

            return Some( GameInput::Draw );
        }

        return None;
    }


    pub fn receive_input(&mut self, playerid: u8, input: GameInput) -> bool{

        if self.is_gameinput_valid(&playerid, &input){

            self.queuedinputs.insert( playerid, input );

            return true;
        }

        return false;
    }
    

    pub fn tick(&mut self) {

        //log::info!("jello");
        for player in self.turnmanager.get_current_players(){

            if let Some(queuedinput) = self.queuedinputs.remove(&player){

                self.perform_input( &player, &queuedinput );

                self.turnmanager.player_took_action(player);
            }

        }

        self.boardengine.tick();

        self.turnmanager.tick();


        {
            if let Some((tick, _)) = &mut self.lastcardeffect{
                *tick = *tick - 1;
            }
    
            if let Some((tick, _)) = self.lastcardeffect.clone(){
                if tick <= 0{
                    self.lastcardeffect = None;
                }
            }
        }


    }

    
    pub fn get_visible_game_state(&self, selected: &Option<GameObject>) -> VisibleGameState{


        let mut pieceid = None;
        if let Some(GameObject::BoardObject(id)) = selected{
            pieceid = Some(*id);
        }
        let boardobjects = self.boardengine.get_visible_board_game_objects( pieceid );


        let player1totalticksleft = self.turnmanager.get_players_total_ticks_left(1);
        let player2totalticksleft = self.turnmanager.get_players_total_ticks_left(2);
        

        let player1ticksleft = self.turnmanager.get_ticks_left_for_players_turn(1);
        let player2ticksleft = self.turnmanager.get_ticks_left_for_players_turn(2);



        let playerswithactiveturns = self.turnmanager.get_current_players();

        let mut lastcardeffect = None;

        if let Some((_, effect)) = &self.lastcardeffect{

            lastcardeffect = Some(effect.clone());
        }

        VisibleGameState{
            
            isgameover: self.is_game_over(),
            
            turnsuntildrawavailable: self.turnmanager.turns_until_draw(),
            
            player1totalticksleft,
            player2totalticksleft,
            
            player1ticksleft,
            player2ticksleft,
            
            playerswithactiveturns,
        
            gameeffects: gameeffect::get_card_effect_textures( self.get_effect_x() ),
            
            lastcardeffect,
            
            boardobjects,
        }
        
    }



    fn is_game_over(&self) -> Option<u8>{

        return None;
    }


    fn is_gameinput_valid(&self, playerid: &u8, input: &GameInput) -> bool{
        
        if let GameInput::FullAction(piece, action) = input{

            if self.boardengine.is_action_valid( &piece, &action){

                return true;
            }
        }

        if let GameInput::Draw = input{

            return self.turnmanager.can_player_draw(playerid);
        }


        false
    }



    fn perform_input(&mut self, player: &u8,  input: &GameInput){

        if let GameInput::FullAction(piece, action) = input{

            if self.boardengine.is_action_valid( &piece, &action){


                self.boardengine.perform_action(&piece, &action);
            }
        }

        if let GameInput::Draw = input{

            gameeffect::draw( self.get_mut_effect_x() );

            self.turnmanager.player_drew();
        }    

    }




    fn get_effect_x(& self) -> Vec<& dyn EffectTrait>{

        let mut toreturn: Vec<& dyn EffectTrait> = Vec::new();

        toreturn.push(  & self.turnmanager );
        toreturn.push(  & self.boardengine );

        toreturn
    }


    fn get_mut_effect_x(&mut self) -> Vec<& mut dyn EffectTrait>{

        let mut toreturn: Vec<&mut dyn EffectTrait> = Vec::new();

        toreturn.push(  &mut self.turnmanager );
        toreturn.push(  &mut self.boardengine );

        toreturn
    }
    
}
