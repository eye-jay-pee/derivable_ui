use super::{super::Demo, put_in_frame};
use crate::Viewable;
use eframe::egui::{Color32, Response, Ui};
pub struct ViewDemo<'a, V: Viewable> {
    data: &'a V,
}
impl<'a, V: Viewable> Demo<'a, V> for ViewDemo<'a, V> {
    type DemoClosure = Box<dyn FnOnce(&mut Ui) -> Response + 'a>;

    fn new(data: &'a V) -> Self {
        ViewDemo { data }
    }
    fn get(self) -> Self::DemoClosure {
        Box::new(put_in_frame("get_viewer()", Color32::LIGHT_GREEN, |ui| {
            ui.add(self.data.get_viewer())
        }))
    }
}
