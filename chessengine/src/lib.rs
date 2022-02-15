mod game;

pub use game::VisibleGameBoardObject;
pub use game::VisibleGameState;
pub use game::GameObject;
pub use game::GameInput;
use game::Game;

pub use board::BoardObject;

pub use board::rapier3d;

use rapier3d::na::Point3;
use rapier3d::na::Vector3;

pub use rapier3d::na as nalgebra;

pub use rapier3d::geometry::Shape;
pub use rapier3d::geometry::TypedShape;


pub struct PlayerInterface{

    playerid: u8,

    game: Game,
}

impl PlayerInterface{

    pub fn new(playerid: u8) -> PlayerInterface{


        PlayerInterface{

            playerid,

            game: Game::new(),

        }

    }


    
    //the non player takes actions
    pub fn opponent_takes_action(&mut self){

        let opponent;

        if self.playerid == 1{
            opponent = 2;
        }
        else{
            opponent =1;
        }

        self.game.automatically_set_player_actions( opponent );
        
    }
    


    pub fn get_id(&self) -> u8{
        return self.playerid;
    }

    //draw when i dont have a way to know when im clicking on a deck
    pub fn draw(&mut self, pile: u16) -> Vec<u8>{

        self.game.receive_input(self.playerid, GameInput::Draw(pile) );

        return bincode::serialize( &GameInput::Draw(pile) ).unwrap();
    }

    pub fn click(&mut self, selected: Option<GameObject>, ray: (Point3<f32>, Vector3<f32>)) -> (Option<GameObject>, Option<Vec<u8>>){

        let clicked = self.game.get_gameobject_targeted(ray);

        if let Some(gameinput) = self.game.objects_to_gameinput(&self.playerid, &selected, &clicked){

            if self.game.receive_input(self.playerid, gameinput.clone()){

                return ( None, Some( bincode::serialize(&gameinput).unwrap() ) );
            }
        }

        return (clicked, None);
    }


    //pass what object is selected, and what object is clicked
    //return what object should now be selected
    pub fn clicked_object(&mut self, selected: Option<GameObject>, clicked: Option<GameObject>) -> Option<GameObject>{


        if let Some(gameinput) = self.game.objects_to_gameinput(&self.playerid, &selected, &clicked){

            if self.game.receive_input(self.playerid, gameinput.clone()){

                return None;
            }
        }

        return clicked;
    }


    pub fn set_game_string_state(&mut self, state: Vec<u8>) {

        if let Ok(game) = bincode::deserialize(&state){
            self.game = game;
        }
    }


    pub fn tick(&mut self){
        self.game.tick();
    } 


    pub fn get_visible_game_state(&self, selected: &Option<GameObject>) -> VisibleGameState{

        self.game.get_visible_game_state( selected )
    }

}




pub struct ServerInterface{

    game: Game,
}

impl ServerInterface{


    pub fn new() -> ServerInterface{


        ServerInterface{
            game: Game::new(),
        }
    }



    pub fn get_game_string_state(&self) -> Vec<u8>{

        bincode::serialize(&self.game).unwrap()
    }

    pub fn receive_bin_input(&mut self, player: u8, gameinput: Vec<u8>){

        if let Ok(input) = bincode::deserialize( &gameinput){

            let isvalid = self.game.receive_input(player, input);
            
        }
    }

    pub fn receive_input(&mut self, player: u8, gameinput: GameInput){

        self.game.receive_input(player, gameinput);
    }

    pub fn tick(&mut self){

        self.game.tick();
    }
}









