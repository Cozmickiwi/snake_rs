use std::{cmp::min, collections::VecDeque};

use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const SCREEN_WIDTH: u16 = 600;
const SCREEN_HEIGHT: u16 = 600;
//const HALF_HEIGHT: u16 = 450;
//const SCALE: u16 = 1;
const CLEAR_COLOR: [u8; 3] = [0, 0, 0];

struct Snake {
    coords: [u32; 2],
    history: VecDeque<[u32; 2]>,
    length: u8,
}

fn main() {
    let mut snake = Snake {
        coords: [2, 5],
        history: VecDeque::new(),
        length: 1,
    };
    let mut frame_count: u8 = 0;
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("Snake")
        .build(&event_loop)
        .unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let window_size = builder.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &builder);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
    for pixel in pixels.frame_mut().chunks_exact_mut(4) {
        pixel[0] = CLEAR_COLOR[0]; // R
        pixel[1] = CLEAR_COLOR[1]; // G
        pixel[2] = CLEAR_COLOR[2]; // B
        pixel[3] = 0xff; // A
    }
    println!("{}", window_size.width);
    let mut wasd: [bool; 4] = [false, false, false, false];
    let mut current_dir: char = 'r';
    let mut rng = rand::thread_rng();
    let mut valid_pos = false;
    let mut apple_pos: [u32; 2] = [0, 0];
    while !valid_pos {
        apple_pos[0] = rng.gen_range(0..15);
        apple_pos[1] = rng.gen_range(0..15);
        if apple_pos != snake.coords && !snake.history.contains(&apple_pos) {
            valid_pos = true;
        }
    }
    valid_pos = false;
    event_loop
        .run(move |event, elwt| {
            for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                pixel[0] = CLEAR_COLOR[0]; // R
                pixel[1] = CLEAR_COLOR[1]; // G
                pixel[2] = CLEAR_COLOR[2]; // B
                pixel[3] = 0xff; // A
            }
            frame_count += 1;
            draw_snake(&snake, &window_size, pixels.frame_mut());
            draw_square(
                pixels.frame_mut(),
                &window_size,
                apple_pos[0] * TILE_SIZE as u32,
                apple_pos[1] * TILE_SIZE as u32,
                TILE_SIZE,
                TILE_SIZE,
                RED1,
            );
            pixels.render().unwrap();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                }
                Event::AboutToWait => {
                    builder.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    builder.request_redraw();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    } => match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => {
                            if event.state.is_pressed() {
                                wasd[1] = true;
                            } else {
                                wasd[1] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => {
                            if event.state.is_pressed() {
                                wasd[3] = true;
                            } else {
                                wasd[3] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => {
                            if event.state.is_pressed() {
                                wasd[0] = true;
                            } else {
                                wasd[0] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => {
                            if event.state.is_pressed() {
                                wasd[2] = true;
                            } else {
                                wasd[2] = false
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => (),
            }
            if wasd[0] {
                current_dir = 'u';
            } else if wasd[1] {
                current_dir = 'l';
            } else if wasd[2] {
                current_dir = 'd';
            } else if wasd[3] {
                current_dir = 'r';
            }
            if frame_count == 10 {
                if snake.length > 1 {
                    snake.history.pop_back().unwrap();
                    snake.history.push_front(snake.coords);
                }
                if current_dir == 'd' {
                    if snake.coords[1] == 0 {
                        panic!("You lose!");
                    }
                    snake.coords[1] -= 1;
                } else if current_dir == 'u' {
                    if snake.coords[1] == 14 {
                        panic!("You lose!");
                    }
                    snake.coords[1] += 1;
                } else if current_dir == 'l' {
                    if snake.coords[0] == 0 {
                        panic!("You lose!");
                    }
                    snake.coords[0] -= 1;
                } else if current_dir == 'r' {
                    if snake.coords[0] == 14 {
                        panic!("You lose!");
                    }
                    snake.coords[0] += 1;
                }
                if snake.history.contains(&snake.coords) {
                    panic!();
                }
                if snake.coords == apple_pos {
                    snake.length += 1;
                    println!("{}", snake.length);
                    snake.history.push_front(snake.coords);
                    while !valid_pos {
                        apple_pos[0] = rng.gen_range(0..15);
                        apple_pos[1] = rng.gen_range(0..15);
                        if apple_pos != snake.coords && !snake.history.contains(&apple_pos) {
                            valid_pos = true;
                        }
                    }
                    valid_pos = false;
                }
                frame_count = 0;
            }
        })
        .unwrap();
}

//const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];
//const BLUE1: [u8; 4] = [25, 122, 154, 0];
const PURPLE1: [u8; 4] = [131, 60, 169, 0];
const RED1: [u8; 4] = [154, 25, 70, 0];

fn draw_square(
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    x: u32,
    y: u32,
    width: usize,
    height: usize,
    color: [u8; 4],
) {
    let w_height = window_size.height;
    let mut new_x: usize;
    let mut new_y: usize;
    let mut pixel_index: usize;
    for row in (0..min(height, window_size.height as usize)).rev() {
        for a in 0..width {
            new_x = x as usize + a;
            new_y = w_height as usize - (y as usize + row);
            pixel_index = (new_y * window_size.width as usize + (new_x)) * 4;
            if pixel_index > frame.len() - 3 {
                break;
            }
            for i in frame[pixel_index..pixel_index + 4].chunks_exact_mut(4) {
                i[0] = color[0];
                i[1] = color[1];
                i[2] = color[2];
            }
        }
    }
}

const TILE_SIZE: usize = 50;

fn draw_snake(snake: &Snake, window_size: &PhysicalSize<u32>, frame: &mut [u8]) {
    draw_square(
        frame,
        window_size,
        snake.coords[0] * TILE_SIZE as u32,
        snake.coords[1] * TILE_SIZE as u32,
        TILE_SIZE,
        TILE_SIZE,
        PURPLE1,
    );
    for i in snake.history.iter() {
        draw_square(
            frame,
            window_size,
            i[0] * TILE_SIZE as u32,
            i[1] * TILE_SIZE as u32,
            TILE_SIZE,
            TILE_SIZE,
            PURPLE1,
        );
    }
}
