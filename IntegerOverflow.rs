fn main() {
    let age:u8 = 255;

    // 0 to 255 only allowed for u8
    let weight:u8 = 256; // overflow by 0
    let height:u8 = 257; // overflow by 1

    println!("age is {} ", age);
    println!("weight is {}", weight);
    println!("height is {}", height);
}