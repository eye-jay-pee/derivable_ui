use eframe::egui::{Color32, Frame, Response, RichText, Stroke, Ui};
fn put_in_frame(
    title: &str,
    color: Color32,
    widget_to_be_framed: impl FnOnce(&mut Ui) -> Response,
) -> impl FnOnce(&mut Ui) -> Response {
    let one_point_five_point_colored_border = Stroke::new(1.5, color);
    let bold_and_colored_title = RichText::new(title).strong().color(color);

    move |ui| {
        let widget_with_a_heading = |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.heading(bold_and_colored_title);
                widget_to_be_framed(ui)
            })
            .inner
        };
        Frame::group(ui.style())
            .stroke(one_point_five_point_colored_border)
            .show(ui, widget_with_a_heading)
            .inner
    }
}

pub mod edit;
pub mod empty;
pub mod text;
pub mod view;

#[allow(unused_imports)]
pub use {edit::EditDemo, empty::EmptyDemo, text::TextDemo, view::ViewDemo};
