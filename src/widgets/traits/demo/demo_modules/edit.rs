use super::{super::Demo, put_in_frame};
use crate::Editable;
use eframe::egui::{Color32, Response, Ui};
pub struct EditDemo<'a, E: Editable> {
    data: &'a E,
}
impl<'a, E: Editable + Clone> Demo<'a, E> for EditDemo<'a, E> {
    type DemoClosure = Box<dyn FnOnce(&mut Ui) -> Response + 'a>;

    fn new(data: &'a E) -> Self {
        EditDemo { data }
    }
    fn get(self) -> Self::DemoClosure {
        let mut data_buffer: E = *Box::new(self.data.clone());

        Box::new(put_in_frame("get_editor()", Color32::WHITE, move |ui| {
            ui.add(data_buffer.get_editor())
        }))
    }
    fn show(self, ui: &mut Ui) -> Response {
        let closure = self.get();
        closure(ui)
    }
}
