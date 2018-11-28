fn main() {
    // `{}` автоматически будет заменено на
    // аргументы. Они будут преобразованы в строку.
    println!("{} дней", 31);

    // Без суффиксов, 31 является i32. Можно изменить тип 31u32,
    // используя суффикс.

    // Существует множество способов работы с форматированным выводом. Можно указать
    // позицию для каждого аргумента.
    println!("{0}, это {1}. {1}, это {0}", "Алиса", "Боб");

    // Так же можно именовать аргументы.
    println!("{subject} {verb} {object}",
             object="ленивую собаку",
             subject="быстрая коричневая лиса",
             verb="прыгает через");
    /* {:b} - форматирование в бинарное число */
    println!("{} из {:b} людей знают, что такое двоичный код, а остальные нет.", 1, 2);

    // Можно выравнивать текст, сдвигая его на указанную ширину.
    // Данный макрос отобразит в консоли
    // "     1". 5 пробелов и "1".
    println!("{number:>width$}", number=1, width=6);

    // Можно добавить к цифрам пару нулей. Данный макрос выведет "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Компилятор обязательно проверит, что в макрос передано правильное количество
    // аргументов.
    println!("Меня зовут {0}, {1} {0}", "Бонд","James");
    // ИСПРАВЬТЕ ^ Добавьте недостающий аргумент: "Джеймс"

    // Создаём структуру, которая хранит в себе `i32`. Назовём её `Structure`.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // Однако, пользовательские типы данных, например, как эта структура
    // требуют более сложной обработки для вывода. #[derive(Debug)] перед объявлением структуры и {:?}
    println!("Эта структура `{:?}` не хочет выводится на экран... обычным образом", Structure(3));
    println!("Эта структура `{:#?}` не хочет выводится на экран... обычным образом", Structure(3));

    let pi: f64 = 3.141_592_653_589_793_238_462;
    println!("{:.*}",2, pi);
    println!("{:.4}", pi);



    // Булева логика
    println!("true И false будет {}", true && false);
    println!("true ИЛИ false будет {}", true || false);
    println!("НЕ true будет {}", !true);

    // Побитовые операции
    println!("0011 И 0101 будет {:04b}", 0b0011u32 & 0b0101);
    println!("0011 ИЛИ 0101 будет {:04b}", 0b0011u32 | 0b0101);
    println!("0011 исключающее ИЛИ 0101 будет {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 будет {}", 1u32 << 5);
    println!("0x80 >> 2 будет 0x{:x}", 0x80u32 >> 2);
}