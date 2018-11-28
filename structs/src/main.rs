#![allow(unused_variables)]

struct Color(i32, i32, i32); // кортежный тип записи структур
struct Point(i32, i32, i32); // у кортежных структур имена полей не определяются.

// мы предпочли использовать тип String вместо &str. Это было осознанное решение,
// т.к. мы хотели чтобы экземпляры структур владели действительными данными
// во время своего существования в памяти.
struct User {
    username:       String,
    email:          String,
    sign_in_count:  u64,
    active:         bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // если имя переменных в функции и поля структуры повторяются, то можно не писать их имена.
        email, // вместо email: email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // создали экземпляр структуры
    let mut user1 = User {
        username: String::from("Dustver"),
        email: String::from("d@email.com"),
        sign_in_count: 1,
        active: true,
    };

    // изменяем поле в экземпляре
    user1.email = String::from("anotheremail@example.com"); // обязательно создавать экземпляр с mut для редактирования полей
    println!(
        "[{};{};{};{}]",
        user1.username, // доступ к полям как в кортеже
        user1.email,
        user1.active,
        user1.sign_in_count
    );
    
    let mut user2 = build_user(String::from("someone@example.com"),String::from("someusername123"));
    user2.email = String::from("anotheremail@example.com");

    let user3 = User {
        username: String::from("Michael"),
        email: String::from("michael@email.com"),
        active: user1.active,               // поля нового экземпляра заимствуем у уже созданного!
        sign_in_count: user1.sign_in_count, // 
    };

    let user4 = User {
        email: String::from("rudolf@email.com"),
        username: String::from("Rudolf"),
        ..user2     // незаполненные поля нового экземпляра заимствуем у уже созданного.
    };

    println!("[{};{};{};{}]", user2.username,user2.email,user2.active,user2.sign_in_count);
    println!("[{};{};{};{}]", user3.username,user3.email,user3.active,user3.sign_in_count);
    println!("[{};{};{};{}]", user4.username,user4.email,user4.active,user4.sign_in_count);


}
