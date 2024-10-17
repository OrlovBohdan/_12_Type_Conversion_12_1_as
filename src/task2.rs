#[test]


/*fn main() {
    assert_eq!(u8::MAX, 255);
    // The max of `u8` is 255 as shown above.
    // so the below code will cause an overflow error: literal out of range for `u8`.
    // PLEASE looking for clues within compile errors to FIX it.
    // DON'T modify any code in main.
    let v = 1000 as u8;

    println!("Success!");
}
*/

#[allow(overflowing_literals)]
fn main() {
    assert_eq!(u8::MAX, 255);
    // The max of `u8` is 255 as shown above.
    // so the below code will cause an overflow error: literal out of range for `u8`.
    // PLEASE looking for clues within compile errors to FIX it.
    // DON'T modify any code in main.
    let _v = 1000 as u8;

    println!("Success!");
}

/*
У Rust можна використовувати глобальну анотацію, щоб придушити помилки переповнення під час компіляції.
Анотація #[allow(overflowing_literals)] дозволяє компілятору ігнорувати переповнення для літералів, що виходять за межі діапазону.

Потрібно додати цю анотацію перед функцією main.
У цьому випадку компілятор не буде генерувати помилку переповнення, і значення 1000 as u8 буде перетворено
за допомогою модульного приведення.
*/
