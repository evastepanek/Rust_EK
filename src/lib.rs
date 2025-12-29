use rand::Rng; // Für Abschnitt 7 

// ---------- 1. BASICS ----------
pub fn greet(name: &str) -> String {
    //return "Hello, <name>!"
    format!("Hello, {}!", name)
}
pub fn sum(nums: &[i32]) -> i32 {
    //sum all elements using an iterator
    nums.iter().sum()
}
pub fn flip(b: bool) -> bool {
    //return the opposite boolean
    !b
}


// ---------- 2. OWNERSHIP & BORROWING ----------
pub fn take_ownership(s: String) -> usize {
    //return the length of s (taking ownership)
    // hint: no borrowing here; s moves in
    s.len()
}
pub fn borrow_first_char(s: &str) -> Option<char> {
    //return the first char without taking ownership
    s.chars().next()
}
pub fn push_exclamation(s: &mut String) {
    //mutate s by appending a single '!' character
    s.push('!')
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        // Euclidean distance: sqrt((x2-x1)² + (y2-y1)²)
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        // match on self, compute area
        match self {
            Shape::Circle { radius, .. } => std::f64::consts::PI * radius.powi(2),
            Shape::Rect { w, h, .. } => w * h,
        }
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 { self.0 }
    fn y(&self) -> f64 { self.1 }
}

// Return a reference to the item farthest from the origin.
// Note the explicit lifetime tying the returned reference to the input slice.
pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    // iterate, compute squared distance, track max, return reference
    items.iter().max_by(|a, b| {
        let dist_a = a.x().powi(2) + a.y().powi(2);
        let dist_b = b.x().powi(2) + b.y().powi(2);
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    })
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    // parse string into u16; map errors into a friendly String
    // hint: use s.parse::<u16>()
    s.parse::<u16>().map_err(|e| format!("Invalid port: {}", e))
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    // all even numbers from 0..=n, squared, collected to Vec
    // hint: (0..=n).filter(...).map(...).collect()
    (0..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()  // Nur gerade Zahlen, Quadriren, In einen Vec umwandeln
}

// ---------- 7. USING A CRATE (rand) ----------
pub fn roll_dice(sides: u8) -> u8 {
    // return a value in 1..=sides using rand::Rng
    // note: assume sides >= 1
    rand::rng().random_range(1..=sides)
}
