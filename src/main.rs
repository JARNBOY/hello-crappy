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