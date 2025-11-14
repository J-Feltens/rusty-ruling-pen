use crate::{
    graphics::{Canvas, Color, rgb2u32},
    vectors::{IntegerVector2d, Vector2d},
};

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeTableEntry {
    pub y_lower: i32,
    pub x_lower: i32,
    pub y_upper: i32,

    pub dx_dy: f64,
    pub id: i32,

    pub attr_lower: f64,
    pub attr_upper: f64,
    pub dattr_dy: f64,
}

impl EdgeTableEntry {
    pub fn new(
        y_lower: i32,
        x_lower: i32,
        y_upper: i32,
        dx_dy: f64,
        id: i32,
        attr_lower: f64,
        attr_upper: f64,
        dattr_dy: f64,
    ) -> EdgeTableEntry {
        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
            id,
            attr_lower,
            attr_upper,
            dattr_dy,
        }
    }

    pub fn from_points(p1: IntegerVector2d, p2: IntegerVector2d, id: i32) -> EdgeTableEntry {
        let x_lower = if p1.y < p2.y { p1.x } else { p2.x };
        let y_lower = if p1.y < p2.y { p1.y } else { p2.y };
        let x_upper = if p1.y < p2.y { p2.x } else { p1.x };
        let y_upper = if p1.y < p2.y { p2.y } else { p1.y };
        let dx_dy = (x_upper - x_lower) as f64 / (y_upper - y_lower) as f64;

        let attr_lower = if p1.y < p2.y { p1.attr } else { p2.attr };
        let attr_upper = if p1.y < p2.y { p2.attr } else { p1.attr };
        let dattr_dy = (attr_upper - attr_lower) as f64 / (y_upper - y_lower) as f64;

        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
            id,
            attr_lower,
            attr_upper,
            dattr_dy,
        }
    }

    pub fn to_points(&self) -> (Vector2d, Vector2d) {
        let x_upper =
            (self.y_upper as f64 - self.y_lower as f64 + (1.0 / self.dx_dy) * self.x_lower as f64);

        return (
            Vector2d::new(self.x_lower as f64, self.y_lower as f64),
            Vector2d::new(x_upper, self.y_upper as f64),
        );
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeTable {
    pub list: Vec<EdgeTableEntry>,
}

impl EdgeTable {
    pub fn new() -> EdgeTable {
        EdgeTable { list: (Vec::new()) }
    }

    pub fn add_edge(&mut self, edge: EdgeTableEntry) {
        self.list.push(edge);
    }

    pub fn sort(&mut self) {
        self.list.sort_by(|a, b| a.y_lower.cmp(&b.y_lower));
    }

    pub fn print(&self) {
        println!("\n--- Edge Table ---");
        println!(
            "{0: <12} | {1: <12} | {2: <12} | {3: <12} | {4: <12}",
            "edge", "y_lower", "x_lower", "y_upper", "dx/dy"
        );
        println!("-------------+--------------+--------------+--------------+---------------");

        for (i, edge) in self.list.iter().enumerate() {
            println!(
                "e_{0: <10} | {1: <12} | {2: <12} | {3: <12} | {4: <12}",
                edge.id,
                edge.y_lower,
                edge.x_lower,
                edge.y_upper,
                (edge.dx_dy * 1000.0).round() / 1000.0
            );
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveEdgeTableEntry {
    pub x_intersect: f64,
    pub y_upper: i32,

    pub dx_dy: f64,
    pub id: i32,

    pub attr_intersect: f64,
    pub dattr_dy: f64,
}

impl ActiveEdgeTableEntry {
    pub fn new(
        x_intersect: f64,
        y_upper: i32,
        dx_dy: f64,
        id: i32,
        attr_intersect: f64,
        dattr_dy: f64,
    ) -> ActiveEdgeTableEntry {
        ActiveEdgeTableEntry {
            x_intersect,
            y_upper,
            dx_dy,
            id,
            attr_intersect,
            dattr_dy,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveEdgeTable {
    pub list: Vec<ActiveEdgeTableEntry>,
}

impl ActiveEdgeTable {
    pub fn new() -> ActiveEdgeTable {
        ActiveEdgeTable { list: (Vec::new()) }
    }

    pub fn add_edge(&mut self, edge: ActiveEdgeTableEntry) {
        self.list.push(edge);
    }

    pub fn sort(&mut self) {
        self.list
            .sort_by(|a, b| a.x_intersect.total_cmp(&b.x_intersect));
    }

    pub fn print(&self) {
        println!("\n--- Active Edge Table ---");
        println!(
            "{0: <12} | {1: <12} | {2: <12} | {3: <12}",
            "edge", "x_intersect", "y_upper", "dx_dy"
        );
        println!("-------------+--------------+--------------+--------------");

        for (i, edge) in self.list.iter().enumerate() {
            println!(
                "e_{0: <10} | {1: <12} | {2: <12} | {3: <12}",
                edge.id,
                (edge.x_intersect * 1000.0).round() / 1000.0,
                edge.y_upper,
                (edge.dx_dy * 1000.0).round() / 1000.0
            );
        }
    }
}

pub fn draw_polygon_onto_buffer(
    points: &Vec<IntegerVector2d>,
    canvas: &mut Canvas,
    color: &Color,
    verbose: bool,
) {
    // build edge table, ignore horizontal edges
    let mut edge_table = EdgeTable::new();
    for i in 0..points.len() {
        let p1 = points[i % points.len()];
        let p2 = points[(i + 1) % points.len()];
        if p1.y != p2.y {
            edge_table.add_edge(EdgeTableEntry::from_points(p1, p2, i as i32 + 1));
        }
    }

    edge_table.sort();

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
        if verbose {
            println!(
                "Moving e_{} into active edge table",
                edge_table.list[index].id
            );
        }
        let edge = edge_table.list.remove(index);

        // compute values for active edge entry
        let x_intersect = edge.x_lower as f64;
        let attr_intersect = edge.attr_lower;

        active_edge_table.list.push(ActiveEdgeTableEntry::new(
            x_intersect,
            edge.y_upper,
            edge.dx_dy,
            edge.id,
            attr_intersect,
            edge.dattr_dy,
        ));

        active_edge_table.sort();
    }

    let mut iteration = 0;
    while edge_table.list.len() > 0 || active_edge_table.list.len() > 0 {
        if verbose {
            println!("\n--------------------------------------------------\n");
            println!("Iteration: {iteration}, scanline_y = {y_scan}");
        }
        // remove all edges from AET wich are entirely below y_scan
        while let Some(index) = active_edge_table
            .list
            .iter()
            .position(|edge| y_scan >= edge.y_upper)
        {
            if verbose {
                println!(
                    "Removing e_{} from active edge table",
                    active_edge_table.list[index].id
                );
            }
            active_edge_table.list.remove(index);
        }

        // add all edges from ET with y_lower == y_scan to AET
        while let Some(index) = edge_table
            .list
            .iter()
            .position(|edge| edge.y_lower as i32 == y_scan as i32)
        {
            if verbose {
                println!(
                    "Moving e_{} into active edge table",
                    edge_table.list[index].id
                );
            }
            let edge = edge_table.list.remove(index);

            // compute values for active edge entry
            let x_intersect = edge.x_lower as f64;
            let attr_intersect = edge.attr_lower;

            active_edge_table.list.push(ActiveEdgeTableEntry::new(
                x_intersect,
                edge.y_upper,
                edge.dx_dy,
                edge.id,
                attr_intersect,
                edge.dattr_dy,
            ));

            active_edge_table.sort();
        }

        edge_table.sort();
        active_edge_table.sort();
        if verbose {
            edge_table.print();
            active_edge_table.print();
        }

        // draw between x_1_intersect and x_2_intersect
        if active_edge_table.list.len() >= 2 {
            for i in 0..(active_edge_table.list.len() as f64 / 2.0) as usize {
                let edge1 = &active_edge_table.list[2 * i];
                let edge2 = &active_edge_table.list[2 * i + 1];

                let mut cur_x = edge1.x_intersect;
                let mut cur_attr = edge1.attr_intersect;

                if verbose {
                    println!("Drawing between edges e_{} and 3_{}", edge1.id, edge2.id);
                }

                while cur_x <= edge2.x_intersect {
                    let brightness_u8 = (cur_attr * 255.0) as u8;
                    canvas.set_pixel(
                        (cur_x.round() as i32, y_scan),
                        &Color::new(brightness_u8, brightness_u8, brightness_u8, 1.0),
                    );
                    cur_x += 1.0;
                }
            }
        }

        // increment x_intersect in every edge in AET
        for edge in active_edge_table.list.iter_mut() {
            edge.x_intersect += edge.dx_dy;
            edge.attr_intersect += edge.dattr_dy;
        }

        // increment y_scan
        y_scan += 1;

        iteration += 1;
    }
}
