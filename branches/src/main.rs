fn main() {
    let x = 6;
    if x > 7 {
        println!("x > 7");
    }else if x % 4 ==0{
        println!("x is divisible by 4");
    }else{
        println!("x is not divisible by 4");
    }

    let condition = true;
    let number = if condition {         //if обязательно передавать bool
        5                               //обязательно один тип данных!
    } else {
        6                               //обязательно один тип данных!
    };  
    println!("The value of number is: {}", number);
}
