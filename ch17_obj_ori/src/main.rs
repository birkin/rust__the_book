
fn main() {

    // -- using Trait objects to allow for values of different types
    // misc01();

    // -- implementing OO-design-pattern 'state'
    // misc02();

    // -- encoding States and Behavior as Types
    // misc03();

    // -- next: "Implementing Transitions as Transformations into Different Types"
    misc04()

}



// -- misc04()

// use ch17_obj_ori::Post;
// use ch17_obj_ori::PendingReviewPost;
use ch17_obj_ori::{ DraftPost, PendingReviewPost, Post };


fn misc04() {

    let mut post: DraftPost = Post::new();
    println!( "post after new(), ``{:?}``", post );

    post.add_text( "I ate a salad for lunch today" );
    println!( "post after add_text(), ``{:?}``", post );

    let post: PendingReviewPost = post.request_review();
    println!( "post after request_review, ``{:?}``", post );
    // println!( "post.content, ``{:?}``", post.content() );  // yields: private field, not a method
    // let zz: () = post;  // yields: expected `()`, found struct `ch17_obj_ori::PendingReviewPost`

    let post: Post = post.approve();
    println!( "post, ``{:?}``", post );
    assert_eq!( "I ate a salad for lunch today", post.content() );
}



// -- misc03()

// use ch17_obj_ori::Post;

// fn misc03() {
//     let mut post = Post::new();

//     post.add_text( "I ate a salad for lunch today" );
//     assert_eq!( "", post.content() );
// }



// -- misc02()

// use ch17_obj_ori::Post;

// fn misc02() {
//     let mut post = Post::new();

//     post.add_text( "I ate a salad for lunch today" );
//     assert_eq!( "", post.content() );

//     post.request_review();
//     assert_eq!( "", post.content() );

//     post.approve();
//     assert_eq!( "I ate a salad for lunch today", post.content() );
//     println!( "post.content, ``{:?}``", post.content() );
// }



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
