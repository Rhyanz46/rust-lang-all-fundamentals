//enum akan menjadi tipe data

enum Kontroller{
    Push,
    Kill,
    _Put
}



fn main() {
    println!("Hello, world!");

    //secara default di rust variable adalah imuttable(tidak bisa di ubah)
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
     println!("Jumlah {:?}", num.len());

    //double :: untuk akses variant
    // : untuk akses tipe

    let harga:f64 = 20.000;
     println!("Jumlah {:?}", harga);


    let kontrol =  Kontroller::Kill;
    let _kontrol_2:Kontroller =  Kontroller::Push; 
    //sudah menjadi aturan di rust, variabel yang tidak digunakan akan menampilkan waring, karena buang" ruang, oleh karena itu 
    //di pasang tanda _ pada awal variabel agar tidak memunculkan warning,

    match kontrol{ //mirip switch di bahasa lain
        Kontroller::Kill => println!("bunuh dia"),
        Kontroller::Push => println!("Dorong dia"),
        Kontroller::_Put => println!("Letakkan dia") 
    }


}
