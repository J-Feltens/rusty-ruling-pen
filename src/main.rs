use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::ascii::escape_default;
use std::collections::LinkedList;
use std::process::exit;
use std::{thread, time};

use crate::graphics::colors::rgb2u32;
use crate::graphics::scanline::{ActiveEdgeTable, ActiveEdgeTableEntry, EdgeTableEntry};
use crate::graphics::{
    BLACK, BLUE, CYAN, Canvas, Color, EdgeTable, GREEN, MAGENTA, RED, WHITE, YELLOW,
};
use crate::vectors::{IntegerVector2d, Vector2d};

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 512;
const SIZE_Y: usize = 512;
const SCALE: minifb::Scale = minifb::Scale::X1;
const ANIM_INTERVAL: time::Duration = time::Duration::from_millis(100);

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
    let mut edge_table = EdgeTable::new();
    let scale = 70;
    let p1 = IntegerVector2d::new(7 * scale, 3 * scale);
    let p2 = IntegerVector2d::new(1 * scale, 1 * scale);
    let p3 = IntegerVector2d::new(4 * scale, 7 * scale);
    let p4 = IntegerVector2d::new(4 * scale, 4 * scale);
    let p5 = IntegerVector2d::new(6 * scale, 5 * scale);

    let mut points = vec![p1, p2, p3, p4, p5];

    let mut edge_table = EdgeTable::new();
    edge_table.add_edge(EdgeTableEntry::from_points(p1, p2, 1));
    edge_table.add_edge(EdgeTableEntry::from_points(p2, p3, 2));
    edge_table.add_edge(EdgeTableEntry::from_points(p3, p4, 3));
    edge_table.add_edge(EdgeTableEntry::from_points(p4, p5, 5));
    edge_table.add_edge(EdgeTableEntry::from_points(p5, p1, 4));

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
        .position(|edge| edge.y_lower == y_scan)
    {
        println!(
            "Moving e_{} into active edge table",
            edge_table.list[index].id
        );
        let edge = edge_table.list.remove(index);

        // compute values for active edge entry
        let x_intersect = edge.x_lower as f64;

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
    while edge_table.list.len() > 0 || active_edge_table.list.len() > 0 {
        println!("\n\nIteration: {iteration}, scanline_y = {y_scan}");

        edge_table.sort();
        active_edge_table.sort();
        edge_table.print();
        active_edge_table.print();

        // draw pixel at every edge in AET at (x_intersect, y_scan)
        // for (i, edge) in active_edge_table.list.iter().enumerate() {
        //     canvas.set_pixel((edge.x_intersect as i32, y_scan as i32), &BLACK);
        // }

        // draw between x_1_intersect and x_2_intersect
        if active_edge_table.list.len() >= 2 {
            for i in 0..(active_edge_table.list.len() as f64 / 2.0) as usize {
                let mut cur_x = active_edge_table.list[2 * i].x_intersect;
                while cur_x <= active_edge_table.list[2 * i + 1].x_intersect {
                    canvas.set_pixel((cur_x.round() as i32, y_scan), &BLACK);
                    cur_x += 1.0;
                }
            }
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
        y_scan += 1;

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
            let x_intersect = edge.x_lower as f64;

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

    // draw polygon corners
    // for p in points.iter() {
    //     canvas.set_pixel((p.x, p.y), &MAGENTA);
    // }

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
