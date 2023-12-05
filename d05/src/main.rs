use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Lines},
    ops::RangeBounds,
};

// EXCLUSIVE
#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn new(src: usize, len: usize) -> Self {
        Range {
            start: src,
            end: src + len,
        }
    }

    pub fn contains(&self, idx: usize) -> bool {
        idx >= self.start && idx < self.end
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

impl MapRange {
    pub fn new(src: usize, dst: usize, len: usize) -> Self {
        Self {
            src: Range::new(src, len),
            dst: Range::new(dst, len),
        }
    }

    pub fn src_contains(&self, idx: usize) -> bool {
        self.src.contains(idx)
    }

    pub fn map(&self, idx: usize) -> Option<usize> {
        if self.src_contains(idx) {
            Some(self.dst.start + (idx - self.src.start))
        } else {
            None
        }
    }

    pub fn map_intersect(&self, other: &Range) -> Range {
        let intersect = self.src.intersect(other);

        match intersect {
            Some(i) => {
                let delta = self.dst.start as i64 - self.src.start as i64;
                Range {
                    start: (i.start as i64 + delta) as usize,
                    end: (i.end as i64 + delta) as usize,
                }
            }
            None => Range {
                start: other.start,
                end: other.end,
            },
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

        ranges.push(MapRange::new(dst, src, len))
    }

    ranges.sort_by_key(|r| r.src.start);

    ranges
}

fn find_range(idx: usize, ranges: &Vec<MapRange>) -> Option<&MapRange> {
    ranges
        .binary_search_by(|r| {
            if idx < r.src.start {
                Ordering::Less
            } else if idx >= r.src.end {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
        .map(|idx| &ranges[idx])
        .ok()
}

fn lookup(idx: usize, map: &Vec<MapRange>) -> usize {
    let range = find_range(idx, map);

    match range {
        Some(range) => range.map(idx).unwrap(),
        None => idx,
    }
}

fn lookup_range(ranges: &Vec<Range>, map: &Vec<MapRange>) -> Vec<Range> {
    println!("{ranges:?}");
    println!("{map:?}");
    let mut ret: Vec<Range> = Vec::new();

    for r in ranges {
        let intersected = map.iter().map(|map_range| map_range.map_intersect(&r));
        ret.extend(intersected)
    }

    println!("{ret:?}");

    ret
}

fn main() -> std::io::Result<()> {
    let file = File::open("d05/src/test.txt")?;
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

        // println!("{soil:?}");
        // println!("{fert:?}");
        // println!("{water:?}");
        // println!("{light:?}");
        // println!("{temp:?}");
        // println!("{hum:?}");
        // println!("{location:?}");
        let smallest = location.iter().map(|l| l.start).min().unwrap();

        lowest = lowest.min(smallest);

        println!("--------------")
    }

    println!("{lowest}");

    Ok(())
}

// overlapping ranges at each stage of the lookup, need to attribute output ranges
// and remove overlaps