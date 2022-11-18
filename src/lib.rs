use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use macroquad::color::Color;
use macroquad::prelude::*;

pub const CAR_HEIGHT: f32 = 10_f32;
pub const CAR_LENGTH: f32 = 30_f32;

const CAR_SPEED_NORMAL: f32 = 1.5;
const CAR_SPEED_SLOW: f32 = 0.3;
const CAR_SPEED_FAST: f32 = 3.5;

const BEFORE_CROSS_ROAD: Vec2 = vec2(220.0, 560.0);
const AFTER_CROSS_ROAD: Vec2 = vec2(300.0, 480.0);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(PartialEq)]
pub enum Turning {
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Car {
    pub id: u32,
    pub color: Color,
    pub position: Vec2,
    pub speed: (f32, f32),
    pub rectangle: (f32, f32),
    pub direction: Direction,
}


impl Car {
    pub fn new(
        position: Vec2,
        rectangle: (f32, f32),
        color: Color,
        speed: (f32, f32),
        id: u32,
        direction: Direction,
    ) -> Car {
        Car {
            color,
            rectangle,
            position,
            speed,
            id,
            direction,
        }
    }

    pub fn drive(&mut self) {
        self.position = (
            vec2(self.position.x + self.speed.0,
                 self.position.y + self.speed.1, )
        );
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.rectangle.0, self.rectangle.1, self.color);
    }

    fn before_cross_road(&self) -> bool {
        match self.direction {
            Direction::Right => self.position.x < BEFORE_CROSS_ROAD.x,
            Direction::Left => self.position.x > BEFORE_CROSS_ROAD.y,
            Direction::Down => self.position.y < BEFORE_CROSS_ROAD.x,
            Direction::Up => self.position.y > BEFORE_CROSS_ROAD.y,
        }
    }

    fn after_cross_road(&self) -> bool {
        match self.direction {
            Direction::Right => self.position.x > AFTER_CROSS_ROAD.y,
            Direction::Left => self.position.x < AFTER_CROSS_ROAD.x,
            Direction::Down => self.position.y > AFTER_CROSS_ROAD.y,
            Direction::Up => self.position.y < AFTER_CROSS_ROAD.x,
        }
    }

    fn on_cross_road(&self) -> bool {
        return !self.before_cross_road() && !self.after_cross_road();
    }

    fn speed_up(&mut self) {
        self.speed = match self.direction {
            Direction::Down => (0.0, CAR_SPEED_FAST),
            Direction::Up => (0.0, -CAR_SPEED_FAST),
            Direction::Right => (CAR_SPEED_FAST, 0.0),
            Direction::Left => (-CAR_SPEED_FAST, 0.0),
        }
    }

    fn is_speed_up(&self) -> bool {
        return self.speed.0.abs() == CAR_SPEED_FAST || self.speed.1.abs() == CAR_SPEED_FAST;
    }

    fn is_slow_down(&self) -> bool {
        return self.speed.0.abs() == CAR_SPEED_SLOW || self.speed.1.abs() == CAR_SPEED_SLOW;
    }

    fn slow_down(&mut self) {
        self.speed = match self.direction {
            Direction::Down => (0.0, CAR_SPEED_SLOW),
            Direction::Up => (0.0, -CAR_SPEED_SLOW),
            Direction::Right => (CAR_SPEED_SLOW, 0.0),
            Direction::Left => (-CAR_SPEED_SLOW, 0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Route {
    N_S,
    S_N,
    W_E,
    E_W,
}

impl Route {
    fn get_coordinates(&self) -> Vec2 {
        match *self {
            Route::N_S => vec2(340_f32, 0_f32 - CAR_LENGTH),
            Route::S_N => vec2(430_f32, 800_f32),
            Route::W_E => vec2(0_f32 - CAR_LENGTH, 430_f32),
            Route::E_W => vec2(800_f32, 340_f32),
        }
    }
    fn get_speed(&self) -> (f32, f32) {
        match *self {
            Route::N_S => (0.0, CAR_SPEED_NORMAL),
            Route::S_N => (0.0, -CAR_SPEED_NORMAL),
            Route::W_E => (CAR_SPEED_NORMAL, 0.0),
            Route::E_W => (-CAR_SPEED_NORMAL, 0.0),
        }
    }
    fn get_direction(&self) -> Direction {
        match *self {
            Route::N_S => Direction::Down,
            Route::S_N => Direction::Up,
            Route::W_E => Direction::Right,
            Route::E_W => Direction::Left,
        }
    }

    fn not_allowed_to_go(&self) -> Vec<Route> {
        match *self {
            Route::N_S => vec![Route::E_W, Route::W_E],
            Route::S_N => vec![Route::E_W, Route::W_E],
            Route::W_E => vec![Route::N_S, Route::S_N],
            Route::E_W => vec![Route::N_S, Route::S_N],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    tracks: HashMap<Route, Vec<u32>>,
    car_id: u32,
    occupied_tracks: HashMap<Route, HashSet<u32>>,
    cars: HashMap<u32, Car>,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            tracks: HashMap::new(),
            car_id: 0,
            occupied_tracks: HashMap::new(),
            cars: HashMap::new(),
        }
    }

    pub fn add_car(&mut self, routes: Vec<Route>) {
        let route: Route = generate_route(routes);
        if !self.can_add(route) {
            return;
        }
        self.car_id += 1;

        let mut rectangle: (f32, f32) = (CAR_LENGTH, CAR_HEIGHT);
        if route == Route::N_S || route == Route::S_N {
            rectangle = (CAR_HEIGHT, CAR_LENGTH);
        }

        let car = Car::new(
            route.get_coordinates(),
            rectangle,
            RED,
            route.get_speed(),
            self.car_id,
            route.get_direction(),
        );

        let current_cars_on_track = self.tracks.get_mut(&route);
        let mut cars = match current_cars_on_track {
            Some(value) => value.to_vec(),
            None => Vec::new(),
        };
        cars.push(car.id);
        self.tracks.get_mut(&route);
        self.tracks.insert(route, cars.clone());
        self.cars.insert(car.id, car);
    }

    fn can_add(&mut self, route: Route) -> bool {
        let start_coordinates = route.get_coordinates();
        let cars = self.tracks.get(&route);
        return match cars {
            Some(cars) => {
                let last_car_id = cars.as_slice().last().unwrap();
                let last_car_position = self.cars.get(last_car_id).unwrap().position;
                if route == Route::N_S && last_car_position.y <= start_coordinates.y + CAR_LENGTH * 2.0 {
                    return false;
                }
                if route == Route::S_N && last_car_position.y + CAR_LENGTH * 2.0 >= start_coordinates.y {
                    return false;
                }
                if route == Route::W_E && last_car_position.x <= start_coordinates.x + CAR_LENGTH * 2.0 {
                    return false;
                }
                if route == Route::E_W && last_car_position.x + CAR_LENGTH * 2.0 >= start_coordinates.x {
                    return false;
                }
                true
            }
            None => true,
        };
    }

    pub fn draw_cars(&self) {
        for (_route, cars) in self.tracks.iter() {
            cars.iter().for_each(|id| {
                let car = self.cars.get(id).unwrap();
                car.draw();
            })
        }
    }

    pub fn drive_cars(&mut self) {
        let mut can_go = true;
        let mut not_speed_up = true;
        for (_, car) in self.cars.iter() {
                not_speed_up = not_speed_up && !car.is_speed_up();
        }
        let mut cars = self.cars.clone();
        for (route, cars_ids) in self.tracks.iter() {
            for (ind, car_id) in cars_ids.iter().enumerate() {
                let cars_on_cross_road = self.occupied_tracks.get(route);
                // let mut cars = self.cars.borrow_mut();
                let mut car: &mut Car = self.cars.get_mut(car_id).unwrap();
                route.not_allowed_to_go().iter().for_each(|r| {
                    can_go = can_go && self.occupied_tracks.get(r).is_none() || can_go && not_speed_up;
                });

                if !cars_on_cross_road.is_none() {
                    let mut all_cars = cars_on_cross_road.unwrap().clone();
                    if car.on_cross_road() && !car.is_speed_up() {
                        if can_go {
                            car.speed_up();
                        } else {
                            car.slow_down();
                        }
                        all_cars.insert(car.id);
                    } else if car.after_cross_road() {
                        all_cars.remove(&car.id);
                        car.speed = route.get_speed();
                    }
                    if all_cars.is_empty() {
                        self.occupied_tracks.remove(route);
                    } else {
                        self.occupied_tracks.insert(*route, all_cars);
                    }
                } else if car.on_cross_road() {
                    let cars = HashSet::from_iter(vec![car.id]);
                    if can_go {
                        car.speed_up();
                    } else {
                        car.slow_down();
                    }
                    self.occupied_tracks.insert(*route, cars);
                }
                if car.before_cross_road() && ind >= 1 {
                    // let prev_car = self.cars.get(&cars_ids[ind-1]).unwrap();
                    if cars.get(&cars_ids[ind-1]).unwrap().is_slow_down() {
                        car.slow_down();
                    } else {
                        car.speed = route.get_speed();
                    }
                }
                car.drive();
            }
        }
    }
}

fn generate_route(routes: Vec<Route>) -> Route {
    let n: usize = rand::gen_range(0, routes.len());
    return routes[n];
}