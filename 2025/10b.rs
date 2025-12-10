use std::io::BufRead;

fn gcd(mut a: i64, mut b: i64) -> i64 {
  while b != 0 {
    let tmp = a % b;
    a = b;
    b = tmp;
  }
  a
}

fn solve(buttons: &[u64], joltage: &[u64]) -> Option<i64> {
  // Tracks column (button) swaps.
  let mut cswap = (0..buttons.len()).collect::<Vec<_>>();

  // Build linear equation system.
  let mut rows = joltage
    .iter()
    .enumerate()
    .map(|(n, j)| {
      buttons
        .iter()
        .map(|b| ((1 << n) & b != 0) as i64)
        .chain([*j as i64])
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  // Gaussian elimination.
  for i in 0..rows.len() {
    // Swap
    let sw = (i..buttons.len())
      .flat_map(|swc| {
        rows.iter().enumerate().skip(i).filter_map(
          move |(swr, r)| (r[swc] != 0).then(|| (swr, swc)),
        )
      })
      .next();
    let (swr, swc) = if let Some(sw) = sw {
      sw
    } else {
      break;
    };
    rows.swap(i, swr);
    for r in rows.iter_mut() {
      r.swap(i, swc);
    }
    cswap.swap(i, swc);

    // Eliminate
    let (top, bot) = rows.split_at_mut(i + 1);
    let sub = top.last_mut().expect("last");
    for r in bot.iter_mut() {
      let d = gcd(r[i].abs(), sub[i].abs());
      let fsub = r[i] / d;
      let fr = sub[i] / d;
      for (esub, er) in sub.iter_mut().zip(r.iter_mut()) {
        *er = fr * *er - fsub * *esub;
      }
    }
  }

  // Strip zero rows.
  let nr = rows
    .iter()
    .position(|r| r.iter().rev().skip(1).all(|e| *e == 0));
  if let Some(nr) = nr {
    rows.truncate(nr);
  }

  // Calculate upper limits for button presses.
  let maxpress = cswap
    .iter()
    .map(|c| buttons[*c])
    .map(|b| {
      joltage
        .iter()
        .enumerate()
        .filter(|(k, _)| (1 << k) & b != 0)
        .map(|(_, j)| *j)
        .min()
        .unwrap_or(0)
    })
    .collect::<Vec<_>>();

  // exhaustive search over the variable parameters.
  let mut assign = vec![0; buttons.len()];
  let mut min = None;
  'search: loop {
    // Evaluate linear equation system.
    let mut posint = true;
    for (i, r) in
      rows.iter().take(buttons.len()).enumerate().rev()
    {
      let nom = r.last().unwrap()
        - r
          .iter()
          .zip(assign.iter())
          .skip(i + 1)
          .map(|(a, r)| a * r)
          .sum::<i64>();
      let denom = r[i];
      assign[i] = nom / denom;
      if assign[i] < 0 || nom % denom != 0 {
        posint = false;
        break;
      }
    }
    if posint {
      let v = assign.iter().copied().sum::<i64>();
      min = Some(min.map_or(v, |m| i64::min(v, m)));
    }

    // next permutation
    for i in (rows.len()..assign.len()).rev() {
      assign[i] += 1;
      if assign[i] <= maxpress[i] as i64 {
        continue 'search;
      }
      assign[i] = 0;
    }

    return min;
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let machines = lines
    .map(|l| {
      let (_, l) = l.split_once(']').expect("ind");
      let (buttons, l) =
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
      let joltage = l
        .trim()
        .strip_suffix("}")
        .expect("}")
        .split(',')
        .map(|w| w.trim().parse::<u64>().expect("num"))
        .collect::<Vec<_>>();

      (joltage, buttons)
    })
    .collect::<Vec<_>>();

  let n = machines
    .iter()
    .map(|(joltage, buttons)| {
      solve(buttons, joltage).expect("min")
    })
    .sum::<i64>();

  println!("{n}");
}
