extern crate cursive;

use cursive::event::Key;
use cursive::traits::Boxable;
use cursive::traits::Identifiable;
use cursive::views::DummyView;
use cursive::views::{Button, Dialog, LinearLayout, SelectView};
use cursive::Cursive;
use std::process::Command;

#[derive(Debug)]
enum MenuView {
    Launch,
    Saves,
    Settings,
    Tilesets,
}

fn main() {
    // Creates the cursive root - required for every application
    let mut cursive = Cursive::new();

    cursive.add_global_callback(Key::Esc, |c| c.quit());

    let select = SelectView::<MenuView>::new()
        .item("Launch", MenuView::Launch)
        .item("Saves", MenuView::Saves)
        .item("Settings", MenuView::Settings)
        .item("Tilesets", MenuView::Tilesets)
        .on_submit(update_menu)
        .fixed_size((10, 5));

    let menu = LinearLayout::vertical().with_id("menu");

    cursive.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
            .child(select) // Choose from different "pages" of options
            .child(DummyView) // Spacing
            .child(menu), // The options on the page
        ).title("Cursed DF Launcher"),
    );

    cursive.run();
}

fn launch_df() {
    Command::new("./df")
        .current_dir("df/df_osx/")
        .spawn()
        .expect("The attempt to launch DF failed.");
}

fn update_menu(c: &mut Cursive, view: &MenuView) {
    c.call_on_id("menu", |menu: &mut LinearLayout| {
        match view {
            MenuView::Launch => {
                menu.add_child(Button::new("Launch", |c| {
                    launch_df();
                    c.quit();
                }));
                menu.add_child(Button::new("Quit", Cursive::quit));
            }
            MenuView::Saves => {
                menu.add_child(Button::new("Backup Saves", Cursive::quit));
                menu.add_child(Button::new("Quit", Cursive::quit));
            }
            MenuView::Settings => {
                menu.add_child(Button::new("Launch", |c| {
                    launch_df();
                    c.quit();
                }));
                menu.add_child(Button::new("Quit", Cursive::quit));
            }
            MenuView::Tilesets => {
                menu.add_child(Button::new("Launch", |c| {
                    launch_df();
                    c.quit();
                }));
                menu.add_child(Button::new("Quit", Cursive::quit));
            }
        };
    });
    c.focus_id("menu").expect("Menu didn't take focus");
}
