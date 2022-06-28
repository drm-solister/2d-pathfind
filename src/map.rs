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
    // if distance between p and line is less than the diagonal, it is visible

    let og_coord = coords_from_mouse(my_game, origin);
    let ep_coord = coords_from_mouse(my_game, endpoint);
    let px_line = PxLine { origin, endpoint };

    let mut indicies_seen_this_pass: Vec<usize> = Vec::new();
    flood_and_find(my_game.dimensions, &mut my_game.tile_states, &px_line, og_coord, &mut indicies_seen_this_pass);

    return;

}

// hard coded values in here: horizontal dimension, pixel size
fn flood_and_find(dimensions: (u16, u16), tile_states: &mut Vec<TileState>, px_line: &PxLine, point: Point2<u16>, indicies_seen_this_pass: &mut Vec<usize>) {
    // hard coded because i dont have access to this data but i should
    // if the flood finds a full block, dont continue
    // dont retrace your steps or youll loop forever
    // println!("{:?} is on the line, checking surrounding coordinates", point);
    if point.x == dimensions.0 || point.y == dimensions.1 { return; }   // reached the end of the grid

    let index = (point.x + point.y*30) as usize;
    if indicies_seen_this_pass.contains(&index) { return; } // already "saw" this square
    indicies_seen_this_pass.push(index);

    match tile_states[index] {
        TileState::Full(ref mut x) => {
            *x = true;
            return;
        },
        TileState::Empty(ref mut x) => {
            *x = true;
        },
    }

    //for coords in origin.surrounding
    for i in -1i16..=1i16 {
        for j in -1i16..=1i16 {
            // hard coding to get the surrounding pixels, kinda stupid but maybe somewhat necessary
            if i == 0 && j == 0 { continue; }
            if point.x == 0 && i == -1 { continue; }
            if point.y == 0 && j == -1 { continue; }
            if point.y == dimensions.1 && j == 1 { continue; }
            if point.x == dimensions.0 && i == 1 { continue; }

            // println!("point.x: {:?}", point.x);
            let new_point: Point2<u16> = Point2{x: (point.x as i16+i) as u16, y: (point.y as i16+j) as u16};

            let center_of_target_px = Point2{x: new_point.x as f32 * 9.0, y: new_point.y as f32 * 9.0};
            if distance_from_line(px_line.origin, px_line.endpoint, center_of_target_px) < 6.0 {
                flood_and_find(dimensions, tile_states, px_line, new_point, indicies_seen_this_pass);
            }
        }
    }

}

// helper function for visible_on_line()
// distance from a point to a line (based on endpoints)
pub fn distance_from_line(origin: Point2<f32>, endpoint: Point2<f32>, point: Point2<f32>) -> f32 {
    let dy = endpoint.y - origin.y;
    let dx = endpoint.x - origin.x;

    if dy == 0.0 {
        return (origin.y - point.y).abs();
    } else if dx == 0.0 {
        return (origin.x - point.x).abs();
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

// line represented by pixels on the screen
struct PxLine {
    origin: Point2<f32>,
    endpoint: Point2<f32>,
}

pub fn dda(my_game: &mut MyGame, origin: Point2<f32>, endpoint: Point2<f32>) {
    let dx = endpoint.x - origin.x;
    let dy = endpoint.y - origin.y;

    let steps;
    if dx.abs() > dy.abs() {
        steps = dx.abs();
    } else {
        steps = dy.abs();
    }

    let x_inc = dx / steps;
    let y_inc = dy / steps;

    let mut x = origin.x;
    let mut y = origin.y;

    for i in 0..=steps as u16 {
        let index = index_from_mouse(my_game, Point2{ x, y});

        match my_game.tile_states[index] {
            TileState::Full(ref mut x) => {
                *x = true;
                return;
            }
            TileState::Empty(ref mut x) => *x = true,
        }

        x += x_inc;
        y += y_inc;
    }
}


// my brain is too small to understand why this doesnt work
// pub fn bresenhams(my_game: &mut MyGame, origin: Point2<f32>, endpoint: Point2<f32>) {

//     if (endpoint.y - orgin.y).abs() < (endpoint.x - origin.x).abs() {
//         if origin.x > endpoint.x {
//             plot_low();
//         } else {
//             plot_low();
//         }
//     } else {
//         if origin.y > endpoint.y {
//             plot_high();
//         } else {
//             plot_high();
//         }
//     }

// }

// // helpers for bresenhams
// fn plot_high() {
//     let mut dx = endpoint.x - origin.x;
//     let mut dy = endpoint.y - origin.y;
//     let mut xi = 1.0;

//     if dx < 0 {
//         xi = -1.0;
//         dx = -dx;
//     }

//     let mut d = (2.0 * dx) - dy;
//     x = origin.x;

//     for y in origin.y..endpoint.y {
//         observe_index(tile_states, index_from_mouse(my_game, Point2{x, y}));
//         if d > 0.0 {
//             x += xi;
//             d = d + (2.0 * (dx - dy));
//         } else {
//             d = d + 2.0*dx;
//         }
//     }
// }

// fn plot_low() {
//     let mut dx = endpoint.x - origin.x;
//     let mut dy = endpoint.y - origin.y;
//     let mut yi = 1.0;

//     if dy < 0 {
//         xi = -1.0;
//         dx = -dx;
//     }

//     let mut d = (2 * dx) - dy;
//     y = origin.y;

//     for y in origin.x..endpoint.x {
//         observe_index(tile_states, index_from_mouse(my_game, Point2{x, y}));
//         if d > 0.0 {
//             y += yi;
//             d = d + (2.0 * (dy - dx));
//         } else {
//             d = d + 2.0*dy;
//         }
//     }
// }

// fn observe_index(tile_states: &mut Vec<TileState>, index: usize) {
//     match tile_states[index] {
//         TileState::Full(ref mut x) => *x = true,
//         TileState::Empty(ref mut x) => *x = true,
//         _ => (),
//     }
// }