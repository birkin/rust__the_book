
fn main() {

    // -- using Trait objects to allow for values of different types
    // misc01();

    // -- implementing OO-design-pattern 'state'
    misc02();

}



// -- misc02()

use ch17_obj_ori::Post;

fn misc02() {
    let mut post = Post::new();

    post.add_text( "I ate a salad for lunch today" );
    assert_eq!( "", post.content() );

    post.request_review();
    assert_eq!( "", post.content() );

    post.approve();
    assert_eq!( "I ate a salad for lunch today", post.content() );
}



// -- misc01()

// use ch17_obj_ori::{Button, Screen, SelectBox};

// fn misc01() {
//     println!("starting misc01...");
//     let screen = Screen {
//         components: vec![
//             Box::new( SelectBox{
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from( "Yes" ),
//                     String::from( "Maybe" ),
//                     String::from( "No" ),
//                 ],
//             } ),
//             Box::new( Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from( "Ok" ),
//             } ),
//         ],  // end components: ...
//     };  // end let screen ...
//     screen.run();
// }
