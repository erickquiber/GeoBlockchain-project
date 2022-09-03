fn main() {
    // En este caso poniendolo de este modo rust le pueda signar un tipo sin prblemas
    let numero_1=120;
    let numero_2=200;

    // le asignamos un valor a la variable suma mediante una operacion este sera un unico valor fijo
    let suma=numero_1 + numero_2;

    // se creo una repeticion 
    loop {
        let mut suma_usuario=String::new();
        println!("Por favor escrribir la suma de {} y {}: ",numero_1,numero_2);
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int : i16=suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma{
            println!("Lo que ingreso es correcto");
            // Esta parte de aqui se coloca para finalizar el proceso para poder seguir o cortarla
            break;
        }else {
            println!("Mejor vuelve a unteralo")
        }
    }

}
