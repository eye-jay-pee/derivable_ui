mod demo;
mod traits;
pub use traits::{Editable, Viewable};

use eframe::{
    egui::{CentralPanel, Context, Key, ViewportCommand},
    run_native, App, AppCreator, Error, Frame, NativeOptions,
};

pub struct TopLevel<'a, T: Editable> {
    pub data: &'a mut T,
    pub dry_run: bool,
}

impl<'a, T: Editable + 'a> App for TopLevel<'a, T> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let cmd_pressed: bool = ctx.input(|i| i.modifiers.command);
        let w_pressed: bool = ctx.input(|i| i.key_pressed(Key::W));
        let cmd_w_pressed: bool = cmd_pressed && w_pressed;

        let app_to_be_closed_upon_startup: bool = self.dry_run;

        if cmd_w_pressed || app_to_be_closed_upon_startup {
            let ctx_clone = ctx.clone();
            std::thread::spawn(move || {
                ctx_clone.send_viewport_cmd(ViewportCommand::Close);
            });
        } else {
            CentralPanel::default().show(ctx, self.demo());
        }
    }
}

impl<'a, T: Editable + 'a> TopLevel<'a, T> {
    pub fn new(data: &'a mut T) -> Self {
        TopLevel {
            data: data,
            dry_run: false,
        }
    }

    pub fn launch(self) -> Result<(), Error> {
        let app_title = std::any::type_name::<T>();
        let app_options = NativeOptions::default();
        let app_creator: AppCreator = Box::new(move |_cc| Ok(Box::new(self)));

        run_native(app_title, app_options, app_creator)
    }
    pub fn perform_dry_run(self) {
        let app_title = std::any::type_name::<T>();
        let _app_options = NativeOptions::default();
        let _app_creator: AppCreator = Box::new(move |_cc| Ok(Box::new(self)));

        println!("dry run of {}", app_title);
    }
}
