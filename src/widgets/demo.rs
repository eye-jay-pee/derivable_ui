use super::*;
use eframe::egui::{Color32, Frame, RichText, Stroke, Ui};

impl<'a, T: Editable + 'a> TopLevel<'a, T> {
    fn demo_in_frame<R>(
        ui: &mut Ui,
        title: &str,
        color: Color32,
        widget_to_demonstrate: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        let one_point_five_point_colored_border = Stroke::new(1.5, color);
        let bold_and_colored_title = RichText::new(title).strong().color(color);
        let widget_with_a_heading = |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.heading(bold_and_colored_title);
                widget_to_demonstrate(ui)
            })
            .inner
        };
        Frame::group(ui.style())
            .stroke(one_point_five_point_colored_border)
            .show(ui, widget_with_a_heading)
            .inner
    }

    pub fn demo<'b>(&'b mut self) -> impl FnOnce(&mut Ui) + 'b {
        move |ui: &mut Ui| {
            ui.vertical(|ui| {
                Self::demo_in_frame(ui, "fmt", Color32::LIGHT_BLUE, |ui| {
                    ui.label(self.data.to_string());
                });
                ui.add_space(8.0);

                Self::demo_in_frame(ui, "viewer", Color32::GREEN, |ui| {
                    ui.add(self.data.get_viewer());
                });
                ui.add_space(8.0);

                Self::demo_in_frame(ui, "editor", Color32::RED, |ui| {
                    ui.add(self.data.get_editor());
                });
            });
        }
    }
}
