
use serde::{Serialize, Deserialize};



pub struct BoardInput{

    pub selected: Option<u16>,

    pub clicked: u16,
}

impl BoardInput{

    pub fn new(selected: Option<u16>, clicked: u16) -> BoardInput{

        BoardInput{
            selected,
            clicked
        }
    }

    pub fn to_gameinput(&self) -> GameInput{

        let selected;

        if let Some(sel) = self.selected{

            selected = Some( GameObject::BoardObject(sel) );
        }
        else{
            selected = None;
        }

        GameInput{

            selected,

            clicked: GameObject::BoardObject(self.clicked),
        }
        
    }

}





//things which can be clicked on
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GameObject{

    BoardObject(u16),

    Deck,
}

impl GameObject{


    fn to_boardobject(&self) -> Option<u16>{

        if let Self::BoardObject(boardobject) = self{
            return Some(boardobject.clone());
        }
        else{
            return None;
        }
    }

}






#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameInput{
    
    //an action is given by optionally the selected object
    //plus the newly clicked object

    pub selected: Option<GameObject>,

    pub clicked: GameObject,
}

impl GameInput{

    pub fn new(selected: Option<GameObject>, clicked: GameObject) -> GameInput{

        GameInput{
            selected,
            clicked,
        }
    }


    pub fn to_boardinput(&self) -> Option<BoardInput>{

        //if the clicked object is a boardobject
        if let GameObject::BoardObject(clicked) = self.clicked{

            //if the selected object is none or a some
            if let Some(GameObject::BoardObject(selected)) = self.selected{

                return Some(BoardInput::new( Some(selected), clicked));
            }

            if self.selected.is_none(){

                return Some( BoardInput::new( None, clicked ));
            }
        }

        return None;
    }

}

