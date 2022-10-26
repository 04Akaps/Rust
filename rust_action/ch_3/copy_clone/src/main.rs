#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id : u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage { 
    Ok,
}

fn check_status(sat_id : &CubeSat) -> StatusMessage {
    println!("{:?}", sat_id.id);
    StatusMessage::Ok
}

fn main() {

    let mut sat_a = CubeSat{id : 0};

    let a_status = check_status(&sat_a.clone());
    println!("a : {:?}", a_status);


    sat_a.id = 3;

    let a_status = check_status(&sat_a.clone());
    println!("a : {:?}", a_status);

    // 보이는 바와 같이 clone은 완전히 다른 메모리로 사용이 됩니다.
    // 일반적으로 깊은 복사라고도 하죠


}
