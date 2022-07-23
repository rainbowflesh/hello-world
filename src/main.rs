fn main() {
    let _penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";

    let records = _penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let _fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            unsafe {
                eprintln!("debug: {:?} -> {:?}", record, _fields);
            }
        }
        let name = _fields[0];
        // TODO: 转换 fields[1] 的值为 f32 类型, 成功时复制给 length
        if let Ok(length) = _fields[1].parse::<f32>() {
            unsafe {
                println!("out: {}, {}cm", name, length);
            }
        }
    }
}
