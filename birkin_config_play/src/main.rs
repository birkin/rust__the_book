use std::env;
use std::env::VarError;

#[derive(Debug)]
struct Config {
    setting_a: Result<String, VarError>,
}

impl Config {
    fn new() -> Config {
        let setting_a: Result<String, VarError> = env::var("ENV_FOO");
        match setting_a {
            Ok(_) => {},
            Err(_err) => {
                println!("setting_a not found; quitting"); std::process::exit(-1); },
        };
        Config { setting_a }
    }
}

fn main() {
    let config = Config::new();
    println!("config, ``{:?}``", config);
    println!("config.setting_a, ``{:?}``", config.setting_a );
    // assert_eq!( config.setting_a, "bar".to_string() );
}
