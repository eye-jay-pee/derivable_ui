use eframe::{
    egui::{CentralPanel, Context, Widget},
    run_native, App, AppCreator, Error, Frame, NativeOptions,
};
use epaint::Color32;

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

mod demo;

impl<'a, T: Editable + 'a> App for TopLevel<'a, T> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, self.demo());
    }
}

impl<'a, T: Editable + 'a> TopLevel<'a, T> {
    pub fn new(data: &'a mut T) -> Self {
        TopLevel { data }
    }

    pub fn launch(self) -> Result<(), Error> {
        let app_title = std::any::type_name::<T>();
        let app_options = NativeOptions::default();
        let app_creator: AppCreator = Box::new(move |_cc| Ok(Box::new(self)));

        run_native(app_title, app_options, app_creator)
    }
}
