use specs::{System, ReadStorage, WriteStorage, Join as _};
use macroquad::window::{};

use crate::comp::{Pos, Vel};

pub(crate) struct BordersSys(pub f32, pub f32);

impl<'a> System<'a> for BordersSys {
    // These are the resources required for execution.
    type SystemData = (WriteStorage<'a, Pos>);

    fn run(&mut self, (mut pos_storage): Self::SystemData) {
        for pos in pos_storage.as_mut_slice(){
            if pos.0.x < 0.0{
                pos.0.x = self.1; 
            }else if pos.0.x > self.1{
                pos.0.x = 0.0;
            }else if pos.0.y > self.0{
                pos.0.y = 0.0;
            }else if pos.0.x < 0.0{
                pos.0.x = self.0;
            }else {}
        }
    }
}