

//was the gui clicked on?


use kiss3d::window::{Window};


use kiss3d::scene::SceneNode;
use kiss3d::scene::PlanarSceneNode;

use chessengine::nalgebra;

use nalgebra::Translation2;
use nalgebra::Vector2;


pub struct EffectCard{

    node: SceneNode,

    texture: String,

}



pub struct GuiObject{

    //the planar scene node if this has a texture
    texturenode: PlanarSceneNode,

    //the location of the texture
    texture: String,

    //the text is in the middle of the borders
    text: String,

    topleft: (u32, u32),

    size: (u32, u32),

}


//to a translation
fn topleft_to_center(topleftpos: (u32, u32), size: (u32,u32) , windowsize: & (u32, u32)) -> (f32,f32) {

    let mut topleft = ( -(windowsize.0 as f32 / 2.0), windowsize.1 as f32 / 2.0);

    topleft.0 += size.0 as f32 / 2.0;
    topleft.1 += - (size.1 as f32 / 2.0);


    topleft.0 += topleftpos.0 as f32;
    topleft.1 += -(topleftpos.1 as f32);


    return topleft;
}


impl GuiObject{

    //given a percentage of the screen
    //topleft is in percentage points


    //specify top left as a percentage
    //specify top left, top right, 



    //relative to the top left of the screen

    //percentage from the top



    //how far from the left
    //what percentage from the top

    //the height as a percentage
    //the width as a ratio of height

    

    pub fn new_align_left(topleftpercentage: (f32, f32), heightpercentage: f32, widthratio: f32 , windowsize: & (u32, u32), window: &mut Window) -> GuiObject{

        let widthratio = 1. / widthratio;

        let windowratio = (windowsize.0 as f32) / (windowsize.1 as f32);

        let sizepercentage = (heightpercentage / (widthratio * windowratio) , heightpercentage);

        return GuiObject::new(topleftpercentage, sizepercentage, windowsize, window);
    }





    //the text will be in the middle of the top left and top right
    pub fn new( topleftpercentage: (f32, f32), sizepercentage: (f32,f32) , windowsize: & (u32, u32), window: &mut Window) -> GuiObject{

        let topleft = ((( topleftpercentage.0 / 100.0 ) * windowsize.0 as f32) as u32, (( topleftpercentage.1 / 100.0 ) * windowsize.1 as f32) as u32);
        let size = ( ((sizepercentage.0 / 100.0) * windowsize.0 as f32) as u32, ((sizepercentage.1 / 100.0) * windowsize.1 as f32) as u32 );



        let mut texturenode = window.add_rectangle( size.0 as f32, size.1 as f32);

        let center = topleft_to_center( topleft, size, windowsize);

        texturenode.append_translation(  &Translation2::from( Vector2::new( center.0, center.1) )    );

        GuiObject{
            texturenode,
            
            texture: "white.png".to_string(),
            text: "".to_string(),

            topleft,
            size,
        }

    }

    pub fn get_texture(&self) -> String{

        return self.texture.clone();
    }
    pub fn set_texture(&mut self, texture: String){

        self.texture = texture;
    }

    pub fn get_node(&mut self) -> &mut PlanarSceneNode{

        return &mut self.texturenode;
    }


    pub fn set_visibility(&mut self, is: bool){

        self.texturenode.set_visible(is);
    }


    pub fn is_clicked(&self, point: (u32,u32) ) -> bool{

        let botright = (self.topleft.0 + self.size.0 , self.topleft.1 + self.size.1); 

        if point.0 > self.topleft.0{
            if point.1 > self.topleft.1{
                if point.0 < botright.0{
                    if point.1 < botright.1{

                        return true;
                    }
                }
            }
        }

        return false;
    }




    pub fn set_text(&mut self, text: String){
        self.text = text;
    }


    pub fn render_text(&self, window: &mut Window){

        use kiss3d::text::Font;
        let font = Font::default();


        use nalgebra::geometry::Point2;
        use nalgebra::geometry::Point3;

        let pos = Point2::new( self.topleft.0 as f32 *2., self.topleft.1 as f32 *2.);

        let color = Point3::new( 0., 0., 0. );


        window.draw_text(
            &self.text,
            &pos,
            self.size.1 as f32 * 1.5,
            &font,
            &color
        );

    }

}





/*

        use kiss3d::text::Font;
        let font = Font::default();


        if let Some(turns) = self.gui.turnsuntildraw{

            let text;

            if turns == 0{

                text = "DRAW".to_string();
                window.draw_text(
                    &text,
                    & Point2::new(  75. ,  400.  )  ,
                    120.0,
                    &font,
                    &Point3::new(0.0, 0.5, 0.0),
                );
            }
            else{

                text = turns.to_string();
                window.draw_text(
                    &text,
                    & Point2::new(  150. , 350. )  ,
                    240.0,
                    &font,
                    &Point3::new(0.0, 0.0, 0.0),
                );
            }

        }

        


        let mut player1ypos = self.windowsize.1 as f32  * 1.;
        let mut player2ypos = 0.0;

        if self.playerid == 2{
            player1ypos = 0.0;
            player2ypos = self.windowsize.1 as f32  * 1.;
        }


        //get the requested top left position
        //for text or an image
        //and return the point to use to acheive that



        window.draw_text(
            &self.gui.player1totaltimeleft,
            & Point2::new(  0. , player1ypos  )  ,
            120.0,
            &font,
            &Point3::new(0.0, 0.0, 0.0),
        );

        window.draw_text(
            &self.gui.player2totaltimeleft,
            & Point2::new(  0. , player2ypos  )  ,
            120.0,
            &font,
            &Point3::new(0.0, 0.0, 0.0),
        );


        ConsoleService::log( &format!("{:?}", self.windowsize) );


        if let Some(x) = &self.gui.player1turntimeleft{

            window.draw_text(
                x,
                &Point2::new(  400. , player1ypos  ),
                120.0,
                &font,
                &Point3::new(0.0, 0.0, 0.0),
            );
        }


        if let Some(x) = &self.gui.player2turntimeleft{

            window.draw_text(
                x,
                &Point2::new(  400. , player2ypos  ),
                120.0,
                &font,
                &Point3::new(0.0, 0.0, 0.0),
            );
        }

*/
