use wasm_bindgen::prelude::*;

#[wasm_bindgen] // Esto es para crear un enlace entre la funcion del navegador y la funcion que tenemos aca
extern { // Esto se coloca para las funciones externas (las de navegador)
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn saludar(nombre: &str){
    alert(&format!("Hola, {}, como estas?", nombre))
}
