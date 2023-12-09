fn main() {
	let pfxlen = 209231;
	let slen = 10;
	let mut seq = Vec::<u8>::with_capacity(pfxlen + slen + 5);
	seq.push(3);
	seq.push(7);
	
	let mut p: [usize; 2] = [0, 1];
	while seq.len() < pfxlen + slen {
		let sum: u8 = p.iter().map(|i| seq[*i]).sum();
		
		if sum >= 10 {
			seq.push(sum / 10);
		}
		seq.push(sum % 10);
		
		for i in p.iter_mut() {
			*i = (*i + 1 + seq[*i] as usize) % seq.len();
		}
		
		//println!("{:?}", seq);
	}
	
	println!("{}", seq[pfxlen..(pfxlen + slen)].iter()
		.map(|i| format!("{}", i))
		.collect::<Vec<String>>().join(""));
}