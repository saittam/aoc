use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut blocklist = lines
    .map(|l| {
      let mut wi = l.split('-')
        .map(|w| w.parse::<u32>().expect("num"));
      (wi.next().expect("low"), wi.next().expect("high"))
    })
    .collect::<Vec<_>>();

  blocklist.sort();
  let (mut merged, last) = blocklist.iter()
    .fold((Vec::new(), None), |(mut merged, cur), (l, h)| {
      let cur = Some(match cur {
        None => (*l, *h),
        Some((cl, ch)) => {
          if ch < *l - 1 {
            merged.push((cl, ch));
            (*l, *h)
          } else {
            (cl, u32::max(ch, *h))
          }
        }
      });
      (merged, cur)
    });
  merged.extend(last);

  let avail = if let Some((l, h)) = merged.first() {
    if *l == 0 { *h + 1 } else { 0 }
  } else {
    0
  };
  
  println!("{}", avail);
}