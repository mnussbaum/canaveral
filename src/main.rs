extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use std::process::Command;

use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Quit,
    HandleQuery(String),
    SelectRow(usize),
}

pub struct Model {
    search_results: Vec<String>,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            search_results: vec![],
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::SelectRow(row_index) => {
                if let Some(search_result) = &self.model.search_results.get(row_index) {
                    Command::new("pass")
                        .arg("show")
                        .arg("-c")
                        .arg(search_result)
                        .output()
                        .expect("failed to execute pass process");
                }
            }
            Msg::HandleQuery(q) => {
                let output = Command::new("pass")
                    .arg("find")
                    .arg(q)
                    .output()
                    .expect("failed to execute pass process");

                for search_result in String::from_utf8_lossy(&output.stdout).lines().into_iter() {
                    let row = gtk::Grid::new();
                    row.set_column_spacing(10);
                    row.set_row_spacing(10);
                    let pass_plz =
                        search_result.replace("├── ", "").replace("└── ", "");
                    let label = gtk::Label::new(&pass_plz as &str);
                    row.add(&label);
                    row.show_all();
                    self.model.search_results.push(pass_plz);
                    self.results_list.add(&row);
                }
            }
        }
    }

    view! {
        gtk::Window {
            property_width_request: 800,
            resizable: false,
            decorated: false,
            position: gtk::WindowPosition::Center,
            #[name = "launcher"]
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                property_height_request: 150,

                #[name = "input_box"]
                gtk::Box {
                    visible: true,
                    can_focus: false,
                    orientation: gtk::Orientation::Horizontal,

                    #[name = "input"]
                    gtk::Entry {
                        visible: true,
                        can_focus: true,
                        property_is_focus: true,
                        property_height_request: 80,
                        property_width_request: 800,
                        changed(entry) => Msg::HandleQuery(entry.get_text().unwrap()),
                    },
                },

                gtk::ScrolledWindow {
                    can_focus: false,
                    vexpand: true,

                    #[name="results_list"]
                    gtk::ListBox {
                        activate_on_single_click: true,
                        row_activated(_, row) => Msg::SelectRow(row.get_index() as usize),
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}

// TODO
// * Make all character keystrokes go to search box
// * Make selection close window and reset it
// * Allow multiple types of queries
// * Implement plugin format
// * Make commands async to input
// * Make it look nice
// * Allow customizable CSS
// * Add a preview mode

fn main() {
    Win::run(()).unwrap();
}
