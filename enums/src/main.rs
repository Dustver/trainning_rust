#![allow(unused_variables)]
enum Message {  // может содержать объекты разных типов
    Quit,       // пустой объект
    Move { x: i32, y: i32 },    // анонимная структура
    Write(String),              // Строка
    ChangeColor(i32, i32, i32), // Кортеж
}

impl Message { // Для всех типов данных в enum мы можем определить функцию.
    fn call(&self) {
        // method body would be defined here
    }
}

// Перечисление Option<T> даёт возможность показать, что значение нулевое с помощью одного из своих элементов.
#[derive(Debug)]
enum Option<T> { // можно использовать Some и None без префикса Option::
    Some(T),
    None,
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = Option::None; // здесь тип даже пустого значения задавать ОБЯЗАТЕЛЬНО
}
