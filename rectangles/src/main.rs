// fn main() {
//     let rect1 = (50, 30);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// } передача кортежем, но элементы кортежа безымянны

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }
// fn main() {
//     let length1 = 40;
//     let width1 = 30;
//     println!("Area of rectangle is {} square pixels", area(length1, width1));
// } классический функциональный подход

#[derive(Debug)] // необходимое аннотирование для форматированного вывода структуры через {:?}
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.length
// }

fn main() {
    // let rect1 = &(Rectangle { length: 50, width: 30 });
    // println!("Area {}", area(rect1));
    let rect1 = Rectangle {length: 30, width: 50};
    println!("Rectangle #1 is {:?}", rect1);
    println!("Rectangle #1 in better view is {:#?}", rect1);
    println!("Area of rectangle is {} square pixels", rect1.area());
}