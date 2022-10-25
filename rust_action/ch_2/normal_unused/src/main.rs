#[allow(unused_variables)]
// 실행하는 동안 사용하지 않는 변수를 허용한다고 암시한다


type File = String;

fn open(f : &mut File) -> bool {
    true
}

fn close(f : &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(_f : &mut File, _save_to : &mut Vec<u8>) -> ! {
    unimplemented!()
}
// ! 반환 타입은 rust에게 절대적으로 아무런 값도 반환하지 않을 것이라는 것을 암시한다.

fn main() {
    let mut f1 = File::from("f1.txtx");
    // File은 String 타입이기 떄문에 String::from과 동일하다.

    open(&mut f1);
    

    // 에러 코드를 보면 신기하게도 read라는 함수는 사용하지 않았다는 경고창을 띄우지 않는다.
}
