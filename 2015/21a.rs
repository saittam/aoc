use std::io::BufRead;

fn div_ru(n: usize, d: usize) -> usize {
  (n + d - 1) / d
}

fn damage(damage: usize, armor: usize) -> usize {
  if damage > armor { damage - armor } else { 1 }
}

struct Char {
  hitpoints: usize,
  damage: usize,
  armor: usize,
}

impl Char {
  fn new(hitpoints: usize,
         damage: usize,
         armor: usize) -> Char {
    Char { hitpoints, damage, armor }
  }

  fn fight(&self, opp: &Char) -> bool {
    div_ru(opp.hitpoints, damage(self.damage, opp.armor)) <=
    div_ru(self.hitpoints, damage(opp.damage, self.armor))
  }
}

fn sum_triple(a: (usize, usize, usize),
              b: (usize, usize, usize))
  -> (usize, usize, usize) {
  (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  let mut nums = lines.map(|l| l.rsplit(' ')
                                .next()
                                .expect("field")
                                .parse::<usize>()
                                .expect("number"));

  let opp = Char {
    hitpoints: nums.next().expect("hitpoints"),
    damage: nums.next().expect("damage"),
    armor: nums.next().expect("armor"),
  };

  /*
  Weapons:    Cost  Damage  Armor
  Dagger        8     4       0
  Shortsword   10     5       0
  Warhammer    25     6       0
  Longsword    40     7       0
  Greataxe     74     8       0
  
  Armor:      Cost  Damage  Armor
  Leather      13     0       1
  Chainmail    31     0       2
  Splintmail   53     0       3
  Bandedmail   75     0       4
  Platemail   102     0       5
  
  Rings:      Cost  Damage  Armor
  Damage +1    25     1       0
  Damage +2    50     2       0
  Damage +3   100     3       0
  Defense +1   20     0       1
  Defense +2   40     0       2
  Defense +3   80     0       3
  */

  const WEAPONS: [(usize, usize, usize); 5] = [
    (8, 4, 0),
    (10, 5, 0),
    (25, 6, 0),
    (40, 7, 0),
    (74, 8, 0),
  ];

  const ARMOR: [(usize, usize, usize); 6] = [
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
  ];

  const RINGS: [(usize, usize, usize); 7] = [
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
  ];

  let min = WEAPONS.iter().flat_map(
    |w| ARMOR.iter().flat_map(
      |a| [[*w, *a, (0, 0, 0), (0, 0, 0)]].into_iter().chain(
        RINGS.iter().flat_map(
          |r1| RINGS.iter().map(
            |r2| (*r1, *r2)))
        .filter(|(r1, r2)| r1 > r2)
        .map(|(r1, r2)| [*w, *a, r1, r2]))))
    .filter_map(|l| l.iter().cloned().reduce(sum_triple))
    .map(|(c, d, a)| (c, Char::new(100, d, a).fight(&opp)))
    .filter(|(_, w)| *w)
    .map(|(c, _)| c)
    .min();
    
  println!("{}", min.expect("cannot win"));
}