use super::{super::Demo, put_in_frame};
use eframe::egui::{Color32, Response, Ui};

pub struct EmptyDemo<'a, T: ?Sized> {
    data: &'a T,
}
impl<'a, T: ?Sized> Demo<'a, T> for EmptyDemo<'a, T> {
    type DemoClosure = Box<dyn FnOnce(&mut Ui) -> Response + 'a>;

    fn new(data: &'a T) -> Self {
        EmptyDemo { data }
    }
    fn get(self) -> Self::DemoClosure {
        Box::new(put_in_frame("NOP", Color32::LIGHT_RED, |ui| {
            ui.label("empty demo")
        }))
    }
}
