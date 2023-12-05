use core::panic;
use std::fs;

use day05::Map;
use day05::Mapping;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> u64 {
    let (seeds, maps) = get_data(games);
    seeds
        .iter()
        .map(|s| seed_final_dests(s, &maps))
        .min()
        .unwrap()
}

fn seed_final_dests(seed: &u64, maps: &[Map]) -> u64 {
    [
        Mapping::SeedToSoil,
        Mapping::SoilToFert,
        Mapping::FertToWater,
        Mapping::WaterToLight,
        Mapping::LightToTemp,
        Mapping::TempToHumid,
        Mapping::HumidToLoc,
    ]
    .iter()
    .fold(*seed, |acc, maptype| dest_from_maps(acc, maps, maptype))
}

fn dest_from_maps(source: u64, maps: &[Map], dest_type: &Mapping) -> u64 {
    match maps
        .iter()
        .filter(|m| m.maptype == *dest_type)
        .find_map(|m| m.dest_for_source(&source))
    {
        Some(soil) => soil,
        None => source,
    }
}

fn get_data(games: &str) -> (Vec<u64>, Vec<Map>) {
    let data = games.split("\n\n").collect::<Vec<&str>>();
    let mut data_iter = data.iter();
    let (_, seeds): (_, &str) = data_iter
        .next()
        .unwrap()
        .split_once(' ')
        .expect("seed data");

    let seeds: Vec<u64> = seeds
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soil() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        let (seeds, maps) = get_data(&contents);
        assert_eq!(dest_from_maps(seeds[0], &maps, &Mapping::SeedToSoil), 81,);
        assert_eq!(dest_from_maps(seeds[1], &maps, &Mapping::SeedToSoil), 14,);
        assert_eq!(dest_from_maps(seeds[2], &maps, &Mapping::SeedToSoil), 57,);
        assert_eq!(dest_from_maps(seeds[3], &maps, &Mapping::SeedToSoil), 13,);
    }

    #[test]
    fn test_location() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        let (seeds, maps) = get_data(&contents);
        assert_eq!(seed_final_dests(&seeds[0], &maps), 82);
    }

    #[test]
    #[ignore]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 35);
    }
}
