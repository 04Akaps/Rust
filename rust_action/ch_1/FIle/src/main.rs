use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    // 단순하게 file open 코드.
    // 파일이 없다면 panic 발생

    println!("실행??");
    // panic이 났다는 것을 확인 하는 용

    let mut reader = BufReader::new(f);

    let mut line = String::new();

    // loop {
    //     let len = reader.read_line(&mut line).unwrap();

    //     if len == 0{
    //         break;
    //     }

    //     println!("{}, ({} bytes long)", line, len);

    //     line.truncate(0);
    // }

    for line_ in reader.lines() {
        let line = line_.unwrap();

        println!("{line}");
    }

}
