use crate::RelativeSquare;
use crate::SquarePos;
use crate::BoardObject;
use crate::Piece;
use crate::Square;
use crate::FullAction;
use serde::{Serialize, Deserialize};




#[derive(PartialEq,  Clone, Debug, Serialize, Deserialize)]
pub enum PhysicalEffect{

    //the position, how many ticks to accomplish it in
    Slide( RelativeSquare , u32 ),

    //direction as radians
    //the force
    Flick( f32, f32 ),

    //the distance, how many ticks to accomplish it in
    LiftAndMove(  RelativeSquare , u32 ),

    //long drop for this many ticks
    LongDrop(u32),

    Drop

}



#[derive(PartialEq,  Clone, Debug)]
pub struct PhysicalEffects{

    pub selfeffect: PhysicalEffect,

    //how many ticks in the future, the square, the effect
    pub squareeffects: Vec<(u32, RelativeSquare, PhysicalEffect)>,

}


impl PhysicalEffects{


    //the full action
    //the speed (how many ticks it takes to move one square)
    //if the action should be a flick instead
    pub fn from_fullaction(fullaction: &FullAction, speed: &u32, isflicked: &bool ) -> PhysicalEffects{

        let fullaction = &mut fullaction.clone();
        
        if *isflicked{
            fullaction.into_flick();
        }


        match fullaction{

            FullAction::Flick(dir, force) => {

                let selfeffect = PhysicalEffect::Flick(*dir, *force);

                PhysicalEffects{
                    selfeffect,
                    squareeffects: Vec::new(),
                }

            },
            _ =>{




                let dest = fullaction.destination().unwrap();

                let pdest = dest.to_relative_float();
                let destdist = pdest.0 * pdest.0 + pdest.1 * pdest.1;
                let destdist = destdist.sqrt() as u32;
                let ticks = destdist * speed;
                let ticks = 25.max(ticks);
        

                let selfeffect;
                
                if fullaction.is_lift(){
                    selfeffect = PhysicalEffect::LiftAndMove( dest, ticks );
                }
                else{
                    selfeffect = PhysicalEffect::Slide( dest, ticks );
                }

        
                let droptick = ticks.saturating_sub(25);
                let cap = fullaction.captures().unwrap();
        
                let mut squareeffects: Vec<(u32, RelativeSquare, PhysicalEffect)> = Vec::new();
                squareeffects.push( (droptick , cap.clone(), PhysicalEffect::Drop ) );
        
        
                PhysicalEffects{
                    selfeffect,
                    squareeffects,
                }
            },
        }

    }

}
