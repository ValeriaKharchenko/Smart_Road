use macroquad::prelude::*;
// pub use rand::Rng;
use r::Rng; // 0.8.5


use Smart_Road::*;
// use raster::{Color, Image};

pub mod draw;

use crate::draw::road;

// const CAR_HEIGHT: f32 = 10f32;
// const CAR_LENGTH: f32 = 30f32;
const CAR_SPEED: f32 = 50f32;
const CAR_PADDING: f32 = 20f32;

//Car Starting positions:
const START_FROM_TOP_TO_LEFT: macroquad::math::Vec2 = vec2(370f32, 0f32 - CAR_LENGTH);
const START_FROM_TOP_FORWARD: macroquad::math::Vec2 = vec2(340f32, 0f32 - CAR_LENGTH);
const START_FROM_TOP_TO_RIGHT: macroquad::math::Vec2 = vec2(310f32, 0f32 - CAR_LENGTH);
/**/
const START_FROM_BOTTOM_TO_LEFT: macroquad::math::Vec2 = vec2(400f32, 800f32);
const START_FROM_BOTTOM_TO_FORWARD: macroquad::math::Vec2 = vec2(430f32, 800f32);
const START_FROM_BOTTOM_TO_RIGHT: macroquad::math::Vec2 = vec2(460f32, 800f32);
/**/
const START_FROM_LEFT_TO_LEFT: macroquad::math::Vec2 = vec2(0f32 - CAR_LENGTH, 400f32);
const START_FROM_LEFT_FORWARD: macroquad::math::Vec2 = vec2(0f32 - CAR_LENGTH, 430f32);
const START_FROM_LEFT_TO_RIGHT: macroquad::math::Vec2 = vec2(0f32 - CAR_LENGTH, 460f32);
/**/
const START_FROM_RIGHT_TO_LEFT: macroquad::math::Vec2 = vec2(800f32, 370f32);
const START_FROM_RIGHT_FORWARD: macroquad::math::Vec2 = vec2(800f32, 340f32);
const START_FROM_RIGHT_TO_RIGHT: macroquad::math::Vec2 = vec2(800f32, 310f32);

pub fn draw_title_text(text: &str) {
    let dims = measure_text(text, Default::default(), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams { font: Default::default(), font_size: 50u16, color: WHITE, ..Default::default() })
}

pub struct Statistics {
    passed_intersection: u32,
}

impl Statistics {
    pub fn new() -> Self {
        Self { passed_intersection: 0 }
    }
}


pub enum GameState {
    Menu,
    Game,
    Statistics,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "SMART ROAD".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut statistics = Statistics::new();
    let mut intersection = Intersection::new();

    loop {
        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start");
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                //draw road
                road();
                intersection.drive_cars();
                intersection.remove_cars();
                intersection.draw_cars();

                //Draw new car with direction from right to left
                if is_key_pressed(KeyCode::Left) {
                    let routes = vec![Route::E_W, Route::E_N, Route::E_S];
                    intersection.add_car(routes);
                }

                //Draw new car with direction from left to right
                if is_key_pressed(KeyCode::Right) {
                    let routes = vec![Route::W_E, Route::W_S, Route::W_N];
                    intersection.add_car(routes);
                }

                //Draw new car with direction from bottom to top
                if is_key_pressed(KeyCode::Up) {
                    let routes = vec![Route::S_N, Route::S_E, Route::S_W];
                    intersection.add_car(routes);
                }

                //Draw new car with direction from top to bottom
                if is_key_pressed(KeyCode::Down) {
                    let routes = vec![Route::N_S, Route::N_W, Route::N_E];
                    intersection.add_car(routes);
                }

                //Draw new car with a random direction
                if is_key_pressed(KeyCode::R) {
                    let routes = vec![Route::E_W, Route::W_E, Route::S_N, Route::N_S, Route::E_N, Route::W_S, Route::N_W, Route::S_E, Route::N_E, Route::S_W, Route::W_N, Route::E_S];
                    // let routes = vec![ Route::N_W, Route::N_E, Route::S_W];
                    intersection.add_car(routes);
                }

                //end of simulation
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Statistics
                }
            }

            GameState::Statistics => {
                statistics.passed_intersection = intersection.number_of_passed_vehicles;
                draw_title_text(&format!("STATISTICS: cars finished: {}", statistics.passed_intersection));
            }
        }

        next_frame().await
    }
}



