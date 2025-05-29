mod people;
use people::Person;

mod widgets;
pub use widgets::{Editable, TopLevel, Viewable};

fn main() -> Result<(), eframe::Error> {
    let mut person = Person::new("McLovin", "", 21);
    TopLevel::new(&mut person).launch()
}
