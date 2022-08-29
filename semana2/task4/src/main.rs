struct Filosofo {
    nombre: String,
}

impl Filosofo {
    fn new(nombre: &str) -> Filosofo {
        Filosofo {
            nombre: nombre.to_string(),
        }
    }

    fn comer(&self) {
        println!("{} ha finalizado de comer.", self.nombre);
    }
}

fn main() {
    let filosofos = vec![
        Filosofo::new("Judith Butler"),
        Filosofo::new("Gilles Deleuze"),
        Filosofo::new("Karl Marx"),
        Filosofo::new("Emma Goldman"),
        Filosofo::new("Michel Foucault"),
    ];

    for f in &filosofos {
        f.comer();
    }
}