// Рассмотрим учебную задачу. Необходимо написать функцию,
// входным параметром которой является строка. Выходным значением
// функции является первое слово, которое будет найдено в этой строке.
// Если функция не находит разделителя слов (пробела), она возвращает эту строку.
fn first_word_index(st: &String) -> usize {
    let bytes = st.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || 
        item == b',' ||
        item == b'!' ||
        item == b'.'{
            return i;
        }
    }
    st.len()
// если находит пробел - возвращает позицию пробела или других знаков препинания
// в ином случает - длинну массива.
}

fn first_word(st: &String) -> &str { // &str - срез строки(короткая запись)
    let bytes = st.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &st[0..i];
        }
    }
    &st[..]
}

fn f_w(s: &str) -> &str {
// можно переписать ф-ю first_word с входящим типом строкового литерала (&str).
// это немного удобнее для передачи строки в ф-ю
    let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
    &s[..]
}

fn main() {
    let s = String::from("Hello, my dear world!");
    println!("{:?}",s.as_bytes());
    println!("{}", first_word_index(&s));

    let slice = &s[..first_word_index(&s)]; // equal [0..6]
    println!("{}", slice);
// &s[..] целая строка
// &s[3..] с 3 и до конца
    let word = first_word(&s);
    println!("First word is: {}", word);
//  благодаря срезам нельзя изменить содержание строки, если на неё ссылается срез.
//  s.clear() - выдаст ошибку компиляции.
    let s = "Greetings, my brave new world!"; // s - имеет тип &str, соответственно легко передаётся в ф-ю
    let word = f_w(s); // String так просто в ф-ю передать не получиться
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];// Этот срез имеет тип данных &[i32]
    println!("{:?}", slice);
}