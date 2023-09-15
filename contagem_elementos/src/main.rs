//Escreva uma função que conta quantos elementos em um vetor são iguais a um valor específico
// usando iter() e filter().


fn contar_elementos(vetor: &Vec<i32>, valor: i32) -> usize {
    vetor.iter().filter(|&elemento| *elemento == valor).count()
}

fn main() {
    let numeros = vec![1, 2, 2, 3, 2, 4];
    let valor = 2;
    let contagem = contar_elementos(&numeros, valor);
    println!("O valor {} aparece {} vezes no vetor.", valor, contagem);
}
