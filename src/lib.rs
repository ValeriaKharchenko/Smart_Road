use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet, VecDeque};
use macroquad::color::Color;
use macroquad::prelude::*;
use crate::miniquad::native::apple::frameworks::sel;

pub const CAR_HEIGHT: f32 = 10_f32;
pub const CAR_LENGTH: f32 = 30_f32;

const CAR_SPEED_NORMAL: f32 = 1.5;
const CAR_SPEED_SLOW: f32 = 0.3;
const CAR_SPEED_FAST: f32 = 3.5;

const BEFORE_CROSS_ROAD: Vec2 = vec2(200.0, 580.0);
const AFTER_CROSS_ROAD: Vec2 = vec2(300.0, 480.0);
// const AFTER_FIRST_INTERSECTION: Vec2 = vec2(350.0, 430.0);

pub const COLORS: &'static [Color] = &[LIME, RED, SKYBLUE, VIOLET, GREEN, GRAY, MAROON, MAGENTA];

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
    pub route: Route,
    pub turned: bool,
}


impl Car {
    pub fn new(
        position: Vec2,
        rectangle: (f32, f32),
        color: Color,
        speed: (f32, f32),
        id: u32,
        direction: Direction,
        route: Route,
        turned: bool,
    ) -> Car {
        Car {
            color,
            rectangle,
            position,
            speed,
            id,
            direction,
            route,
            turned,
        }
    }

    pub fn drive(&mut self) {
        if self.on_turn_point() && !self.turned {
            self.turn();
        }
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

    fn in_stop_zone(&self) -> bool {
        return match self.direction {
                    Direction::Right => self.position.x > AFTER_CROSS_ROAD.x - CAR_LENGTH,
                    Direction::Left => self.position.x < AFTER_CROSS_ROAD.y,
                    Direction::Down => self.position.y > AFTER_CROSS_ROAD.x - CAR_LENGTH,
                    Direction::Up => self.position.y < AFTER_CROSS_ROAD.y,
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

    fn on_turn_point(&self) -> bool {
        return match self.route {
            Route::N_W => self.position.y > 300.0 && self.position.y < 350.0,
            Route::S_E => self.position.y < 460.0 && self.position.y > 410.0,
            Route::W_S => self.position.x > 300.0 && self.position.x < 350.0,
            Route::E_N => self.position.x < 460.0 && self.position.x > 350.0,

            Route::N_E => self.position.y > 390.0 && self.position.y < 440.0,
            Route::S_W => self.position.y > 320.0 && self.position.y < 370.0,
            Route::W_N => self.position.x > 390.0 && self.position.x < 440.0,
            Route::E_S => self.position.x > 320.0 && self.position.x < 370.0,
            _ => false,
        };
    }

    fn turn(&mut self) {
        let speed = self.speed;
        let r = self.rectangle;
        self.rectangle.0 = r.1;
        self.rectangle.1 = r.0;
        self.turned = true;

        match self.route {
            Route::N_E => {
                self.speed.0 = speed.1;
                self.speed.1 = speed.0;
                self.direction = Direction::Right;
                self.position.y = 400.0;
            }
            Route::S_W => {
                self.speed.0 = speed.1;
                self.speed.1 = speed.0;
                self.direction = Direction::Left;
                self.position.y = 370.0;
            }
            Route::W_N => {
                self.speed.0 = -speed.1;
                self.speed.1 = -speed.0;
                self.direction = Direction::Up;
                self.position.x = 400.0;
            }
            Route::E_S => {
                self.speed.0 = -speed.1;
                self.speed.1 = -speed.0;
                self.direction = Direction::Down;
                self.position.x = 370.0;
            }
            Route::N_W => {
                self.speed.0 = -speed.1;
                self.speed.1 = -speed.0;
                self.direction = Direction::Left;
                self.position.y = 310.0;
            }
            Route::S_E => {
                self.speed.0 = -speed.1;
                self.speed.1 = -speed.0;
                self.position.y = 460.0;
                self.direction = Direction::Right;
            }
            Route::W_S => {
                self.direction = Direction::Down;
                self.speed.0 = speed.1;
                self.speed.1 = speed.0;
                self.position.x = 310.0;
            }
            Route::E_N => {
                self.direction = Direction::Up;
                self.speed.0 = speed.1;
                self.speed.1 = speed.0;
                self.position.x = 460.0;
            }
            _ => return,
        }
    }

    fn drive_away(&self) -> bool {
        return match self.direction {
            Direction::Right => self.position.x > 800.0,
            Direction::Left => self.position.x < 0.0,
            Direction::Down => self.position.y > 800.0,
            Direction::Up => self.position.y < 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Route {
    N_S,
    S_N,
    W_E,
    E_W,

    N_W,
    S_E,
    W_S,
    E_N,

    N_E,
    S_W,
    W_N,
    E_S,
}

impl Route {
    fn get_coordinates(&self) -> Vec2 {
        match *self {
            Route::N_S => vec2(340_f32, 0_f32 - CAR_LENGTH),
            Route::S_N => vec2(430_f32, 800_f32),
            Route::W_E => vec2(0_f32 - CAR_LENGTH, 430_f32),
            Route::E_W => vec2(800_f32, 340_f32),

            Route::N_W => vec2(310_f32, 0_f32 - CAR_LENGTH),
            Route::S_E => vec2(460_f32, 800_f32),
            Route::W_S => vec2(0_f32 - CAR_LENGTH, 460_f32),
            Route::E_N => vec2(800_f32, 310_f32),

            Route::N_E => vec2(370_f32, 0f32 - CAR_LENGTH),
            Route::S_W => vec2(400_f32, 800_f32),
            Route::W_N => vec2(0f32 - CAR_LENGTH, 400f32),
            Route::E_S => vec2(800f32, 370f32),
        }
    }
    fn get_speed(&self) -> (f32, f32) {
        match *self {
            Route::N_S => (0.0, CAR_SPEED_NORMAL),
            Route::N_W => (0.0, CAR_SPEED_NORMAL),
            Route::N_E => (0.0, CAR_SPEED_NORMAL),

            Route::S_N => (0.0, -CAR_SPEED_NORMAL),
            Route::S_E => (0.0, -CAR_SPEED_NORMAL),
            Route::S_W => (0.0, -CAR_SPEED_NORMAL),

            Route::W_E => (CAR_SPEED_NORMAL, 0.0),
            Route::W_S => (CAR_SPEED_NORMAL, 0.0),
            Route::W_N => (CAR_SPEED_NORMAL, 0.0),

            Route::E_W => (-CAR_SPEED_NORMAL, 0.0),
            Route::E_N => (-CAR_SPEED_NORMAL, 0.0),
            Route::E_S => (-CAR_SPEED_NORMAL, 0.0),
        }
    }
    fn get_direction(&self) -> Direction {
        match *self {
            Route::N_S => Direction::Down,
            Route::N_W => Direction::Down,
            Route::N_E => Direction::Down,

            Route::S_N => Direction::Up,
            Route::S_E => Direction::Up,
            Route::S_W => Direction::Up,

            Route::W_E => Direction::Right,
            Route::W_S => Direction::Right,
            Route::W_N => Direction::Right,

            Route::E_W => Direction::Left,
            Route::E_N => Direction::Left,
            Route::E_S => Direction::Left,
        }
    }

    fn not_allowed_to_go(&self) -> Vec<Route> {
        match *self {
            Route::N_S => vec![Route::E_W, Route::W_E, Route::W_N, Route::S_W],
            Route::S_N => vec![Route::N_E, Route::W_E, Route::E_S, Route::E_W],
            Route::W_E => vec![Route::N_S, Route::S_W, Route::S_N, Route::E_S],
            Route::E_W => vec![Route::N_S, Route::N_E, Route::S_N, Route::W_N],

            Route::N_W => vec![],
            Route::S_E => vec![],
            Route::W_S => vec![],
            Route::E_N => vec![],

            Route::N_E => vec![Route::E_W, Route::S_N, Route::S_W, Route::W_N, Route::E_S],
            Route::S_W => vec![Route::N_S, Route::N_E, Route::W_E, Route::W_N, Route::E_S],
            Route::W_N => vec![Route::N_S, Route::N_E, Route::S_W, Route::E_W, Route::E_S],
            Route::E_S => vec![Route::N_E, Route::S_N, Route::S_W, Route::W_E, Route::W_N],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    tracks: HashMap<Route, Vec<u32>>,
    car_id: u32,
    occupied_tracks: HashMap<Route, HashSet<u32>>,
    cars: HashMap<u32, Car>,
    queue: VecDeque<u32>,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            tracks: HashMap::new(),
            car_id: 0,
            occupied_tracks: HashMap::new(),
            cars: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn add_car(&mut self, routes: Vec<Route>) {
        let route: Route = generate_route(routes);
        if !self.can_add(route) {
            return;
        }

        self.car_id += 1;

        let mut rectangle: (f32, f32) = (CAR_LENGTH, CAR_HEIGHT);
        let direction = route.get_direction();
        if direction == Direction::Up || direction == Direction::Down {
            rectangle = (CAR_HEIGHT, CAR_LENGTH);
        }
        let n: usize = rand::gen_range(0, COLORS.len());

        let car = Car::new(
            route.get_coordinates(),
            rectangle,
            COLORS[n],
            route.get_speed(),
            self.car_id,
            direction,
            route,
            false,
        );

        let current_cars_on_track = self.tracks.get_mut(&route);
        let mut cars = match current_cars_on_track {
            Some(value) => value.to_vec(),
            None => Vec::new(),
        };
        // self.queue.push_back(car.id);
        cars.push(car.id);
        self.tracks.get_mut(&route);
        self.tracks.insert(route, cars.clone());
        self.cars.insert(car.id, car);
    }

    fn can_add(&mut self, route: Route) -> bool {
        let start_coordinates = route.get_coordinates();
        let mut cars = self.tracks.get_mut(&route);
        return match cars {
            Some(cars) => {
                let last_car_id = cars.as_slice().last().unwrap();
                let last_car_position = self.cars.get(last_car_id).unwrap().position;
                if (route == Route::N_S || route == Route::N_W || route == Route::N_E) && last_car_position.y <= start_coordinates.y + CAR_LENGTH * 2.0 {
                    return false;
                }
                if (route == Route::S_N || route == Route::S_E || route == Route::S_W) && last_car_position.y + CAR_LENGTH * 2.0 >= start_coordinates.y {
                    return false;
                }
                if (route == Route::W_E || route == Route::W_S || route == Route::W_N) && last_car_position.x <= start_coordinates.x + CAR_LENGTH * 2.0 {
                    return false;
                }
                if (route == Route::E_W || route == Route::E_N || route == Route::E_S) && last_car_position.x + CAR_LENGTH * 2.0 >= start_coordinates.x {
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
        for (route, cars_ids) in self.tracks.iter() {
            for (ind, car_id) in cars_ids.iter().enumerate() {
                let mut cars = self.cars.clone();
                let cars_on_cross_road = self.occupied_tracks.get(route);
                let mut car: &mut Car = self.cars.get_mut(car_id).unwrap();
                let mut can_go = route.not_allowed_to_go().len() == 0 || self.queue.is_empty() || self.queue[0] == car.id;

                route.not_allowed_to_go().iter().for_each(|r| {
                    let not_speed_up = match self.occupied_tracks.get(r) {
                        Some(a) => {
                            let mut res = true;
                            a.iter().for_each(|f| {
                                res = res && !cars.get(f).unwrap().is_speed_up();
                            });
                            res
                        }
                        None => true
                    };
                    can_go = can_go && (self.occupied_tracks.get(r).is_none() || not_speed_up);
                });

                if !cars_on_cross_road.is_none() {
                    let mut all_cars = cars_on_cross_road.unwrap().clone();
                    if !car.before_cross_road() && !car.is_speed_up() {
                        if can_go {
                            car.speed_up();
                            if !self.queue.is_empty() && self.queue[0] == car.id {
                                self.queue.pop_front();
                            }
                            println!("Q1:{:?}", self.queue)
                        } else {
                            car.slow_down();
                            if !self.queue.contains(&car.id) {
                                self.queue.push_back(car.id);
                            }
                        }
                        all_cars.insert(car.id);
                    } else if car.after_cross_road() {
                        all_cars.remove(&car.id);
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
                        if !self.queue.is_empty() && self.queue[0] == car.id {
                            self.queue.pop_front();
                        }
                        println!("Q2:{:?}", self.queue)
                    } else {
                        car.slow_down();
                        if !self.queue.contains(&car.id) {
                            self.queue.push_back(car.id);
                        }
                    }
                    self.occupied_tracks.insert(*route, cars);
                }
                if car.before_cross_road() && ind >= 1 {
                    if cars.get(&cars_ids[ind - 1]).unwrap().is_slow_down() {
                        car.slow_down();
                    } else {
                        car.speed = route.get_speed();
                    }
                }
                let mut car_clone = car.clone();
                car_clone.drive();
                if (car_clone.is_speed_up() || !car_clone.in_stop_zone())&& !cars.values().any(|c| {
                    c.id != car_clone.id && intersect(car_clone.position, c.position,
                                                      vec2(car_clone.position.x + car_clone.rectangle.0 + 5.0, car_clone.position.y + car_clone.rectangle.1 + 5.0),
                                                      vec2(c.position.x + c.rectangle.0 + 5.0, c.position.y + c.rectangle.1 + 5.0))
                }) {
                    car.drive();
                    // if car.drive_away() {
                    //     self.cars.remove(car_id);
                    //     let mut left_cars = self.tracks.get_mut(route);
                    //     if !left_cars.is_none() {
                    //         let retained = left_cars.unwrap().retain(|id| id != car_id);
                    //         self.tracks.insert(*route,retained);
                    //     }
                    // }
                }
            }
        }
    }
}

fn generate_route(routes: Vec<Route>) -> Route {
    let n: usize = rand::gen_range(0, routes.len());
    return routes[n];
}

fn intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    return (
        (
            (
                (a.x >= b.x && a.x <= d.x) || (c.x >= b.x && c.x <= d.x)
            ) && (
                (a.y >= b.y && a.y <= d.y) || (c.y >= b.y && c.y <= d.y)
            )
        ) || (
            (
                (b.x >= a.x && b.x <= c.x) || (d.x >= a.x && d.x <= c.x)
            ) && (
                (b.y >= a.y && b.y <= c.y) || (d.y >= a.y && d.y <= c.y)
            )
        )
    ) || (
        (
            (
                (a.x >= b.x && a.x <= d.x) || (c.x >= b.x && c.x <= d.x)
            ) && (
                (b.y >= a.y && b.y <= c.y) || (d.y >= a.y && d.y <= c.y)
            )
        ) || (
            (
                (b.x >= a.x && b.x <= c.x) || (d.x >= a.x && d.x <= c.x)
            ) && (
                (a.y >= b.y && a.y <= d.y) || (c.y >= b.y && c.y <= d.y)
            )
        )
    );
}