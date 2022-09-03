fn main() {
    // Vamos a declar la variable con let mut para que se puedad modificar
    let mut edad: String=String::new();
    
    // le pedimos al usuario que ingrese su usuario
    println!("Por favor introduce tu edad: ");
    std::io::stdin().read_line(&mut edad).unwrap();

    //Aca transformamos para que la edad sea un numero
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("Tienes {} años ",edad_int);

    // Usamos condicionales para dividir en caso puedad entrar a la discoteca o no 
    // Tiene la misma logica que en C++
    if edad_int >=18 {
        println!("Ustdes con {} puedes ingresar a la discoteca", edad_int);
    } else  {
        println!("Ustdes con {} no puedes ingresar a la discoteca", edad_int);
    }
}
