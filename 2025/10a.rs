use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let machines = lines
    .map(|l| {
      let (ind, l) = l.split_once(']').expect("ind");
      let ind = ind.trim().strip_prefix("[").expect("[");
      let ind = ind
        .chars()
        .rev()
        .fold(0, |ind, c| (ind << 1) | (c == '#') as u64);
      let (buttons, _) =
        l.split_once('{').expect("buttons");
      let buttons = buttons
        .trim()
        .strip_suffix(")")
        .expect(")")
        .split(')')
        .map(|button| {
          button
            .trim()
            .strip_prefix("(")
            .expect("(")
            .split(',')
            .map(|w| w.trim().parse::<u64>().expect("num"))
            .fold(0, |button, i| button | (1 << i))
        })
        .collect::<Vec<_>>();
      (ind, buttons)
    })
    .collect::<Vec<_>>();

  let n = machines
    .iter()
    .map(|(ind, buttons)| {
      (0..(1 << buttons.len() as u64))
        .filter(|pat| {
          buttons
            .iter()
            .enumerate()
            .filter(|(i, _)| (1 << i) & pat != 0)
            .fold(0, |a, (_, b)| a ^ b)
            == *ind
        })
        .map(|pat: u64| pat.count_ones())
        .min()
        .expect("min")
    })
    .sum::<u32>();

  println!("{n}");
}
