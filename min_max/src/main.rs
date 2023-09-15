fn valor_min_max(vetor: &Vec<i32>) -> Option<(i32, i32)> {
    let minimo = vetor.iter().min();
    let maximo = vetor.iter().max();

    match (minimo, maximo) {
        (Some(min), Some(max)) => Some((*min, *max)),
        _ => None,
    }
}

fn main() {
    let meu = vec![1, 2, 3];
    match valor_min_max(&meu) {
        Some((min, max)) => {
            println!("Mínimo: {}", min);
            println!("Máximo: {}", max);
        }
        None => {
            println!("O vetor está vazio.");
        }
    }
}
