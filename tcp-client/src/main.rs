// use bigdecimal::BigDecimal;
// use num_bigint::BigInt;
// use std::io::{self, BufRead, BufReader, Write};
// use std::net::TcpStream;
// use std::str::FromStr;
// use std::time::Instant;

// fn main() -> std::io::Result<()> {
//     let mut ip = String::new();
//     print!("Digite o IP do servidor: ");
//     io::stdout().flush()?;
//     io::stdin().read_line(&mut ip)?;
//     let ip = ip.trim();

//     let mut port = String::new();
//     print!("Digite a porta do servidor: ");
//     io::stdout().flush()?;
//     io::stdin().read_line(&mut port)?;
//     let port = port.trim();

//     let addr = format!("{}:{}", ip, port);
//     let mut stream = TcpStream::connect(addr)?;
//     eprintln!("Conectado ao servidor.");

//     let mut reader = BufReader::new(stream.try_clone()?);
//     let stdin = io::stdin();

//     loop {
//         let mut line = String::new();
//         print!("Você: ");
//         io::stdout().flush()?;
//         stdin.read_line(&mut line)?;
//         let msg = line.clone();

//         let start = Instant::now();

//         stream.write_all(msg.as_bytes())?;

//         let mut resp = String::new();
//         reader.read_line(&mut resp)?;

//         let duration = start.elapsed().as_nanos();

//         let nanos = BigInt::from(duration as u64);
//         let mut bd = BigDecimal::new(nanos, 9);
//         bd = bd.with_scale(20);

//         println!("Tempo de resposta: {} segundos", bd);
//         print!("Servidor: {}", resp);
//     }
// }

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Instant;

const MAX_SIZE: usize = 1 << 30;

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:9000";
    let mut stream = TcpStream::connect(&addr)?;
    println!("Conectado ao servidor em {}\n", addr);

    let mut reader = stream.try_clone()?;

    let mut size = 2;
    while size <= MAX_SIZE {
        let payload = vec![b'A'; size];

        let start = Instant::now();

        stream.write_all(&payload)?;

        let mut resp = vec![0u8; size];
        reader.read_exact(&mut resp)?;

        let elapsed_ns = start.elapsed().as_nanos();

        let nanos = BigInt::from(elapsed_ns as u64);
        let mut bd = BigDecimal::new(nanos, 9);
        bd = bd.with_scale(6);

        println!(
            "Tamanho: {:6} bytes → Tempo de resposta: {} s",
            size,
            bd
        );

        size *= 2;
    }

    Ok(())
}
