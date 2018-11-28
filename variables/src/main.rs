const CONSTANTA: u32 = 300_000;             //this const will print second
/*
* Скалярный тип данных (scalar data type) содержит одно значение
* и не имеет внутренних компонентов. 
* Скалярные типы данных делятся на четыре категории:
    Числовые.
    Символьные.
    Даты.
    Логические данные.
В состав скалярных типов Rust входят:
    целые числа,
    числа с плавающей запятой,
    логические,
    символьные.

Целые числа подразделяются на:
    Length  Signed  Unsigned
    8-bit   i8      u8
    16-bit  i16     u16
    32-bit  i32     u32
    64-bit  i64     u64
    arch    isize   usize

    Type    Signed min              Signed max
    i8      -128                    127
    i16     -32768                  32767
    i32     -2147483648             2147483647
    i64     -9223372036854775808    9223372036854775807
    isize   -9223372036854775808    9223372036854775807

    Type    Unsigned min            Unsigned max
    u8      0                       255
    u16     0                       65535
    u32     0                       4294967295
    u64     0                       18446744073709551615
    usize   0                       18446744073709551615

    Системы счисления:
    Number literals:    Example:
    Decimal             98_222
    Hex                 0xff
    Octal               0o77
    Binary              0b1111_0000
    Byte (u8 only)      b'A'


Сложные типы данных это Кортежи и Массивы.
*/


fn main() {
    print_const_inside_func();
    print();
    redefine_type_of_variables();
    all_might_values();
    different_base_types();
    std_types_limit_values();
    tuples_type();
    array_type();
}

fn print_const_inside_func() {
    const CONSTANTA: u32 = 500_000;         //first print this const ↓
    let mut x = 5;                          //                       ↓
    println!("variable x: {}", x);          //                       ↓
    x = 6;                                  //                       ↓
    println!("variable x: {}", x);          //                       ↓
    println!("constanta = {}", CONSTANTA);  //                       ←
    print();
}

fn print() {
    println!("constanta = {}", CONSTANTA);
}
//Константы доступны в своей области видимости. 
//Также они могут скрываться одноименными константами во вложенной области видимости.
//Константы доступны в любом месте области видимости.

fn redefine_type_of_variables() {
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let x = "_";
    println!("The value of x is: {}", x);
    let x = x.len();
    println!("The value of x is: {}", x);
    let x = "Привет!";
    println!("The value of x is: {}", x);
    let x: u32 = x.len() as u32;
    println!("The value of x is: {}", x);
    let x = "Привет! - кириллические символы в 2 байта, а ascii в 1 байт.";
    println!("The value of x is: {}", x);
    let x: f32 = 3.1415;
    println!("The value of x is: {}", x);
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    Приведёт к ошибке mismatched types !
    */
}

fn all_might_values() {
    //  Вывод всех возможных значений через цикл
    let max = <u8>::max_value();
    let mut value = <u8>::min_value();
    loop {
        println!("value = {}", value);
        value = value + 1;

        if value == max {
            println!("value = {}", value);
            break;
        }
    }
}

fn different_base_types() {
    let x = b'a';
    println!("value = {}", x);

    let mut value = 0xff;
    println!("value = {}", value);
    value = 0x_ff;
    println!("value = {}", value);

    let mut value = 0o77;
    println!("value = {}", value);
    value = 0o_77;
    println!("value = {}", value);

    let mut value = 0b1111_0000;
    println!("value = {}", value);
    value = 0b1_111_0000;
    println!("value = {}", value);
}

fn std_types_limit_values() {
    //  стандартная библиотека предлагает вывод предельных значений типов данных:
    let value32_min = std::f32::MIN;
    println!("value f32 min = {}", value32_min);
    let value32_max = std::f32::MAX;
    println!("value f32 max = {}", value32_max);

    let value64_min = std::f64::MIN;
    println!("value f64 min = {}", value64_min);
    let value64_max = std::f64::MAX;
    println!("value f64 max = {}", value64_max);

    println!("std::f32");
    // Approximate number of significant digits in base 10.
    println!("DIGITS = {}", std::f32::DIGITS);

    // Difference between 1.0 and the next largest representable number.
    println!("EPSILON = {}", std::f32::EPSILON);

    // Infinity (∞).
    println!("INFINITY = {}", std::f32::INFINITY);

    // Number of significant digits in base 2.
    println!("MANTISSA_DIGITS = {}", std::f32::MANTISSA_DIGITS);

    // Largest finite f32 value.
    println!("MAX = {}", std::f32::MAX);

    // Maximum possible power of 10 exponent.
    println!("MAX_10_EXP = {}", std::f32::MAX_10_EXP);

    // Maximum possible power of 2 exponent.
    println!("MAX_EXP = {}", std::f32::MAX_EXP);

    // Smallest finite f32 value.
    println!("MIN = {}", std::f32::MIN);

    // Minimum possible normal power of 10 exponent.
    println!("MIN_10_EXP = {}", std::f32::MIN_10_EXP);

    // One greater than the minimum possible normal power of 2 exponent.
    println!("MIN_EXP = {}", std::f32::MIN_EXP);

    // Smallest positive normal f32 value.
    println!("MIN_POSITIVE = {}", std::f32::MIN_POSITIVE);

    // Not a Number (NaN).
    println!("NAN = {}", std::f32::NAN);

    // Negative infinity (-∞).
    println!("NEG_INFINITY = {}", std::f32::NEG_INFINITY);

    // The radix or base of the internal representation of f32.
    println!("RADIX = {}", std::f32::RADIX);
}

fn tuples_type() {
    //Кортежи: 
    let tup: (i32, f64, u8) = (512, 2.718261826, 5);
    println!("tuple = {:#?}", tup);

    //распаковка кортежа в переменные, как в python
    let (a, b, c) = tup;
    println!("The value of a = {}, b = {}, c = {}", a,b,c);

    //ещё один способ доступа к элементам кортежа через индекс:
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let v1 = x.0;
    let v2 = x.1;
    let v3 = x.2;
    println!("The value of (x, y, z) is: ({}, {}, {})", v1,v2,v3);
}

fn array_type() {
    //Массивы
    //Только один тип данных и строго заданный размер(при создании)
    let a = [1, 2, 3, 4, 5];
    let b: [u8; 5] = [1, 2, 3, 4, 5];
    println!("a is: {:?}", a);
    println!("b is: {:?}", b);
    //В стандартной библиотеке есть тип данных, аналогичный массиву,
    //но имеющий возможность изменения содержания - это вектор.
    let first = a[0];
    let second = a[1];

    println!("first = {}, second = {}", first, second);
    // При попытке доступа к несуществующему индексу массива - программа аварийно завершиться. 
    // Важной особенностью языка Rust является предотвращения доступа к памяти,
    // если произошла ошибка какого-либо рода.

}