fn main() {
    adivinar_num(5);
}

fn adivinar_num(x: i32){

if x == 1 {
    println!("x es uno!");
} else if x == 2 {
    println!("x es 2!");
} else {
    println!("x no es ni 1 ni 2 :(");
}
}