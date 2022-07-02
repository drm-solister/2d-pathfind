#![allow(dead_code, unused_imports)]
use ggez;
use ggez::graphics::{self, Mesh, Color, DrawParam, Image, spritebatch::SpriteBatch, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;

// use glam::*;
use mint::*;

mod img_tile;
mod map;
mod actor;
mod save_manager;
use save_manager::SaveManager;

fn main() {

    let map_to_load: Option<&str> = Some("saves_work!");

    let dimensions: (u16, u16) = (60, 50);
    let tile_size = 10.0;

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
                height: height+40.0,
                ..ggez::conf::WindowMode::default()
            }
        )
        .build()
        .expect("could not create ggez context");
    

    let my_game = MyGame::new(&mut ctx, dimensions, tile_size, map_to_load);

    event::run(ctx, event_loop, my_game);
}

pub struct MyGame {
    pub map: map::Map,
    pub spritesheet: SpriteSheet,
    pub actor: actor::Actor,
    clicked_on: Option<map::TileState>,
    save_manager: SaveManager,
}

impl MyGame {
    pub fn new(ctx: &mut Context, dimensions: (u16, u16), tile_size: f32, map_to_load: Option<&str>) -> MyGame {

        // values that one change whether or not a map is being loaded
        let spritesheet = SpriteSheet {
            image: img_tile::new(ctx, tile_size as u16),
            // described the location of each tile on the spritesheet
            full: Rect::new(0.0, 0.0, 1.0, 0.33333),
            empty: Rect::new(0.0, 0.3333, 1.0, 0.3333),
            goal: Rect::new(0.0, 0.6666, 1.0, 0.3333)
        };

        if let Some(file_name) = map_to_load {
            let save_file = save_manager::load_map(file_name).unwrap();

            let goal_pos = match save_file.goal_position {
                None => None,
                Some(position) => {
                    Some(Point2 {x: position.0, y: position.1})
                }
            };

            MyGame {
                map: map::Map {
                    dimensions: save_file.dimensions,
                    tile_size: save_file.tile_size,
                    tile_states: save_file.tile_states,
                    scoreboard_txt: None,
                    goal: map::Goal { pos: goal_pos},
                },
                spritesheet,
                actor: actor::Actor::new(ctx, Point2{x: save_file.starting_point.0, y: save_file.starting_point.1}, 60.0, 10, 5.0, true),
                clicked_on: None,
                save_manager: SaveManager::new(),
            }

        } else {
            // return a map with some defaults
            MyGame {
                map: map::Map::new(dimensions, tile_size),
                spritesheet,
                // context, location, view radius, view rays, size, collision
                actor: actor::Actor::new(ctx, Point2{x: 10.0, y: 10.0}, 60.0, 10, 5.0, true),
                clicked_on: None,
                save_manager: SaveManager::new(),
            }
        }

    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        
        let pressed = ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left);
        if pressed {
            let mouse_pos = ggez::input::mouse::position(ctx);
            let clicked_index = self.map.index_from_mouse(mouse_pos);

            // click and drag to change one type of tile at a time
            if let None = self.clicked_on {
                self.clicked_on = Some(self.map.tile_states[clicked_index]);
            }

            match self.map.tile_states[clicked_index] {
                map::TileState::Full(_) => {
                    if let Some(map::TileState::Full(_)) = self.clicked_on {
                        self.map.tile_states[clicked_index] = map::TileState::Empty(false)
                    }
                },
                map::TileState::Empty(_) => {
                    if let Some(map::TileState::Empty(_)) = self.clicked_on {
                        self.map.tile_states[clicked_index] = map::TileState::Full(false)
                    }
                },
                _ => ()
            }

            //self.map.tile_states[ clicked_index ] = map::TileState::Full(false);
        } else {
            self.clicked_on = None;
        }

        //let origin = Point2{x: 10.0, y: 10.0};
        // map::draw_line(ctx, origin, mouse_pos);

        // worse
        //map::find_visible_on_line(self, origin, mouse_pos);

        // better - highlights all the pixels that the line crosses from origin to mouse
        //map::Map::dda(self, origin, mouse_pos);

        // --- handle inputs ---
        let mut v_x = 0.0;
        let mut v_y = 0.0;
        let keys_pressed = ggez::input::keyboard::pressed_keys(ctx);

        for key in keys_pressed.iter() {
            match key {
                KeyCode::W => v_y -= 1.0,
                KeyCode::A => v_x -= 1.0,
                KeyCode::S => v_y += 1.0,
                KeyCode::D => v_x += 1.0,
                KeyCode::G => self.map.set_goal(ggez::input::mouse::position(ctx)),
                KeyCode::M => {
                    self.save_manager.save_map(&mut self.map, &mut self.actor);
                    println!("saving map");
                },
                _ => (),
            }
        }

        self.actor.update_pos(ctx, Vector2{x: v_x, y: v_y}, &mut self.map);
        // it would maybe make more sense to put this in actor but then again... it works as is
        self.map.look_around(&mut self.actor);

        // --- fps counter ---
        // if ggez::timer::ticks(&ctx) % 100 == 0 {
        //     println!("fps: {:?}", ggez::timer::fps(&ctx));
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, Color::WHITE);

        // this creates a spritebatch which sources everything from a single image, which is generated in img_tile.rs
        // based on the TileState that corresponds which each tile, the tile will be hollow or full and have whatever
        // color is desired. (empty tiles are hollow, obstructions are filled)

        let mut batch = SpriteBatch::new(self.spritesheet.image.clone());

        for i in 0..self.map.dimensions.0 {
            for j in 0..self.map.dimensions.1 {
                
                let mut param = DrawParam::default().dest( mint::Point2{ x: i as f32*(self.map.tile_size+0.0), y: j as f32*(self.map.tile_size+0.0)} );

                let mut alpha = 255;
                match self.map.tile_states[(i+self.map.dimensions.0*j) as usize] {
                    map::TileState::Full(known) => {
                        if !known { alpha = 125 }
                        param = param
                            .src( self.spritesheet.full )
                            .color(Color::from_rgba(9,17,51,alpha));
                    },
                    map::TileState::Empty(ref mut known) => {
                        if !*known { alpha = 125 }
                        param = param
                            .src( self.spritesheet.empty )
                            .color(Color::from_rgba(124,174,195,alpha));
                        //*known = false; // tiles dont stay "known"
                    },
                    map::TileState::Goal(ref mut known) => {
                        if !*known { alpha = 125 }
                        param = param
                            .src(self.spritesheet.goal)
                    }
                    _ => (),
                }

                batch.add(param);
            }
        }
        graphics::draw(ctx, &batch, DrawParam::default())?;

        if let Some(x) = &self.map.scoreboard_txt {
            // param is for placement of text
            let param = DrawParam::default()
                .dest( Point2{x: 50.0, y: self.map.dimensions.1 as f32*self.map.tile_size} )
                .color(Color::from_rgba(0,0,0,255));
            graphics::draw(ctx, x, param)?;
        }


        // for debugging, this draws a line
        // let mouse_pos = ggez::input::mouse::position(ctx);
        // let origin = Point2{x: 10.0, y: 10.0};
        // let target = Point2{x: 300.0, y: 400.0};
        // map::draw_line(ctx, origin, mouse_pos);
        self.map.update_score(&mut self.actor);

        self.actor.draw(ctx)?;
        self.actor.draw_range(ctx)?;
        
        graphics::present(ctx)

    }
}

// add more for different states. the boolean described whether the actor has seen it.
#[derive(Clone, Copy, Debug)]
pub enum TileState {
    Empty(bool),
    Full(bool),
}

// when i make a sprite sheet this struct should have one image and several other fields describing where the bounds of each
// seperate sprite/tile idk the term is located
pub struct SpriteSheet {
    image: Image,
    empty: Rect,
    full: Rect,
    goal: Rect,
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
// [x] - make the screen size fit the tile map
// add the actor code back in 
// [x] - get back on finding out how to draw lines on a discrete grid - rudimentary implementation in place
//      is DDA faster than flood fill for my needs?
//      probably. implement DDA or breshzanms or however you spell that
//      but also i may just have slight math mistakes in my algorithm
//      but it literally uses inverse square roots but slow
//      [x] - DDA implemented
// [x] - make movement speed independent of framerate (when a human is controlling it), and make it normalized (cant move faster diagonally)
// [x] - collision
//      [ ] - improve collision (mainly let the actor slide up walls when youre holding in to it. maybe remove the overlaps_with function its unneeded atm)
//      i actually cant figure out how to do this am i stupid
// [x] - add back in clicking to add full tiles
//      [x] - make it so that if you start clicking on a full tile, all the tiles you drag over become empty
//      [ ] - add heavier stroke drawing
// [ ] - make sprite for actor and target
// [ ] - refactor everything to make it easier to read
// [ ] - put text on the screen for debugging
// [x] - test performance on a better computer
// [x] - add exporting and loading maps
//      [ ] - make all tiles invisible in a saved map
// [ ] - add timer, score, win condition
// [ ] - add way to know if a button is pressed rather than held


// could still only update the tiles i need to update and speed up the drawing massively
// an easy thing to do would be to only update tiles in a radius around the actor

// i think its bad to be exporting things from main to sub modules. the data needed by the sub module should be able to be passed to 
// it or contained in it from the start

// possibly use lazy static to keep a copy of my_game in each module
// major problem in the way that data is shared between the modules.
// the look_around() function in actor is looking jankier by the second
// some functions just take my_game but others cant
// can my_game not own actor but just a reference to it? but then what would the lifetime of it be.