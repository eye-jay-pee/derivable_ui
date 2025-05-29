use super::*;
use crate::widgets::Editable;
use eframe::egui::widgets::{DragValue, TextEdit};

pub struct PersonEditor<'a> {
    pub data: &'a mut Person,
}

impl<'a> Widget for PersonEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("given name");
                ui.add(
                    TextEdit::singleline(&mut self.data.first)
                        .desired_width(f32::INFINITY),
                );
            });
            ui.horizontal(|ui| {
                ui.label("family name");
                ui.add(
                    TextEdit::singleline(&mut self.data.last)
                        .desired_width(f32::INFINITY),
                );
            });
            ui.horizontal(|ui| {
                ui.label("age");
                ui.add(DragValue::new(&mut self.data.age));
            });
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
