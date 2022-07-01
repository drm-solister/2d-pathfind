use ggez::graphics::{self, Image, Color};
use ggez::Context;

pub fn new(ctx: &mut Context, length: u16) -> Image {
    
    let mut solid = vec![0; (length*length*4) as usize];

    for i in (0..solid.len()).step_by(4) {
        solid[i] = 255;    // red
        solid[i+1] = 255;  // green
        solid[i+2] = 255;  // blue
        solid[i+3] = 255;  // alpha
    }

    let mut hollow = vec![255; (length*length*4) as usize];
    let stroke = 3;

    if length < stroke {
        panic!("the width of a tile is less than the stroke that would draw it")
    }

    for i in (0..hollow.len()).step_by(4) {
        let index = i/4;
        let x = index%length as usize;
        let y = index/length as usize;

        if x >= (stroke).into() && x < (length-stroke).into() {
            if y >= (stroke).into() && y < (length-stroke).into() {
                hollow[i+3] = 0;  // alpha
            }
        }

    }

    let mut goal = vec![0; (length*length*4) as usize];

    for i in (0..goal.len()).step_by(4) {
        goal[i] = 239;    // red
        goal[i+1] = 187;  // green
        goal[i+2] = 33;  // blue
        goal[i+3] = 255;  // alpha
    }

    let tilesheet = [solid, hollow, goal].concat();


    Image::from_rgba8(ctx, length as u16, (length*3) as u16, &tilesheet[..]).expect("could not create image")
}