fn main() {
    let dez = |teste| -> i32 {
        let teste = teste + 10;
        return teste;
    };
    
    dez(4);

    let num = dez();

    println!("{}", num);
}
