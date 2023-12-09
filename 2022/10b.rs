use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut x: isize = 1;
  let mut pos = lines
    .take_while(|l| l.len() > 0)
    .flat_map(move |s| {
      let mut w = s.split(' ');
      match w.next().expect("instruction") {
        "noop" => vec![x],
        "addx" => {
          let r = vec![x, x];
          let a = w.next().expect("arg");
          x += a.parse::<isize>().expect("num");
          r
        }
        insn => panic!("instruction {}", insn)
      }
    });
  let r =
    (0..6).map(|_|
      pos
      .by_ref()
      .take(40)
      .enumerate()
      .map(|(p, x)| (x - p as isize).abs() <= 1)
      .map(|b| if b { '#' } else { ' ' })
      .collect::<String>())
    .collect::<Vec<_>>()
    .join("\n");
  println!("{}", r);
}