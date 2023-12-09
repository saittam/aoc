use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = lines
    .flat_map(|l| l.split_whitespace()
                   .take(3)
                   .map(|w| w.parse::<u32>().expect("num"))
                   .collect::<Vec<_>>());
  let mut tris = Vec::new();
  loop {
    let nine = nums.by_ref().take(9).collect::<Vec<_>>();
    if nine.len() != 9 {
      break;
    }
    tris.extend(
      (0..3).map(|i| {
        let mut tri = [nine[i], nine[i + 3], nine[i + 6]];
        tri.sort();
        tri
      }));
  }

  let n = tris.iter().filter(|t| t[0] + t[1] > t[2]).count();
  println!("{}", n);
}