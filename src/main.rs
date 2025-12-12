use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    os::fd::AsRawFd,
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
    let buffer = mmap();

    let mut map: HashMap<Vec<u8>, Stats> = HashMap::new();

    for line in buffer.split(|c| *c == b'\n') {
        if line.is_empty() {
            break;
        }
        let mut fields = line.rsplitn(2, |c| *c == b';');
        let temp = fields.next().unwrap_or_else(|| {
            panic!("Temp was none: {:?}", unsafe {
                std::str::from_utf8_unchecked(line)
            })
        });
        let city = fields.next().unwrap_or_else(|| {
            panic!("City was none: {:?}", unsafe {
                std::str::from_utf8_unchecked(line)
            })
        });
        let temp = unsafe { std::str::from_utf8_unchecked(temp) }
            .parse::<f64>()
            .unwrap();
        match map.get_mut(city) {
            Some(stats) => stats.update(temp),
            None => {
                map.insert(city.into(), Stats::new(temp));
            }
        }
        map.entry(city.to_owned())
            .and_modify(|stats: &mut Stats| stats.update(temp))
            .or_insert(Stats::new(temp));
    }

    let map = map
        .into_iter()
        .map(|(k, v)| (unsafe { String::from_utf8_unchecked(k) }, v))
        .collect::<BTreeMap<_, _>>();

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

fn mmap() -> &'static [u8] {
    let file = File::open("./measurements.txt").unwrap();
    let len = file.metadata().unwrap().len();
    unsafe {
        let pointer = libc::mmap(
            std::ptr::null_mut(),
            len as libc::size_t,
            libc::PROT_READ,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );

        assert!(
            pointer != libc::MAP_FAILED,
            "Memory map failed: {:?}",
            std::io::Error::last_os_error()
        );

        assert!(libc::madvise(pointer, len as libc::size_t, libc::MADV_SEQUENTIAL) == 0);

        std::slice::from_raw_parts(pointer as *const u8, len as usize)
    }
}
