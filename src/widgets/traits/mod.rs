#![allow(dead_code)]
use eframe::egui::Widget;

pub mod demo;

pub trait Viewable: std::fmt::Display {
    type Viewer<'a>: Widget + 'a
    where
        Self: 'a;

    fn get_viewer(&self) -> Self::Viewer<'_>;
}
pub trait Editable: Viewable + Clone {
    type Editor<'a>: Widget + 'a
    where
        Self: 'a;

    fn get_editor(&mut self) -> Self::Editor<'_>;
}
