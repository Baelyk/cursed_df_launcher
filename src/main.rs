extern crate cursive;

use cursive::views::{Dialog, TextView};
use cursive::Cursive;

fn main() {
    // Creates the cursive root - required for every application
    let mut cursive = Cursive::new();

    // Creates a dialog with a single "Quit" button
    cursive.add_layer(
        Dialog::around(TextView::new("Hello World!"))
            .title("Cursed DF Launcher")
            // .button("Launch", |c| c.quit())
            .button("Quit", |c| c.quit()),
    );

    cursive.run();
}
