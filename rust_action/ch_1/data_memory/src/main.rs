use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // 스택
    let b = Box::new(20); // 힙
    let c = Rc::new(Box::new(30)); // 참조 카운터
    let d = Arc::new(Mutex::new(40));   // 원자적 참조 카운터

    println!("a : {:?}, b: {:?}. c : {:?}, d : {:?}",a,b,c,d);

    // rust는 다양한 데이터 저장 공간을 지원한다.
}
