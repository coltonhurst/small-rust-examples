/*
	Explanation:

	In this example we create a struct and add methods
	to that struct. The methods have access to the data
	in the struct *and* the other methods via the keyword
	`self`. You use an `impl` block to define methods
	for a struct.

	We make a Rectangle, defining the 4 coordinates.
	Then we print the area of the Rectangle.
	(Area should be 6)

	Note- I removed bottom_right and the new()
	for Rectangle as they are unused. Trying
	to avoid any warnings.

	This is a fairly unrefined example, and you shouldn't
	necessarily do things this way. This was just for
	educational purposes...
*/

fn main() {
	println!("Let's make a rectangle!");

	let rect = Rectangle {
		top_left: Coordinate::new(1.0, 3.0),
		top_right: Coordinate::new(4.0, 3.0),
		bottom_left: Coordinate::new(1.0, 1.0),
		//bottom_right: Coordinate::new(4.0, 1.0),
	};

	println!("The rectangle has an area of {}", rect.area());
}

struct Coordinate {
	x: f64,
	y: f64,
}

impl Coordinate {
	fn new(x: f64, y: f64) -> Coordinate {
		Coordinate { x: x, y: y }
	}
}

struct Rectangle {
	top_left: Coordinate,
	top_right: Coordinate,
	bottom_left: Coordinate,
	//bottom_right: Coordinate,
}

impl Rectangle {
	fn area(&self) -> f64 {
		// `self` gives access to struct fields via the dot operator
		// Here we deconstruct p1 into x1 and y1, etc.
		let Coordinate {x: x1, y: y1} = self.top_left;
		let Coordinate {x: x2, ..} = self.top_right;
		let Coordinate {y: y3, ..} = self.bottom_left;

		// `abs` is a `f64` method that returns the absolute value
		// of the caller
		let length = x2 - x1;
		let width = y1 - y3;

		length * width
	}

	/*
	fn new(tl: Coordinate, tr: Coordinate, bl: Coordinate, br: Coordinate) -> Rectangle {
		Rectangle { top_left: tl, top_right: tr, bottom_left: bl, bottom_right: br }
	}
	*/
}