use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let input = lines.next().unwrap();
  
  const red: [char; 6] = [':', '"', 'r', 'e', 'd', '"'];
  let v = input.chars()
    .fold(vec![(true, 0, 0, 0, 1)],
          |mut v, c| {
            match c {
              '{' | '[' => v.push((true, 0, 0, 0, 1)),
              '}' | ']' => {
                let (ca, _, cr, cn, cs) = v.pop().unwrap();
                v.last_mut().unwrap().2 += 
                  (cr + cn * cs) * ca as i64;
              }
              c => {
                let (a, i, r, n, s) = v.last_mut().unwrap();
                *i = if c == red[*i] { *i + 1 } else { 0 };
                if *i == red.len() {
                  *a = false;
                  *i = 0;
                }
                match c.to_digit(10) {
                  Some(d) => *n = *n * 10 + d as i64,
                  None => {
                    *r = *r + *n * *s;
                    *n = 0;
                    *s = if c == '-' { -1 } else { 1 };
                  }
                };
              }
            }
            v
          });
  let (_, _, r, n, s) = v.last().unwrap();
  println!("{}", r + n * s);
}