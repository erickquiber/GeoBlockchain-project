use std::string;

fn main() {
    let mut nombres : Vec<String> =Vec::new ();

    for i in 0..3 {
        println!("Ingresa los nombres por favor: ");
        let mut nombre = String::new();
        std::io::stdin().read_line(&mut nombre).unwrap();

        nombres.push(nombre);

        println!("El {} nombre ingresado es {} ", i+1,nombres[i]);
    }

    for nombre in nombres {
        println!("El nombre es: {}",nombre);
    
    }

 
}
