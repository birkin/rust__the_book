use std::env;
use std::env::VarError;

#[derive(Debug)]
struct Config {
    setting_a: String,
}

impl Config {
    fn new() -> Config {
        let setting_a_try: Result<String, VarError> = env::var("ENV_FOO");
        match setting_a_try {
            Ok(_) => {},
            Err(_err) => {
                println!("setting_a not found; quitting");
                std::process::exit(-1);
            }
        };
        let setting_a = setting_a_try.unwrap();
        Config { setting_a }
    }
}

fn main() {
    /*
    If envar is not set, yields output: "setting_a not found; quitting".
    If envar is set, like: `$ export ENV_FOO="bar"`, yields output:
        config, ``Config { setting_a: Ok("bar") }``
        config.setting_a, ``Ok("bar")``
    */

    let config = Config::new();

    assert_eq!( config.setting_a, "bar".to_string() );

    println!("config, ``{:?}``", config);
    println!("config.setting_a, ``{:?}``", config.setting_a);
}
