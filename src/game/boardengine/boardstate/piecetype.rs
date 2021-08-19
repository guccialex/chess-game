use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;


use super::fullaction::FullAction;
use super::relativesquare::RelativeSquare;



fn dir_from_pers(objectdirection: f32, playerdirection: f32) -> f32{

    return objectdirection + playerdirection;
}





#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceType{

    Nothing,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Checker
}

impl PieceType{

    pub fn value(&self) -> u8{

        match self{
            PieceType::Nothing => 0,
            PieceType::Pawn => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Rook => 4,
            PieceType::Queen => 5,
            PieceType::King => 12,
            PieceType::Checker => 2,
        }
    }

    pub fn get_actions(&self, ownerdirection: &f32, hasmoved: &bool) -> Vec<FullAction>{

        let mut toreturn: Vec<FullAction> = Vec::new();

        match *self{
            
            PieceType::Pawn =>{

                toreturn.push( 
                    FullAction::new_cant_capture_slide(&dir_from_pers(0.0, *ownerdirection), &1)
                );
                
                if hasmoved == &false{
                    toreturn.push( 
                        FullAction::new_cant_capture_slide(&dir_from_pers(0.0, *ownerdirection), &2)
                    );
                }
                
                toreturn.push( 
                    FullAction::new_must_capture_slide(&dir_from_pers(0.875, *ownerdirection), &1)
                );

                toreturn.push( 
                    FullAction::new_must_capture_slide(&dir_from_pers(0.125, *ownerdirection), &1)
                );

                return toreturn;
            },
            PieceType::Knight =>{
                
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (1,2), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (2,1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (2,-1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (1,-2), *ownerdirection ).unwrap()
                    )
                );


                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-1,-2), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-2,-1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-2,1), *ownerdirection ).unwrap()
                    )
                );
                toreturn.push(
                    FullAction::new_lift_and_move(
                        &RelativeSquare::new_from_perspective( (-1,2), *ownerdirection ).unwrap()
                    )
                );
                
                return toreturn;
            },
            PieceType::Bishop =>{
                
                for dir in (1..8).step_by(2){
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &dist)
                        );
                    }
                }
                
                return toreturn;
            },
            PieceType::Rook => {
                
                for dir in (0..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                    
                    for dist in 1..8{
                        
                        toreturn.push(
                            FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &dist)
                        );
                    }
                }
                
                return toreturn;
            },
            PieceType::Queen => {
                
                for dir in 0..8{
                    
                    let rotation = dir as f32 / 8.0;

                    for dist in 1..8{
                        toreturn.push(
                            FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &dist)
                        );
                    }
                }
                
                return toreturn;
            },
            PieceType::King => {
                for dir in 0..8{
                    let rotation = dir as f32 / 8.0;

                    toreturn.push(
                        FullAction::new_slide(&dir_from_pers(rotation, *ownerdirection), &1)
                    );
                }
                
                return toreturn;
            },
            PieceType::Checker => {
                
                for dir in (1..8).step_by(2){

                    let rotation = dir as f32 / 8.0;
                
                    toreturn.push(

                        FullAction::new_checkers_capture( &dir_from_pers(rotation, *ownerdirection) )
                    );

                    toreturn.push(

                        FullAction::new_cant_capture_slide( &dir_from_pers(rotation, *ownerdirection), &1)
                    );
                }
                
                return toreturn;
            },
            PieceType::Nothing => return toreturn,  
        };
    }

    //the name of the file that represents this objects image
    pub fn image_file(&self) -> String{

        match self{
            PieceType::Nothing => format!("none.png"),
            PieceType::Pawn => format!("pawn.png"),
            PieceType::Knight => format!("knight.png"),
            PieceType::Bishop => format!("bishop.png"),
            PieceType::Rook => format!("rook.png"),
            PieceType::Queen => format!("queen.png"),
            PieceType::King => format!("king.png"),
            PieceType::Checker => format!("checker.png"),
        }
    }
}

