pub mod demo;
pub use demo::all_demos;

pub mod viewable {
    use eframe::egui::Widget;
    use std::fmt::Display;
    pub trait Viewable: Display {
        type Viewer<'a>: Widget + 'a
        where
            Self: 'a;

        fn get_viewer(&self) -> Self::Viewer<'_>;
    }
}
pub use viewable::Viewable;

pub mod editable {
    use super::Viewable;
    use eframe::egui::Widget;

    pub trait Editable: Viewable + Clone {
        type Editor<'a>: Widget + 'a
        where
            Self: 'a;

        fn get_editor(&mut self) -> Self::Editor<'_>;
    }
}
pub use editable::Editable;
