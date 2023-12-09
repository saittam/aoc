use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let layers = lines.map(|l| {
      let mut ni = l.split(|c: char| !c.is_numeric())
        .filter_map(|w| w.parse::<u32>().ok());
      (ni.next().expect("layer"), ni.next().expect("depth"))
    })
    .collect::<Vec<_>>();

  let sev = layers.iter()
    .filter(|(l, d)| l % (2 * (d - 1)) == 0)
    .map(|(l, d)| l * d)
    .sum::<u32>();
  
  println!("{}", sev);
}