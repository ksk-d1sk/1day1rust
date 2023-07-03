fn main() {
    let mut v = [2, 1, 5, 7, 9, 10, 4, 6, 3, 8];
    println!("정렬 전: {:?}", v);
    quicksort(&mut v);
    println!("정렬 후: {:?}", v);

    let hi: u8 = 0xe0;
    assert_eq!(!hi, 0x1f);

    let x = 17_i32;
    let _index = x as usize;

    assert_eq!(-1.99_f64 as i32, -1);
    assert_eq!(1e6 as u8, 255);
    assert_eq!([65 as char, 66 as char, 67 as char], ['A', 'B', 'C']);

    let is_even = |x: u64| -> bool { x & 1 == 0 };
    assert!(is_even(14));
    assert!(!is_even(21));
}

// fn while_expression() -> i32 {      // Error!
//     while true {
//         return 1;
//     }
// }

// fn loop_expression() -> i32 {       // Ok
//     loop {
//         break 1;
//     }
// }

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() > 1 {
        let pivot_index = partition(slice);
        quicksort(&mut slice[..pivot_index]);
        quicksort(&mut slice[(pivot_index + 1)..]);
    }
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot_index = slice.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);
    i
}
