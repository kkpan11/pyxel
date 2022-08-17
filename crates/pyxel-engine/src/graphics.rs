use array_macro::array;

use crate::image::{Image, SharedImage};
use crate::settings::{
    CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, FONT_DATA, FONT_HEIGHT, FONT_WIDTH, IMAGE_SIZE,
    NUM_FONT_ROWS, NUM_IMAGES, NUM_TILEMAPS, TILEMAP_SIZE,
};
use crate::tilemap::{SharedTilemap, Tilemap};
use crate::types::Color;
use crate::{Pyxel, SCREEN};

pub struct Graphics {
    images: [SharedImage; NUM_IMAGES as usize],
    tilemaps: [SharedTilemap; NUM_TILEMAPS as usize],
}

unsafe_singleton!(Graphics);

impl Graphics {
    pub fn init() {
        let images = array![_ => Image::new(IMAGE_SIZE, IMAGE_SIZE); NUM_IMAGES as usize];
        let tilemaps = array![_ => Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, images[0].clone()); NUM_TILEMAPS as usize];
        Self::set_instance(Self { images, tilemaps });
    }

    pub fn new_cursor_image() -> SharedImage {
        let image = Image::new(CURSOR_WIDTH, CURSOR_HEIGHT);
        image.lock().set(0, 0, &CURSOR_DATA);
        image
    }

    pub fn new_font_image() -> SharedImage {
        let width = FONT_WIDTH * NUM_FONT_ROWS;
        let height = FONT_HEIGHT * ((FONT_DATA.len() as u32 + NUM_FONT_ROWS - 1) / NUM_FONT_ROWS);
        let image = Image::new(width, height);
        {
            let mut image = image.lock();
            for (fi, data) in FONT_DATA.iter().enumerate() {
                let row = fi as u32 / NUM_FONT_ROWS;
                let col = fi as u32 % NUM_FONT_ROWS;
                let mut data = *data;
                for yi in 0..FONT_HEIGHT {
                    for xi in 0..FONT_WIDTH {
                        let x = FONT_WIDTH * col + xi;
                        let y = FONT_HEIGHT * row + yi;
                        let color = if (data & 0x800000) == 0 { 0 } else { 1 };
                        image.canvas.data[y as usize][x as usize] = color;
                        data <<= 1;
                    }
                }
            }
        }
        image
    }
}

impl Pyxel {
    pub fn image(&self, image_no: u32) -> SharedImage {
        Graphics::instance().images[image_no as usize].clone()
    }

    pub fn image_no(&self, image: SharedImage) -> Option<u32> {
        for (i, builtin_image) in Graphics::instance().images.iter().enumerate() {
            if builtin_image.data_ptr() == image.data_ptr() {
                return Some(i as u32);
            }
        }
        None
    }

    pub fn tilemap(&self, image_no: u32) -> SharedTilemap {
        Graphics::instance().tilemaps[image_no as usize].clone()
    }

    pub fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        SCREEN.lock().clip(x, y, width, height);
    }

    pub fn clip0(&mut self) {
        SCREEN.lock().clip0();
    }

    pub fn camera(&mut self, x: f64, y: f64) {
        SCREEN.lock().camera(x, y);
    }

    pub fn camera0(&mut self) {
        SCREEN.lock().camera0();
    }

    pub fn pal(&mut self, src_color: Color, dst_color: Color) {
        SCREEN.lock().pal(src_color, dst_color);
    }

    pub fn pal0(&mut self) {
        SCREEN.lock().pal0();
    }

    pub fn cls(&mut self, color: Color) {
        SCREEN.lock().cls(color);
    }

    pub fn pget(&mut self, x: f64, y: f64) -> Color {
        SCREEN.lock().pget(x, y)
    }

    pub fn pset(&mut self, x: f64, y: f64, color: Color) {
        SCREEN.lock().pset(x, y, color);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        SCREEN.lock().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        SCREEN.lock().rect(x, y, width, height, color);
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        SCREEN.lock().rectb(x, y, width, height, color);
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        SCREEN.lock().circ(x, y, radius, color);
    }

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        SCREEN.lock().circb(x, y, radius, color);
    }

    pub fn elli(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        SCREEN.lock().elli(x, y, width, height, color);
    }

    pub fn ellib(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        SCREEN.lock().ellib(x, y, width, height, color);
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        SCREEN.lock().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        SCREEN.lock().trib(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn fill(&mut self, x: f64, y: f64, color: Color) {
        SCREEN.lock().fill(x, y, color);
    }

    pub fn blt(
        &mut self,
        x: f64,
        y: f64,
        image_no: u32,
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        SCREEN.lock().blt(
            x,
            y,
            Graphics::instance().images[image_no as usize].clone(),
            image_x,
            image_y,
            width,
            height,
            color_key,
        );
    }

    pub fn bltm(
        &mut self,
        x: f64,
        y: f64,
        tilemap_no: u32,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        SCREEN.lock().bltm(
            x,
            y,
            Graphics::instance().tilemaps[tilemap_no as usize].clone(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            color_key,
        );
    }

    pub fn text(&mut self, x: f64, y: f64, string: &str, color: Color) {
        SCREEN.lock().text(x, y, string, color);
    }
}
