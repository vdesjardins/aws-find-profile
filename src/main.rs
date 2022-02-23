use clap::Clap;
use configparser::ini::{Ini, IniDefault};
use shellexpand;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clap, Debug)]
#[clap(name = "account")]
struct Account {
    #[clap(short, long)]
    id: String,
}

const SSO_ACCOUNT_ID: &str = "sso_account_id";

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
    let config = config.load(&*config_path)?;

    let profile_names = find_profile_by_account_id(&account.id, &config);

    profile_names.iter().for_each(|p| {
        println!("{}", p);
    });

    Ok(())
}

fn find_profile_by_account_id<'a>(
    account_id: &'a str,
    config: &'a HashMap<String, HashMap<String, Option<String>>>,
) -> Vec<&'a str> {
    config
        .iter()
        .filter_map(|(k, v)| {
            if let Some(Some(id)) = v.get(SSO_ACCOUNT_ID) {
                if id == &account_id {
                    return k.strip_prefix("profile ");
                }
            }
            None
        })
        .collect()
}
