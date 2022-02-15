use yew::prelude::*;



use std::f32::consts::FRAC_1_PI;
use std::sync::Arc;
use std::sync::Mutex;

use std::time::Duration;



const FRAMEINTERVALMS: u64 = 33;
pub enum Msg{

    Tick,
}


pub struct Finder{

}


impl Component for Finder{

    type Message = Msg;
    type Properties = ();


    fn create( ctx: &Context<Self> ) -> Self{


        
        ctx.link().send_future(

            async move {

                std::thread::sleep( Duration::from_millis( FRAMEINTERVALMS ) );

                return Msg::Tick;
            }
        );


        let toreturn = Self{
        };

        toreturn
    }

    fn update(&mut self, ctx: &Context<Self>,  msg: Self::Message) -> bool{

        match msg{
        
            Msg::Tick => {
                
                
                ctx.link().send_future(

                    async move {
        
                        std::thread::sleep( Duration::from_millis( FRAMEINTERVALMS ) );
        
                        return Msg::Tick;
                    }
                );
        
            }


        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html{


        html! {
            <div class="container-xl">

                <br/>
                {"If you change the zoom of your browser from 100% it will break"}
                <br/>
                {"This works on mobile, you just have to turn your device thing sideways right now, before you join the game"}
                <br/>
                {"Drag the screen to change the position of the camera (not available on mobile)"}
                <br/>
                {"Scroll to zoom in and out (also not on mobile)"}
                <br/>
                <br/>
                <br/>


            
                <a class="btn btn-success" href="../chesscheckersgame"> {"join single player game"} </a>

            </div>
        }

    }

}


