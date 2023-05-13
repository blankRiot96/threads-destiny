#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use raylib::{
    ffi::{GetMousePosition, IsMouseButtonPressed},
    prelude::*,
};
use std::time;
mod utils;
use rand::Rng;
use utils::Time;

const SCREEN_WIDTH: i32 = 1100;
const SCREEN_HEIGHT: i32 = 650;

fn move_towards(vec_1: &mut Vector2, vec_2: Vector2, distance: f32) {
    // Moves the first vector towards the other vector by a
    // certain distance
    let dx = vec_2.x - vec_1.x;
    let dy = vec_2.y - vec_1.y;
    let magnitude = (dx * dx + dy * dy).sqrt();

    if magnitude <= distance {
        // The first vector is already within the desired distance from the second vector
        return;
    }

    let direction_x = dx / magnitude;
    let direction_y = dy / magnitude;

    vec_1.x = vec_1.x + direction_x * distance;
    vec_1.y = vec_1.y + direction_y * distance;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Minijam Game")
        .build();

    let mut start = Vector2::new(30.0, 60.0);
    let mut end = Vector2::new(100.0, 120.0);
    let mut camera = Vector2::new(0.0, 0.0);
    let mut dt = 0.0;
    let mut raw_dt = 0.0;
    let mut chance = true;

    let mut squares: Vec<(Vector2, f32, f32)> = vec![];
    let mut spawn_timer = Time {
        time_to_pass: 1.0,
        ..Default::default()
    };

    while !rl.window_should_close() {
        let dt_start = time::Instant::now();
        let mut d = rl.begin_drawing(&thread);
        let mut mouse_pos = Vector2::zero();
        unsafe {
            mouse_pos = GetMousePosition().into();
            mouse_pos = mouse_pos + camera;
        }

        let midpoint = Vector2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
        // Camera smoothness
        camera.x += (midpoint.x - camera.x - (SCREEN_WIDTH as f32 / 2.0)) * 0.001;
        camera.y += (midpoint.y - camera.y - (SCREEN_HEIGHT as f32 / 2.0)) * 0.001;

        // On mouse clicking
        unsafe {
            if IsMouseButtonPressed(1) {
                chance = !chance;
                if chance {
                    start = mouse_pos;
                } else {
                    end = mouse_pos;
                }
            }
        }

        // Spawn and update squares
        if spawn_timer.tick() {
            let mut coord_x = rand::thread_rng().gen_range(0..=SCREEN_WIDTH) as f32;
            let mut coord_y = rand::thread_rng().gen_range(0..=SCREEN_HEIGHT) as f32;
            let chance = rand::thread_rng().gen_bool(0.5);
            if chance {
                coord_x = 0.0;
            } else {
                coord_y = 0.0;
            }

            let coord = Vector2::new(coord_x, coord_y);
            let radius = rand::thread_rng().gen_range(2..=5) as f32;
            let angle = rand::thread_rng().gen_range(0..=360) as f32;
            squares.push((coord, radius, angle));
        }

        // Update squares
        for square in &mut squares {
            square.0.x += square.2.cos() * 10.0 * dt;
            square.0.y += square.2.sin() * 10.0 * dt;
        }

        d.clear_background(Color::BLACK);

        // Draw squares
        for square in &mut squares {
            d.draw_rectangle_v(
                square.0,
                Vector2::new(square.1, square.1),
                Color::YELLOWGREEN,
            )
        }

        // Draw thread
        d.draw_line_ex(start - camera, end - camera, 6.0, Color::WHITE);
        d.draw_circle_v(start - camera, 10.0, Color::RED);
        d.draw_circle_v(end - camera, 10.0, Color::BLUE);

        // Framerate independence
        raw_dt = dt_start.elapsed().as_secs_f32();
        dt = raw_dt * 60.0;
    }
}
