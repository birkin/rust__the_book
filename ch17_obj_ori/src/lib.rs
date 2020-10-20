// -- misc02()

pub struct Post {
    state: Option< Box<dyn State> >,
   content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some( Box::new(Draft {}) ),
            content: String::new(),
        }
    }

    pub fn add_text( &mut self, text: &str ) {
        self.content.push_str( text );
    }

    pub fn content( &self ) {
        ""
    }

    pub fn request_review( &mut self ) {
        if let Some(s) = self.state.take() {
            self.state = Some( s.request_review() )
        }
    }
}


trait State {
    fn request_review( self: Box<Self> ) -> Box<dyn State>;
}


struct Draft{}

impl State for Draft{
    fn request_review( sef: Box<Self> ) -> Box<dyn State> {
        Box::new( PendingReview{} )
    }
}

struct PendingReview{}

impl State for PendingReview {
    fn request_review( self: Box<Self> ) -> Box<dyn State> {
        self
    }
}


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
