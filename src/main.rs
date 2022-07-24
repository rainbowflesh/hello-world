// tutorial 1
// fn main() {
// let _penguin_data = "\
// common name, length (cm)
// Little penguin, 33
// Yellow-eyed penguin, 65
// Fiordland penguin, 60
// Invalid, data
// ";
// let records = _penguin_data.lines();
// for (i, record) in records.enumerate() {
//     if i == 0 || record.trim().len() == 0 {
//         continue;
//     }
//     let _fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
//     if cfg!(debug_assertions) {
//         unsafe {
//             eprintln!("debug: {:?} -> {:?}", record, _fields);
//         }
//     }
//     let name = _fields[0];
//     // TODO: 转换 fields[1] 的值为 f32 类型, 成功时复制给 length
//     if let Ok(length) = _fields[1].parse::<f32>() {
//         unsafe {
//             println!("release: {}, {}cm", name, length);
//         }
//     }
// }
// }

// tutorial 2, Rust 1.59
// struct Struct {
//     e: i32,
// }
// fn main() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };
//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
//     unsafe {}
// }

// tutorial 3
// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);
//     unsafe {
//         println!("a = {:?}, b = {:?}", a, b);
//     }
//     b = true;
//     assert_eq!(a, b);
// }

// tutorial 4
// fn main() {
//     let x = 1;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         unsafe {
//             println!("inner x = {:?}", x);
//         }
//     }
//     unsafe {
//         println!("outer x = {:?}", x);
//     }
// }

// tutorial 5
// fn main() {
//     let liu_han_huang_dou = '😅';
//     println!(
//         "流汗黄豆占用了 {} 个字节的内存大小",
//         std::mem::size_of_val(&liu_han_huang_dou)
//     );
// }

// tutorial 6
// fn main() {
//     another_function(5, 6.1);
// }

// fn another_function(x: i32, y: f32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// tutorial 7
