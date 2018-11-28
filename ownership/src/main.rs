fn main() {
    let s = "hello";
    let mut mss = String::from("Thunder");
    println!("{}", s);
    println!("{}", mss);
    mss.push_str(" in paradice!");
    println!("{}", mss);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
// Список типов, которые имею типаж Copy:
//  Все целочисленные типы, такие как u32.
//  Логический тип данных bool, значения которых true и false.
//  Все числа с плавающей запятой такие как f64.
//  Кортежи, но только если они содержат типы, которые также Copy. (i32, i32) Copy, но (i32, String) нет.


// тип String содержит типаж drop. drop и copy взаимоисключающие
    let s1 = String::from("Hello, World!"); // s1 - указатель на адрес в heap. Сама строка "Hello" располагается в куче. 
    // let s2 = s1; // s1 более не действительна, т.к. она перемещена в переменную s2.

    let s2 = s1.clone(); // full copy method
    println!("s1 = {}, s2 = {}", s1, s2);

    let (s2, len) = calc_len(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


// у одной переменной может быть только одна изменяемая ссылочная переменная
// в данной области видимости
    let mut mutable_string = String::from("Heil The Braves!");
    change(&mut mutable_string);
    println!("{}", mutable_string);

}

fn calc_len(s: String) -> (String, usize) { // передача владения в функцию и обратно через кортеж
    let length = s.len();
    (s, length)
}

// переменные ссылочного типа не имеют владения и не могут влиять на указываемые значения
fn calculate_length(s: &String) -> usize { // передача в функцию ссылки на значение из кучи
    s.len()
}

fn change(st: &mut String) {
    st.push_str("\n Heil The Courage!");
}