fn main() {
	let input = 209231;
	let imod = 1000000;
	let slen = 10;
	let mut seq = [0u8; 50000000];
	seq[0] = 3;
	seq[1] = 7;
	seq[2] = 1;
	seq[3] = 0;
	seq[4] = 1;
	seq[5] = 0;
	
	let mut val: usize = 371010;
	let mut p: [usize; 2] = [4, 3];
	let mut hist: [usize; 10] = [0; 10];
	
	
   let mut i = 6;
'o:
	while i < seq.len() - 1 {
		let mut sum: u8 = seq[p[0]] + seq[p[1]];
	   
		if sum >= 10 {
			seq[i] = 1;
			val = (val * 10 + 1) - (seq[i - 6] as usize * 1000000);
			i += 1;
			//println!("{}", val);
			if val == input {
				break 'o;
			}
			sum -= 10;
		};
		
		let d = sum;
      seq[i] = d;
      val = (val * 10 + d as usize) - (seq[i - 6] as usize * 1000000);
		i += 1;
		//println!("{}", val);
	   if val == input {
			break 'o;
		}
		
		let l = i;
		p[0] += 1 + seq[p[0]] as usize;
		p[1] += 1 + seq[p[1]] as usize;
		if p[0] >= l { p[0] -= l }
		if p[1] >= l { p[1] -= l }
		
		//println!("{:?}", seq);
	}
	
	println!("{}", val);
	println!("{}", i - 6);
	//println!("{:?}", hist);
}
    
