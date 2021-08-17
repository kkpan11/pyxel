use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::canvas::Canvas;
use crate::rectarea::RectArea;
use crate::settings::COLOR_COUNT;
use crate::tilemap::Tilemap;
use crate::types::{Color, Rgb8, Tile};
use crate::utility::{parse_hex_string, simplify_string};

pub struct Image {
    self_rect: RectArea,
    clip_rect: RectArea,
    data: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            data: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub fn with_arc_mutex(width: u32, height: u32) -> Arc<Mutex<Image>> {
        Arc::new(Mutex::new(Image::new(width, height)))
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = data_str[0].len() as u32;
        let height = data_str.len() as u32;
        let mut dst_image = Image::new(width, height);

        for i in 0..height {
            let src_data = simplify_string(data_str[i as usize]);

            for j in 0..width {
                if let Some(value) = parse_hex_string(&src_data[j as usize..j as usize + 1]) {
                    dst_image._set_value(j as i32, i as i32, value as Color);
                } else {
                    panic!("invalid image data");
                }
            }
        }

        self.blt(
            x,
            y,
            &dst_image,
            0,
            0,
            width as i32,
            height as i32,
            None,
            None,
        );
    }

    pub fn load(&mut self, x: i32, y: i32, filename: &str, colors: &[Rgb8]) {
        let src_image = image::open(&Path::new(&filename)).unwrap().to_rgb8();
        let (width, height) = src_image.dimensions();
        let mut dst_image = Image::new(width, height);
        let mut color_table = HashMap::<(u8, u8, u8), Color>::new();

        for i in 0..height {
            for j in 0..width {
                let p = src_image.get_pixel(j, i);
                let src_rgb = (p[0], p[1], p[2]);

                if let Some(color) = color_table.get(&src_rgb) {
                    dst_image._set_value(j as i32, i as i32, *color);
                } else {
                    let mut closest_color: Color = 0;
                    let mut closest_dist: f64 = f64::MAX;

                    for k in 0..=COLOR_COUNT {
                        let pal_color = colors[k as usize];
                        let pal_rgb = (
                            ((pal_color >> 16) & 0xff) as u8,
                            ((pal_color >> 8) & 0xff) as u8,
                            (pal_color & 0xff) as u8,
                        );
                        let dist = Image::color_dist(src_rgb, pal_rgb);

                        if dist < closest_dist {
                            closest_color = k as Color;
                            closest_dist = dist;
                        }
                    }

                    color_table.insert(src_rgb, closest_color);
                    dst_image._set_value(j as i32, i as i32, closest_color);
                }
            }
        }

        self.blt(
            x,
            y,
            &dst_image,
            0,
            0,
            width as i32,
            height as i32,
            None,
            None,
        );
    }

    fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
        let (r1, g1, b1) = rgb1;
        let (r2, g2, b2) = rgb2;

        let dx = (r1 as f64 - r2 as f64) * 0.30;
        let dy = (g1 as f64 - g2 as f64) * 0.59;
        let dz = (b1 as f64 - b2 as f64) * 0.11;

        dx * dx + dy * dy + dz * dz
    }

    pub fn save(&self, filename: &str, colors: &[Rgb8], scale: u32) {
        let _ = (filename, colors, scale); // dummy

        //
    }

    pub fn bltm(
        &mut self,
        x: i32,
        y: i32,
        tilemap: &Tilemap,
        tilemap_x: i32,
        tilemap_y: i32,
        width: i32,
        height: i32,
        transparent: Option<Tile>,
    ) {
        let _ = (
            x,
            y,
            tilemap,
            tilemap_x,
            tilemap_y,
            width,
            height,
            transparent,
        ); // dummy

        /*
        Tilemap* tilemap = GetTilemapBank(tilemap_index);
        int32_t image_index = tilemap->ImageIndex();

        int32_t left = clip_area_.Left() / TILEMAP_CHIP_WIDTH;
        int32_t top = clip_area_.Top() / TILEMAP_CHIP_WIDTH;
        int32_t right =
            (clip_area_.Right() + TILEMAP_CHIP_WIDTH - 1) / TILEMAP_CHIP_WIDTH;
        int32_t bottom =
            (clip_area_.Bottom() + TILEMAP_CHIP_HEIGHT - 1) / TILEMAP_CHIP_HEIGHT;
        Rectangle dst_rect = Rectangle(left, top, right - left + 1, bottom - top + 1);

        Rectangle::CopyArea copy_area =
            dst_rect.GetCopyArea(x / TILEMAP_CHIP_WIDTH, y / TILEMAP_CHIP_HEIGHT,
                                 tilemap->Rectangle(), u, v, width, height);

        if (copy_area.IsEmpty()) {
          return;
        }

        int32_t** src_data = tilemap->Data();

        copy_area.x = copy_area.x * TILEMAP_CHIP_WIDTH + x % TILEMAP_CHIP_WIDTH;
        copy_area.y = copy_area.y * TILEMAP_CHIP_HEIGHT + y % TILEMAP_CHIP_HEIGHT;

        for (int32_t i = 0; i < copy_area.height; i++) {
          int32_t* src_line = src_data[copy_area.v + i];
          int32_t dst_y = copy_area.y + TILEMAP_CHIP_HEIGHT * i;

          for (int32_t j = 0; j < copy_area.width; j++) {
            int32_t chip = src_line[copy_area.u + j];
            int32_t cu =
                (chip % (IMAGE_BANK_WIDTH / TILEMAP_CHIP_WIDTH)) * TILEMAP_CHIP_WIDTH;
            int32_t cv = (chip / (IMAGE_BANK_HEIGHT / TILEMAP_CHIP_HEIGHT)) *
                         TILEMAP_CHIP_HEIGHT;

            DrawImage(copy_area.x + TILEMAP_CHIP_WIDTH * j, dst_y, image_index, cu,
                      cv, TILEMAP_CHIP_WIDTH, TILEMAP_CHIP_HEIGHT, color_key);
          }
        }
        */
    }

    pub fn text(&mut self, x: i32, y: i32, string: &str, color: Color, font: &Image) {
        let _ = (x, y, string, color, font); // dummy

        /*
        int32_t draw_color = GET_DRAW_COLOR(color);
        int32_t cur_color = palette_table_[FONT_COLOR];
        palette_table_[FONT_COLOR] = draw_color;

        int32_t left = x;

        for (const char* ch = text; *ch != '\0'; ch++) {
          if (*ch == 10) {  // new line
            x = left;
            y += FONT_HEIGHT;
            continue;
          }

          if (*ch == 32) {  // space
            x += FONT_WIDTH;
            continue;
          }

          if (*ch < MIN_FONT_CODE || *ch > MAX_FONT_CODE) {
            continue;
          }

          int32_t code = *ch - MIN_FONT_CODE;
          int32_t u = (code % FONT_ROW_COUNT) * FONT_WIDTH;
          int32_t v = (code / FONT_ROW_COUNT) * FONT_HEIGHT;

          DrawImage(x, y, IMAGE_BANK_FOR_SYSTEM, FONT_X + u, FONT_Y + v, FONT_WIDTH,
                    FONT_HEIGHT, 0);

          x += FONT_WIDTH;
        }

        palette_table_[FONT_COLOR] = cur_color;
        */
    }
}

impl Canvas<Color> for Image {
    fn width(&self) -> u32 {
        self.self_rect.width()
    }

    fn height(&self) -> u32 {
        self.self_rect.height()
    }

    fn _value(&self, x: i32, y: i32) -> Color {
        self.data[y as usize][x as usize]
    }

    fn _set_value(&mut self, x: i32, y: i32, color: Color) {
        self.data[y as usize][x as usize] = color;
    }

    fn _self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn _clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn _clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }
}
