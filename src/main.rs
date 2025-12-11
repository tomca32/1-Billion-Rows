use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

struct Stats {
    min: f64,
    max: f64,
    count: u32,
    sum: f64,
}

impl Stats {
    pub fn new(temp: f64) -> Self {
        Self {
            min: temp,
            max: temp,
            count: 1,
            sum: temp,
        }
    }

    pub fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }

    pub fn average(&self) -> f64 {
        self.sum / f64::from(self.count)
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}/{:.2}/{:.2}", self.min, self.average(), self.max)
    }
}

fn main() {
    let file = File::open("./measurements.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: HashMap<String, Stats> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (city, temp) = line.split_once(';').unwrap();
        let temp = temp.parse::<f64>().unwrap();
        match map.get_mut(city) {
            Some(stats) => stats.update(temp),
            None => {
                map.insert(city.to_string(), Stats::new(temp));
            }
        }
        map.entry(city.to_owned())
            .and_modify(|stats: &mut Stats| stats.update(temp))
            .or_insert(Stats::new(temp));
    }

    let map = BTreeMap::from_iter(map);

    print!("{{");
    let mut iter = map.into_iter().peekable();
    while let Some((city, stats)) = iter.next() {
        print!("{city}:{stats}");
        if iter.peek().is_some() {
            print!(", ");
        }
    }

    print!("}}");
}
