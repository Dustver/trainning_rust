extern crate rand;

use std::io;
use std::cmp::Ordering; //Ordering нужен для функции сравнения Less.Greater.Equal.
use rand::Rng;      //Random Number Generator. Нужно объявить в глобальной области видимости.

fn main() {
    println!("Guess the number!\n\nPlease, input your guess: ");
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is: {}", secret_number);

    loop {
        
        let mut guess = String::new();   // mut == mutable variable String::new() - новый объект строкового типа
        
        io::stdin().read_line(&mut guess)//строки можно переносить без проблем в компиляции
            .expect("Failed to read line!"); //при ошибке выведет это.
        
        let guess: u32 = match guess.trim().parse() { // trim - очищает от пробелов. parse - читает содержимое переменной
            Ok(num) => num,     // при успехе
            Err(_)  => continue,// при ошибке продолжить. (_) - любой ошибке
        }; //если убрать match(как switch) и поставить метод expect(), то будет break при ошибке.
                    
       // println!("You guesed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less      =>  println!("Too small!"),
            Ordering::Greater   =>  println!("Too big!"),
            Ordering::Equal     =>  {
                println!("You win!");
                break;   // при "попадании" выйдти из программы.
            }
        }
    }
}