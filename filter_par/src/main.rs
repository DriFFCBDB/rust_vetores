//Crie uma função que filtre e retorne apenas os números pares de um vetor usando iter() e filter().

fn filtrar_par(vetor: Vec<u32>) -> Vec<u32> {
    let pares: Vec<u32> = vetor.iter().cloned().filter(|&elem| elem % 2 == 0).collect();
    pares
}

fn main() {
    let meu = vec![1, 672, 3, 44, 5];
    let numeros_pares = filtrar_par(meu.clone());

    match numeros_pares.len() {
        0 => println!("Não há números pares no vetor: {:?}", meu),
        _ => println!("Números pares no vetor: {:?}", numeros_pares),
    }
}

