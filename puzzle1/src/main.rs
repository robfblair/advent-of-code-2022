use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let ifile = File::open(&args[1])?;

  let mut max_cals = 0;
  let mut mid_cals = 0;
  let mut low_cals = 0;
  let mut curr_cals = 0;

  for line in BufReader::new(ifile).lines() {
    let cals = line?.parse().unwrap_or(-1);
    if cals != -1 {
      curr_cals += cals;
    } else {
      if curr_cals > max_cals {
        low_cals = mid_cals;
        mid_cals = max_cals;
        max_cals = curr_cals;
      } else if curr_cals > mid_cals {
        low_cals = mid_cals;
        mid_cals = curr_cals;
      } else if curr_cals > low_cals {
        low_cals = curr_cals;
      }
      curr_cals = 0;
    }
  }

  dbg!(max_cals);
  dbg!(max_cals + mid_cals + low_cals);
  Ok(())
}
