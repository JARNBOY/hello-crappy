use std::fmt::format;

fn main() {
    println!("Hello, world!");

    // 1. Immutetable, Mutable
    let mut name = "BOY";
    name = "JB";
    println!("{}", name);

    // 2. Data type
    let x: i32 = 1;
    let y: f64 = 0.5;

    let z: i32 = x + y as i32;

    let msg: String = String::from("Papon Sup.");
    let msg2: String = "Papon Sup.".to_string();
    let msg3: &str = "Papon Sup."; // borrow string
    let msg4: String = format!("Point {}, {}", x, y); // string ที่ยัดค่าไปด้วยแนว format string
    println!("{}", msg4);

    // 3. Control flow : if else ,match เหมือน switch, loop เหมือน while 
    let weather: &str = "rainy";

    if weather == "sunny" {
        println!("Crabby will cross the river by swiming!")
    } else if weather == "rainy" {
        println!("Crabby will build a bridge to stay dry.")
    } else {
        println!("Crabby will wait for better weather.")
    }

    let monster: &str = "dragon";
    match monster {
        "goblin" => println!("he used rusty sword to attack."),
        "troll" => println!("Crappy set trap"),
        "dragon" => println!("Crappy runs for cover!."),
        _ => println!("Crappy is confuse")
    }

    // crabby needs to build a boat by collecting 10 pieces of wood!
    let mut wood: i32 = 0;

    loop {
        wood += 1;

        println!("Crabby gathered {} pieces of wood.", wood);

        if wood == 10 {
            println!("Crabby finished the boat!");
            break;;
        }
    }

    // 4. Functions
    let result: String = crabby_task("gathering coins", 12);
    println!("{}", result);
    // Add more function calls here!
    // Then, print the result of each function call
    let result_2: String = crabby_task("cooking", 30);
    println!("{}", result_2);
    let result_3: String = crabby_task("hunting", 60);
    println!("{}", result_3);

    // 5. Ownership & Borrowing หัวใจของ rust ในการจัดการ memory ในการจัดการ Data
    let mut treasure = String::from("gold coins");
    
    // Multiple friends borrow immutably!
    // code here ...
    let friend1: &String = &treasure;
    let friend2: &String = &treasure;
    println!("Friend 1 sees: {}", friend1);
    println!("Friend 2 sees: {}", friend2);

    // Trusted friend borrows mutably
    // code here ...
    let trusted_friend = &mut treasure;

    trusted_friend.push_str(" and silver coins");
    println!("Trusted friend updates: {}", trusted_friend);

    // 6. Lifetimes ทำให้แน่ใจว่า crabby จะไม่หายไป ที่ ref จะไม่หายไป
    /// โจทย์ แก้โดยใช้ Lifetime generic
    let map1 = "Ancient Map of the sea";
    let map2 = "Map to hidden Gold";
    let chosen_map = longest_map(map1, map2);

    // 7. Structs
    let mut crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
    };

    crabby.take_damage(50);
    println!("{}'s health after taking damage: {}", crabby.name, crabby.health);
    crabby.healing(70);
    println!("{}'s health after healing: {}", crabby.name, crabby.health);

    crabby.take_damage(100);
    crabby.take_damage(20);
    println!("{}'s health after taking damage: {}", crabby.name, crabby.health);
    crabby.healing(60);
    println!("{}'s health after healing: {}", crabby.name, crabby.health);

    // 8. Enums
    let figthing_state = CrabbyState::Figting;
    let collecting_state = CrabbyState::Collecting(50);
    let defending_state = CrabbyState::Defending;

    figthing_state.state_represent();
    collecting_state.state_represent();
    defending_state.state_represent();

    // 9. Traits
    let gold: Inventory<i32> = Inventory { item: 100 };
    gold.display();

    let sword: Inventory<&str> = Inventory { item: "Sword" };
    sword.display();

    // 10. String vs &str
    let map: String = String::from("Old map");
    let borrow_map: &str = map.as_str();
    let mut crabby_new_map: String = borrow_map.to_string();
    crabby_new_map.push_str(" to new map");
    println!("Original map: {}", map);
    println!("Borrowed map: {}", borrow_map);
    println!("Crabby's map: {}", crabby_new_map);


}

// 4. Functions
fn crabby_task(task: &str, time: i32) -> String {
    format!("Crabby has successfully done {} in {} miniute!", task, time)
}

// 6. Lifetimes ทำให้แน่ใจว่า crabby จะไม่หายไป ที่ ref จะไม่หายไป
/// Before ก่อนแก้โดยใช้ Lifetime generic
/* 
fn longest_map(map1: &str, map2: &str) -> &str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}
*/

/// After แก้โดยใช้ Lifetime generic
fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}

// 7. Structs
struct Crabby {
    name: String,
    health: u8,
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
    }
}

// 8. Enums
enum CrabbyState {
    Figting,
    Collecting(u32),
    Defending,
}

impl CrabbyState {
    fn state_represent(&self) {
        match self {
            CrabbyState::Figting => println!("Crabby is in fighting mode!"),
            CrabbyState::Collecting(coins) => println!("Crabby is collecting {} coins.", coins),
            CrabbyState::Defending => println!("Crabby is in defending mode!"),
        }
    }
}

// 9. Traits
struct Inventory<T> {
    item: T,
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T> 
where T: std::fmt::Debug,
{
    fn display(&self) {
        println!("{:?}", self.item);
    }
}