use std::error::Error;

use reqwest;

 fn main() -> Result<(), Box<dyn Error>> {

    // 일반적인 API 요청

    let url = "http://www.rustinaction.com/";

    let mut res = reqwest::get(url)?;

    let content = res.text()?;

    // let content = res.json()?;
    // json은 타입이 필요하기 떄문에 일단 불가능

    println!("{:?}", content);

Ok(())

}
