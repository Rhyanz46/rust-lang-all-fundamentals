
//untuk buat project baru dengan cargo cukup `cargo new hello_world`
//untuk running, cargo run

// jika anda menambah depedencies in cargo.toml, maka update package dengan `cargo build`, dan akan di simpan di

use std::io; //memakai module dengan kata kunci `use`
//list  mudule build in https://doc.rust-lang.org/1.2.0/std/index.html

// untuk bisa menggunakan library external kita harus mengimport dengan `extern crate namapaket;`

extern crate rand;

//kemudian untuk memakainya dengan kata kunci `use` sama halnya ketika kita ingin menggunakan std::io
//kali ini kita hanya ingin menggunakan 'Rng' saja, maka tambahkan prefix ::
use rand::Rng;

//misalnya ingin menggunakan semua method yang ada di dalam rand makan bisa di tulis menggunakan `use rand::*`

//untuk compile terpisah, bisa pake rustc, dan untuk running, maka langsung ./hello_word

// let _variabel_mantap:u8 = 1; tidak bisa membuat variabel disini

// contants bisa di buat di luar sini(variable global), nilainya tidak bisa berubah, dan tidak bisa dijadikan mut
// disarankan untuk menggunakan huruf kapital, jika tidak maka akan warning, tujuannya mebantu konverter

const JUMLAH_ORANG:u8 = 2;

//enum akan menjadi tipe data
enum Kontroller{
    Push,
    Kill,
    _Put
}


// Cargo.toml adalah file konfigurasi, setelah di konfigurasu benar maka akan masuke Cargo.lock
//Cargo.lock bisa untuk downgrade aplikasi jg



fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101); //menggunakan paket rand

    println!("secret number : {}", secret_number);
    println!("Hello, world!");

    //secara default di rust variable adalah imuttable(tidak bisa di ubah)
    let mut nama = "arian";

    let reader = io::stdin(); //memasukkan modul ke dalam object

    let mut guess = String::new(); //membuat object String

    reader.read_line(&mut guess) //memasukkan nilai ke guess
        .ok()
        .expect("Failed to read line");

    print!("Masukkan angka : ");    

    let mut angka = String::new(); //membuat object String
    reader.read_line(&mut angka) //& disebut referensi https://doc.rust-lang.org/1.2.0/book/references-and-borrowing.html
        .ok()
        .expect("Failed to read line");

    let input: u32 = angka.trim().parse().expect("Please type a number!");

    println!("Hasil  {}", input*2);

    println!("Hello  {}", guess);
    println!("Hello  {}", nama);

    nama = "Cargo";

    println!("Hello  {}", nama);

    if nama == "Cargo"{
        println!("you're my brothers")
    }

    let tup: (i32, i64) = (12, 23); // tuple

    let (_a, _b) = tup; //ekstract tuple

    let tuple_1 = (123, "Arian"); //tipe data bebas

    println!("Nama : {}", tuple_1.1);

    println!("data {}", tup.1); //akses tuple

    let mut a = 0;

    loop {
        a +=1;

        println!("angka ke {}", a);

        if a == 5{
            break;
        }

    }

    while a <= 30{
        if a % 5 == 0{
            println!("number of a is {}", a);
        }
        a+=1;
    }

 

    for a in a..7{
        println!("for = {}", a)
    }

    let num = 1..10;
     println!("Jumlah {:?}", num.len()); //ambil jumlah dengan len()

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

    let jumlah = 2;
    match jumlah {
        2 => println!("dapat diskon 40%"),
        3 => println!("dapat diskon 90%"),
        _ => println!("gk dapat diskon") //harus ada pengecualian
    }

    println!("Jumlah Orang : {}", JUMLAH_ORANG);// mengakses variabel  global

    let kebenaran:bool = false;
    let hasil = match kebenaran{ //nge return ke variabel
        false => 0,
        true => 1 //tidak harus ada pengecualian, karena boolean hanya true dan false
    }; //karena akan ngereturn makan kita tutup dengan ;

    println!("baca : {}", hasil)


}
