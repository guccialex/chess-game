
mod boardengine;
use boardengine::BoardEngine;
//use boardengine::FullAction;
pub use board::VisibleGameBoardObject;

mod turnmanager;
use turnmanager::TurnManager;

mod gameinput;
pub use gameinput::GameInput;

mod gameobject;
pub use gameobject::GameObject;

mod visiblegamestate;
pub use visiblegamestate::VisibleGameState;


mod cards;
use cards::Cards;
use cards::CardEffect;
use cards::EffectTrait;

use rapier3d::na::Point3;
use rapier3d::na::Vector3;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};



//raw data structs

//turn into values


#[derive(Serialize, Deserialize)]
pub struct Game{

    boardengine: BoardEngine,

    turnmanager: TurnManager,


    gameover: Option<u8>,

    queuedinputs: HashMap<u8, GameInput>,


    cards: Cards,

    //how many ticks ago was the last effect drawn
    lastcardeffect: i32,

    //ticks to wait before applying a new effect
    tickstotryaction: i32,
}


impl Game{

    pub fn new()  -> Game{

        let mut toreturn = Game{

            boardengine: BoardEngine::new(),
            turnmanager: TurnManager::new_two_player(1, 2, 5000, 2),
            gameover: None,
            queuedinputs: HashMap::new(),

            cards: Cards::new(),
            lastcardeffect: 0,

            tickstotryaction: 10,
        };


        //toreturn.perform_card_effect( CardEffect::TurnsTimed(10) );
        


        toreturn
    }


    pub fn automatically_set_player_actions(&mut self, playerid: u8){

        let fullaction = self.boardengine.get_players_ideal_action(playerid);

        let input = GameInput::FullAction( fullaction.0, fullaction.1 );

        self.receive_input(playerid, input);

    }






    //get the boardobject targeted
    //or get the game object?
    pub fn get_gameobject_targeted(&self, ray: (Point3<f32>, Vector3<f32>)) -> Option<GameObject>{

        if let Some( objectid) = self.boardengine.get_object_intersection(ray){

            return Some( GameObject::BoardObject(objectid) );
        }

        return None;
    }
    

    pub fn objects_to_gameinput(&self, playerid: &u8, selected: &Option<GameObject>, clicked: &Option<GameObject>) -> Option<GameInput>{


        if let Some(GameObject::BoardObject(selected)) = selected {

            if let Some(GameObject::BoardObject(clicked)) = clicked {

                if let Some(fullaction) = self.boardengine.clicked_to_fullaction( Some(selected.clone()), Some(clicked.clone()) ){

                    return Some( GameInput::FullAction(fullaction.0, fullaction.1) );
                }
            }
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

        if self.tickstotryaction <= 0{

            for player in self.turnmanager.get_current_players(){

                //if there is an input queued
                if let Some(queuedinput) = self.queuedinputs.get(&player){

                    if self.is_gameinput_valid(&player, &queuedinput){

                        if let Some(queuedinput) = self.queuedinputs.remove(&player){ 

                            self.perform_input( &player, &queuedinput );
            
                            self.turnmanager.player_took_action(player);
        
                            self.tickstotryaction = 20;
            
                            break;
                        }

                    }

                }


            }

        }


        self.tickstotryaction += -1;


        self.boardengine.tick();


        self.turnmanager.tick();
    
        self.lastcardeffect += 1;

    }

    
    pub fn get_visible_game_state(&self, selected: &Option<GameObject>) -> VisibleGameState{


        let mut temp = None;
        if let Some(GameObject::BoardObject(x)) = selected{
            temp = Some(x.clone());
        }
        let boardobjects = self.boardengine.get_visible_board_game_objects( &temp );


        let player1totalticksleft = self.turnmanager.get_players_total_ticks_left(1);
        let player2totalticksleft = self.turnmanager.get_players_total_ticks_left(2);
        

        let player1ticksleft = self.turnmanager.get_ticks_left_for_players_turn(1);
        let player2ticksleft = self.turnmanager.get_ticks_left_for_players_turn(2);



        let playerswithactiveturns = self.turnmanager.get_current_players();

        let mut lastcardeffect = None;

        if self.lastcardeffect < 10{

            lastcardeffect = self.cards.get_last_effect_texture();
        }

        VisibleGameState{
            
            isgameover: self.is_game_over(),
            
            player1totalticksleft,
            player2totalticksleft,
            
            player1ticksleft,
            player2ticksleft,

            piles: self.cards.get_card_pile_textures(),
            
            playerswithactiveturns,
        
            gameeffects: Cards::get_active_card_effect_textures( self.get_effect_x() ),
            
            lastcardeffect,
            
            boardobjects,
        }
        
    }


    fn is_game_over(&self) -> Option<u8>{

        return None;
    }


    fn is_gameinput_valid(&self, playerid: &u8, input: &GameInput) -> bool{
        
        if let GameInput::FullAction(piece, action) = input{

            if self.boardengine.is_action_valid( playerid, &piece, &action){

                return true;
            }
        }

        if let GameInput::Draw(_) = input{
            return self.turnmanager.can_player_draw(playerid);
        }


        false
    }



    fn perform_input(&mut self, player: &u8,  input: &GameInput){

        if self.is_gameinput_valid(player, input){

            if let GameInput::FullAction(piece, action) = input{

                self.boardengine.perform_action(&piece, &action);
            }
            if let GameInput::Draw(pile) = input{
    
                let mut temp = self.cards.clone();
                temp.draw_card_from_pile(pile, self.get_mut_effect_x() );
                self.cards = temp;
    
                self.turnmanager.player_drew();

                self.lastcardeffect = 0;
            }
        }

    }

    fn perform_card_effect(&mut self, effect: CardEffect){

        let mut temp = self.cards.clone();
        temp.set_card_effect( effect , self.get_mut_effect_x() );
        self.cards = temp;
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


