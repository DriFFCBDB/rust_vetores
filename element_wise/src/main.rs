//Escreva uma função que some dois vetores element-wise (elemento por elemento) e
// retorne o resultado como um novo vetor.

fn main() {
    let vetor1 = vec![1, 2, 3, 4, 5];
    let vetor2 = vec![10, 20, 30, 40, 50];


    let resultado: Vec<_> = vetor1.iter().zip(&vetor2).map(|(a, b)| a + b).collect();

    println!("{:?}", resultado);
}
