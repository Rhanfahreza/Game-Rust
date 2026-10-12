use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=== SELAMAT DATANG DI GAME TEBAK ANGKA ===");
    println!("Saya telah memilih sebuah angka antara 1 sampai 100.");

    let angka_rahasia = rand::random_range(1..=100);

    loop {
        println!("\nSilakan masukkan tebakan Anda:");

        let mut tebakan = String::new();

        io::stdin()
            .read_line(&mut tebakan)
            .expect("Gagal membaca baris input");

        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(angka) => angka,
            Err(_) => {
                println!("Tolong masukkan angka yang valid, jangan huruf!");
                continue;
            }
        };

        println!("Anda menebak: {tebakan}");

        match tebakan.cmp(&angka_rahasia) {
            Ordering::Less => println!("Tebakan Anda terlalu KECIL!"),
            Ordering::Greater => println!("Tebakan Anda terlalu BESAR!"),
            Ordering::Equal => {
                println!("🎉 SELAMAT! Tebakan Anda BENAR! 🎉");
                break;
            }
        }
    }
}