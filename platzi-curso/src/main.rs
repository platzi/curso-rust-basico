fn main() {
    // Obtener la edad de la consola
    println!("Por favor introduce tu edad: ");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // Convertir esa edad a un numero
    let edad_int : u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 && edad_int != 30 {
        println!("Puedes entrar a la discoteca");
    } else if edad_int == 30 {
        println!("No admitimos personas de exactamente 30 años");
    }
    else {
        println!("Eres menor de edad todavia");
    } 

    println!("Tienes {} años", edad_int);
}