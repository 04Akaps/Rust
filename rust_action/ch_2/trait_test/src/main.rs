#[derive(Debug)]
struct File;

trait Read {
    fn read(
        self : &Self,
        save_to : &mut Vec<u8>,
    ) -> Result<usize ,String>;
}
// trait는 일반적으로 알고 있는 interface처럼 동작한다.

impl Read for File {
    // Read를 사용했기 떄문에 반드시 read라는 메서드를 만들어 주어야 한다.
    fn read(self: &File, save_to : &mut Vec<u8>) -> Result<usize ,String>{
        Ok(0)
    }
}

fn main() {
    let f = File{};

    let mut buffer = vec![];

    let n_bytes = f.read(&mut buffer).unwrap();

    println!("{} btes(s) read from {:?}", n_bytes, f);
}
