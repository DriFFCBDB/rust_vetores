//Crie um vetor de números inteiros e use Some para representar números pares e None para números ímpares. Escreva uma
//função que use match para imprimir "Par" ou "Ímpar" para cada elemento.

fn main() {

    let meu_vetor :Vec<i32 >= vec![0,1,2,3,4,5,6,7,8,9];

    for i in meu_vetor {
        match i % 2 {
            0 => println!("{} é par",i),
            _ => print!("{} não é par.",i),
        }
    }

}
