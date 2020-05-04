use std::collections::HashMap;


fn main() {

    // misc01();

    // misc02();

    misc03();

}


fn misc03() {
    let field_name = String::from("Some key");
    let field_value = 42;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map, ``{:?}``", map);

    // println!("field_name, ``{:?}``", field_name); // nope, because the String type ownes the value
    println!("field_value, ``{:?}``", field_value);  // yes, because the Integer type implements the Copy trait
}



// fn misc02() {
//     let teams = vec![
//         String::from("Blue"),
//         String::from("Yellow")
//         ];
//     let initial_scores = vec![10, 50];

//     //  -- hmm; 'book' says "let mut scores...", but cargo check
//     //      ...said mut isn't needed, and it does work without it.
//     let scores: HashMap<_, _> =
//         teams.into_iter().zip( initial_scores.into_iter() ).collect();

//     println!("scores, ``{:?}``", scores);
// }



// fn misc01() {
//     let mut scores = HashMap::new();

//     scores.insert( String::from("Blue"), 10 );
//     scores.insert( String::from("Yellow"), 50 );

//     // scores.insert( String::from("foo"), "bar" );  // nope; "mismatched types" error

//     println!("scores, ``{:?}``", scores);
// }