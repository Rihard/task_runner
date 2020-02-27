use super::config::Config;
use gtk::prelude::*;
use gdk::{WindowState};
use gtk::{Entry, Box, Orientation, Button, WidgetExt};
use gio::prelude::*;
use std::env;


fn show_input(cfg: Config){
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");

    // EventMask::StructureMask(true);

    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(500, 50);
        win.set_title("Basic example");
        win.set_decorated(false);

        //Containers and borders
        let box_container = Box::new(Orientation::Vertical, 0);
        let child_box_container = Box::new(Orientation::Horizontal, 10);
        box_container.pack_start(&child_box_container, true, true, 10);

        //Input field
        let entry = Entry::new();
        entry.set_activates_default(true);
        entry.connect_activate(|entry| {
            match entry.get_text() {
                Option::Some(txt) => {
                    execute_str(txt.to_string());
                    entry.set_text("")
                },
                Option::None => ()
            }
        });
        child_box_container.pack_start(&entry, true, true, 10);

        //Button
        let entry_clone = entry.clone();
        let button = Button::new();
        button.set_label("Go");
        button.connect_clicked(move |_| {
            match entry_clone.get_text() {
                Option::Some(txt) => {
                    execute_str(txt.to_string());
                    entry.set_text("")
                },
                Option::None => ()
            }
        });
        child_box_container.pack_start(&button, false, false, 10);

        win.add(&box_container);

        //AutoShutdown
        let app_clone = app.clone();
        win.connect_window_state_event(move |_, state| {
            if !state.get_new_window_state().contains(WindowState::FOCUSED) {
                app_clone.quit();
            }
            Inhibit(false)
        });

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn execute_str(request: String){
    println!("Request: {}", request);
}