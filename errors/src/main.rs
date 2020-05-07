fn main() {

    // -- straight panic --
    // misc01();

    // -- out of bounds --
    misc02();

}

fn misc02() {
    let v = vec![1, 2, 3];

    v[99];
}



// fn misc01() {
//     panic!( "crash" );
// }
