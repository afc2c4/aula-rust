fn main() {
    
    let x = 10;
    println!("O NUMERO É: {}", x);

    {   
        let mut x = 2;
        println!("O NUMERO É: {}", x);

        x = 6;
        if x > 5 {
            println!("O x é maior que cinco!");
        }
    }
    
   
}
