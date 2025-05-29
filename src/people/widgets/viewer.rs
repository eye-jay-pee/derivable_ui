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
