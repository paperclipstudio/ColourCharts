#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::shape::Polygon;
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};
use speedy2d::Graphics2D;
use speedy2d::Window;
use std::marker::PhantomData;

use core::f32;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::time::{self, Duration, SystemTime};

const UVEC_ORIGIN: UVec2 = UVec2 { x: 0, y: 0 };
const TOP_LEFT: Vec2 = Vec2 { x: 0.0, y: 0.0 };

struct MyWindowHandler {
    mouse_position: Vec2,
    window_size: UVec2,
    window_size_f32: Vec2,
    start_time: SystemTime,
}

impl MyWindowHandler {
    fn render_colour_chart(&self, graphics: &mut Graphics2D) {
        let colours = [
            Color::from_rgb(1.0, 0.0, 0.0),
            Color::from_rgb(0.0, 1.0, 0.0),
            Color::from_rgb(0.0, 0.0, 1.0),
            Color::from_rgb(1.0, 1.0, 0.0),
            Color::from_rgb(1.0, 0.0, 1.0),
            Color::from_rgb(0.0, 1.0, 1.0),
            Color::from_gray(0.0),
            Color::from_gray(0.5),
            Color::from_gray(1.0),
        ];
        let square_width = self.window_size_f32.x / 3.0;
        let square_height = self.window_size_f32.y / 3.0;

        for x in 0..3 {
            for y in 0..2 {
                let location = speedy2d::shape::Rectangle::new(
                    Vec2::new(square_width * x as f32, square_height * y as f32),
                    Vec2::new(
                        square_width * (x as f32 + 1.0),
                        square_height * (y as f32 + 1.0),
                    ),
                );
                let colour = colours[x + (y * 3)];
                graphics.draw_rectangle(location, colour);
                graphics.draw_quad_four_color(
                    [
                        Vec2::new(0.0, square_height * 2.0),
                        Vec2::new(self.window_size_f32.x, square_height * 2.0),
                        Vec2::new(self.window_size_f32.x, square_height * 3.0),
                        Vec2::new(0.0, square_height * 3.0),
                    ],
                    [
                        Color::from_gray(0.0),
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
                        Color::from_gray(0.0),
                    ],
                )
            }
        }
        graphics.draw_rectangle(
            speedy2d::shape::Rectangle::new(Vec2::new(20.0, 20.0), Vec2::new(40.0, 40.0)),
            Color::from_rgb(1.0, 0.0, 0.0),
        );
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper, window: WindowStartupInfo) {
        self.window_size = *window.viewport_size_pixels();
        self.window_size_f32 = window.viewport_size_pixels().into_f32();
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.window_size = size_pixels;
        self.window_size_f32 = size_pixels.into_f32();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let t = self.start_time.elapsed().unwrap().as_millis() % 10_000;
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        /*
                let cor = [
                    Vec2::new(0.0, self.window_size.into_f32().y),
                    Vec2::new(self.window_size_f32.x / 2.0, 0.0),
                    Vec2::new(self.window_size_f32.x, self.window_size_f32.y),
                ];
                let col = [
                    Color::from_rgb(1.0, 0.0, 0.0),
                    Color::from_rgb(0.0, 1.0, 0.0),
                    Color::from_rgb(0.0, 0.0, 1.0),
                ];
                graphics.draw_triangle_three_color(cor, col);
        */
        self.render_colour_chart(graphics);
        std::thread::sleep(Duration::from_millis(16));
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        self.mouse_position = position;
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        println!(
            "Clicked at {:.1} {:.1}",
            self.mouse_position.x, self.mouse_position.y
        );
        match button {
            MouseButton::Left => {}
            MouseButton::Right => helper.terminate_loop(),
            MouseButton::Middle => {}
            MouseButton::Other(_) => {}
        }
    }
}

fn main() {
    let window = Window::new_fullscreen_borderless("Colours").unwrap();
    let state = MyWindowHandler {
        window_size: UVEC_ORIGIN,
        window_size_f32: UVEC_ORIGIN.into_f32(),
        mouse_position: TOP_LEFT,
        start_time: SystemTime::now(),
    };
    window.run_loop(state);
}
