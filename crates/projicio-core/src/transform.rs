use crate::{Coord, Error, Geographic, projection::*};

type InverseFn = Box<dyn Fn(Coord) -> Result<Geographic, Error> + Send + Sync>;
type ForwardFn = Box<dyn Fn(Geographic) -> Result<Coord, Error> + Send + Sync>;

/// High-level transform between two CRS identified by EPSG codes.
pub struct Transform {
    source_to_geo: InverseFn,
    geo_to_target: ForwardFn,
}

impl Transform {
    /// Create a transform between two EPSG codes.
    pub fn new(from: &str, to: &str) -> Result<Self, Error> {
        let source_epsg = parse_epsg(from)?;
        let target_epsg = parse_epsg(to)?;

        let source_to_geo = make_inverse(source_epsg)?;
        let geo_to_target = make_forward(target_epsg)?;

        Ok(Self {
            source_to_geo,
            geo_to_target,
        })
    }

    /// Transform a single coordinate.
    pub fn convert(&self, x: f64, y: f64) -> Result<(f64, f64), Error> {
        let geo = (self.source_to_geo)(Coord::new(x, y))?;
        let result = (self.geo_to_target)(geo)?;
        Ok((result.x, result.y))
    }

    /// Transform a batch of coordinates.
    pub fn convert_batch(&self, coords: &[(f64, f64)]) -> Result<Vec<(f64, f64)>, Error> {
        coords.iter().map(|&(x, y)| self.convert(x, y)).collect()
    }
}

fn parse_epsg(code: &str) -> Result<u32, Error> {
    let num_str = code
        .strip_prefix("EPSG:")
        .or_else(|| code.strip_prefix("epsg:"))
        .unwrap_or(code);
    num_str
        .parse::<u32>()
        .map_err(|_| Error::UnsupportedCrs(code.to_string()))
}

fn make_inverse(epsg: u32) -> Result<InverseFn, Error> {
    match epsg {
        // WGS84 geographic — identity (input is lon/lat degrees)
        4326 => Ok(Box::new(|c: Coord| Ok(Geographic::new(c.x, c.y)))),
        // Web Mercator
        3857 => {
            let proj = WebMercator::new();
            Ok(Box::new(move |c: Coord| proj.inverse(c)))
        }
        // UTM zones 1-60 North
        32601..=32660 => {
            let zone = (epsg - 32600) as u8;
            let proj = TransverseMercator::utm(zone, true);
            Ok(Box::new(move |c: Coord| proj.inverse(c)))
        }
        // UTM zones 1-60 South
        32701..=32760 => {
            let zone = (epsg - 32700) as u8;
            let proj = TransverseMercator::utm(zone, false);
            Ok(Box::new(move |c: Coord| proj.inverse(c)))
        }
        _ => Err(Error::UnsupportedCrs(format!("EPSG:{epsg}"))),
    }
}

fn make_forward(epsg: u32) -> Result<ForwardFn, Error> {
    match epsg {
        // WGS84 geographic — identity (output is lon/lat)
        4326 => Ok(Box::new(|g: Geographic| Ok(Coord::new(g.lon, g.lat)))),
        // Web Mercator
        3857 => {
            let proj = WebMercator::new();
            Ok(Box::new(move |g: Geographic| proj.forward(g)))
        }
        // UTM zones 1-60 North
        32601..=32660 => {
            let zone = (epsg - 32600) as u8;
            let proj = TransverseMercator::utm(zone, true);
            Ok(Box::new(move |g: Geographic| proj.forward(g)))
        }
        // UTM zones 1-60 South
        32701..=32760 => {
            let zone = (epsg - 32700) as u8;
            let proj = TransverseMercator::utm(zone, false);
            Ok(Box::new(move |g: Geographic| proj.forward(g)))
        }
        _ => Err(Error::UnsupportedCrs(format!("EPSG:{epsg}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_4326_to_3857() {
        let t = Transform::new("EPSG:4326", "EPSG:3857").unwrap();
        let (x, y) = t.convert(-74.006, 40.7128).unwrap();
        // Known approximate values for NYC in Web Mercator
        assert!((x - (-8_238_310.0)).abs() < 100.0);
        assert!((y - 4_970_072.0).abs() < 100.0);
    }

    #[test]
    fn test_transform_roundtrip() {
        let t1 = Transform::new("EPSG:4326", "EPSG:32618").unwrap();
        let t2 = Transform::new("EPSG:32618", "EPSG:4326").unwrap();
        let (x, y) = t1.convert(-74.006, 40.7128).unwrap();
        let (lon, lat) = t2.convert(x, y).unwrap();
        assert!((lon - (-74.006)).abs() < 1e-6);
        assert!((lat - 40.7128).abs() < 1e-6);
    }
}
