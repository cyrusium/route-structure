# Route Structure

## Versions

- `0.0.1` [Pre-alpha] (Latest)

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

## Structure

```rs
struct Accessibilities {
  visual: bool,
  auditory: bool,
  motor: bool,
}

struct Bus {
  id: u32,
  accessibilities: Accessibilities,
  capacity: u32,
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
```