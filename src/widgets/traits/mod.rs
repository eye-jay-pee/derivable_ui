pub mod demo {
    use eframe::egui::{Color32, Frame, Response, RichText, Stroke, Ui};
    pub trait Demo<'a, D: ?Sized> {
        type DemoClosure: FnOnce(&mut Ui) -> Response + 'a;

        fn new(data: &'a D) -> Self
        where
            Self: Sized;

        fn get(self) -> Self::DemoClosure;
        fn show(self, ui: &mut Ui) -> Response;
    }
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
}
pub mod none {
    pub mod demo {}
}
pub mod textonly {
    pub mod demo {}
}
pub mod viewable {
    use eframe::egui::Widget;
    pub trait Viewable: std::fmt::Display {
        type Viewer<'a>: Widget + 'a
        where
            Self: 'a;

        fn get_viewer(&self) -> Self::Viewer<'_>;
    }
    pub mod demo {}
}
pub use viewable::Viewable;

pub mod editable {
    use super::Viewable;
    use eframe::egui::Widget;

    pub trait Editable: Viewable + Clone {
        type Editor<'a>: Widget + 'a
        where
            Self: 'a;

        fn get_editor(&mut self) -> Self::Editor<'_>;
    }
    pub mod demo {}
}
pub use editable::Editable;
