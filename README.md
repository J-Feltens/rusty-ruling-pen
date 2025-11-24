# rusty-ruling-pen

## Data structures:

- Vector2d, -3d, -4d
    - trivial vector structs
    - fields:
        - x: f64
        - y: f64
        - (z: f64)
        - (u: f64)
- IntegerVector2d
    - used during scanline algorithm
    - values in the attrs-vector are interpolated during rasterization
    - current attrs configuration:
        - 0-2: pos in camera space
        - 3: projected z
        - 4-6: normal in camera space
        - 7-10: rgba, each [0.0, ... 1.0]
    - fields:
        - x: i32
        - y: i32
        - attrs: Vec<f64>
- Triangle3d
    - housing shell for 3 vertices
    - on creation, computes surface normal
    - vertex-order determines surface normal
      -fields:
        - vertices: Vec<Vector3d>
        - n: Vector3d
        - color: Color

## Render Pipeline:

- shapes are represented by a Vec\<Triangle3d>
- a Triangle3d object consists of:
    - 3 vertices (Vector3d)
    - surface normal (Vector3d)
    - color (Color)
      additional features like an attribute vector (Vec<f64>) to store vertex attributes
- projection
    - projection from 3d to 2d is done using a 4x4 homogenous perspective-projection-matrix
    - during projection, the surface normal and color and projected z of a triangle are transferred into the attrs
      vector
- rasterization:
    - rasterization is done using scanline algorithm
    - during rasterization attrs are interpolated
    - z-buffer for handeling intersecting faces/general z-order
