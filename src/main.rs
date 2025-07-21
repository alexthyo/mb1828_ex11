// Exercise 0: Reimplementation of the ruler function from Exercise Sheet 1

fn ruler(n: i32, verbose: bool) -> i32 {
	if n <= 0 {
		return 0;
	}
	let mut m = 2 * n;
	let mut k = 0;
	while m % 2 == 0 {
		if verbose {
			print!("{} ", m);
		}
		m = m / 2;
		k += 1;
	}
	k
}

fn main() {
	ruler(1 << 28, true);
	println!();
}

// Exercise 1: Testing
#[test]
fn test_ruler_function() {
	assert_eq!(ruler(0, false), 0); // Edge case: n = 0
	assert_eq!(ruler(1, false), 1); // 2 * 1 = 2, k = 1
	assert_eq!(ruler(999999, false), 1); // 2 * 999999 is not odd, k = 1
	assert_eq!(ruler(10, false), 2); // 2 * 10 = 20, k = 2
}