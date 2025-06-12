#![allow(dead_code)]
use crate::{Editable, Viewable};
use eframe::egui::{Color32, Frame, Response, RichText, Stroke, Ui};
use std::fmt::Display;

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

fn null_demo<A: ?Sized>(_data: &A) -> impl FnOnce(&mut Ui) -> Response {
    put_in_frame("NOP", Color32::LIGHT_RED, |ui| ui.label("empty demo"))
}
fn text_demo<D: Display>(data: &D) -> impl FnOnce(&mut Ui) -> Response {
    put_in_frame("to_string()", Color32::LIGHT_RED, |ui| {
        ui.label(data.to_string())
    })
}
fn view_demo<V: Viewable>(data: &V) -> impl FnOnce(&mut Ui) -> Response {
    put_in_frame("get_viewer()", Color32::LIGHT_GREEN, |ui| {
        ui.add(data.get_viewer())
    })
}
fn edit_demo<E: Editable>(data: &mut E) -> impl FnOnce(&mut Ui) -> Response {
    put_in_frame("get_editor()", Color32::LIGHT_GREEN, |ui| {
        ui.add(data.get_editor())
    })
}

pub fn all_demos<E: Editable>(
    data: &mut E,
) -> impl FnOnce(&mut Ui) -> Response {
    put_in_frame("every demo", Color32::WHITE, |ui| {
        let td = text_demo(data)(ui);
        let vd = view_demo(data)(ui);
        let ed = edit_demo(data)(ui);
        td.union(vd).union(ed)
    })
}
