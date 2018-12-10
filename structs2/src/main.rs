#[derive(Debug)]
struct Person <'a>{
    name: &'a str,
    age: u8
}

// Единичная структура
struct Nil;

// Кортежная структура
struct Pair(i32, f64);

// Структура с двумя полями
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Структуры могут быть использованы как поля другой структуры
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
    }
}

impl Point {
    fn square(&self, side: f32) -> Rectangle {
        let side = side as f64;
        let p2x = self.x + side;
        let p2y = self.y + side;
        let Point{x: p1x, y: p1y} = self;
        let square_out: Rectangle  = Rectangle {
            p1: Point {x: *p1x,y: *p1y},   // я хз зачем так делать...
            p2: Point {x: p2x, y: p2y},
        };
        
        square_out
    }
}

fn main() {
    // Создаём структуру с помощью короткой инициализации полей
    let name = "Петя";
    let age = 27;
    let peter = Person { name, age };
    
    // Дебаг вывод структуры
    println!("{:?}", peter);
    
    
    // Создаём структуру `Point`
    let point1: Point = Point { x: 12.0, y: 15.0 };

    // Получаем доступ к полям структуры `Point`
    println!("Координаты точки: ({}, {})", point1.x, point1.y);

    // Деструктурируем `Point` создавая связь с помощью `let`
    let Point { x: my_x, y: my_y } = point1;

    let _rectangle = Rectangle {
        // инициализация структуры так же является выражением
        p1: Point { x: my_y, y: my_x },
        p2: point1,
    };

    // Создаём связь с единичной структурой
    let _nil = Nil;

    // Создаём связь с кортежной структурой
    let pair = Pair(1, 0.1);

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;

    println!("Pair хранит в себе {:?} и {:?}", integer, decimal);

    let rect1 = Rectangle{
        p1: Point{x: 12.0, y: 14.5},
        p2: Point{x: 82.0, y: 43.5},
    };
    println!("rectarea of rect1 is: {}", rect1.area());

    let square_point = Point {x: 4.0, y: 4.0};
    let side: f32 = 45.0;
    let rect2 = square_point.square(side);
    println!("rectangle::square is {:?}", rect2);

}