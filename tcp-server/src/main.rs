// use std::io::{BufRead, BufReader, Write};
// use std::net::{TcpListener, TcpStream};
// use std::thread;

// fn handle_connection(mut stream: TcpStream)  -> std::io::Result<()> {
//     let peer = stream.peer_addr()?;
//     eprintln!("Conexão recebida de {}", peer);

//     let reader = BufReader::new(stream.try_clone()?);
//     for line in reader.lines() {
//         let msg = line?;
//         let trimmed = msg.trim();
//         eprintln!("Cliente disse: {:?}", trimmed);

//         let resposta = match trimmed.to_lowercase().as_str() {
//             "ping" => "pong\n".to_string(),
//             "hello" => "Olá!\n".to_string(),
//             _ => format!("Você disse: {}\n", trimmed),
//         };

//         stream.write_all(resposta.as_bytes())?;
//     }

//     eprintln!("Conexão encerrada: {}", peer);
//     Ok(())
// }

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("0.0.0.0:9000")?;
//     eprintln!("Servidor TCP rodando na porta 9000...");

//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 thread::spawn(|| {
//                     if let Err(e) = handle_connection(stream) {
//                         eprintln!("Erro na conexão: {}", e);
//                     }
//                 });
//             }
//             Err(e) => eprintln!("Falha ao aceitar: {}", e),
//         }
//     }
//     Ok(())
// }

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    eprintln!("Conexão recebida de {}", peer);

    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => {
                eprintln!("Conexão encerrada: {}", peer);
                return Ok(());
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Erro ao ler: {}", e);
                return Err(e);
            }
        };

        if let Err(e) = stream.write_all(&buf[..n]) {
            eprintln!("Erro ao escrever: {}", e);
            return Err(e);
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    eprintln!("Servidor TCP rodando na porta 9000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Erro na conexão: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Falha ao aceitar: {}", e),
        }
    }
    Ok(())
}
