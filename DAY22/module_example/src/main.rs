mod math;

// mod math {
//     pub mod upgrade_math {
//         pub fn mul(a: i32, b: i32) -> i32 {
//             a * b
//         }

//         pub fn div(a: i32, b: i32) -> i32 {
//             a / b
//         }
//     }

//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }

//     pub fn sub(a: i32, b: i32) -> i32 {
//         a - b
//     }

//     fn greeting() {
//         println!("Hello, world!");
//     }
// }

fn main() {
    let _result1 = math::add(3, 6);
    let _result2 = math::sub(5, 1);
    let _result3 = math::upgrade_math::mul(11, 13);
    let _result4 = math::upgrade_math::div(225, 15);

    // math::greeting()     // error!
}
