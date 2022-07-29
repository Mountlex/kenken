use std::path::PathBuf;

use crate::{
    gen::DifficultyConfig,
    kenken::{Field, KenKen, Type},
};
use anyhow::Result;
use image::{GrayImage, ImageBuffer, Luma};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};

const BLACK: Luma<u8> = Luma([0]);
const WHITE: Luma<u8> = Luma([255]);

pub struct DrawConfig {
    field_size: u16,
    thick: u16,
    thin: u16,
    offset: u16,
    target_x: u16,
    target_y: u16,
}

pub const DEFAULT_CONFIG: DrawConfig = DrawConfig {
    field_size: 200,
    thick: 12,
    thin: 2,
    offset: 50,
    target_x: 20,
    target_y: 20,
};

pub fn draw(
    kenken: &KenKen,
    file: &PathBuf,
    config: &DrawConfig,
    gen_config: Option<&DifficultyConfig>,
) -> Result<()> {
    let image_size = kenken.size * config.field_size + 2 * config.offset;
    let mut img: GrayImage =
        ImageBuffer::from_pixel(image_size as u32, image_size as u32 + 100, WHITE);

    let font = Vec::from(include_bytes!("../assets/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let font_size = 50.0;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    print_horizontal_separators(&mut img, kenken, 0, config);
    print_vertical_separators(&mut img, kenken, 0, config);

    for i in 0..=kenken.size {
        print_vertical_separators(&mut img, kenken, i + 1, config);
        print_horizontal_separators(&mut img, kenken, i + 1, config);
    }

    print_targets(&mut img, kenken, config, scale, &font);

    if let Some(gen_config) = gen_config {
        draw_text_mut(
            &mut img,
            BLACK,
            config.offset as i32,
            10,
            scale,
            &font,
            &format!(
                "KnKn {} (s = {}, add = {}, mul = {}, sub = {}, div = {})",
                kenken.id,
                gen_config.size_factor,
                gen_config.p_add,
                gen_config.p_mul,
                gen_config.p_sub,
                gen_config.p_div
            ),
        );
    } else {
        draw_text_mut(
            &mut img,
            BLACK,
            config.offset as i32,
            10,
            scale,
            &font,
            &format!("KnKn {}", kenken.id),
        );
    }

    img.save(file)?;
    Ok(())
}

fn print_horizontal_separators(
    c: &mut GrayImage,
    kenken: &KenKen,
    after_row: u16,
    config: &DrawConfig,
) {
    for i in 0..kenken.size {
        if after_row == 0 || !kenken.same_area(&Field(i, after_row - 1), &Field(i, after_row)) {
            draw_filled_rect_mut(
                c,
                Rect::at(
                    (i * config.field_size + config.offset) as i32,
                    (after_row * config.field_size + config.offset + 100) as i32,
                )
                .of_size(config.field_size as u32, config.thick as u32),
                BLACK,
            );
        } else {
            draw_filled_rect_mut(
                c,
                Rect::at(
                    (i * config.field_size + config.offset) as i32,
                    (after_row * config.field_size + config.offset + 100) as i32,
                )
                .of_size(config.field_size as u32, config.thin as u32),
                BLACK,
            );
        }
    }
}

fn print_vertical_separators(
    c: &mut GrayImage,
    kenken: &KenKen,
    after_column: u16,
    config: &DrawConfig,
) {
    for i in 0..kenken.size {
        if after_column == 0
            || !kenken.same_area(&Field(after_column - 1, i), &Field(after_column, i))
        {
            draw_filled_rect_mut(
                c,
                Rect::at(
                    (after_column * config.field_size + config.offset) as i32,
                    (i * config.field_size + config.offset + 100) as i32,
                )
                .of_size(config.thick as u32, config.field_size as u32),
                BLACK,
            );
        } else {
            draw_filled_rect_mut(
                c,
                Rect::at(
                    (after_column * config.field_size + config.offset) as i32,
                    (i * config.field_size + config.offset + 100) as i32,
                )
                .of_size(config.thin as u32, config.field_size as u32),
                BLACK,
            );
        }
    }
}

fn print_targets<'a>(
    c: &mut GrayImage,
    kenken: &KenKen,
    config: &DrawConfig,
    scale: Scale,
    font: &'a Font<'a>,
) {
    for i in 0..kenken.size {
        for j in 0..kenken.size {
            if let Some(area) = kenken.is_id_field(Field(i, j)) {
                let text = match area.ty {
                    Type::Add => format!("{}{:<width$}", "+", area.solution, width = 20 - 1),
                    Type::Mul => format!("{}{:<width$}", "*", area.solution, width = 20 - 1),
                    Type::Div => format!("{}{:<width$}", "÷", area.solution, width = 20 - 1),
                    Type::Sub => format!("{}{:<width$}", "-", area.solution, width = 20 - 1),
                    Type::Single => format!("{:<width$}", area.solution, width = 20),
                };
                draw_text_mut(
                    c,
                    BLACK,
                    (config.offset + i * config.field_size + config.target_x) as i32,
                    (config.offset + j * config.field_size + config.target_y + 100) as i32,
                    scale,
                    font,
                    &text,
                );
            }
        }
    }
}
