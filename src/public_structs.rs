enum Fuel {
  Diesel,
  Gasoline,
  Electric,
  Hydrogen,
  NaturalGas,
  Biodiesel,
  Ethanol,
  Methanol,
  Other(String),
}

struct Accessibility {
  visual: bool,
  auditory: bool,
  motor: bool,
}

struct Bus {
  id: u32,
  accessibilities: Accessibility,
  capacity: u32,
  fuel: Fuel,
}

struct Post {
  id: u32,
  name: String,
  aliases: Vec<String>,
  coordinates: (f64, f64),
  accessibility: Accessibility,
}

struct Point {
  post: u32,
  time: u32, // In minutes from midnight
}

struct Path {
  id: u32,
  bus: u32,
  points: Vec<Point>,
}

struct Main {
  company: String,
  buses: Vec<Bus>,
  posts: Vec<Post>,
  paths: Vec<Path>,
}