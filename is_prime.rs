fn is_prime(n: u32) -> bool {
	if n <=1 {
		return false;
	}
	let limit = (n as f64).sqrt() as i32;
	for a in 2..n {
		if n % a == 0 {
			return false;
		}
	}
	return true;
}