use std::io::BufRead;
use std::cmp::Ordering;

#[derive(Debug)]
enum Step {
  Pop,
  Inc,
  Push,
  Done,
}

struct PackIter<'a> {
  nums: &'a mut [u32],
  target_weight: u32,
  current_weight: u32,
  cursor: Vec<usize>,
  step: Step,
}

impl<'a> PackIter<'a> {
  fn new(nums: &'a mut[u32],
         target_weight: u32) -> PackIter<'a> {
    PackIter {
      nums,
      target_weight,
      current_weight: 0,
      cursor: vec![0],
      step: Step::Push,
    }
  }

  fn pop(&mut self) -> Option<()> {
    let split = self.cursor.len() - 1;
    self.current_weight -= self.nums[split];
    let cursor = *self.cursor.last().unwrap();
    self.nums.swap(cursor, split);
    self.inc()
  }

  fn inc(&mut self) -> Option<()> {
    if *self.cursor.last().unwrap() >= self.nums.len() - 1 {
      self.cursor.pop();
      if self.cursor.is_empty() {
        self.step = Step::Done;
        None
      } else {
        self.pop()
      }
    } else {
      let cursor_next = self.cursor.last_mut().unwrap();
      *cursor_next += 1;
      self.push()
    }
  }

  fn push(&mut self) -> Option<()> {
    let split = self.cursor.len() - 1;
    self.nums.swap(*self.cursor.last().unwrap(), split);
    self.current_weight += self.nums[split];
    Some(())
  }

  fn next<'s>(&'s mut self)
      -> Option<(&'s mut[u32], &'s mut[u32])> {
    loop {
      match self.step {
        Step::Pop => self.pop(),
        Step::Inc => self.inc(),
        Step::Push => self.push(),
        Step::Done => None
      }?;

      match self.current_weight.cmp(&self.target_weight) {
        Ordering::Less => {
          self.cursor.push(*self.cursor.last().unwrap());
          self.step = Step::Inc;
        }
        Ordering::Equal => {
          self.step = Step::Pop;
          let split = self.cursor.len();
          return Some(self.nums.split_at_mut(split));
        }
        Ordering::Greater => self.step = Step::Pop,
      }
    }
  }
}

impl<'a> Drop for PackIter<'a> {
  fn drop(&mut self) {
    while let Some(pos) = self.cursor.pop() {
      self.nums.swap(pos, self.cursor.len());
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = lines
    .map(|l| l.parse::<u32>().expect("num"))
    .collect::<Vec<_>>();

  let weight = nums.iter().sum::<u32>() / 3;

  let mut qe = u128::MAX;
  let mut size = usize::MAX;
  let mut pi = PackIter::new(&mut nums, weight);
  while let Some((l, r)) = pi.next() {
    if l.len() <= size {
      if PackIter::new(r, weight).next().is_some() {
        let qec: u128 =
          l.iter().map(|n| *n as u128).product();
        if l.len() < size || qec < qe {
          size = l.len();
          qe = qec;
        }
      }
    }
  }

  println!("{}", qe);
}