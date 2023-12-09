use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let id = lines.next().expect("id");

  let password = (0..)
    .map(|n| format!("{:x}", md5::compute(&format!("{}{}", id, n))))
    .filter(|s| s.starts_with("00000"))
    .filter_map(|s| {
      let mut ci = s.chars().skip(5);
      let pos = ci.next().expect("pos").to_digit(8)? as usize;
      let char = ci.next().expect("char");
      Some((pos, char))
    })
    .scan([None; 8], |pw, (p, c)| {
      pw[p] = pw[p].or(Some(c));
      Some(*pw)
    })
    .find_map(|pw| pw.iter()
                     .copied()
                     .collect::<Option<String>>())
    .expect("pw");
  println!("{}", password);
}