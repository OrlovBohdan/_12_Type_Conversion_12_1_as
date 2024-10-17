#[test]

/*
fn main() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), __)
    }

    println!("Success!");
}
*/

fn main() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64; 13] = &arr; // Вказівник на масив з 13 елементів
    let b = a as *const [u8; 13 * 8]; // Перетворення вказівника на масив байтів
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 8 * 13); // Перевірка розміру масиву в байтах
    }

    println!("Success!");
}



/*
Щоб заповнити пропуск у коді, потрібно врахувати, що масив arr має тип [u64; 13],
а u64 займає 8 байт. Таким чином, загальний розмір масиву дорівнює 8 × 13 = 104 8×13=104 байти.
*/