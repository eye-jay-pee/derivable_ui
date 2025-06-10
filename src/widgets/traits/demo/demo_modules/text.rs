use super::{super::Demo, put_in_frame};
use eframe::egui::{Color32, Response, Ui};
use std::fmt::Display;
pub struct TextDemo<'a, D: Display> {
    data: &'a D,
}
impl<'a, D: Display> Demo<'a, D> for TextDemo<'a, D> {
    type DemoClosure = Box<dyn FnOnce(&mut Ui) -> Response + 'a>;

    fn new(data: &'a D) -> Self {
        TextDemo { data }
    }
    fn get(self) -> Self::DemoClosure {
        Box::new(put_in_frame("to_string()", Color32::LIGHT_GREEN, |ui| {
            ui.label(self.data.to_string())
        }))
    }
    fn show(self, ui: &mut Ui) -> Response {
        let closure = self.get();
        closure(ui)
    }
}
