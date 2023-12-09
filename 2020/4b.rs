use std::io::BufRead;
use std::collections::HashMap;
use std::ops::BitAnd;

fn year_valid(year: Option<&String>, min: u32, max: u32) -> Option<bool> {
  let v = year?.parse::<u32>().ok()?;
  Some(year?.len() == 4 && v >= min && v <= max)
}

fn height_valid(s: Option<&String>) -> Option<bool> {
  let l = s?.len();
  if l < 2 {
    return None;
  }
  let (ns, u) = s?.split_at(l - 2);
  let n = ns.parse::<u32>().ok()?;
  Some(match u {
    "cm" => 150 <= n && n <= 193,
    "in" => 59 <= n && n <= 76,
    _ => false
  })
}

fn hair_color_valid(v: Option<&String>) -> Option<bool> {
  Some(v?.len() == 7 && v?.chars().next() == Some('#') &&
       v?.chars().skip(1).all(|c| c.is_ascii_hexdigit()))
}

fn eye_color_valid(s: Option<&String>) -> Option<bool> {
  let clrs = [ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" ];
  Some(clrs.contains(&s?))
}

fn passport_id_valid(v: Option<&String>) -> Option<bool> {
  Some(v?.len() == 9 && v?.chars().all(char::is_numeric))
}

fn is_valid(r: &&HashMap<String, String>) -> bool {
  [
    year_valid(r.get("byr"), 1920, 2002),
    year_valid(r.get("iyr"), 2010, 2020),
    year_valid(r.get("eyr"), 2020, 2030),
    height_valid(r.get("hgt")),
    hair_color_valid(r.get("hcl")),
    eye_color_valid(r.get("ecl")),
    passport_id_valid(r.get("pid")),
  ].iter()
    .map(|r| r.unwrap_or(false))
    .fold(true, bool::bitand)
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {
    let mut d = String::new();
    loop {
      let mut buf = String::new();
      handle.read_line(&mut buf);
    
      if buf.trim().len() == 0 {
        break;
      }
      d += &buf;
    }
    
    if d.trim().len() == 0 {
      break;
    }
    
    let r = d.split(char::is_whitespace)
      .filter(|s| s.len() > 0)
      .map(|s| s.split(':'))
      .map(|mut a| (a.next().unwrap().to_string(),
                    a.next().unwrap().to_string()))
      .collect::<HashMap<String, String>>();
    m.push(r);
  }
  
  let valid = m.iter().filter(is_valid).count();
    
  println!("{:?}", valid);
}