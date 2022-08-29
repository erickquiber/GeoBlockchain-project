fn main() {
    print_sum(30, 10);
    print_rest(30, 10);
    print_div(30, 10);
    print_mult(30, 10);
}

fn print_sum(x: i32,y: i32){
    print!("la suma es:{} ",x+y)
}
fn print_rest(x: i32,y: i32){
    print!("/ la resta es:{} ",x-y)
}
fn print_mult(x: i32,y: i32){
    print!("/ la multiplicacion es:{} ",x*y)
}
fn print_div(x: i32,y: i32){
    print!("/ la division es:{} ",x/y)
}