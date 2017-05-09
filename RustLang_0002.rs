fn main()
{
	let mut x = vec!["Hello", "World"];
	let y = x[0].clone();
	x.push("foo");

	{
		let y1 = &x[0];
	}
	x.push("foo");
}
