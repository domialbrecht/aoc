#[derive(Clone, Debug, PartialEq)]
pub enum Mapping {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLoc,
}

#[derive(Debug)]
pub struct Map {
    source: u64,
    dest: u64,
    offset: u64,
    pub maptype: Mapping,
}

impl Map {
    //79 source, dest 81
    pub fn dest_for_source(&self, source: &u64) -> Option<u64> {
        let sources = self.source..self.source + self.offset;
        if sources.contains(source) {
            Some(self.dest + (source - self.source))
        } else {
            None
        }
    }
}

impl Map {
    pub fn build(
        mut mapdata: impl Iterator<Item = String>,
        maptype: Mapping,
    ) -> Result<Map, &'static str> {
        let dest = match mapdata.next() {
            Some(arg) => arg.parse().expect("can parse source {source}"),
            None => return Err("Didn't get a source string"),
        };
        let source = match mapdata.next() {
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
