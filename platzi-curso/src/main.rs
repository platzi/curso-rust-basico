fn sumar_uno(numero_a_sumar: i32) -> i32 {
    let numero_final = numero_a_sumar + 1;
    println!("{}", numero_final);


    return numero_final;
}



fn main() {
    println!("Hola Platzi");

    let diez_mas_uno = sumar_uno(10);
    sumar_uno(diez_mas_uno);
    sumar_uno(12);
    sumar_uno(13);
    sumar_uno(14);


}
