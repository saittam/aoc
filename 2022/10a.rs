use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut x: isize = 1;
  let r = lines
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
    })
    .enumerate()
    .map(|(n, x)| (n + 1) as isize * x)
    .skip(19)
    .step_by(40)
    .take(6)
    .sum::<isize>();
  
  println!("{}", r);
}