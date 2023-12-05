use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

pub struct Input {
    pub seeds: Vec<u64>,
    pub seed_ranges: Vec<Range<u64>>,
    pub seed_to_soil: Vec<RangeMap>,
    pub soil_to_fertilizer: Vec<RangeMap>,
    pub fertilizer_to_water: Vec<RangeMap>,
    pub water_to_light: Vec<RangeMap>,
    pub light_to_temperature: Vec<RangeMap>,
    pub temperature_to_humidity: Vec<RangeMap>,
    pub humidity_to_location: Vec<RangeMap>,
}

impl Input {
    fn new() -> Input {
        Input {
            seeds: Vec::new(),
            seed_ranges: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }
}

pub struct RangeMap {
    pub source: Range<u64>,
    pub dest: Range<u64>,
}

pub fn parse_input(filename: &str) -> Input {
    let file = File::open(filename).expect("Failed to open input file");
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut input = Input::new();

    loop {
        match lines.next() {
            None => return input,
            Some(line) => {
                let colon = line.find(":").unwrap();

                match &line[0..colon] {
                    "seeds" => parse_list(
                        &mut input.seeds,
                        &mut input.seed_ranges,
                        &line[(colon + 1)..],
                        &mut lines,
                    ),
                    "seed-to-soil map" => parse_map(&mut input.seed_to_soil, &mut lines),
                    "soil-to-fertilizer map" => {
                        parse_map(&mut input.soil_to_fertilizer, &mut lines)
                    }
                    "fertilizer-to-water map" => {
                        parse_map(&mut input.fertilizer_to_water, &mut lines)
                    }
                    "water-to-light map" => parse_map(&mut input.water_to_light, &mut lines),
                    "light-to-temperature map" => {
                        parse_map(&mut input.light_to_temperature, &mut lines)
                    }
                    "temperature-to-humidity map" => {
                        parse_map(&mut input.temperature_to_humidity, &mut lines)
                    }
                    "humidity-to-location map" => {
                        parse_map(&mut input.humidity_to_location, &mut lines)
                    }

                    _ => panic!("Unknown section {}", line),
                }
            }
        }
    }
}

fn parse_list(
    seeds: &mut Vec<u64>,
    seed_ranges: &mut Vec<Range<u64>>,
    line: &str,
    lines: &mut std::iter::Map<
        std::io::Lines<BufReader<File>>,
        impl FnMut(Result<String, std::io::Error>) -> String,
    >,
) {
    line.split_whitespace()
        .map(|id_str| id_str.parse::<u64>().unwrap())
        .for_each(|id| seeds.push(id));

    let mut nums = line
        .split_whitespace()
        .map(|id_str| id_str.parse::<u64>().unwrap());

    loop {
        let start = nums.next();
        if start.is_none() {
            break;
        }

        let start = start.unwrap();
        let length = nums.next().unwrap();
        seed_ranges.push(Range {
            start,
            end: start + length,
        })
    }

    seed_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    lines.next();
}

fn parse_map(
    ranges: &mut Vec<RangeMap>,
    lines: &mut std::iter::Map<
        std::io::Lines<BufReader<File>>,
        impl FnMut(Result<String, std::io::Error>) -> String,
    >,
) {
    loop {
        match lines.next() {
            None => break,
            Some(line) if line.len() == 0 => break,
            Some(line) => {
                let digits: Vec<u64> = line
                    .split_whitespace()
                    .map(|id_str| id_str.parse::<u64>().unwrap())
                    .collect();

                assert_eq!(digits.len(), 3);

                let (dest_start, source_start, length) = (digits[0], digits[1], digits[2]);

                ranges.push(RangeMap {
                    source: Range {
                        start: source_start,
                        end: source_start + length,
                    },
                    dest: Range {
                        start: dest_start,
                        end: dest_start + length,
                    },
                });
            }
        }
    }

    ranges.sort_unstable_by(|a, b| a.source.start.cmp(&b.source.start));
}
