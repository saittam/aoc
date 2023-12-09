use std::io::BufRead;

fn mask(s: &str) -> u8 {
  s.chars().fold(0u8,
    |a, c| a | (1 << "abcdefg".find(c).unwrap()))
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut sum = 0;
  for l in lines.take_while(|l| l.len() > 0) {
    let mut si = l.split_whitespace();
    
    let mut p = si.by_ref()
      .take(10)
      .map(mask)
      .collect::<Vec<_>>();
    p.sort_by_key(|v| v.count_ones());

    let neg = p[9];
    let p6n = (p[6] ^ p[7] ^ p[8]) ^ p[9];
    let ma = p[0] ^ p[1];
    let mc = p[0] & p6n;
    let mf = mc ^ p[0];
    let me = (p[2] ^ neg) & p6n;
    let md = p6n ^ (me | mc);
    let mb = p[2] ^ (p[0] | md);
    let mg = neg ^ (p[2] | ma | me);
    
    let num = [
      neg ^ md,
      p[0],
      neg ^ (mb | mf),
      neg ^ (mb | me),
      p[2],
      neg ^ (mc | me),
      neg ^ mc,
      p[1],
      p[9],
      neg ^ me,
    ];
    
    sum += si.skip(1).map(mask)
      .map(|m| num.iter().position(|n| *n == m).unwrap())
      .fold(0, |a, d| 10 * a + d);
  }

  println!("{}", sum);
}