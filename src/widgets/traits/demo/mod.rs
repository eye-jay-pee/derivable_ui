//use crate::{Editable, Viewable};
//use std::fmt::Display;

//pub use demo_modules::{EditDemo, EmptyDemo, TextDemo, ViewDemo};

//pub DemoEngine<'a, S: ?Sized, D: Display, V: Viewable, E: Editable> {
//    Empty(EmptyDemo<'a, S>),
//    TextOnly(TextDemo<'a, D>),
//    GraphicalReadonly(ViewDemo<'a, V>),
//    FullGraphicalEditor(EditDemo<'a, E>),
//}
//impl<'a, S: ?Sized

// impl<'a, S: ?Sized, D: Display, V: Viewable, E: Editable>
//     DemoEngine<'a, S, D, V, E>
// {
//     fn new_empty(data: &'a S) -> Self {
//         DemoEngine::Empty { data }
//     }
//     fn new_text_only(data: &'a D) -> Self {
//         DemoEngine::TextOnly { data }
//     }
//     fn new_viewable(data: &'a V) -> Self {
//         DemoEngine::GraphicalReadonly { data }
//     }
//     fn new_editable(data: &'a E) -> Self {
//         DemoEngine::FullGraphicalEditor { data }
//     }
//     fn get(self) -> Box<dyn FnOnce(&mut Ui) -> Response + 'a> {
//         match self {
//             DemoEngine::Empty(e) => e.get(),
//             DemoEngine::TextOnly(t) => t.get(),
//             DemoEngine::GraphicalReadonly(r) => r.get(),
//             DemoEngine::FullGraphicalEditor(e) => e.get(),
//         }
//     }
// }
