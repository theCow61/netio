use std::io::prelude::*;

fn crun(ipnport: &str, nickn: &str) {
    let mut stream = std::net::TcpStream::connect(ipnport).expect("Could not connect...");
    let mut srey = stream.try_clone().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move|| {
        synco(&mut srey, rx);
    });
    loop {
        let mut input = String::from(format!("{}: ", nickn));
        let in_data: &[u8];
        let _ = std::io::stdout().flush();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                in_data = input.as_bytes();
            },
            Err(_) => {
                println!("Failed to read input...");
                continue;
            },
        }
        // println!("{}", String::from_utf8_lossy(in_data));
        match stream.write(in_data) {
            Ok(_) => continue,
            Err(_) => {
                println!("Failed to write to stream...");
                break;
            },
        }
    }
    let _ = tx.send(1);
}

fn main() -> Result<(), std::io::Error> {
    /*let listener = std::net::TcpListener::bind("127.0.0.1:42069")?;
    let (mut stream, socketadd) = listener.accept()?;
    let datawrt = b"Hello writer\n";
    stream.write(&datawrt[0..5])?;
    let mut datard = [0; 128];
    stream.read(&mut datard)?;
    let datahmn = String::from_utf8_lossy(&datard);
    println!("{}", datahmn);*/


    let mut args = std::env::args().skip(1);
    let mut is_sorver = true;
    match args.next().expect("Usage: ./netio [server/client] [ip:port] [nickname]").as_str() {
        "server" => { is_sorver = true; },
        "client" => { is_sorver = false; },
        _ => println!("Usage: ./netio [server/client] [ip:port] [nickname]"),
    }
    let hostip = args.next().expect("Usage: ./netio [server/client] [ip:port] [nickname]");
    let nick = args.next().expect("Usage: ./netio [server/client] [ip:port] [nickname]");
    match is_sorver {
        true => {
            println!("Hosting on {}", hostip);
            srun(&hostip, &nick);
        },
        false => {
            println!("Connecting to {}", hostip);
            crun(&hostip, &nick);
        },
    }

    // srun("127.0.0.1:42069");
    Ok(())
}

fn srun(ipnport: &str, nickn: &str) {
    let listener = std::net::TcpListener::bind(ipnport).expect("Could not bind...");
    loop {
        let (mut stream, socket_addr) = listener.accept().expect("Failed to accept...");
        let mut srey = stream.try_clone().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move|| {
            synco(&mut srey, rx);
        });
        println!("{} has connected...", socket_addr);
        loop {
            let mut input = String::from(format!("{}: ", nickn));
            let in_data: &[u8];
            let _ = std::io::stdout().flush();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    in_data = input.as_bytes();
                },
                Err(_) => {
                    println!("Failed to read input...");
                    continue;
                },
            }
            // println!("{}", String::from_utf8_lossy(in_data));
            match stream.write(in_data) {
                Ok(_) => continue,
                Err(_) => {
                    println!("Failed to write to stream...");
                    break;
                },
            }
        }
        println!("{} has disconnected...", socket_addr);
        let _ = tx.send(1);
    }

}

fn synco(stream: &mut std::net::TcpStream, rx: std::sync::mpsc::Receiver<i32>) {
    loop {
        match rx.try_recv() {
            Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                println!("Terminating...");
                // stream.shutdown(std::net::Shutdown::Write).expect("Failed to shutdown stream...");
                break;
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {},
        }
        let mut ou_data = [0 as u8; 1000];
        stream.read(&mut ou_data).expect("Could not read data...");
        let read = String::from_utf8_lossy(&ou_data);
        print!("{}", read);
    }
}









// fn input(input: &mut String) -> Result<(), std::io::Error> {
//     std::io::stdin().read_line(input)?;
//     Ok(())
// }
