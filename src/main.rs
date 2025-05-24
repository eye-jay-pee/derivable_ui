mod people {
    use std::fmt;
    pub struct Person {
        first: String,
        last: String,
        age: i32,
    }
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}, {}", self.first, self.last, self.age)
        }
    }
    impl Person {
        pub fn new(first: &str, last: &str, age: i32) -> Self {
            Person {
                first: String::from(first),
                last: String::from(last),
                age: age,
            }
        }
    }
    pub mod widgets {
        use super::Person;
        use eframe::egui::{Response, Ui, Widget};

        pub mod viewer {
            use super::*;
            use crate::widgets::Viewable;
            pub struct PersonViewer<'a> {
                pub data: &'a Person,
            }
            impl<'a> Widget for PersonViewer<'a> {
                fn ui(self, ui: &mut Ui) -> Response {
                    ui.label(self.data.to_string())
                }
            }
            impl Viewable for Person {
                type Viewer<'a> = PersonViewer<'a>;
                fn get_viewer(&self) -> Self::Viewer<'_> {
                    PersonViewer { data: self }
                }
            }
        }

        pub mod editor {
            use super::*;
            use crate::widgets::Editable;
            pub struct PersonEditor<'a> {
                pub data: &'a mut Person,
            }
            impl<'a> Widget for PersonEditor<'a> {
                fn ui(self, ui: &mut Ui) -> Response {
                    ui.vertical(|ui| {
                        ui.label("given name");
                        ui.text_edit_singleline(&mut self.data.first);
                        ui.label("family name");
                        ui.text_edit_singleline(&mut self.data.last);
                        ui.label("age");
                    })
                    .response
                }
            }
            impl Editable for Person {
                type Editor<'a> = PersonEditor<'a>;
                fn get_editor(&mut self) -> Self::Editor<'_> {
                    PersonEditor { data: self }
                }
            }
        }
    }
}
use people::Person;

mod widgets {
    use eframe::{
        egui::{CentralPanel, Context, Widget},
        run_native, App, AppCreator, Error, Frame, NativeOptions,
    };

    pub trait Viewable: std::fmt::Display {
        type Viewer<'a>: Widget + 'a
        where
            Self: 'a;

        fn get_viewer(&self) -> Self::Viewer<'_>;
    }
    pub trait Editable: Viewable {
        type Editor<'a>: Widget + 'a
        where
            Self: 'a;

        fn get_editor(&mut self) -> Self::Editor<'_>;
    }

    pub struct TopLevel<'a, T: Editable> {
        pub data: &'a mut T,
    }

    impl<'a, T: Editable + 'a> App for TopLevel<'a, T> {
        fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
            CentralPanel::default().show(ctx, |ui| {
                ui.add(self.data.get_viewer());
                ui.add(self.data.get_editor());
            });
        }
    }

    impl<'a, T: Editable + 'a> TopLevel<'a, T> {
        pub fn new(data: &'a mut T) -> Self {
            TopLevel { data }
        }

        pub fn launch(self) -> Result<(), Error> {
            let app_title = std::any::type_name::<T>();
            let app_options = NativeOptions::default();
            let app_creator: AppCreator =
                Box::new(move |_cc| Ok(Box::new(self)));

            run_native(app_title, app_options, app_creator)
        }
    }
}
use widgets::TopLevel;
pub use widgets::{Editable, Viewable};
fn main() -> Result<(), eframe::Error> {
    let mut person = Person::new("McLovin", "", 21);
    TopLevel::new(&mut person).launch()
}
