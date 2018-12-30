use image::{RgbImage, Rgb};

use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::{Rect};

const BORDER_WIDTH: u32 = 3;

#[derive(Debug)]
pub struct Dialog {
    pub text: &'static str,

    pub image: RgbImage,
    pub current_char: char,

    pub x: u16,
    pub y: u16,

    pub is_2x: bool, // required for japanese font to not be squished down
}

impl Dialog {
    pub fn new(text: &'static str, is_2x: bool) -> Dialog {
        let white = Rgb([255u8, 255u8, 255u8]);

        let mut image = RgbImage::new(289, 76);

        // draw left border
        let rect = Rect::at(0, 0)
            .of_size(BORDER_WIDTH, image.height());
        draw_filled_rect_mut(&mut image, rect, white);

        // draw right border
        let rect = Rect::at((image.width() - BORDER_WIDTH) as i32, 0)
            .of_size(BORDER_WIDTH, image.height());
        draw_filled_rect_mut(&mut image, rect, white);

        // draw upper border
        let rect = Rect::at(BORDER_WIDTH as i32, 0)
            .of_size(image.width() - (BORDER_WIDTH * 2), BORDER_WIDTH);
        draw_filled_rect_mut(&mut image, rect, white);

        // draw bottom border
        let rect = Rect::at(BORDER_WIDTH as i32, (image.height() - BORDER_WIDTH) as i32)
            .of_size(image.width() - (BORDER_WIDTH * 2), BORDER_WIDTH);
        draw_filled_rect_mut(&mut image, rect, white);

        Dialog {
            text: text,
            image: image,
            current_char: ' ',
            x: 0, y: 0,
            is_2x,
        }
    }

    pub fn advance_frame(&mut self) {
        unimplemented!();
    }
}
