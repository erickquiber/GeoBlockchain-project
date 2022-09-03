// Funcion para que no devuelva parametros
fn sumar_uno (numero_a_sumar: i32){
    let numero_sumado: i32 = numero_a_sumar + 1;
    println!("{}", numero_sumado);
}

// Funcion para que si devuelva un valor
fn sumar_100 (numero_a_sumar: i32) -> i32 {
    let numero_devuelto: i32 = numero_a_sumar + 100;

    return numero_devuelto;

}

fn main() {
    
    let primero_mas_100:i32 = sumar_100(1);
    sumar_uno(primero_mas_100);
    sumar_uno(3);
    sumar_uno(4);
    sumar_uno(5);

    println!("Primero se sumo 100 y salio {}",primero_mas_100);

}
