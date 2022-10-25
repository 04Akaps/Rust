fn main() {

    // 벡터는 일반적으로 동적으로 할당되며 정적으로 할당되는 배열과 다릅니다.
    // 그래서 배열에 비해서 더 느리다는 단점이 있지만 이러한 문제점은 유동적인 코드가 동작할 수 있는 장점으로
    // 일반적인 배열보다 더 많은 곳에서 사용이 됩니다.
    // 일반적인 배열은 [1,2,3] 이런식으로 선언이 되며, 한번 할당되면 안에 있는 값을 바꾸는 행위는 가능하나.
    // 추가적인 값을 넣는 행위는 불가능 합니다.

    let context_lines  = 2;
    let needle: &str = "00";

    let hashstack = "\
    Every 00 face, every shop,
    test100, test2, and
    hojin 00 is best ,
    what is work??,
    it is the same with books.
    what do we seek";

    let mut tags : Vec<usize> = Vec::new();
    // 하나의 벡터를 만들어 준다.

    let mut ctx : Vec<Vec<(usize ,String)>> = Vec::new();
    // 또 하나의 벡터를 만들어 준다.
    // 이 벡터는 벡터 안에 usize, String을 가지고 있는 2차원 벡터이다.

    for (i, line ) in hashstack.lines().enumerate() {
        // lines는 한 줄씩 읽어 오는 역할을 수행한다.
        // enumerate는 일반적인 iter와 같이 동작을 하지만 추가적으로 index를 제공한다.

        if line.contains(needle){
            // 만약 해당 줄에서 "00"을 포함하고 있따면

            tags.push(i);

            let v = Vec::with_capacity(2*context_lines + 1);
            // 빈값으로 용량을 가지고 있는 Vec를 만들어 줍니다.
            // 쉽게 말해 길이는 정해져 잇지만 안에 비여있는 Vec를 의미합니다.

            println!("{:?}", v);

            ctx.push(v);
        }
    }

    println!("{:?}", ctx);
    // 빈 배열로 3개가 담겨있는 2차원 배열이 만들어져 있을 것이다. --> [[], [], []]

    println!("{:?}", tags);
    // 배열안에 index가 들어가 있을 것이다.

    if(tags.is_empty()){
        // 만약 들어가있는 값이 없다면 종료시킨다.
        return;
    }

    for( i,line) in hashstack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            // saturating_sub은 두 값을 뺴는 작업을 의미한다.
            // 기존에 usize는 0보다 작아지면 panic을 유발하는데 panic을 발생시키지 않고 0을 반환하는 메서드이다.
            let upper_bound = tag + context_lines;

            if(i>= lower_bound) && (i<=upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i+1;
            println!("{}: {}", line_num, line);
        }
    }

    
}
