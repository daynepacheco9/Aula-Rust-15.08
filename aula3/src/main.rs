fn main() {
    //teste ponto flutuante
    println!("\nTeste de ponto flutuante");
    let x: f64 = 10.;
    let y: f32 = 3.1;
    let z = x+y as f64;
    let resto = x % y as f64;
    println!{"\n{}", z};
    println!{"\n{}", resto};


    //teste tupla()
    print!("\n\nTeste de tupla");
    let my_tuple = (123,1.23,"hello");
    print!("\nPrimeiro valor {}", my_tuple.0);
    print!("\nultimo valor {}", my_tuple.2);
    let (v1,v2,v3) = my_tuple;
    print!("\nvalor v1 {}",v1);
    print!("\nvalor v2 {}",v2);
    print!("\nvalor v3 {}",v3);

    //teste array []
    print!("\n\nTeste Array");
    let my_array:[i32;3] = [159, 12 ,13 ];
    print!("\n{} {} {}", my_array[0],my_array[1],my_array[2]);


    print!("\n\nteste função");
    print!("\nfunção inicial");
    my_function();
    print!("\nfunção soma");

    let one = 12;
    let two = 15;
    print!("\n{}",my_sum(one, two));

}

fn my_function(){
    print!("\nfrom my_fuction");
}

fn my_sum(x:i32,y:i32) -> i32{
    return x +y;
}