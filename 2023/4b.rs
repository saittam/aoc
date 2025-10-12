use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let cards = lines
    .map(|l| {
      let mut pi = l.split(&[':', '|']).skip(1)
        .map(|p| p.split_whitespace()
                  .map(|w| w.parse::<i32>().expect("num"))
                  .collect::<HashSet<_>>());
      (pi.next().expect("winning"),
       pi.next().expect("nums"))
    })
    .collect::<Vec<_>>();

  let (_, _, n) = cards.iter()
    .map(|(w, n)| w.intersection(n).count())
    .enumerate()
    .fold((BinaryHeap::new(), 1, 0),
          |(mut q, mut a, n), (i, k)| {
            while matches!(q.peek(),
                           Some(&(Reverse(e), _)) if e == i) {
              let (_, da) = q.pop().unwrap();
              a -= da;
            }
            q.push((Reverse(i + k + 1), a));
            let ap = a;
            a += a;
            (q, a, n + ap)
          });
 
  println!("{}", n);
}