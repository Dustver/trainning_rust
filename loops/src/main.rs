fn main() {
    loop {              //бесконечный цикл. Остановка только через break.
        println!("again!");
        break;
    }

    let mut num = 5;
    while num != 0 {
        println!("number in while = {}", num);
        num = num -1;
    }

    let a = [10, 20, 30, 40, 50];
    /*let mut index = 0;
    while index < 5 {
        println!("iteration of array: {}", a[index]);
        index = index + 1;
    }------------------------небезопасный код. возможны ошибки с выходом за пределы массива*/
    for element in a.iter() {
        println!("ieration of array: {}", element); // более безопасный перебор массива.
    }

    // Обратный отсчёт с помощью for
    for number in (1..5).rev() {
        println!("  {} !", number);
    }

    
}
