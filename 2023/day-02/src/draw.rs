use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

lazy_static! {
    static ref DRAW_REGEX: Regex = Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();
}

pub struct Draw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Draw {
    pub fn parse(line: &str) -> Draw {
        let mut draw = Draw {
            blue: 0,
            red: 0,
            green: 0,
        };

        line.split(", ")
            .map(|count| DRAW_REGEX.captures(count).unwrap())
            .map(|capture| {
                (
                    capture
                        .name("count")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap(),
                    capture.name("color").unwrap().as_str(),
                )
            })
            .for_each(|(count, color)| match color {
                "blue" => draw.blue = count,
                "red" => draw.red = count,
                "green" => draw.green = count,
                color => panic!("Unknown color {}", color),
            });

        draw
    }

    pub fn allows(&self, other: &Draw) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    pub fn ensure(mut self, other: &Draw) -> Self {
        self.red = cmp::max(self.red, other.red);
        self.green = cmp::max(self.green, other.green);
        self.blue = cmp::max(self.blue, other.blue);
        self
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
