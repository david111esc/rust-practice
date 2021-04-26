pub trait Animal {
    fn bark(&self);
    fn get_name(&self) -> String;
    fn count_legs(&self) -> u32;
}

pub struct Dog {
    pub name: String,
}
impl Animal for Dog {
    fn bark(&self) {
        println!("woof");
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn count_legs(&self) -> u32 {
        4
    }
}
pub struct Cat {
    pub name: String,
}
impl Animal for Cat {
    fn bark(&self) {
        for i in 0..10 {
            println!("{} Meow Meow Meow!", i);
        }
    }
    fn get_name(&self) -> String {
        format!("{} ^_^!!", self.name.clone())
    }
    fn count_legs(&self) -> u32 {
        4
    }
}
pub struct Ducks {
    pub name: String,
    pub numbers: u32,
}
impl Animal for Ducks {
    fn bark(&self) {
        for i in 0..=(self.numbers * 2) {
            println!("{} quack!", i);
        }
    }
    fn get_name(&self) -> String {
        format!("{:>6}", self.name.clone())
    }
    fn count_legs(&self) -> u32 {
        2
    }
}
pub struct Toy {
    pub name: String,
    pub legs: u32,
    pub sound: String,
}
impl Animal for Toy {
    fn bark(&self) {
        for i in 0..=3 {
            println!("{} {}!", i, self.sound);
        }
    }
    fn get_name(&self) -> String {
        if self.legs > 2 {
            format!("Toy:{:>6}, legs:{} ", self.name.clone(), self.legs)
        } else {
            format!("Toy:{:>6}", self.name.clone())
        }
    }
    fn count_legs(&self) -> u32 {
        self.legs
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
