// https://github.com/softprops/envy

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
  foo: u16,
  bar: bool,
  baz: String,
  boom: Option<u64>,
  // zz: String,
}

impl Config {
    fn new() -> Config {
        match envy::from_env::<Config>() {
            Ok(config) => {
                let foo = config.foo;
                let bar = config.bar;
                let baz = config.baz;
                let boom = config.boom;
                // let zz = config.zz;
                Config { foo, bar, baz, boom }
            },
            Err(error) => panic!("{:#?}", error)
        }

    }
}


fn main() {

    // --- works ---
    // match envy::from_env::<Config>() {
    //    Ok(config) => println!("{:#?}", config),
    //    Err(error) => panic!("{:#?}", error)
    // }

    // --- works (config.baz) ---
    // match envy::from_env::<Config>() {
    //    Ok(config) => println!("{:#?}", config.baz),
    //    Err(error) => panic!("{:#?}", error)
    // }

    // // --- works ---
    // match envy::from_env::<Config>() {
    //     Ok(config) => {
    //         println!("foo, {:#?}", config.foo);
    //         println!("baz, {:#?}", config.baz);  // checked, config.baz is type String
    //     },
    //    Err(error) => panic!("{:#?}", error)
    // }

    let config = Config::new();
    assert_eq!( config.baz, "hello world".to_string() );

    println!("config, ``{:?}``", config);
    println!( "config.foo, ``{:?}``", config.foo );
}



// impl Config {
//     fn new() -> Config {
//         let setting_a_try: Result<String, VarError> = env::var("ENV_FOO");
//         match setting_a_try {
//             Ok(_) => {},
//             Err(_err) => {
//                 println!("setting_a not found; quitting");
//                 std::process::exit(-1);
//             }
//         };
//         let setting_a = setting_a_try.unwrap();
//         Config { setting_a }
//     }
// }
