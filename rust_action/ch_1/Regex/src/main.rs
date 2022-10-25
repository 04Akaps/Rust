use regex::Regex;

fn main() {
    /*
        러스트에서는 일반적으로 말하는 모듈, 디펜던시를 크레이트 라고 한다.

        이번 코드는 정규식에 관한 내용으로 regex라는 크레이트를 활용한다.
    */

    let re = Regex::new("test").unwrap();


    let quote = "test 첫번쨰 입니다.
    test 두번쨰 입니다.
    없는데요..?
    없어요!!?
    있습니다. test
    ";

    for line in quote.lines() {
        let check_contain = re.find(line);
      
        match check_contain {
            Some(_) => println!("{:?}", line),
            None => (),
        }
    }
}
