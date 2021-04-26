extern crate traits;
use traits::*;
// pub trait Animal {
//     fn bark(&self);
//     fn get_name(&self) -> String;
//     fn count_legs(&self) -> u32;
// }
fn party_in_the_zoo(a1: &dyn Animal, a2: &dyn Animal, a3: &dyn Animal, a4: &dyn Animal) {
    a1.bark();
    println!(
        "a2 name is {} and it has {} legs",
        a2.get_name(),
        a2.count_legs()
    );
    if a3.count_legs() >= 3 {
        println!(
            "a3 and a4 totoal has {} legs",
            a3.count_legs() + a4.count_legs()
        );
    } else if a3.count_legs() < 3 && a4.count_legs() > a3.count_legs() {
        println!(
            "a4 has {} legs and a3 has {} legs",
            a4.count_legs(),
            a3.count_legs()
        );
    } else {
        println!("a3's legs are less than 3, and a4's legs are less than a3's legs!");
        a4.bark();
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Lucky"),
    };
    let cat = Cat {
        name: String::from("Kitty"),
    };
    let duck = Ducks {
        name: String::from("Gua"),
        numbers: 3,
    };
    let toy_1 = Toy {
        name: String::from("Spider"),
        legs: 8,
        sound: String::from("Zzzz"),
    };
    let toy_2 = Toy {
        name: String::from("Fish"),
        legs: 0,
        sound: String::from(".ooO0"),
    };
    //
    party_in_the_zoo(&dog, &dog, &dog, &dog);
    party_in_the_zoo(&cat, &cat, &cat, &cat);
    party_in_the_zoo(&duck, &duck, &duck, &duck);
    party_in_the_zoo(&toy_1, &toy_1, &toy_1, &toy_1);
    party_in_the_zoo(&toy_2, &toy_2, &toy_2, &toy_2);
    //
    party_in_the_zoo(&dog, &cat, &duck, &toy_1);
    party_in_the_zoo(&toy_2, &dog, &cat, &duck);
    party_in_the_zoo(&toy_1, &toy_2, &dog, &cat);
    party_in_the_zoo(&duck, &toy_1, &toy_2, &dog);
    party_in_the_zoo(&cat, &duck, &toy_1, &toy_2);
    //
    party_in_the_zoo(&dog, &toy_2, &cat, &toy_1);
    party_in_the_zoo(&toy_2, &cat, &toy_1, &duck);
    party_in_the_zoo(&cat, &toy_1, &duck, &dog);
    party_in_the_zoo(&toy_1, &duck, &dog, &toy_2);
    party_in_the_zoo(&duck, &dog, &toy_2, &cat);
    //
    party_in_the_zoo(&toy_1, &cat, &duck, &toy_2);
}
