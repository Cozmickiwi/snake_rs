use std::{cmp::min, f32::consts::PI, time::Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const SCREEN_WIDTH: u16 = 600;
const SCREEN_HEIGHT: u16 = 600;
//const HALF_HEIGHT: u16 = 450;
const SCALE: u16 = 1;
const CLEAR_COLOR: [u8; 3] = [0, 0, 0];

struct Snake {
    coords: [u32; 2],
    history: Vec<[u32; 2]>,
}

fn main() {
    let mut snake = Snake {
        coords: [1, 5],
        history: Vec::new(),
    };
    let mut frame_count: u8 = 0;
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
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
    event_loop
        .run(move |event, elwt| {
            //            print!("\r");
            //            let now = Instant::now();
            for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                pixel[0] = CLEAR_COLOR[0]; // R
                pixel[1] = CLEAR_COLOR[1]; // G
                pixel[2] = CLEAR_COLOR[2]; // B
                pixel[3] = 0xff; // A
            }
            frame_count += 1;
            draw_snake(&snake, &window_size, pixels.frame_mut());
            /*
            if snake.coords[0] < 14 && frame_count == 20 {
                snake.coords[0] += 1;
                frame_count = 0;
            }
            */
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
            //            print!("{:?}fps", (1.0 / now.elapsed().as_secs_f32()) as u32);
            if wasd[0] {
                current_dir = 'u';
            } else if wasd[1] {
                current_dir = 'l';
            } else if wasd[2] {
                current_dir = 'd';
            } else if wasd[3] {
                current_dir = 'r';
            }
            if frame_count == 20 {
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
                frame_count = 0;
            }
        })
        .unwrap();
}

//const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];
const BLUE1: [u8; 4] = [25, 122, 154, 0];
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
        (snake.coords[0] * TILE_SIZE as u32),
        (snake.coords[1] * TILE_SIZE as u32),
        TILE_SIZE,
        TILE_SIZE,
        PURPLE1,
    );
}
