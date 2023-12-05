use std::{cmp, ops::Range};

use input::{parse_input, Input, RangeMap};

mod input;

fn main() {
    let input = parse_input("input.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|id| map_id(&input.seed_to_soil, *id))
        .map(|id| map_id(&input.soil_to_fertilizer, id))
        .map(|id| map_id(&input.fertilizer_to_water, id))
        .map(|id| map_id(&input.water_to_light, id))
        .map(|id| map_id(&input.light_to_temperature, id))
        .map(|id| map_id(&input.temperature_to_humidity, id))
        .map(|id| map_id(&input.humidity_to_location, id))
        .reduce(|min, next| cmp::min(min, next))
        .unwrap()
}

fn map_id(ranges: &Vec<RangeMap>, id: u64) -> u64 {
    let range = ranges.iter().find(|range| range.source.contains(&id));

    match range {
        None => id,
        Some(range) => range.dest.start + id - range.source.start,
    }
}

fn part_two(input: &Input) -> u64 {
    let ranges = map_ranges(&input.seed_ranges, &input.seed_to_soil);
    let ranges = map_ranges(&ranges, &input.soil_to_fertilizer);
    let ranges = map_ranges(&ranges, &input.fertilizer_to_water);
    let ranges = map_ranges(&ranges, &input.water_to_light);
    let ranges = map_ranges(&ranges, &input.light_to_temperature);
    let ranges = map_ranges(&ranges, &input.temperature_to_humidity);
    let ranges = map_ranges(&ranges, &input.humidity_to_location);

    ranges.first().unwrap().start
}

fn map_ranges(ranges: &Vec<Range<u64>>, maps: &Vec<RangeMap>) -> Vec<Range<u64>> {
    let mut result = Vec::new();
    let mut map_iter = maps.iter();
    let mut map = map_iter.next();

    for input in ranges {
        let mut input = input.clone();

        loop {
            match map {
                Some(m) => {
                    if m.source.start <= input.start {
                        // map entirely before range
                        //   Input:        |-----|
                        //   Map:   |---|
                        if m.source.end <= input.start {
                            // get a new map before proceeding
                            map = map_iter.next();
                        }
                        // Map overlaps start of range.
                        //   Input:    |-----|
                        //   Map:   |---|
                        else if m.source.end < input.end {
                            // map first part of input.
                            let offset = input.start - m.source.start;
                            let length = m.source.end - input.start;

                            result.push(Range {
                                start: m.dest.start + offset,
                                end: m.dest.start + offset + length,
                            });

                            input.start = m.source.end;
                            map = map_iter.next();
                        }
                        // map overlaps entire range
                        //   Input:   |-----|
                        //   Map:   |--------|
                        else {
                            let offset = input.start - m.source.start;
                            let length = input.end - input.start;

                            result.push(Range {
                                start: m.dest.start + offset,
                                end: m.dest.start + offset + length,
                            });
                            break;
                        }
                    } else if m.source.start < input.end {
                        // map overlaps part of range
                        //   Input: |-------|
                        //   Map:     |---|
                        if m.source.end < input.end {
                            // first input section is directly mapped.
                            result.push(Range {
                                start: input.start,
                                end: m.source.start,
                            });

                            // mapped section pushed directly.
                            result.push(m.dest.clone());

                            // update range to get next map.
                            input.start = m.source.end;
                            map = map_iter.next();
                        }
                        // map overlaps end of range
                        //   Input: |-----|
                        //   Map:      |---|
                        else {
                            // first input section is directly mapped.
                            result.push(Range {
                                start: input.start,
                                end: m.source.start,
                            });

                            let length = input.end - m.source.start;
                            result.push(Range {
                                start: m.dest.start,
                                end: m.dest.start + length,
                            });
                            break;
                        }
                    }
                    // map entirely after range
                    //   Input: |-----|
                    //   Map:           |---|
                    else {
                        result.push(input);
                        break;
                    }
                }

                // no more maps
                None => {
                    result.push(input);
                    break;
                }
            }
        }
    }

    result.sort_by(|a, b| a.start.cmp(&b.start));
    result
}
