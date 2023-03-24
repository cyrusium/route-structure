# Route Structure

## Versions

- [`0.0.1`](https://github.com/Cyrusium/route-structure/blob/29e1e0e5053f309094b214661e7229f869dd2621/README.md) [Pre-alpha]
- `0.0.2` [Pre-alpha] (Latest)

## Languages

- English
- [Português](README.pt.md)

## Constants

Declaration of constants used in the structure.

### Fuel

```rs
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
```

## Main Structure

```rs
struct Main {
  company: String,
  buses: Vec<Bus>,
  posts: Vec<Post>,
  paths: Vec<Path>,
}
```

## General Structures

### Accessibilities

```rs
struct Accessibilities {
  visual: bool,
  auditory: bool,
  motor: bool,
}
```

### Bus Definition

```rs
struct Bus {
  id: u32,
  accessibilities: Accessibilities,
  capacity: u32,
  fuel: Fuel,
}
```

### Bus Post

```rs
struct Post {
  id: u32,
  name: String,
  aliases: Vec<String>,
  coordinates: (f64, f64),
  accessibilities: Accessibilities,
  is_non_named: bool,
}
```

### Bus Route

```rs
struct Point {
  post: u32,
  time: u32, // In minutes from midnight
}

struct Path {
  id: u32,
  bus: u32,
  points: Vec<Point>,
}
```
