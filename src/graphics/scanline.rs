use crate::vectors::{IntegerVector2d, Vector2d};

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeTableEntry {
    pub y_lower: i32,
    pub x_lower: i32,
    pub y_upper: i32,

    pub dx_dy: f64,
    pub id: i32,
}

impl EdgeTableEntry {
    pub fn new(y_lower: i32, x_lower: i32, y_upper: i32, dx_dy: f64, id: i32) -> EdgeTableEntry {
        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
            id,
        }
    }

    pub fn from_points(p1: IntegerVector2d, p2: IntegerVector2d, id: i32) -> EdgeTableEntry {
        let x_lower = if p1.y < p2.y { p1.x } else { p2.x };
        let y_lower = if p1.y < p2.y { p1.y } else { p2.y };
        let x_upper = if p1.y < p2.y { p2.x } else { p1.x };
        let y_upper = if p1.y < p2.y { p2.y } else { p1.y };
        let dx_dy = (x_upper - x_lower) as f64 / (y_upper - y_lower) as f64;

        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
            id,
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
                edge.id, edge.y_lower, edge.x_lower, edge.y_upper, edge.dx_dy
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
}

impl ActiveEdgeTableEntry {
    pub fn new(x_intersect: f64, y_upper: i32, dx_dy: f64, id: i32) -> ActiveEdgeTableEntry {
        ActiveEdgeTableEntry {
            x_intersect,
            y_upper,
            dx_dy,
            id,
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
                edge.id, edge.x_intersect, edge.y_upper, edge.dx_dy
            );
        }
    }
}
