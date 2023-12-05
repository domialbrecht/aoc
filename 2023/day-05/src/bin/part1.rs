use core::panic;
use std::fs;

use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("input-test.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

#[derive(Clone, Debug)]
enum Mapping {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLoc,
}

#[derive(Debug)]
struct Map {
    source: u32,
    dest: u32,
    offset: u32,
    maptype: Mapping,
}

impl Map {
    fn build(
        mut mapdata: impl Iterator<Item = String>,
        maptype: Mapping,
    ) -> Result<Map, &'static str> {
        let source = match mapdata.next() {
            Some(arg) => arg.parse().expect("can parse source {source}"),
            None => return Err("Didn't get a source string"),
        };
        let dest = match mapdata.next() {
            Some(arg) => arg.parse().expect("can parse dest"),
            None => return Err("Didn't get a dest path"),
        };
        let offset = match mapdata.next() {
            Some(arg) => arg.parse().expect("can parse offset"),
            None => return Err("Didn't get a offset path"),
        };

        Ok(Map {
            source,
            dest,
            offset,
            maptype,
        })
    }
}

fn process(games: &str) -> u32 {
    let (seeds, maps) = get_data(games);
    dbg!(maps);

    0
}

fn get_data(games: &str) -> (Vec<u32>, Vec<Map>) {
    let data = games.split("\n\n").collect::<Vec<&str>>();
    let mut data_iter = data.iter();
    let seeds: Vec<u32> = data_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse seed"))
        .collect();
    let mut maps: Vec<Map> = Vec::new();

    for line in data_iter {
        let (maptype, mapdata) = line.split_once(':').expect("process expects quote");
        let maptype = match maptype {
            s if s.starts_with("seed") => Mapping::SeedToSoil,
            s if s.starts_with("soil") => Mapping::SoilToFert,
            s if s.starts_with("fert") => Mapping::FertToWater,
            s if s.starts_with("water") => Mapping::WaterToLight,
            s if s.starts_with("light") => Mapping::LightToTemp,
            s if s.starts_with("temp") => Mapping::TempToHumid,
            s if s.starts_with("hum") => Mapping::HumidToLoc,
            x => panic!("Invalid maptype {}", x),
        };

        for mapline in mapdata.lines().filter(|l| !l.is_empty()) {
            maps.push(
                Map::build(
                    mapline.split_whitespace().map(|m| m.to_string()),
                    maptype.clone(),
                )
                .unwrap_or_else(|err| {
                    panic!("Problem building map: {err}");
                }),
            );
        }
    }

    (seeds, maps)
}

// fn get_mapping(input: u32, mapping: Mapping) -> u32 {
//     match mapping {
//         Mapping::SeedToSoil => todo!(),
//         Mapping::SoilToFert => todo!(),
//         Mapping::FertToWater => todo!(),
//         Mapping::WaterToLight => todo!(),
//         Mapping::LightToTemp => todo!(),
//         Mapping::TempToHumid => todo!(),
//         Mapping::HumidToLoc => todo!(),
//     }
//     todo!()
// }

fn seed_to_soil(seed: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soil() {
        assert_eq!(seed_to_soil(79), 81);
        assert_eq!(seed_to_soil(14), 14);
        assert_eq!(seed_to_soil(55), 57);
        assert_eq!(seed_to_soil(13), 13);
    }

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 35);
    }
}
