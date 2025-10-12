use std::io::BufRead;

const CARDS: &'static str = "23456789TJQKA";

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut hands = lines.map(|l| {
    let mut wi = l.split_whitespace();
    let hand = wi.next().expect("hand")
      .chars()
      .map(|hc| CARDS.chars()
                     .position(|c| c == hc)
                     .expect("card"))
      .collect::<Vec<_>>();
    let bid = wi.next().expect("bid")
      .parse::<u64>().expect("num");
    (hand, bid)
  })
    .collect::<Vec<_>>();

  hands.sort_by_key(|(h, _)| {
      let mut freq = h.iter().fold(
        [0; CARDS.len()],
        |mut freq, c| {
          freq[*c] += 1;
          freq
        });
      freq.sort();
      freq.reverse();
      (freq, h.clone())
    });

  let n = hands.iter()
    .enumerate()
    .map(|(i, (_, b))| (i as u64 + 1) * b)
    .sum::<u64>();
 
  println!("{}", n);
}