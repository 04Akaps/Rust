// 여기에서는 impl을 활용하여 struct 메서드를 관리하고,
// 이후 에러 처리 까지 하는 모든 작업을 진행해볼 예정이다.
#[allow(unused_variables)]

use rand::prelude::*;

fn one_in(denominator : u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
    // thread_rng는 일반적인 난수 생성기 있다.
    // 이후 gen_ratio을 통해서 (m/m) 을 계산하여 n/m의 확률을 가지는 불값을 반환한다.
}

#[derive(Debug)]
struct File {
    name : String,
    data : Vec<u8>,
}

impl File {
    fn new(name_ : &str) -> Self {
        Self {
            name : String::from(name_),
            data : Vec::new(),
        }
    }

    fn new_with_data(
        name : &str,
        data : &Vec<u8>
    ) -> Self {
        // f라는 값을 새로 만들고 이후 Vec를 바로 할당하는 함수
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read (
        self : &File, save_to : &mut Vec<u8>
    ) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        // struct의 Vec를 clone
        let read_length = tmp.len();
        // 단순히 Vec의 길이
        
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        // Go라는 언어랑 되게 유사한것 같다.
        // Go에서도 capacity가 정해져 있고 이 용량을 넘어서면 새로운 메모리를 할당하면서 동작하는데
        // rust에서 이렇게 capacity를 수정 가능하다.
        
        // save_to 에게 append를 토해서 struct에 있는 vec값을 모두 할당해 준다.
        // 이렇게 한다고 struct에 있는 Vec값은 바뀌지 않는다.
        // 왜냐하면 어차피 레퍼런스 타입으로 인자를 받았고 내부엥서 clone을 땄기 떄문데

        Ok(read_length)
    }
}

// 이쪽 부분에 impl을 통해서 관리가 가능하다.

fn open (f :  File) -> Result<File, String> {
    if one_in(10_00) {
        let err_msg = String::from(" 이걸 걸리네...");
        return Err(err_msg);
    }

    Ok(f)
}

fn close(f :  File) -> Result<File, String>{
    if one_in(10_000) {
        let err_msg = String::from(" 이걸 걸리네...");
        return Err(err_msg);
    }

    Ok(f)
}

static mut ERROR : isize = 0;
// 정적 가변 변수의 값을 바꾸려면 unsafe라는 스코프로 감싸주어야 한다,
// 이를 통해서 에러처리도 가능한데 나중에 알아보도록 하겠다.

fn main() {
    let f3_data : Vec<u8> = vec![240, 159, 146, 150];

    let mut f3 = File::new_with_data("2.txt", &f3_data);

    let mut buffer : Vec<u8> = vec![];

    f3 = open(f3).unwrap();

    let f3_length = f3.read(&mut buffer).unwrap();

    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", text);

    // 아까 어떤 부분에서 에러가 발생하였는지 기억이 안난다..

    // let f3_length = f3.read( &mut buffer);
    // // read를 통해서 File 구조체에 있는 값을 buffer에 넣어준 모습이다.

    // println!("{:?}", buffer);

    // let text = String::from_utf8_lossy(&buffer);
    // // bytes를 string으로 바꾸는 메서드
    // println!("{:?}", text);
}
