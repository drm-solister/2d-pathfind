use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, Color, DrawParam, Image};
use mint::*;
use crate::{MyGame, TileState};

// pub fn index_from_mouse(my_game: &mut MyGame, coords: Point2<f32>) -> usize {

//     let index = y as u16 * self.dim_x + x as u16; // i thought x and y should be swapped on this line, may cause problems
//     return index as usize
// }

pub fn index_from_mouse(my_game: &mut MyGame, mouse: Point2<f32>) -> usize {
    let mut x = (mouse.x - mouse.x % my_game.tile_size) / my_game.tile_size;
    let mut y = (mouse.y - mouse.y % my_game.tile_size) / my_game.tile_size;

    let x = x.clamp(0.0, my_game.dimensions.0 as f32 -1.0); // why tf is x and y switched ??
    let y = y.clamp(0.0, my_game.dimensions.1 as f32 -1.0);

    let index = y as u16 * my_game.dimensions.0 + x as u16;
    return index as usize
}

pub fn coords_from_mouse(my_game: &mut MyGame, mouse: Point2<f32>) -> Point2<u16> {
    let mut x = (mouse.x - mouse.x % my_game.tile_size) / my_game.tile_size;
    let mut y = (mouse.y - mouse.y % my_game.tile_size) / my_game.tile_size;

    let x = x.clamp(0.0, my_game.dimensions.0 as f32 -1.0); // why tf is x and y switched ??
    let y = y.clamp(0.0, my_game.dimensions.1 as f32 -1.0);

    return Point2{x: x as u16, y: y as u16}
}

// find all the tiles on a line (based on endpoints) that are visible and return them in a vector. flood-fill algorithm
pub fn find_visible_on_line(my_game: &mut MyGame, origin: Point2<f32>, endpoint: Point2<f32>) {
    
    // start with the pixel containing the origin
    // for all surrounding pixels p...
    // if distance between p and line is less than 0.5, it is visible

    let og_coord = coords_from_mouse(my_game, origin);
    let ep_coord = coords_from_mouse(my_game, endpoint);

    flood_and_find(&mut my_game.tile_states, og_coord, ep_coord, og_coord);

    return;

}

// the points here are coordinates on the grid
fn flood_and_find(tile_states: &mut Vec<TileState>, origin: Point2<u16>, endpoint: Point2<u16>, point: Point2<u16>) {
    // hard coded because i dont have access to this data but i should
    // if the flood finds a full block, dont continue

    if let TileState::Full(_) = tile_states[(origin.x + origin.y*80) as usize] { 
        tile_states[(origin.x + origin.y*80) as usize] = TileState::Full(true);
        return;
    }

    //for coords in origin.surrounding
}

// helper function for visible_on_line()
// distance from a point to a line (based on endpoints)
pub fn distance_from_line(origin: Point2<f32>, endpoint: Point2<f32>, point: Point2<f32>) -> f32 {
    let dy = endpoint.y - origin.y;
    let dx = endpoint.x - origin.x;

    if dy == 0.0 {
        return origin.y - point.y;
    } else if dx == 0.0 {
        return origin.x - point.x;
    }

    let a = dy/dx;

    let b = -1.0;

    let c = origin.y - origin.x * a;

    let distance = (a*point.x + b*point.y + c).abs() / f32::sqrt(a*a + b*b);
    //println!("distance: {:?}", distance);
    return distance;
}

pub fn point_to_point_dist() {
    todo!()
}


pub fn draw_line(ctx: &mut Context, origin: Point2<f32>, endpoint: Point2<f32>) -> GameResult<()> {
    let line = graphics::Mesh::new_line(ctx, &[origin, endpoint], 3.0, Color::RED).unwrap();
    graphics::draw(ctx, &line, DrawParam::default())
}

// should there be a struct to represent a line ? maybe not 
// use glam's vec2