fn main() {}

enum BusCompany {
  TCI
}

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

struct Accessibilities {
  visual: bool,
  auditory: bool,
  motor: bool,
}

struct Bus {
  id: u32,
  accessibilities: Accessibilities,
  capacity: u32,
  company: BusCompany,
  fuel: Fuel,
}

struct Post {
  id: u32,
  name: String,
  aliases: Vec<String>,
  coordinates: (f64, f64),
  accessibilities: Accessibilities,
  is_non_named: bool,
}

struct Path {
}
