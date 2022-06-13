pub struct Color {
    r: i32,
    g: i32,
    b: i32,
    a: i32,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
}