//Calcule a média de um vetor de números inteiros.
// Use iter() para iterar pelos elementos e unwrap() para obter o valor Some.


// soma dos elementos / numero de elementos(tamanho)

fn media(vetor: &Vec<i32> ) -> i32{

    let soma :i32 = vetor.iter().sum();
    //usamos Some para criar um Option<usize> ,numero_elementos é um Option que pode conter um valor usize (ou seja, Some) ou ser vazio (ou seja, None)
    let numero_elementos :Option<usize> = Some(vetor.len());

    match numero_elementos {
        //O uso de _ é uma convenção em Rust para dizer ao compilador que você está ciente de que há um valor ali, mas não o está utilizando naquele momento.
        Some(_) => soma / numero_elementos.unwrap() as i32,
        None => 0,
    }

}

fn main() {
    let numeros= vec![8,2,3];
    let media = media(&numeros);
    println!(" {} ",media);
}
