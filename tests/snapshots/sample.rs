struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // width and height of a rectangle can be different
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Tab alignment:
/*
	Indent
	1	2	3	4
1	?
22	?
333	?
4444	?
55555	?
666666	?
7777777	?
88888888	?
*/