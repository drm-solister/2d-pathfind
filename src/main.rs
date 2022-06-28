#![allow(dead_code, unused_imports)]
use ggez;
use ggez::graphics::{self, Mesh, Color, DrawParam, Image, spritebatch::SpriteBatch};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

// use glam::*;
use mint::*;

mod img_tile;
mod map;

fn main() {

    let dimensions: (u16, u16) = (80, 40);
    let tile_size = 20.0;

    let width = dimensions.0 as f32 * tile_size;
    let height = dimensions.1 as f32 * tile_size;

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("2d test", "rj")
        .window_setup(
            ggez::conf::WindowSetup {
                title: "testing !".to_owned(),
                vsync: false,
                ..ggez::conf::WindowSetup::default()
            }
        )
        .window_mode(
            ggez::conf::WindowMode {
                width,
                height,
                ..ggez::conf::WindowMode::default()
            }
        )
        .build()
        .expect("could not create ggez context");
    

    let my_game = MyGame::new(&mut ctx, dimensions, tile_size);

    event::run(ctx, event_loop, my_game);
}

pub struct MyGame {
    pub dimensions: (u16, u16),
    pub tile_size: f32,
    pub tile_states: Vec<TileState>,
    pub spritesheet: SpriteSheet,
}

impl MyGame {
    pub fn new(_ctx: &mut Context, dimensions: (u16, u16), tile_size: f32) -> MyGame {

        // let dimensions: (u16, u16) = (100, 100);
        // let tile_size = 10.0;

        let mut tile_states = vec![TileState::Empty(false); (dimensions.0 as u32 *dimensions.1 as u32) as usize];
        // for i in 0..dimensions.0*dimensions.1 {
        //     if i%3 == 0 {
        //         tile_states[i as usize] = TileState::Full(false);
        //     }
        // }

        MyGame {
            dimensions,
            tile_size,
            tile_states,
            spritesheet: SpriteSheet {
                image: img_tile::new(_ctx, tile_size as u16),
                empty: (0.0, 0.5),
                full: (0.0, 0.0),
            },
        }

    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mouse_pos = ggez::input::mouse::position(ctx);
        // let pressed = ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left);

        let origin = Point2{x: 10.0, y: 10.0};
        // map::draw_line(ctx, origin, mouse_pos);

        map::find_visible_on_line(self, origin, mouse_pos);

        // if ggez::timer::ticks(&_ctx) % 50 == 0 {
        //     println!("fps: {:?}", ggez::timer::fps(&_ctx));
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        // this creates a spritebatch which sources everything from a single image, which is generated in img_tile.rs
        // based on the TileState that corresponds which each tile, the tile will be hollow or full and have whatever
        // color is desired. (empty tiles are hollow, obstructions are filled)

        graphics::clear(ctx, Color::WHITE);

        let mut batch = SpriteBatch::new(self.spritesheet.image.clone());

        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                
                let mut param = DrawParam::default().dest( mint::Point2{ x: i as f32*(self.tile_size+0.0), y: j as f32*(self.tile_size+0.0)} );

                let mut alpha = 255;
                match self.tile_states[(i+self.dimensions.0*j) as usize] {
                    TileState::Full(known) => {
                        if !known { alpha = 125 }
                        param = param
                            .src( graphics::Rect{ x: 0.0, y:0.0, w:1.0, h:0.5} )
                            .color(Color::from_rgba(9,17,51,alpha));
                    },
                    TileState::Empty(known) => {
                        if !known { alpha = 125 }
                        param = param
                            .src( graphics::Rect{ x: 0.0, y:0.5, w:1.0, h:0.5} )
                            .color(Color::from_rgba(124,174,195,alpha));
                    },
                    _ => (),
                }

                batch.add(param);
            }
        }
        graphics::draw(ctx, &batch, DrawParam::default())?;


        // for debugging, this code draws a line

        let mouse_pos = ggez::input::mouse::position(ctx);
        let origin = Point2{x: 10.0, y: 10.0};
        map::draw_line(ctx, origin, mouse_pos);
        //map::distance_from_line(origin, mouse_pos, Point2{x: 0.0, y: 0.0});
        
        graphics::present(ctx)

    }
}

// add more for different states
#[derive(Clone, Copy, Debug)]
pub enum TileState {
    Empty(bool),
    Full(bool),
}

// when i make a sprite sheet this struct should have one image and several other fields describing where the bounds of each
// seperate sprite/tile idk the term is located
pub struct SpriteSheet {
    image: Image,
    empty: (f32, f32),
    full: (f32, f32),
}


// attempts to make this fast:
// first:
// create only one mesh and draw it in a different place every time

// second:
// create iamges and create a sprite batch to draw once a frame

// third: (not yet explored, may not be needed)
// meshbuilder?


// current sprite batch approach might have to cut it, at least for now.

// todo
// make the screen size fit the tile map
// add the actor code back in 
// get back on finding out how to draw lines on a discrete grid
// add back in clicking to add full tiles
// make sprite for actor and target
// refactor everything to make it easier to read
// put text on the screen for debugging
// test performance on a better computer

// thigs break with some dimensions

// could still only update the tiles i need to update and speed up the drawing massively
// an easy thing to do would be to only update tiles in a radius around the actor

// i think its bad to be exporting things from main to sub modules. the data needed by the sub module should be able to be passed to 
// it or contained in it from the start