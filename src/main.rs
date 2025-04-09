#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use colortemp;
use rand::seq::SliceRandom;
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::font::Font;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use speedy2d::shape::{Polygon, Rectangle};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};
use speedy2d::Graphics2D;
use speedy2d::Window;
use std::marker::PhantomData;

use core::f32;
mod colour_temp;
use std::borrow::{Borrow, BorrowMut};
use std::ops::DerefMut;
use std::rc::Rc;
use std::time::{self, Duration, SystemTime};

const UVEC_ORIGIN: UVec2 = UVec2 { x: 0, y: 0 };
const TOP_LEFT: Vec2 = Vec2 { x: 0.0, y: 0.0 };

const SCREENS: [&str; 7] = [
    "WHITE",
    "RGB Triangle",
    "CYM Triangle",
    "Colour Chart variable",
    "Gray Tones",
    "Colour chart static",
    "AWB Colours",
];

struct MyWindowHandler {
    mouse_position: Vec2,
    window_size: UVec2,
    window_size_f32: Vec2,
    start_time: SystemTime,
    count: u32,
    hold: bool,
    last_delta: f32,
    font: Font,
}

fn scale_color(color: Color, scale: f32) -> Color {
    Color::from_rgb(color.r() * scale, color.g() * scale, color.b() * scale)
}
impl MyWindowHandler {
    fn render_awb_colour_chart(&self, graphics: &mut Graphics2D) {
        let padding = 0.3;
        const BARS: usize = 7;
        let delta = (self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0) % 3.0;
        let colours = (1..5).map(|x| (x as f32) / 5.0);
        let square_width = (self.window_size_f32.x * (1.0 - (2.0 * padding))) / BARS as f32;
        let square_height = self.window_size_f32.y * (1.0 - (2.0 * padding));

        let padding_x = self.window_size_f32.x * padding;
        let padding_y = self.window_size_f32.y * padding + 400.0;
        let padding = Vec2::new(padding_x, padding_y);

        let temp: f32 = self.last_delta * 1_000.0;
        let temp_str = format!("{}", temp);
        let colour = colour_temp::get_colour(temp);
        let colour_tmp = colortemp::temp_to_rgb(temp as i64);
        let colour = Color::from_rgb(
            colour_tmp.r as f32 / 255.0,
            colour_tmp.g as f32 / 255.0,
            colour_tmp.b as f32 / 255.0,
        );
        println!("Colour {:?} temp{}", colour, temp);
        graphics.draw_rectangle(Rectangle::new(Vec2::ZERO, self.window_size_f32), colour);

        let block = self
            .font
            .layout_text(temp_str.as_str(), 32.0, TextOptions::new());

        graphics.draw_text(self.window_size_f32 / 2.0, Color::BLACK, &block);

        for x in 0..BARS + 1 {
            let location = speedy2d::shape::Rectangle::new(
                Vec2::new(square_width * x as f32, 0.0) + padding,
                Vec2::new(square_width * (x as f32 + 1.0), square_height) + padding,
            );

            let colour_scaled = scale_color(colour, 1.0 / BARS as f32 * x as f32);
            graphics.draw_rectangle(location, colour_scaled);
        }
    }
    fn render_colour_chart_var(&self, graphics: &mut Graphics2D) {
        let padding = 0.3;
        let delta = (self.start_time.elapsed().unwrap().as_millis() as f32 / 1000.0) % 3.0;
        let blacks = f32::max(0.0, delta - 2.0);
        let colors = f32::min(1.0, delta);
        let colours = [
            Color::from_rgb(colors, blacks, blacks),
            Color::from_rgb(blacks, colors, blacks),
            Color::from_rgb(blacks, blacks, colors),
            Color::from_rgb(colors, colors, blacks),
            Color::from_rgb(colors, blacks, colors),
            Color::from_rgb(blacks, colors, colors),
            Color::from_gray(colors),
            Color::from_gray(0.5),
            Color::from_gray(colors),
        ];
        let square_width = (self.window_size_f32.x * (1.0 - (2.0 * padding))) / 3.0;
        let square_height = (self.window_size_f32.y * (1.0 - (2.0 * padding))) / 3.0;

        let padding_x = self.window_size_f32.x * padding;
        let padding_y = self.window_size_f32.y * padding + 400.0;
        let padding = Vec2::new(padding_x, padding_y);

        for x in 0..3 {
            for y in 0..2 {
                let location = speedy2d::shape::Rectangle::new(
                    Vec2::new(square_width * x as f32, square_height * y as f32) + padding,
                    Vec2::new(
                        square_width * (x as f32 + 1.0),
                        square_height * (y as f32 + 1.0),
                    ) + padding,
                );
                let colour = colours[x + (y * 3)];
                graphics.draw_rectangle(location, colour);
                graphics.draw_quad_four_color(
                    [
                        Vec2::new(self.window_size_f32.x, square_height * 2.0) + padding,
                        Vec2::new(self.window_size_f32.x, square_height * 2.0) + padding,
                        Vec2::new(self.window_size_f32.x, square_height * 3.0) + padding,
                        Vec2::new(0.0, square_height * 3.0) + padding,
                    ],
                    [
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
                    ],
                )
            }
        }
    }
    fn render_grays_changing(&mut self, graphics: &mut Graphics2D) {
        let delta = if !self.hold {
            self.last_delta
        } else {
            (self.start_time.elapsed().unwrap().as_millis() as f32 / 5000.0) % 3.0
        };
        self.last_delta = delta;
        let tone = Color::from_gray(delta / 3.0);
        //let tone = Color::from_rgb(1.0, 1.0, delta / 3.0);
        graphics.draw_rectangle(Rectangle::new(Vec2::ZERO, self.window_size_f32), tone);
    }
    fn render_colour_chart(&self, graphics: &mut Graphics2D) {
        let padding = 0.3;
        let colours = [
            Color::from_rgb(1.0, 0.0, 0.0),
            Color::from_rgb(0.0, 1.0, 0.0),
            Color::from_rgb(0.0, 0.0, 1.0),
            Color::from_rgb(1.0, 1.0, 0.0),
            Color::from_rgb(1.0, 0.0, 1.0),
            Color::from_rgb(0.0, 1.0, 1.0),
        ];

        //let mut rng = rand::rng();
        //       colours.shuffle(&mut rng);
        let square_width = (self.window_size_f32.x * (1.0 - (2.0 * padding))) / 3.0;
        let square_height = (self.window_size_f32.y * (1.0 - (2.0 * padding))) / 3.0;

        let padding_x = self.window_size_f32.x * padding;
        let padding_y = self.window_size_f32.y * padding + 400.0;
        let padding = Vec2::new(padding_x, padding_y);

        for x in 0..3 {
            for y in 0..2 {
                let location = speedy2d::shape::Rectangle::new(
                    Vec2::new(square_width * x as f32, square_height * y as f32) + padding,
                    Vec2::new(
                        square_width * (x as f32 + 1.0),
                        square_height * (y as f32 + 1.0),
                    ) + padding,
                );
                let colour = colours[x + (y * 3)];
                graphics.draw_rectangle(location, colour);
                graphics.draw_quad_four_color(
                    [
                        Vec2::new(self.window_size_f32.x, square_height * 2.0) + padding,
                        Vec2::new(self.window_size_f32.x, square_height * 2.0) + padding,
                        Vec2::new(self.window_size_f32.x, square_height * 3.0) + padding,
                        Vec2::new(0.0, square_height * 3.0) + padding,
                    ],
                    [
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
                        Color::from_gray(1.0),
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
        const PADDING: Vec2 = Vec2::new(30.0, 30.0);
        const MARGIN: Vec2 = Vec2::new(20.0, 20.0);

        let t = self.start_time.elapsed().unwrap().as_millis() % 10_000;
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        //      graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));
        graphics.draw_rounded_rectangle(
            Rectangle::new(Vec2::ZERO, self.window_size_f32)
                .rounded(self.window_size_f32.magnitude() * 0.015),
            Color::WHITE,
        );

        match self.count % 7 {
            0 => {
                self.render_awb_colour_chart(graphics);
            }
            1 => {
                let cor = [
                    Vec2::new(MARGIN.x, self.window_size.into_f32().y - MARGIN.y),
                    Vec2::new(self.window_size_f32.x / 2.0, MARGIN.y),
                    Vec2::new(self.window_size_f32.x, self.window_size_f32.y) - MARGIN,
                ];
                let col = [
                    Color::from_rgb(1.0, 0.0, 0.0),
                    Color::from_rgb(0.0, 1.0, 0.0),
                    Color::from_rgb(0.0, 0.0, 1.0),
                ];
                graphics.draw_triangle_three_color(cor, col);
            }
            2 => {
                let cor = [
                    Vec2::new(MARGIN.x, self.window_size.into_f32().y - MARGIN.y),
                    Vec2::new(self.window_size_f32.x / 2.0, MARGIN.y),
                    Vec2::new(self.window_size_f32.x, self.window_size_f32.y) - MARGIN,
                ];
                let col = [
                    Color::from_rgb(0.0, 1.0, 1.0),
                    Color::from_rgb(1.0, 0.0, 1.0),
                    Color::from_rgb(1.0, 1.0, 0.0),
                ];
                graphics.draw_triangle_three_color(cor, col);
            }
            3 => self.render_colour_chart_var(graphics),
            4 => self.render_grays_changing(graphics),
            5 => self.render_awb_colour_chart(graphics),
            _ => self.render_colour_chart(graphics),
        }

        let text = format!(
            "Screen {}\n{}",
            self.count % (SCREENS.len() as u32),
            SCREENS
                .get((self.count % (SCREENS.len() as u32)) as usize)
                .unwrap_or(&"N/A")
        );

        let block = self
            .font
            .layout_text(text.as_str(), 32.0, TextOptions::new());

        graphics.draw_rounded_rectangle(
            Rectangle::new(
                Vec2::ZERO + MARGIN + Vec2::new(-1.0, -1.0),
                block.size() + MARGIN + (PADDING * 2.0) + Vec2::new(4.0, 4.0),
            )
            .rounded(10.0),
            Color::from_gray(0.70),
        );

        graphics.draw_rounded_rectangle(
            Rectangle::new(Vec2::ZERO + MARGIN, block.size() + MARGIN + (PADDING * 2.0))
                .rounded(10.0),
            Color::WHITE,
        );

        graphics.draw_text(Vec2::ZERO + MARGIN + PADDING, Color::BLACK, &block);
        std::thread::sleep(Duration::from_millis(16));
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        self.mouse_position = position;
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<()>,
        button: Option<speedy2d::window::VirtualKeyCode>,
        another: u32,
    ) {
        match button {
            None => {}
            Some(VirtualKeyCode::Q) => helper.terminate_loop(),
            Some(_) => {}
        }
        self.hold = !self.hold;
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<()>,
        distance: MouseScrollDistance,
    ) {
        let change = match distance {
            MouseScrollDistance::Lines { x: _, y, z: _ } => y as f32,
            MouseScrollDistance::Pixels { x: _, y, z: _ } => y as f32,
            MouseScrollDistance::Pages { x: _, y, z: _ } => y as f32,
        };
        println!("{:?}", distance);
        self.last_delta += change * 0.05;
        println!("{} {}", self.last_delta, change);
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        println!(
            "Clicked at {:.1} {:.1}",
            self.mouse_position.x, self.mouse_position.y
        );
        match button {
            MouseButton::Left => {}
            MouseButton::Right => helper.terminate_loop(),
            MouseButton::Middle => {
                self.count += 1;
            }
            MouseButton::Other(_) => {}
        }
    }
}

fn main() {
    let window = Window::new_fullscreen_borderless("Colours").unwrap();
    let bytes = include_bytes!("./Boldonse-Regular.ttf");
    let font = Font::new(bytes).unwrap();
    let state = MyWindowHandler {
        window_size: UVEC_ORIGIN,
        window_size_f32: UVEC_ORIGIN.into_f32(),
        mouse_position: TOP_LEFT,
        start_time: SystemTime::now(),
        count: 0,
        hold: false,
        last_delta: 0.0,
        font,
    };
    window.run_loop(state);
}
