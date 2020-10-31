// -- misc04()

// --------------------
// Post
// --------------------

#[derive(Debug)]
pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content( &self ) -> &str {
        &self.content
    }
}

// --------------------
// DraftPost
// --------------------

#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text( &mut self, text: &str ) {
        self.content.push_str( text );
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// --------------------
// PendingReviewPost
// --------------------

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}



// -- misc03()
// -- NOTE: this doesn't work, yielding:

// 26 |     assert_eq!("", post.content());
//    |                         ^^^^^^^ private field, not a method

// -- if, though, I make lib.rs the full content of 'Listing 17-20' (minus the main{}) -- it cargo-checks fine.


// pub struct Post {
//     content: String,
// }

// pub struct DraftPost {
//     content: String,
// }

// impl Post {
//     pub fn new() -> DraftPost {
//         DraftPost {
//             content: String::new(),
//         }
//     }

//     pub fn content( &self ) -> &str {
//         &self.content
//     }
// }

// impl DraftPost {
//     pub fn add_text( &mut self, text: &str ) {
//         self.content.push_str( text );
//     }
// }



// -- misc02()

// pub struct Post {
//     state: Option< Box<dyn State> >,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some( Box::new(Draft {}) ),
//             content: String::new(),
//         }
//     }

//     pub fn add_text( &mut self, text: &str ) {
//         self.content.push_str( text );
//     }

//     pub fn content( &self ) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review( &mut self ) {
//         if let Some(s) = self.state.take() {
//             self.state = Some( s.request_review() )
//         }
//     }

//     pub fn approve( &mut self ) {
//         if let Some(s) = self.state.take() {
//             self.state = Some( s.approve() )
//         }
//     }
// }


// trait State {
//     fn request_review( self: Box<Self> ) -> Box<dyn State>;
//     fn approve( self: Box<Self> ) -> Box<dyn State>;
//     fn content<'a>( &self, post: &'a Post ) -> &'a str {
//         ""
//     }
// }


// struct Draft{}

// impl State for Draft{
//     fn request_review( self: Box<Self> ) -> Box<dyn State> {
//         Box::new( PendingReview{} )
//     }

//     fn approve( self: Box<Self> ) -> Box<dyn State> {
//         self
//     }
// }


// struct PendingReview{}

// impl State for PendingReview {
//     fn request_review( self: Box<Self> ) -> Box<dyn State> {
//         self
//     }

//     fn approve( self: Box<Self> ) -> Box<dyn State> {
//         Box::new( Published {} )
//     }
// }


// struct Published {}

// impl State for Published {
//     fn request_review( self: Box<Self> ) -> Box<dyn State> {
//         self
//     }

//     fn approve( self: Box<Self> ) -> Box<dyn State> {
//         self
//     }

//     fn content<'a>( &self, post: &'a Post ) -> &'a str {
//         &post.content
//     }
// }



// -- misc01()


// pub trait Draw {
//     fn draw( &self );
// }


// pub struct Screen {
//     pub components: Vec< Box<dyn Draw> >,
// }

// impl Screen {
//     pub fn run( &self ) {
//         for component in self.components.iter() {
//             println!("component about to draw");
//             component.draw();
//         }
//     }
// }


// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw( &self ) {
//         // code to actually draw a button
//     }
// }


// pub struct SelectBox {
//     pub width: u32,
//     pub height: u32,
//     pub options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw( &self ) {
//         // code to actually draw a select-box
//     }
// }
