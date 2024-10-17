#[test]

/*

// FILL in the blanks
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 __;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address __; // p2 points to the 2nd element in values
    unsafe {
        // Add one to the second element
        __
    }

    assert_eq!(values[1], 3);

    println!("Success!");
}
*/

fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; // Перетворюємо p1 в usize
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 вказує на другий елемент в values
    unsafe {
        // Додаємо одиницю до другого елемента
        *p2 += 1;
    }

    assert_eq!(values[1], 3); // Перевіряємо, що другий елемент тепер 3

    println!("Success!");
}

/*
first_address: Тут потрібно перетворити вказівник p1 у usize, щоб працювати з адресами безпосередньо.
second_address: Перетворюємо second_address назад у вказівник типу *mut i32, щоб він вказував на другий елемент масиву.
*p2 += 1;: Всередині небезпечного блоку збільшуємо значення другого елемента масиву через вказівник p2.
*/