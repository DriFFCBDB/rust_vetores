
//Crie um enum que represente cores (por exemplo, vermelho, verde e azul).
//Crie um vetor de cores e itere sobre ele, imprimindo o nome de cada cor.

fn main() {
    //enum sempre CamelCase
    enum EnumCores{
        //&'static str especificar que são strings estáticas.
        Cor1(&'static str),
        Cor2(&'static str),
        Cor3(&'static str),
    }

    let cores = vec![EnumCores::Cor1("vermelho"), EnumCores::Cor2("verde"), EnumCores::Cor3("azul")];

    for cor in &cores {
        match cor {
            EnumCores::Cor1(nome) => println!("Cor 1: {}", nome),
            EnumCores::Cor2(nome) => println!("Cor 2: {}", nome),
            EnumCores::Cor3(nome) => println!("Cor 3: {}", nome),
        }
    }

}
