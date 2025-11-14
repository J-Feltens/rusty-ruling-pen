use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::ascii::escape_default;
use std::collections::LinkedList;
use std::process::exit;

use crate::graphics::colors::rgb2u32;
use crate::graphics::scanline::{ActiveEdgeTable, ActiveEdgeTableEntry, EdgeTableEntry};
use crate::graphics::{
    BLACK, BLUE, CYAN, Canvas, Color, EdgeTable, GREEN, MAGENTA, RED, WHITE, YELLOW,
};
use crate::vectors::Vector2d;

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 32;
const SIZE_Y: usize = 32;
const SCALE: minifb::Scale = minifb::Scale::X16;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X,
        SIZE_Y,
        WindowOptions {
            borderless: false,
            title: true,
            scale: SCALE,
            resize: false,
            scale_mode: minifb::ScaleMode::UpperLeft,
            topmost: false,
            transparency: false,
            none: false,
        },
    )?;

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, &WHITE);
    canvas.checker(
        &WHITE,
        &Color {
            r: (200),
            g: (200),
            b: (200),
            a: (1.0),
        },
    );

    // define polygon
    let p1 = Vector2d::new(1.0, 1.0);
    let p2 = Vector2d::new(4.0, 7.0);
    let p3 = Vector2d::new(4.0, 4.0);
    let p4 = Vector2d::new(6.0, 5.0);
    let p5 = Vector2d::new(7.0, 3.0);

    // let p1 = Vector2d::new(1.0, 1.0);
    // let p2 = Vector2d::new(12.0, 21.0);
    // let p3 = Vector2d::new(12.0, 12.0);
    // let p4 = Vector2d::new(18.0, 15.0);
    // let p5 = Vector2d::new(21.0, 9.0);

    let points = vec![p5, p1, p2, p3, p4];

    let mut edge_table = EdgeTable::new();
    edge_table.list = vec![
        EdgeTableEntry::new(1.0, 1.0, 3.0, 3.0, 1),
        EdgeTableEntry::new(1.0, 1.0, 7.0, 1.5, 2),
        EdgeTableEntry::new(3.0, 7.0, 5.0, -1.5, 4),
        EdgeTableEntry::new(4.0, 4.0, 7.0, 0.0, 3),
        EdgeTableEntry::new(4.0, 4.0, 5.0, 2.0, 5),
    ];

    edge_table.sort();
    edge_table.print();

    let mut active_edge_table = ActiveEdgeTable::new();

    /*
        scanline

        initialize ET
        set AET to empty
        set yscan to ylower of first entry in ET
            move all edges from ET with yscan =| ylower to AET

        while ET not empty or AET not empty
            sort AET for x
            draw lines from (AET[0].x,yscan) to (AET[1].x,yscan),
                from (AET[2].x,yscan) to (AET[3].x,yscan), ……
            remove all edges from AET with yscan >= yupper
            for all edges in AET
                x:= x + 1/m
            yscan += 1
            move all edges from ET with yscan == ylower to AET
    */
    let mut y_scan = edge_table.list[0].y_lower;

    while let Some(index) = edge_table
        .list
        .iter()
        .position(|edge| edge.y_lower as i32 == y_scan as i32)
    {
        println!(
            "Moving e_{} into active edge table",
            edge_table.list[index].id
        );
        let edge = edge_table.list.remove(index);

        // compute values for active edge entry
        let x_intersect = edge.x_lower;

        active_edge_table.list.push(ActiveEdgeTableEntry::new(
            x_intersect,
            edge.y_upper,
            edge.dx_dy,
            edge.id,
        ));

        active_edge_table.sort();
    }

    active_edge_table.print();

    let mut iteration = 0;
    while edge_table.list.len() > 0 && active_edge_table.list.len() > 0 {
        println!("\n\nIteration: {iteration}, scanline_y = {y_scan}");

        edge_table.print();
        active_edge_table.print();

        // draw pixel at every edge in AET at (x_intersect, y_scan)
        for (i, edge) in active_edge_table.list.iter().enumerate() {
            canvas.set_pixel((edge.x_intersect as i32, y_scan as i32), &BLACK);
        }

        // remove all edges from AET wich are entirely below y_scan
        while let Some(index) = active_edge_table
            .list
            .iter()
            .position(|edge| y_scan >= edge.y_upper)
        {
            println!(
                "Removing e_{} from active edge table",
                active_edge_table.list[index].id
            );
            active_edge_table.list.remove(index);
        }

        // increment x_intersect in every edge in AET
        for edge in active_edge_table.list.iter_mut() {
            edge.x_intersect += edge.dx_dy;
        }

        // increment y_scan
        y_scan += 1.0;

        // move all edges from ET with y_lower == y_scan to AET

        while let Some(index) = edge_table
            .list
            .iter()
            .position(|edge| edge.y_lower as i32 == y_scan as i32)
        {
            println!(
                "Moving e_{} into active edge table",
                edge_table.list[index].id
            );
            let edge = edge_table.list.remove(index);

            // compute values for active edge entry
            let x_intersect = edge.x_lower;

            active_edge_table.list.push(ActiveEdgeTableEntry::new(
                x_intersect,
                edge.y_upper,
                edge.dx_dy,
                edge.id,
            ));

            active_edge_table.sort();
        }

        iteration += 1;
    }

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        for p in points.iter() {
            canvas.set_pixel((p.x as i32, p.y as i32), &MAGENTA);
        }

        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
