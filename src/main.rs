// use core::panic;
// use std::collections::HashMap;
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;
// use std::{cell::RefCell, rc::Rc};
use tokio;

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

//     // 1. Immutetable, Mutable
//     let mut name = "BOY";
//     name = "JB";
//     println!("{}", name);

//     // 2. Data type
//     let x: i32 = 1;
//     let y: f64 = 0.5;

//     let z: i32 = x + y as i32;

//     let msg: String = String::from("Papon Sup.");
//     let msg2: String = "Papon Sup.".to_string();
//     let msg3: &str = "Papon Sup."; // borrow string
//     let msg4: String = format!("Point {}, {}", x, y); // string ที่ยัดค่าไปด้วยแนว format string
//     println!("{}", msg4);

//     // 3. Control flow : if else ,match เหมือน switch, loop เหมือน while 
//     let weather: &str = "rainy";

//     if weather == "sunny" {
//         println!("Crabby will cross the river by swiming!")
//     } else if weather == "rainy" {
//         println!("Crabby will build a bridge to stay dry.")
//     } else {
//         println!("Crabby will wait for better weather.")
//     }

//     let monster: &str = "dragon";
//     match monster {
//         "goblin" => println!("he used rusty sword to attack."),
//         "troll" => println!("Crappy set trap"),
//         "dragon" => println!("Crappy runs for cover!."),
//         _ => println!("Crappy is confuse")
//     }

//     // crabby needs to build a boat by collecting 10 pieces of wood!
//     let mut wood: i32 = 0;

//     loop {
//         wood += 1;

//         println!("Crabby gathered {} pieces of wood.", wood);

//         if wood == 10 {
//             println!("Crabby finished the boat!");
//             break;;
//         }
//     }

//     // 4. Functions
//     let result: String = crabby_task("gathering coins", 12);
//     println!("{}", result);
//     // Add more function calls here!
//     // Then, print the result of each function call
//     let result_2: String = crabby_task("cooking", 30);
//     println!("{}", result_2);
//     let result_3: String = crabby_task("hunting", 60);
//     println!("{}", result_3);

//     // 5. Ownership & Borrowing หัวใจของ rust ในการจัดการ memory ในการจัดการ Data
//     let mut treasure = String::from("gold coins");
    
//     // Multiple friends borrow immutably!
//     // code here ...
//     let friend1: &String = &treasure;
//     let friend2: &String = &treasure;
//     println!("Friend 1 sees: {}", friend1);
//     println!("Friend 2 sees: {}", friend2);

//     // Trusted friend borrows mutably
//     // code here ...
//     let trusted_friend = &mut treasure;

//     trusted_friend.push_str(" and silver coins");
//     println!("Trusted friend updates: {}", trusted_friend);

//     // 6. Lifetimes ทำให้แน่ใจว่า crabby จะไม่หายไป ที่ ref จะไม่หายไป
//     /// โจทย์ แก้โดยใช้ Lifetime generic
//     let map1 = "Ancient Map of the sea";
//     let map2 = "Map to hidden Gold";
//     let chosen_map = longest_map(map1, map2);

//     // 7. Structs
//     let mut crabby = Crabby {
//         name: "Crabby".to_string(),
//         health: 100,
//     };

//     crabby.take_damage(50);
//     println!("{}'s health after taking damage: {}", crabby.name, crabby.health);
//     crabby.healing(70);
//     println!("{}'s health after healing: {}", crabby.name, crabby.health);

//     crabby.take_damage(100);
//     crabby.take_damage(20);
//     println!("{}'s health after taking damage: {}", crabby.name, crabby.health);
//     crabby.healing(60);
//     println!("{}'s health after healing: {}", crabby.name, crabby.health);

//     // 8. Enums
//     let figthing_state = CrabbyState::Figting;
//     let collecting_state = CrabbyState::Collecting(50);
//     let defending_state = CrabbyState::Defending;

//     figthing_state.state_represent();
//     collecting_state.state_represent();
//     defending_state.state_represent();

//     // 9. Traits
//     let gold: Inventory<i32> = Inventory { item: 100 };
//     gold.display();

//     let sword: Inventory<&str> = Inventory { item: "Sword" };
//     sword.display();

//     // 10. String vs &str
//     let map: String = String::from("Old map");
//     let borrow_map: &str = map.as_str();
//     let mut crabby_new_map: String = borrow_map.to_string();
//     crabby_new_map.push_str(" to new map");
//     println!("Original map: {}", map);
//     println!("Borrowed map: {}", borrow_map);
//     println!("Crabby's map: {}", crabby_new_map);

//     // 11. Loop
//     // Loop Control Recap
// //  • loop : Infinite looping
// //  • while : Repeat while a condition is true.
// //  • for : Iterate over collections or ranges effortlessly.
// //  • break : Exit the loop early when the job is done.
// //  • continue : Skip over specific items in the loop.
//     let treasures = ["Gold", "Silver", "Ruby Gem", "Emerald"];
//     let mut energy = 5;

//     for treasure in treasures.iter() {
//         if energy == 0 {
//             println!("Crabby is too tired to continue, Game over.");
//             break;
//         } else if treasure == &"Ruby Gem" {
//             println!("Crabby found a Ruby Gem! It's too heavy to carry, so he leaves it behind, You win!");
//             break;
//         }
//         println!("Crabby found a {}!", treasure);
//         energy -= 1;
//     }

//     // 12. Vector -> like flexible array

//     // case add items ก่อน ทำให้ capacity เพิ่มขึ้น
//     let mut items = vec!["Gold", "Silver", "Bronze", "Ruby Gem"];
//     items.push("Emerald");
//     println!("Crabby's inventory items: {:?}", items);
//     println!("Crabby's inventory items: {:?}", items.len());
//     println!("Crabby's inventory items: {:?}", items.capacity());

//     items.remove(1);
//     println!("Crabby's inventory items: {:?}", items);
//     println!("Crabby's inventory items: {:?}", items.len());
//     println!("Crabby's inventory items: {:?}", items.capacity());

//     // case remove items ก่อน ทำให้ capacity ไม่เปลี่ยนแปลง ถ้า size ของ vector ยังไม่ถึง capacity
//     let mut items2 = vec!["Gold", "Silver", "Bronze", "Ruby Gem"];
//     items2.remove(1);
//     println!("Crabby's inventory items: {:?}", items2);
//     println!("Crabby's inventory items: {:?}", items2.len());
//     println!("Crabby's inventory items: {:?}", items2.capacity());

//     items2.push("Emerald");
//     println!("Crabby's inventory items: {:?}", items2);
//     println!("Crabby's inventory items: {:?}", items2.len());
//     println!("Crabby's inventory items: {:?}", items2.capacity());

//     // 13. Iterators & Closures: ทางลัดในวังวนของ crabby
//     let treasures = vec![100, 50, 200, 400];
//     let double_treasures: Vec<i32> = treasures.iter().map(|x: &i32| x * 2).collect();
//     println!("Crabby's treasures: {:?}", double_treasures);

//     // 14. HashMap: Key Value
//     let mut treasures: HashMap<&str, i32> = HashMap::new();
//     treasures.insert("Gold", 100);
//     treasures.insert("Silver", 50);
//     treasures.insert("Ruby Gem", 200);
//     treasures.insert("Emerald", 400);
    
//     println!("treasures 14: {:?}:", treasures);
    
//     if let Some(golds) = treasures.get_mut("Gold") {
//         *golds += 50;
//     }
//     println!("treasures 14: {:?}:", treasures);

//     // 15. Error Handling: Rust
//     let chest_result: String = match open_chest(true) {
//         Some(treasure) => treasure,
//         None => "The chest is empty.".to_string(),
//     };
//     println!("{}", chest_result);

//     let chest_found_result: String = match open_chest(false) {
//         Some(treasure) => treasure,
//         None => "The chest is empty.".to_string(),
//     };
//     println!("{}", chest_found_result);


//     let door_safe_result: String = match open_door(false) {
//         Ok(safe) => safe,
//         Err(error) => panic!("{}", error),
//     };
//     println!("{}", door_safe_result);
    
//     // let door_result: String = match open_door(true) {
//     //     Ok(safe) => safe,
//     //     Err(error) => panic!("{}", error),
//     // };
//     // println!("{}", door_result);

//     // 16. Smart Pointers: Box<T>, Rc<T>, RefCell<T>
//     /// Style 1:
//      let chest = Box::new(10);
//      let shared_chest = Rc::new(RefCell::new(chest));

//      **shared_chest.borrow_mut() += 5;
//      println!("Shared chest value: {}", shared_chest.borrow());
//      **shared_chest.borrow_mut() += 10;
//      println!("Shared chest value: {}", shared_chest.borrow());
//      /// Style 2 in real case:
//      let chest2 = 10;
//      let shared_chest2 = Rc::new(RefCell::new(chest2));

//      *shared_chest2.borrow_mut() += 5;
//      println!("Shared chest 2 value: {}", shared_chest2.borrow());
//      *shared_chest2.borrow_mut() += 10;
//      println!("Shared chest 2 value: {}", shared_chest2.borrow());

//      // 17. Traits as a Type
//      let crabby_sword = Sword;
//      let crabby_bow = Bow;
//      let crabby_potion = Potion;

//      use_gear(crabby_sword);
//      use_gear(crabby_bow);
//      use_gear(crabby_potion);

//      // 18. Threading และ Concurrency
//      let crabby_gold = Arc::new(Mutex::new(10));
     
//      let loot_1 = thread::spawn({
//         let crabby_gold_artifact = Arc::clone(&crabby_gold);
//         move || {
//             let mut gold = crabby_gold_artifact.lock().unwrap();
//             *gold += 100;
//          }
//      });

//      let loot_2 = thread::spawn({
//         let crabby_gold_artifact = Arc::clone(&crabby_gold);
//         move || {
//             let mut gold = crabby_gold_artifact.lock().unwrap();
//             *gold += 200;
//          }
//      });

//      let loot_3 = thread::spawn({
//         let crabby_gold_artifact = Arc::clone(&crabby_gold);
//         move || {
//             let mut gold = crabby_gold_artifact.lock().unwrap();
//             *gold += 80;
//          }
//      });

//      loot_1.join().unwrap();
//      loot_2.join().unwrap();
//      loot_3.join().unwrap();

//      println!("Gold {}", crabby_gold.lock().unwrap());

//      // 19. Channels in Rust thread เครือข่ายสื่อสารของ crabby
//      let loots: Vec<i32> = vec![10, 20, 30];
//      let mut crabby_gold_coin:i32 = 100;

//      let (sender, receiver) = mpsc::sync_channel(3);
//      let sender_arc = Arc::new(sender);

//      for loot in loots.clone().into_iter() {
//         thread::spawn({
//             let sender = Arc::clone(&sender_arc);
//             move || {
//                 sender.send(loot).unwrap();
//              }
//         });
//      }

//      for _ in 0..loots.len() {
//         let loot = receiver.recv().unwrap();
//         crabby_gold_coin += loot;
//         println!("Crabby received loot: {}, Total gold coin: {}", loot, crabby_gold_coin);
//      }

    // 20. Tokio Async Runtime: Crabby's async adventure
    // Spawn all three tasks here using tokio::spawn()
    // Crabby wants all quests to run at the same time!
    let task_1 = tokio::spawn(gather_herbs());
    let task_2 = tokio::spawn(collect_gold_coins());
    let task_3 = tokio::spawn(fight_monster());

    let _ = tokio::join!(task_1, task_2, task_3);

}

// // 4. Functions
// fn crabby_task(task: &str, time: i32) -> String {
//     format!("Crabby has successfully done {} in {} miniute!", task, time)
// }

// // 6. Lifetimes ทำให้แน่ใจว่า crabby จะไม่หายไป ที่ ref จะไม่หายไป
// /// Before ก่อนแก้โดยใช้ Lifetime generic
// /* 
// fn longest_map(map1: &str, map2: &str) -> &str {
//     if map1.len() > map2.len() {
//         map1
//     } else {
//         map2
//     }
// }
// */

// /// After แก้โดยใช้ Lifetime generic
// fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
//     if map1.len() > map2.len() {
//         map1
//     } else {
//         map2
//     }
// }

// // 7. Structs
// struct Crabby {
//     name: String,
//     health: u8,
// }

// impl Crabby {
//     fn take_damage(&mut self, damage: u8) {
//         self.health = self.health.saturating_sub(damage);
//     }

//     fn healing(&mut self, heal: u8) {
//         if self.health + heal >= 100 {
//             self.health = 100;
//             return;
//         }
//         self.health += heal;
//     }
// }

// // 8. Enums
// enum CrabbyState {
//     Figting,
//     Collecting(u32),
//     Defending,
// }

// impl CrabbyState {
//     fn state_represent(&self) {
//         match self {
//             CrabbyState::Figting => println!("Crabby is in fighting mode!"),
//             CrabbyState::Collecting(coins) => println!("Crabby is collecting {} coins.", coins),
//             CrabbyState::Defending => println!("Crabby is in defending mode!"),
//         }
//     }
// }

// // 9. Traits
// struct Inventory<T> {
//     item: T,
// }

// trait DisplayItem {
//     fn display(&self);
// }

// impl<T> DisplayItem for Inventory<T> 
// where T: std::fmt::Debug,
// {
//     fn display(&self) {
//         println!("{:?}", self.item);
//     }
// }

// // 15. Error Handling: Rust
// fn open_chest(is_empty: bool) -> Option<String> {
//     if is_empty {
//         None
//     } else {
//         Some("You found a treasure!".to_string())
//     }
// }

// fn open_door(is_danger: bool) -> Result<String, String> {
//     if is_danger {
//         Err("A monster is behind the door!".to_string())
//     } else {
//         Ok("The door is safe to open.".to_string())
//     }
// }

// // 17. Traits as a Type
// trait Gear {
//     fn use_gear(&self);
// }

// struct Sword;
// struct Bow;
// struct Potion;

// fn use_gear<T: Gear>(item: T) {
//     item.use_gear();
// }

// impl Gear for Sword {
//     fn use_gear(&self) {
//         println!("Swing the sword!");
//     }
// }

// impl Gear for Bow {
//     fn use_gear(&self) {
//         println!("Shoot an arrow!");
//     }
// }

// impl Gear for Potion {
//     fn use_gear(&self) {
//         println!("Drink the potion!");
//     }
// }

// 20. Tokio Async Runtime: Crabby's async adventure

async fn gather_herbs() {
println!("Crabby is gathering herbs...");
}
async fn collect_gold_coins() {
println!("Crabby is collecting gold coins...");
}
async fn fight_monster() {
println!("Crabby is fighting the monster!");
}