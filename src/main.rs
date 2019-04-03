fn main() {
    println!("Hello, world!");

    let mut nama = "arian";

    println!("Hello  {}", nama);

    nama = "Cargo";

    println!("Hello  {}", nama);



    if nama == "Cargo"{
        println!("you're my brothers")
    }

    let tup: (i32, i64) = (12, 23);

    let (_a, _b) = tup;

    println!("data {}", tup.1);

    let mut a = 0;

    loop {
        a +=1;

        println!("angka ke {}", a);

        if a == 10{
            break;
        }

    }

    while a <= 50{
        if a % 5 == 0{
            println!("number of a is {}", a);
        }
        a+=1;
    }

    for a in a..100{
        println!("for = {}", a)
    }

    let num = 1..10;
    println!("the range is {}", num);
}
