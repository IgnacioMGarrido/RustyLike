use app::App;
use chargrid_graphical::{Config, Context, Dimensions, FontBytes};
use coord_2d::Size;

mod app;
mod game;
mod terrain;
mod world;
mod visibility;

fn main(){
    const CELL_SIZE_PX: f64 = 24.;

    let context = Context::new(Config { 
        title: "Simple Roguelike".to_string(),
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec()
        },
        window_dimensions_px: Dimensions {
            width: 960., 
            height: 720., 
        },
        cell_dimensions_px: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        font_scale:  Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        underline_width_cell_ratio: 0.1,
        underline_top_offset_cell_ratio: 0.8,
        resizable: false 
    });
    
    let screen_size = Size::new(40, 30);
    let app = App::new(screen_size);
    context.run_app(app);
}