use std::{
    fs::File,
    io::{BufRead, BufReader, Lines}, fmt::Display,
};

// EXCLUSIVE
#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl Range {
    pub fn new(src: usize, len: usize) -> Self {
        Range {
            start: src,
            end: src + len,
        }
    }

    pub fn len(&self) -> u64 {
        if self.end < self.start {
            0
        }
        else {
            (self.end - self.start) as u64
        }
    }

    pub fn intersect(&self, other: &Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if end < start {
            None
        } else {
            Some(Range { start, end })
        }
    }

    pub fn map(&self, other: &Vec<MapRange>) -> Vec<Range> {
        println!("Mapping {self}");
        display(other); 

        let mut ret: Vec<Range> = Vec::new();

        let mut range = Range {
            start: self.start,
            end: self.end
        };

        for mr in other {
            match mr.src.intersect(&range) {
                Some(intersected) => {
                    // If we're past the start of our range:
                    if intersected.start > range.start {
                        // Output the "leftover" identity mapped portion
                        ret.push(Range {
                            start: range.start,
                            end: intersected.start
                        });
                    }

                    // Output the mapped portion
                    ret.push(mr.map(&intersected));

                    // Adjust our range to start "past" what we've already processed.
                    range = Range {
                        start: intersected.end,
                        end: range.end
                    }
                },
                None => {},
            };

            if range.len() == 0 {
                break
            }
        };

        // If we have anything left, output the remainder
        if range.len() != 0 {
            ret.push(range);
        }

        println!("Result:");
        display(&ret);

        ret
    }
}

fn parse_seeds(line: &str) -> Vec<Range> {
    let mut components = line.split_whitespace();
    components.next();

    let digits: Vec<usize> = components
        .map(|c| usize::from_str_radix(c, 10).unwrap())
        .collect();

    digits
        .chunks(2)
        .map(|pair| Range {
            start: pair[0],
            end: pair[0] + pair[1],
        })
        .collect()
}

#[derive(Debug)]
struct MapRange {
    src: Range,
    dst: Range,
}

impl Display for MapRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} -> {} (delta {})]", self.src, self.dst, self.delta())
    }
}

impl MapRange {
    pub fn new(src: usize, dst: usize, len: usize) -> Self {
        Self {
            src: Range::new(src, len),
            dst: Range::new(dst, len),
        }
    }

    pub fn delta(&self) -> i64 {
        return self.dst.start as i64 - self.src.start as i64;
    }

    pub fn map(&self, range: &Range) -> Range {
        // We assume someone has already checked that this intersects the src.
        Range {
            start: (range.start as i64 + self.delta()) as usize,
            end: (range.end as i64 + self.delta()) as usize
        }
    }
}

fn parse_ranges(reader: &mut Lines<BufReader<File>>) -> Vec<MapRange> {
    reader.next();

    let mut ranges: Vec<MapRange> = Vec::new();

    for l in reader.take_while(|l| !l.as_ref().unwrap().is_empty()) {
        let l = l.unwrap();
        let numbers: Vec<usize> = l
            .split_whitespace()
            .map(|c| usize::from_str_radix(c, 10).unwrap())
            .collect();

        let dst = numbers[0];
        let src = numbers[1];
        let len = numbers[2];

        ranges.push(MapRange::new(src, dst, len))
    }

    ranges.sort_by_key(|r| r.src.start);

    ranges
}

fn lookup_range(ranges: &Vec<Range>, map: &Vec<MapRange>) -> Vec<Range> {
    let mut ret: Vec<Range> = Vec::new();

    for r in ranges {
        ret.extend(r.map(map))
    }

    ret
}

fn display<T>(v: &Vec<T>) where T: ToString {
    let s = v.iter().map(|mr| mr.to_string()).collect::<Vec<String>>().join(", ");
    println!("{s}");
}

fn main() -> std::io::Result<()> {
    let file = File::open("d05/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();

    let seeds = parse_seeds(&lines.next().unwrap()?);
    lines.next();

    let seed_to_soil = parse_ranges(&mut lines);
    let soil_to_fert = parse_ranges(&mut lines);
    let fert_to_water = parse_ranges(&mut lines);
    let water_to_light = parse_ranges(&mut lines);
    let light_to_temp = parse_ranges(&mut lines);
    let temp_to_hum = parse_ranges(&mut lines);
    let hum_to_locate = parse_ranges(&mut lines);

    let mut lowest = usize::MAX;

    for range in seeds {
        let soil = lookup_range(&vec![range], &seed_to_soil);
        let fert = lookup_range(&soil, &soil_to_fert);
        let water = lookup_range(&fert, &fert_to_water);
        let light = lookup_range(&water, &water_to_light);
        let temp = lookup_range(&light, &light_to_temp);
        let hum = lookup_range(&temp, &temp_to_hum);
        let location = lookup_range(&hum, &hum_to_locate);

        println!("Soil:");
        display(&soil);
        println!("Fert:");
        display(&fert);
        println!("Water:");
        display(&water);
        println!("Light:");
        display(&light);
        println!("Temp:");
        display(&temp);
        println!("Hum");
        display(&hum);
        println!("Location:");
        display(&location);
        let smallest = location.iter().map(|l| l.start).min().unwrap();

        lowest = lowest.min(smallest);

    }

    println!("{lowest}");

    Ok(())
}

// overlapping ranges at each stage of the lookup, need to attribute output ranges
// and remove overlaps