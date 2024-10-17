#[test]

/*
// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal = 97.123_f32;

    let integer: __ = decimal as u8;

    let c1: char = decimal as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8);

    println!("Success!");
}
*/

fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let _c1: char = decimal as u8 as char;
    let _c2 = integer as char;

    assert_eq!(integer, 'a' as u8);

    println!("Success!");
}




/*
Тип змінної integer: Заповнив пропуск, вказавши тип u8 (тип беззнакового цілого числа).
Конверсія decimal до char: Додана конверсія decimal як u8 перед перетворенням в char.
Умову перевірки: Змінив 'b' на 'a', оскільки 97 відповідає символу 'a' у таблиці ASCII.
*/