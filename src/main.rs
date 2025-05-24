mod people {
    use std::fmt;
    pub struct Person {
        first: String,
        last: String,
        age: i32,
    }
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}, {}", self.first, self.last, self.age)
        }
    }
    impl Person {
        pub fn new(first: &str, last: &str, age: i32) -> Self {
            Person {
                first: String::from(first),
                last: String::from(last),
                age: age,
            }
        }
    }
}
use people::Person;

mod widgets {
    use eframe::{AppCreator, NativeOptions};

    pub struct TopLevel<'a, T: std::fmt::Display> {
        pub data: &'a mut T,
    }

    impl<'a, T: std::fmt::Display + 'a> eframe::App for TopLevel<'a, T> {
        fn update(
            &mut self,
            ctx: &eframe::egui::Context,
            _frame: &mut eframe::Frame,
        ) {
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(self.data.to_string());
            });
        }
    }

    impl<'a, T: std::fmt::Display + 'a> TopLevel<'a, T> {
        pub fn new(data: &'a mut T) -> Self {
            TopLevel { data }
        }

        pub fn launch(self) -> Result<(), eframe::Error> {
            let app_title = std::any::type_name::<T>();
            let app_options = NativeOptions::default();
            let app_creator: AppCreator =
                Box::new(move |_cc| Ok(Box::new(self)));

            eframe::run_native(app_title, app_options, app_creator)
        }
    }
}
use widgets::TopLevel;

fn main() -> Result<(), eframe::Error> {
    TopLevel::new(&mut Person::new("McLovin", "", 21)).launch()
}
