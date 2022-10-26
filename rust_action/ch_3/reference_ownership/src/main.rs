
#![allow(unused_variables)]

// 사실 어떻게 동작하는지가 중요한 것이 아닙니다.
// 어차피 이러한 코드들은 작성을 하면 늘어나느 것이기 떄문에 중요한 것은 패턴 입니다.
// 전역 변수 같이 오래 지속되는 객체는 더 분리되고 일시적인 객체로 만드는 것이 좋다는 패턴 방식이라고 합니다.

// 어떻게 막 복잡하게 꼬여 있는데 일단 그렇다고 합니다..

#[derive(Debug)]
struct CubeSat {
    id : u64,
}

#[derive(Debug)]
struct Mailbox{
    messages : Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to : u64,
    content : String,
}

struct GroundStation {}

impl Mailbox {
    fn post(&mut self, msg : Message){
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) ->Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                // 배열의 값을 삭제한다.
                // let x = [1,2,3].remove(1) 
                // x = 2   && [1,2,3] => [1,3]
                return Some(msg);
            }
        }

        None
    }
}

impl GroundStation{
    fn connect(&self, sat_id : u64) -> CubeSat {
        CubeSat {
            id : sat_id,
        }
    }

    fn send(&self, mailbox : &mut Mailbox, msg : Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox : &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

fn main() {
    let mut mail = Mailbox { messages : vec![]};

    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to : sat_id, content : String::from("hello")};

        base.send(&mut mail,msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);

        println!("{:?} : {:?}", sat, msg);
    }
}


