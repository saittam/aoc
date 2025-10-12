use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let robots = lines
    .map(|l| {
      let mut li = l
        .split(|c: char| !c.is_digit(10) && c != '-')
        .filter_map(|w| w.parse::<i32>().ok());
      ((li.next().expect("x"), li.next().expect("y")),
       (li.next().expect("vx"), li.next().expect("vy")))
    })
    .collect::<Vec<_>>();

  const W: i32 = 101;
  const H: i32 = 103;
  const N: i32 = 100;

  let freq = robots.iter()
    .map(|(p, (vx, vy))| (p, ((vx + W) % W, (vy + H) % H)))
    .map(|((x, y), (vx, vy))|
         ((x + N * vx) % W, (y + N * vy) % H))
    .map(|(x, y)|
         ((x - W / 2).signum(), (y - H / 2).signum()))
    .filter(|(sx, sy)| *sx != 0 && *sy != 0)
    .map(|(sx, sy)| (3 * sy + sx + 4) as usize)
    .fold([0; 9], |mut f, q| { f[q] += 1; f });

  let n = freq[0] * freq[2] * freq[6] * freq[8];
  
  println!("{n}");
}