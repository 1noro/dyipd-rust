
// # USE
use std::env;

// # CONST
//const LOOP_TIME: i32= 1800; // 1m = 60s, 5m = 300s, 10m = 600s, 15m = 900s, 30m = 1800s

//const CONFIG_FILE: &str = "conf.json";
//const LASTIP_FILE: &str = "lastip.json";

// # MAIN
fn main() {
    // variables de los parámetros
    let mut verbose = 0;
    //let mut send_mail = false;
    let mut looping = false;

    // obtenemos la lista de parámetros
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // parseamos los parámetros
    let mut i = 1;
    while i < args.len() {
        if args[i] == "verbose" {
            match args[i + 1].parse::<i32>() {
                Ok(n) => verbose = n,
                Err(e) => {eprintln!("Got error: {}", e); return ();}
            }
            i += 1;
        } else if args[i] == "looping" {
            looping = true;
        }
        i += 1;
    }

    println!("(verbose, looping) = {:?}", (verbose, looping));
}
