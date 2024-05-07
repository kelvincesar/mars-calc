// RUST OWNERSHIP RULES:
// 1. Each value in rust is owned by a variable;
// 2. When the owner goes out of scope, the value will be deallocated;
// 3. There can only be ONE owner at time

// Importa o módulo de I/O
use std::io;

fn main() {
    // Cria variável usando String;
    let mut input = String::new();
    println!("Digite o seu peso em kg na terra:");
    io::stdin().read_line(&mut input).unwrap();

    // Remove caracteres nulos e converte para f32
    let weight: f32 = input.trim().parse().unwrap();

    //dbg!(weight);

    // Não é necessário especificar o tipo, mas para fazer isso utilizar ':f32'
    // Para tornar a variável mutável, utilizar a palavra `mut`
    let mut mars_weight: f32 = calculate_weight_on_mars(weight);

    println!("Peso em Marte: {} kg", mars_weight);
    mars_weight *= 1000.0;
    println!("Peso em Marte: {} g", mars_weight);
}
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.82) * 3.711
}
