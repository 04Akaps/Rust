#[derive(Debug)]
struct CubeSat{
    id: u64,
}

#[derive(Debug)]
enum StatusMessage { 
    Ok,
}

fn check_status(sat_id : CubeSat) -> CubeSat {
    // StatusMessage::Ok
    sat_id
}


fn main() {
    let sat_a = CubeSat{ id : 8};
    let sat_b = CubeSat{id : 5};
    let sat_c = CubeSat{id : 10};

    println!("{:#?}", sat_a);
    println!("{:#?}", sat_b);
    println!("{:#?}", sat_c);

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    println!("{:#?}", sat_a);
    println!("{:#?}", sat_b);
    println!("{:#?}", sat_c);
}
