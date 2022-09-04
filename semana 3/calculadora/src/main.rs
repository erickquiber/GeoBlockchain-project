use regex::Regex;

fn main() {
    // Cosas que traemos de regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap(); // (15 + 20) o (15+20)
    let re_resta = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap(); // (15 + 20) o (15+20)
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap(); // (15 + 20) o (15+20)
   // let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap(); // (15 + 20) o (15+20)


    //Pedirle los datos al usuario
    println!("Por favor introdusca la expresion: ");
    let mut expresion: String = String::new();

    std::io::stdin().read_line(&mut expresion).unwrap();
    
    loop {
        // Capturar los datos que queremos para operarlos
        let caps = re_mul.captures(expresion.as_str()); // caps 1: 15 y caps 2:: 20

        if caps.is_none(){
            break;
        }
        
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let multi = left_value * right_value;

        expresion=expresion.replace(cap_expression, &multi.to_string());
    }


    // Loop para las restas
    loop {
        // Capturar los datos que queremos para operarlos
        let caps = re_resta.captures(expresion.as_str()); // caps 1: 15 y caps 2:: 20

        if caps.is_none(){
            break;
        }
        
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let resta = left_value - right_value;

        expresion=expresion.replace(cap_expression, &resta.to_string());
    }

    // Loop para las sumas
    loop {
        // Capturar los datos que queremos para operarlos
        let caps = re_add.captures(expresion.as_str()); // caps 1: 15 y caps 2:: 20

        if caps.is_none(){
            break;
        }
        
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let suma = left_value + right_value;

        expresion=expresion.replace(cap_expression, &suma.to_string());
    }
    
   
 // nos dice con mayor detalle que esta haciendo esa variable

    println!("Resultado: {} ", expresion);


}
