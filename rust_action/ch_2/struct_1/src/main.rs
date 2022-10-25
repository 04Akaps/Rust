
struct File {
    name : String,
    data : Vec<u8>,
}

struct Hostname(String);

fn connect(h : Hostname) {
    println!("{:?}", h.0);
}

fn main() {
    let mut f1  = File {
        name : String::from("hojin"),
        data : Vec::new()
    };

    f1.data.push(3);

    let f1_name : &String = &f1.name;
    let f1_data : &usize = &f1.data.len() ;
    // 아마도 uszie가 더 큰 값이기 떄문에 cast가 불가능 한것 같다.

    println!("{} is {} long", f1_name, f1_data);

    let ordinary_string = String::from("localhost");
    let host = Hostname (ordinary_string.clone());


    // connect(ordinary_string);
    // // String 타입이 아니라 Hostname 타입이 필요하기 떄문에 당연히 에러

    connect(host);


}
