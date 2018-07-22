extern crate cursive;

use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use std::process::Command;

fn main() {
    // Creates the cursive root - required for every application
    let mut cursive = Cursive::new();

    // Creates a dialog with a single "Quit" button
    cursive.add_layer(
        Dialog::around(TextView::new("Hello World!"))
            .title("Cursed DF Launcher")
            .button("Launch", |c| {
                launch_df();
                c.quit();
            })
            .button("Quit", |c| c.quit()),
    );

    cursive.run();
}

fn launch_df() {
    Command::new("./df")
        .current_dir("df/df_osx/")
        .spawn()
        .expect("The attempt to launch DF failed.");
}
