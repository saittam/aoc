use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  let mut nums = lines.map(|s| s.parse::<u64>().unwrap());
  
  let card_pkey = nums.next().unwrap();
  let door_pkey = nums.next().unwrap();

  let mut val = 1;
  let mut card_loop_size = 0;
  while val != card_pkey {
    card_loop_size += 1;
    val = (val * 7) % 20201227;
  };

  val = 1;
  for _ in 0..card_loop_size {
    val = (val * door_pkey) % 20201227;
  }

  println!("{}", val);
}

