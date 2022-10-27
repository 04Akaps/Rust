use rand::{self};
use rand::Rng;
use rand::prelude::SliceRandom;

#[derive(Debug)]
struct Dewarf{}

#[derive(Debug)]
struct Elf{}

#[derive(Debug)]
struct Human{}

#[derive(Debug)]
enum Thing{
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug{
    fn competency(&self) -> f64;

    fn enchant(&self, thing : &mut Thing) {
        let probability_of_success = self.competency();

        let spell_is_success = rand::thread_rng()
        .gen_bool(probability_of_success);
        // gen_bool은 되게 특이한 메서드이다.
        // 인자로 들어가는 값에 따라서 bool값이 나온다.
        // 예를들어 0.5가 들어가면 50%의 확률로 true가 나온다.

        print!("{:?} mutters incoherently. ", self);

        if spell_is_success {
            println!("The {:?} glows brightly.", thing);
        } else {
            println!("The {:?} fizzes", thing);

            *thing = Thing::Trinket{};
        }
    }
}

impl Enchanter for Dewarf {
    fn competency(&self) -> f64{
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64{
        0.95
    }
}


impl Enchanter for Human {
    fn competency(&self) -> f64{
        0.8
    }
}


fn main() {

    let mut it = Thing::Sword;

    let d = Dewarf {};
    let e = Elf {};
    let h = Human {};

    let party : Vec<&dyn Enchanter> = vec![&d, &h, &e];
    // trait에 대한 타입을 사용할 떄에는 &dyn을 넣어 주어야 한다.

    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();
    // 해당 값중 랜덤한 값을 가져 온다.

    spellcaster.enchant(&mut it);

}
