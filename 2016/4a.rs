use std::io::BufRead;
use std::cmp::Reverse;

fn checksum(s: &str) -> String {
  let mut freq = ('a'..='z')
    .map(|c| (Reverse(s.chars().filter(|x| *x == c).count()),
              c))
    .collect::<Vec<_>>();
  freq.sort();
  freq.iter()
    .take(5)
    .map(|(_, c)| c)
    .collect::<String>()
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let rooms = lines
    .map(|l| {
      let mut rsi = l.rsplitn(2, '-');
      let tail = rsi.next().expect("tail");
      let name = rsi.next().expect("name");
      let mut taili =
        tail.split(|c: char| !c.is_alphanumeric());
      let sector_id = taili.next().expect("sector ID")
        .parse::<u32>().expect("num");
      let checksum = taili.next().expect("checksum");
      (name.to_string(), sector_id, checksum.to_string())
    })
    .collect::<Vec<_>>();

  let n = rooms.iter()
    .filter(|(n, _, c)| checksum(n) == *c)
    .map(|(_, s, _)| *s)
    .sum::<u32>();
  println!("{}", n);
}