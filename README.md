# Projicio

**Pure-Rust coordinate reference system and map projection engine.**

Zero C dependencies. No PROJ, no GDAL. Just fast, correct coordinate transformations.

## Features

- **Web Mercator** (EPSG:3857) — forward and inverse
- **Transverse Mercator / UTM** (EPSG:32601–32660, 32701–32760) — all 120 zones
- **Mercator** (EPSG:3395) — ellipsoidal
- **Lambert Conformal Conic** — 2SP variant
- **EPSG code dispatch** — `Transform::new("EPSG:4326", "EPSG:3857")`
- **Batch transforms** — transform thousands of coordinates efficiently
- **Pure Rust** — no unsafe, no C dependencies, no build scripts

## Quick Start

```bash
# Transform a coordinate
projicio transform --from EPSG:4326 --to EPSG:3857 -- -74.006 40.7128

# Library usage
cargo add projicio-core
```

```rust
use projicio_core::Transform;

let t = Transform::new("EPSG:4326", "EPSG:3857").unwrap();
let (x, y) = t.convert(-74.006, 40.7128).unwrap();
println!("NYC in Web Mercator: {x}, {y}");
```

## Architecture

```
projicio-core    — Projection math, ellipsoids, CRS registry, Transform API
projicio-cli     — Command-line interface
```

## Supported CRS

| Family | EPSG Codes | Status |
|--------|------------|--------|
| WGS84 Geographic | 4326 | ✅ |
| Web Mercator | 3857 | ✅ |
| UTM North | 32601–32660 | ✅ |
| UTM South | 32701–32760 | ✅ |
| Mercator | 3395 | ✅ |
| Lambert Conformal Conic | 2154, custom | ✅ |

## Roadmap

- [ ] Albers Equal Area
- [ ] Polar Stereographic
- [ ] Lambert Azimuthal Equal Area
- [ ] Oblique Stereographic
- [ ] NTv2 grid shifts (NAD27→NAD83)
- [ ] WKT/PROJ string parsing
- [ ] EPSG database embedding

## License

AGPL-3.0-or-later
