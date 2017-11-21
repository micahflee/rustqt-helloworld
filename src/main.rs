extern crate qt_widgets;
extern crate qt_core;

use qt_widgets::application::Application;
use qt_widgets::label::Label;
use qt_core::string::String;


fn main() {
    Application::create_and_exit(|_app| {
        let mut label = Label::new(&String::from("hello world"));
        label.set_margin(10);
        label.show();

        Application::exec()
    })
}
