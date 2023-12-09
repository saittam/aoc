use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Effect {
  Shield = 0,
  Poison = 1,
  Recharge = 2,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
  spent: Reverse<u32>,
  boss_hp: Reverse<i32>,
  player_hp: i32,
  mana: u32,
  effects: [u8; 3],
}

impl State {
  fn tick(&mut self) {
    if self.effects[Effect::Poison as usize] > 0 {
        self.boss_hp.0 -= 3;
    }
    if self.effects[Effect::Recharge as usize] > 0 {
      self.mana += 101;
    }
    for t in self.effects.iter_mut() {
      *t = t.saturating_sub(1);
    }
  }

  fn push(&mut self, d: u8, e: Effect) -> bool {
    if self.effects[e as usize] > 0 {
      false
    } else {
      self.effects[e as usize] = d;
      true
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  let mut nums = lines.map(|l| l.rsplit(' ')
                                .next()
                                .expect("field")
                                .parse::<i32>()
                                .expect("number"));

  let boss_hp = nums.next().expect("hitpoints");
  let boss_damage = nums.next().expect("damage");

  let mut q = BinaryHeap::new();
  q.push(State {
    spent: Reverse(0),
    boss_hp: Reverse(boss_hp),
    player_hp: 50,
    mana: 500,
    effects: [0; 3],
  });
  let mut seen = HashSet::new();
'top:
  while let Some(mut s) = q.pop() {
    if !seen.insert(s.clone()) {
      continue;
    }

    s.player_hp -= 1;
    s.tick();
    
    const SPELLS: [(u32, fn(&mut State) -> bool); 5] = [
      (53, |s| { s.boss_hp.0 -= 4; true }),
      (73, |s| { s.boss_hp.0 -= 2; s.player_hp += 2; true }),
      (113, |s| { s.push(6, Effect::Shield) }),
      (173, |s| { s.push(6, Effect::Poison) }),
      (229, |s| { s.push(5, Effect::Recharge) }),
    ];
    for (cost, spell) in &SPELLS {
      if *cost > s.mana {
        continue;
      }
      let mut snew = s.clone();
      snew.mana -= cost;
      snew.spent.0 += cost;
      if !spell(&mut snew) {
        continue;
      }

      snew.tick();
      
      if snew.boss_hp.0 <= 0 {
        println!("{}", snew.spent.0);
        break 'top;
      }

      let dshield = snew.effects[Effect::Shield as usize];
      let armor = if dshield > 0 { 7 } else { 0 };
      snew.player_hp -= i32::max(boss_damage - armor, 1);

      if snew.player_hp > 0 {
        q.push(snew);
      }
    }
  }
    }