
// # USE
use serde::Deserialize;

// # STRUCT
#[derive(Debug, Deserialize)]
//#[serde(rename_all = "camelCase")] // si el JSON estuviese en camelCase se podría renombrar
pub struct MailFrom {
    pub mail: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct NamecheapRecord {
    pub domain: String,
    pub key: String,
    pub hosts: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mail_from: MailFrom,
    pub mails_to: Vec<String>,
    pub namecheap: Vec<NamecheapRecord>
}

// # FN
pub fn list_domains(config: &Config) {
    println!("configured domains:");
    println!("- namecheap:");
    for namecheap_record in config.namecheap.iter() {
        println!("  {} {:?}", namecheap_record.domain, namecheap_record.hosts);
    }
}

pub fn vlog(verbose: &i8, verbose_level: i8, text: &str) {
    if *verbose >= verbose_level {
        println!("{}", text);
    }
}