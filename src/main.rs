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
            use eframe::{
                egui::widgets::{DragValue, TextEdit},
                egui::{vec2, Grid},
            };
            pub struct PersonEditor<'a> {
                pub data: &'a mut Person,
            }
            impl<'a> Widget for PersonEditor<'a> {
                fn ui(self, ui: &mut Ui) -> Response {
                    Grid::new(format!("{}_editor_form", self.data.to_string()))
                        .spacing(vec2(8.0, 4.0))
                        .striped(false)
                        .show(ui, |ui| {
                            ui.label("given name");
                            ui.add(
                                TextEdit::singleline(&mut self.data.first)
                                    .min_size(vec2(128.0, 0.0)),
                            );
                            ui.end_row();

                            ui.label("family name");
                            ui.add(
                                TextEdit::singleline(&mut self.data.last)
                                    .min_size(vec2(128.0, 0.0)),
                            );
                            ui.end_row();

                            ui.label("age");
                            ui.add(DragValue::new(&mut self.data.age));
                            ui.end_row();
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

    fn framed_demo<R>(
        ui: &mut eframe::egui::Ui,
        title: &str,
        inner: impl FnOnce(&mut eframe::egui::Ui) -> R,
    ) -> R {
        use eframe::egui::{vec2, Frame, Grid, RichText};

        Frame::group(ui.style())
            .inner_margin(8.0)
            .show(ui, |ui| {
                Grid::new(format!("{}_grid", title))
                    .spacing(vec2(8.0, 4.0))
                    .striped(false)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(title).strong());
                        });
                        ui.end_row();
                        inner(ui)
                    })
                    .inner
            })
            .inner
    }

    impl<'a, T: Editable + 'a> App for TopLevel<'a, T> {
        fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
            CentralPanel::default().show(ctx, |ui| {
                framed_demo(ui, "viewer", |ui| {
                    ui.add(self.data.get_viewer());
                });

                ui.add_space(8.0);

                framed_demo(ui, "editor", |ui| {
                    ui.add(self.data.get_editor());
                });
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
    pub mod helpers {
        use eframe::egui::{Align, InnerResponse, Layout, Ui};
        pub trait UiLayoutExt {
            fn horizontal_right<R>(
                &mut self,
                add_contents: impl FnOnce(&mut Ui) -> R,
            ) -> InnerResponse<R>;
        }

        impl UiLayoutExt for Ui {
            fn horizontal_right<R>(
                &mut self,
                add_contents: impl FnOnce(&mut Ui) -> R,
            ) -> InnerResponse<R> {
                self.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    add_contents(ui)
                })
            }
        }
    }
}
use widgets::TopLevel;
pub use widgets::{helpers::UiLayoutExt, Editable, Viewable};
fn main() -> Result<(), eframe::Error> {
    let mut person = Person::new("McLovin", "", 21);
    TopLevel::new(&mut person).launch()
}
