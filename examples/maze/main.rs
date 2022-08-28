mod grid;
mod maze;

use maze::{Tile, Maze};
use nannou::prelude::*;

const ROWS:usize = 10;
const COLUMNS:usize =10;

struct AppModel {
    maze:Maze::<ROWS,COLUMNS>
}

fn view(app:&App,model:&AppModel,frame:Frame){
    let window = app.window_rect().wh();
    let window_half = window/2.0;
    let tile_size = window/Vec2::new(COLUMNS as f32, ROWS as f32);
    let draw = app.draw();
    let tile_draw = draw.xy(tile_size/2.0-window_half);
    draw.background().color(GREEN);

    // let maze = maze::Maze::<ROWS, COLUMNS>::generate();
    let tiles = model.maze.rows().enumerate().flat_map(|(r,row)|row.into_iter().enumerate().map(move |(c,tile)|(Vec2::new(c as f32, r as f32),tile)));
    for (pos,tile) in tiles {
        let color = match tile {
            Tile::Open => WHITE,
            Tile::Reward => LIMEGREEN,
            Tile::Trap => RED
        };

        tile_draw.rect().stroke_color(BLACK).stroke_weight(1.0).color(color).wh(tile_size).xy(pos*tile_size);
    }

    draw.to_frame(app, &frame).unwrap()
}
fn model(app:&App)->AppModel{
    app.new_window().build().expect("Error creating window");
    AppModel {
        maze:maze::Maze::<ROWS, COLUMNS>::generate()
    }
    
}

fn main() {
    nannou::app(model).view(view).run()  
}
