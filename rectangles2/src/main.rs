#[derive(Debug)] // необходимое аннотирование для форматированного вывода структуры через {:?}
struct Rectangle {
    length: u32,
    width: u32,
}

// определяем метод в контексте структуры
impl Rectangle {
// первый параметр обязательно &self, указывающий на экземпляр
// Если нам понадобиться изменять значения экземпляра структуры,
// мы должны вызвать &mut self. Очень редко может понадобиться получить владение
// self, т.к. это может лишь понадобиться для трансформации экземпляра во что-то другое.
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
// Ассоциированные *функции* не требуют self и находятся внутри структуры
// Они используются например для создания нового экземпляра через ::
    fn square(size: u32) -> Rectangle {
        Rectangle{length: size, width: size}
    }
}

fn main() {
    let rect1 = Rectangle {length: 44, width: 45};
    let rect2 = Rectangle {length: 40, width: 34};
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1 ? {}", rect2.can_hold(&rect1));
    let rect3 = Rectangle::square(4);
    println!("Area of rect3 is {}", rect3.area());
}
