use clap::Clap;
use configparser::ini::{Ini, IniDefault};
use shellexpand;
use std::error::Error;

#[derive(Clap, Debug)]
#[clap(name = "account")]
struct Account {
    #[clap(short, long)]
    id: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let account = Account::parse();

    let config_file = if let Ok(path) = std::env::var("AWS_CONFIG_FILE") {
        path
    } else {
        String::from("~/.aws/config")
    };
    let config_path = shellexpand::tilde(&config_file);

    let config_default = IniDefault {
        default_section: "default".to_owned(),
        comment_symbols: vec!['#'],
        delimiters: vec!['='],
        case_sensitive: true,
    };

    let mut config = Ini::new_from_defaults(config_default.clone());
    let map = config.load(&*config_path)?;

    let mut section = String::from("");
    for (k, v) in map {
        if let Some(Some(id)) = v.get("sso_account_id") {
            if id == &account.id {
                section = k;
                break;
            }
        }
    }

    let profile_name = section.strip_prefix("profile ").unwrap();
    println!("{}", profile_name);

    Ok(())
}
