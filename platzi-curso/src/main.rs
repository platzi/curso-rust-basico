fn main() {
    // Dos numeros que vamos a sumar
    let numero_1 = 120;
    let numero_2 = 321;

    let suma = numero_1 + numero_2;

    loop {
        // Mostrar los dos numeros en pantalla
        println!("Por favor escribir la suma de {} y {}: ", numero_1, numero_2);

        // Obtener del usuario el numero que representa la suma
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int : i32 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Lo has hecho muy bien, el resultado {} es correcto", suma);
            break;
        } else {
            println!("El resultado {} no es correcto por favor intentalo de nuevo", suma_usuario_int);
        }
    }

}