fn main() {
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!("rect1 is {:#?}", rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn areaTuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn areaStruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
