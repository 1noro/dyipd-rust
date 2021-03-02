
// # USE
use std::env;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;
use std::{thread, time};

// # CONST
const LOOP_TIME: time::Duration = time::Duration::from_millis(3000); // 1m = 60000ms, 5m = 300000ms, 10m = 600000ms, 15m = 900000ms, 30m = 1800000ms

const CONFIG_LOC: &str = "config.json";
//const LASTIP_LOC: &str = "lastip.json";

// # STRUCT
#[derive(Debug, Deserialize)]
//#[serde(rename_all = "camelCase")] // si el JSON estuviese en camelCase se podría renombrar
struct MailFrom {
    mail: String,
    password: String
}

#[derive(Debug, Deserialize)]
struct NamecheapRecord {
    domain: String,
    key: String,
    hosts: Vec<String>
}

#[derive(Debug, Deserialize)]
struct Config {
    mail_from: MailFrom,
    mails_to: Vec<String>,
    namecheap: Vec<NamecheapRecord>
}

// # FN
fn list_domains(config: &Config) {
    println!("configured domains:");
    println!("- namecheap:");
    for namecheap_record in config.namecheap.iter() {
        println!("  {} {:?}", namecheap_record.domain, namecheap_record.hosts);
    }
}

// # MAIN
fn main() {
    // variables de los parámetros
    let mut verbose = 0;
    //let mut send_mail = false;
    let mut looping = false;

    // obtenemos la lista de parámetros
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

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

    // println!("(verbose, looping) = {:?}", (verbose, looping));

    // inicia el programa
    if verbose >= 1 {println!("starting dyipd-rust")}

    // leeomos la configuración
    let json_file_path = Path::new(CONFIG_LOC);
    let file = File::open(json_file_path).expect("file not found");
    let config: Config = serde_json::from_reader(file).expect("error while reading");
    // println!("{:?}", config)

    if verbose >= 2 {
        list_domains(&config);
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
