
// # MOD
mod utils;


// # USE
use std::env;
use std::path::Path;
use std::fs::File;
use std::{thread, time};
// use config::Config;


// # CONST
const LOOP_TIME: time::Duration = time::Duration::from_millis(3000); // 1m = 60000ms, 5m = 300000ms, 10m = 600000ms, 15m = 900000ms, 30m = 1800000ms
const CONFIG_LOC: &str = "config.json";
//const LASTIP_LOC: &str = "lastip.json";


// # MAIN
fn main() {
    // variables de los parámetros
    let mut verbose: i8 = 0;
    //let mut send_mail = false;
    let mut looping = false;

    // obtenemos la lista de parámetros
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // parseamos los parámetros
    let mut i = 1;
    while i < args.len() {
        if args[i] == "verbose" {
            match args[i + 1].parse::<i8>() {
                Ok(n) => verbose = n,
                Err(e) => {eprintln!("Got error: {}", e); return ();}
            }
            i += 1;
        } else if args[i] == "looping" {
            looping = true;
        }
        i += 1;
    }

    // println!("(verbose, looping) = {:?}", (verbose, looping));

    // inicia el programa
    if verbose >= 1 {println!("starting dyipd-rust")}

    // leeomos la configuración
    let json_file_path = Path::new(CONFIG_LOC);
    let file = File::open(json_file_path).expect("file not found");
    let config: utils::Config = serde_json::from_reader(file).expect("error while reading");
    // println!("{:?}", config)

    if verbose >= 2 {
        utils::list_domains(&config);
        println!("notification mail: {}", config.mail_from.mail);
        println!("mails to notify: {:?}", config.mails_to);
    }

    // empieza el bucle
    loop {
        if verbose >= 1 && looping {println!("> beginning of the cycle");}
        if !looping {break;}
        thread::sleep(LOOP_TIME);
    }

}
