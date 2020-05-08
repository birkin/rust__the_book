fn main() {

    // -- straight panic --
    // misc01();

    // -- out of bounds --
    // misc02();

    // -- starting Result work --
    misc03();

}

fn misc03() {
    use std::fs::File;
    let f = File::open( "foo.txt" );
    // let zz: () = f;  yields: found enum `std::result::Result<std::fs::File, std::io::Error>`

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: ``{:?}``", error)
    };
}



// fn misc02() {
//     let v = vec![1, 2, 3];
//     v[99];
// }



// fn misc01() {
//     panic!( "crash" );
// }
