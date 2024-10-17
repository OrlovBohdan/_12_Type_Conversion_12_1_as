#[test]

/*
fn main() {
    assert_eq!(1000 as u16, __);

    assert_eq!(1000 as u8, __);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, __);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.
    assert_eq!(300.1_f32 as u8, __);
    assert_eq!(-100.1_f32 as u8, __);


    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
*/

fn main() {
    assert_eq!(1000 as u16, 1000); // 1000 входить у діапазон `u16`, тому результат 1000

    // Використовуємо `1000_u16`, щоб уникнути переповнення літерала
    assert_eq!(1000_u16 as u8, 232); // 1000 % 256 = 232 (модульне приведення)

    // Для додатних чисел це буде те саме, що й модуль
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255); // -1 в `i8` дорівнює 255 в `u8`

    // Починаючи з Rust 1.45, `as` виконує *насичене приведення* з float до int
    assert_eq!(300.1_f32 as u8, 255); // 300.1 перевищує діапазон `u8`, тому буде 255
    assert_eq!(-100.1_f32 as u8, 0);  // -100.1 менше нуля, тому буде 0

    unsafe {
        // Для числа 300.0 отримаємо 44 (300 % 256)
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // Для -100.0 отримаємо 156 (-100 % 256)
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // Значення NaN отримає значення 0 при unsafe-методі
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
/*
1000 as u8: Результат 1000 модулю 256 дорівнює 232.
-1_i8 as u8: У беззнаковому представленні -1 стає 255.
300.1_f32 as u8: Значення перевищує максимальне значення для u8, тому відбувається насичене приведення до 255.
-100.1_f32 as u8: Значення менше мінімуму для u8, тому результатом буде 0.

Rust розглядає 1000 як літерал типу u8, і це викликає переповнення, оскільки u8 має максимальне значення 255.
Щоб вирішити цю проблему, можна явно привести 1000 до типу u16 перед приведенням до u8.
Це дозволить уникнути переповнення під час компіляції.
*/