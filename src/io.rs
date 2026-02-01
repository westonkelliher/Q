use std::collections::HashMap;
use std::fs;
use serde::{Deserializer, Serializer};
use serde::de::Visitor;
use std::fmt;
use crate::types::Land;

pub fn serialize_terrain<S>(terrain: &HashMap<(i32, i32), Land>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(terrain.len()))?;
    for (k, v) in terrain {
        let key = format!("{},{}", k.0, k.1);
        map.serialize_entry(&key, v)?;
    }
    map.end()
}

pub fn deserialize_terrain<'de, D>(deserializer: D) -> Result<HashMap<(i32, i32), Land>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TerrainVisitor;

    impl<'de> Visitor<'de> for TerrainVisitor {
        type Value = HashMap<(i32, i32), Land>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = access.next_entry::<String, Land>()? {
                let parts: Vec<&str> = key.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        map.insert((x, y), value);
                    }
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(TerrainVisitor)
}

pub fn save_world(world: &crate::types::World) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.json", world.name);
    let json = serde_json::to_string_pretty(world)?;
    fs::write(filename, json)?;
    Ok(())
}

pub fn load_world(path: &str) -> Result<crate::types::World, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let world: crate::types::World = serde_json::from_str(&contents)?;
    Ok(world)
}
