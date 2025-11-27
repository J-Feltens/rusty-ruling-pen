use crate::vectors::{IntegerVector2d, Vector2d};

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeTableEntry {
    pub y_lower: i32,
    pub x_lower: i32,
    pub y_upper: i32,

    pub dx_dy: f64,
    pub id: i32,

    pub attrs_lower: Vec<f64>,
    pub attrs_upper: Vec<f64>,
    pub dattrs_dy: Vec<f64>,
}

impl EdgeTableEntry {
    pub fn new(
        y_lower: i32,
        x_lower: i32,
        y_upper: i32,
        dx_dy: f64,
        id: i32,
        attrs_lower: Vec<f64>,
        attrs_upper: Vec<f64>,
        dattrs_dy: Vec<f64>,
    ) -> EdgeTableEntry {
        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
            id,
            attrs_lower,
            attrs_upper,
            dattrs_dy,
        }
    }

    pub fn from_points(p1: IntegerVector2d, p2: IntegerVector2d, id: i32) -> EdgeTableEntry {
        assert_eq!(p1.attrs.len(), p2.attrs.len());

        let x_lower = if p1.y < p2.y { p1.x } else { p2.x };
        let y_lower = if p1.y < p2.y { p1.y } else { p2.y };
        let x_upper = if p1.y < p2.y { p2.x } else { p1.x };
        let y_upper = if p1.y < p2.y { p2.y } else { p1.y };
        let dx_dy = (x_upper - x_lower) as f64 / (y_upper - y_lower) as f64;

        let attrs_lower = if p1.y < p2.y {
            p1.attrs.clone()
        } else {
            p2.attrs.clone()
        };
        let attrs_upper = if p1.y < p2.y {
            p2.attrs.clone()
        } else {
            p1.attrs.clone()
        };

        let mut dattrs_dy = vec![0.0; attrs_lower.len()];
        for i in 0..attrs_lower.len() {
            dattrs_dy[i] = (attrs_upper[i] - attrs_lower[i]) as f64 / (y_upper - y_lower) as f64;
        }

        EdgeTableEntry {
            y_lower,
            x_lower,
            y_upper,
            dx_dy,
            id,
            attrs_lower,
            attrs_upper,
            dattrs_dy,
        }
    }

    pub fn to_points(&self) -> (Vector2d, Vector2d) {
        let x_upper =
            self.y_upper as f64 - self.y_lower as f64 + (1.0 / self.dx_dy) * self.x_lower as f64;

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

        for edge in self.list.iter() {
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

    pub attrs_intersect: Vec<f64>,
    pub dattrs_dy: Vec<f64>,
}

impl ActiveEdgeTableEntry {
    pub fn new(
        x_intersect: f64,
        y_upper: i32,
        dx_dy: f64,
        id: i32,
        attrs_intersect: Vec<f64>,
        dattrs_dy: Vec<f64>,
    ) -> ActiveEdgeTableEntry {
        ActiveEdgeTableEntry {
            x_intersect,
            y_upper,
            dx_dy,
            id,
            attrs_intersect,
            dattrs_dy,
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

        for edge in self.list.iter() {
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
