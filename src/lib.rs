mod game;

pub use game::VisibleGameBoardObject;
pub use game::VisibleGameState;
pub use game::GameObject;
pub use game::GameInput;
use game::Game;

pub use rapier3d;

use rapier3d::na::Point3;
use rapier3d::na::Vector3;

pub use rapier3d::na as nalgebra;


pub struct PlayerInterface{

    playerid: u8,

    game: Game,
}

impl PlayerInterface{

    pub fn new() -> PlayerInterface{

        use log::Level;
        use log::info;
        
        console_log::init_with_level(Level::Debug);

        PlayerInterface{

            playerid: 1,

            game: Game::new(),

        }

    }


    //draw when i dont have a way to know when im clicking on a deck
    pub fn draw(&mut self) -> Vec<u8>{

        self.game.receive_input(self.playerid, GameInput::Draw);

        return bincode::serialize( &GameInput::Draw ).unwrap();
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
            
            self.game.receive_input(player, input);
        }
    }

    pub fn receive_input(&mut self, player: u8, gameinput: GameInput){

        self.game.receive_input(player, gameinput);
    }

    pub fn tick(&mut self){

        self.game.tick();
    }
}






