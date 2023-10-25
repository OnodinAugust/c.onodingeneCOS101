fn main() {
	let qtoshiba:f64 = 2.0;
	let atoshiba:f64 = 450_000.00;
	let qmac:f64 = 1.0;
	let amac:f64 = 1_500_000.0;
	let qhp:f64 = 3.0;
	let ahp:f64 = 750_000.00;
	let qdell:f64 = 3.0;
	let adell:f64 = 2_850_000.00;
	let qacer:f64 = 1.0;
	let aacer:f64 = 250_000.00;

	//sum
	let s = (qtoshiba * atoshiba) + (qmac * amac) + (qhp * ahp) + (qdell * adell) +(qacer * aacer);
	println!("The Sum is{}", s);
	// average
	let a = s / 5.0;
	println!("The Average is {}", a);
}