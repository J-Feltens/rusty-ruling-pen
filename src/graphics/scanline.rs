use crate::vectors::Vector2d;

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeTableEntry {
    pub y_lower: f64,
    pub x_lower: f64,
    pub y_upper: f64,

    pub dx_dy: f64,
}

impl EdgeTableEntry {
    pub fn new(y_lower: f64, x_lower: f64, y_upper: f64, dx_dy: f64) -> EdgeTableEntry {
        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
        }
    }

    pub fn from_points(p1: Vector2d, p2: Vector2d) -> EdgeTableEntry {
        let x_lower = if p1.y < p2.y { p1.x } else { p2.x };
        let y_lower = if p1.y < p2.y { p1.y } else { p2.y };
        let x_upper = if p1.y < p2.y { p2.x } else { p1.x };
        let y_upper = if p1.y < p2.y { p2.y } else { p1.y };
        let dx_dy = (x_upper - x_lower) / (y_upper - y_lower);

        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
        }
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
        self.list.sort_by(|a, b| a.y_lower.total_cmp(&b.y_lower));
    }

    pub fn print(&self) {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "y_lower", "x_lower", "y_upper", "dx/dy"
        );

        for edge in self.list.iter() {
            println!(
                "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
                edge.y_lower, edge.x_lower, edge.y_upper, edge.dx_dy
            );
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveEdgeTableEntry {
    pub x_intersect: f64,
    pub y_upper: f64,

    pub dx_dy: f64,
}

impl ActiveEdgeTableEntry {
    pub fn new(x_intersect: f64, y_upper: f64, dx_dy: f64) -> ActiveEdgeTableEntry {
        ActiveEdgeTableEntry {
            x_intersect,
            y_upper,
            dx_dy,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActiveEdgeTable {
    list: Vec<ActiveEdgeTableEntry>,
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
        println!(
            "{0: <10} | {1: <10} | {2: <10}",
            "x_intersect", "y_upper", "dx_dy"
        );

        for edge in self.list.iter() {
            println!(
                "{0: <10} | {1: <10} | {2: <10}",
                edge.x_intersect, edge.y_upper, edge.dx_dy
            );
        }
    }
}
