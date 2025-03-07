use std::process::exit;
use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::*;
use gtk::gio::{resources_register, Resource, SimpleAction};
use gtk::glib::Bytes;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::ui::activity::devices_activity::DevicesActivity;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::bottombar::BottomBar;
use crate::ui::titlebar::TitleBar;
use crate::ui::widgets::hex_editor::HexEditor;
use crate::ui::widgets::terminal::Terminal;
use crate::VERSION;
//use crate::config::VERSION;

#[derive(Clone)]
pub struct OApplication {
    app: Application
}

impl OApplication {

    pub fn new() -> Self {
        let app = Application::new(Some("com.ethernaut.rust"), Default::default());

        Self {
            app
        }
    }

    pub fn run(&self) {
        let _self = self.clone();
        self.app.connect_activate(move |app| {
            HexEditor::static_type();
            Terminal::static_type();

            let resource_data = include_bytes!("../../res/resources.gresources");

            let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
            resources_register(&resource);

            let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/window.ui");//Builder::from_file("res/ui/gtk3/window.ui");

            let provider = CssProvider::new();
            provider.load_from_resource("/com/ethernaut/rust/res/ui/gtk3/window.css");
            //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().expect("Failed to get default screen."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let window: ApplicationWindow = builder
                .object("MainWindow")
                .expect("Failed to get the 'MainWindow' from window.ui");

            window.set_application(Some(app));
            window.connect_destroy(|_| exit(0));
            //window.set_decorated(false);
            window.set_border_width(1);

            window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

            let mut titlebar = TitleBar::new(_self.clone());
            window.set_titlebar(Some(titlebar.on_create()));

            let window_content: gtk::Box = builder
                .object("window_content")
                .expect("Failed to get the 'window_content' from window.ui");

            let stack = Stack::new();
            window_content.add(&stack);
            stack.show();

            _self.start_activity(Box::new(DevicesActivity::new(_self.clone())));

            let mut bottombar = BottomBar::new(_self.clone());
            window_content.add(bottombar.on_create());

            _self.init_actions(&window);

            window.show();
        });

        self.app.run();
    }

    pub fn start_activity(&self, mut activity: Box<dyn Activity>) {
        let stack = self.app.active_window().unwrap().child().unwrap().downcast_ref::<Container>().unwrap().children()[0].clone().downcast::<Stack>().unwrap();

        let name = activity.get_name();
        let title = activity.get_title();
        let root = activity.on_create();
        stack.add_titled(root, &name, &title);
        stack.set_visible_child_name(&activity.get_name());
    }

    /*
    //WE NEED TO POSSIBLY UPDATE BUTTONS AFTER PRESSED...
    pub fn on_back_pressed(&self) {
        let stack = self.app.active_window().unwrap().children()[0].clone().downcast::<Stack>().unwrap();

        let children = stack.children();
        if let Some(current) = stack.visible_child() {
            if let Some(pos) = children.iter().position(|child| child == &current) {
                if pos > 0 {
                    stack.set_visible_child(&children[pos - 1]);
                }
            }
        }
    }

    //WE NEED TO ADJUST STACK AFTER - IE REMOVING SOME AND UPDATE BUTTONS...
    pub fn on_next_pressed(&self) {
        let stack = self.app.active_window().unwrap().children()[0].clone().downcast::<Stack>().unwrap();

        let children = stack.children();
        if let Some(current) = stack.visible_child() {
            if let Some(pos) = children.iter().position(|child| child == &current) {
                if pos + 1 < children.len() {
                    stack.set_visible_child(&children[pos + 1]);
                }
            }
        }
    }
    */

    pub fn get_application(&self) -> Application {
        self.app.clone()
    }

    pub fn get_window(&self) -> Option<Window> {
        self.app.active_window()
    }

    pub fn get_titlebar(&self) -> Option<Widget> {
        self.app.active_window().unwrap().titlebar()
    }

    pub fn get_bottombar(&self) -> Option<Widget> {
        None
    }

    fn init_actions(&self, window: &ApplicationWindow) {
        let action = SimpleAction::new("quit", None);
        let app = self.app.clone();
        action.connect_activate(move |_, _| {
            app.quit();
        });
        window.add_action(&action);

        let action = SimpleAction::new("show-about-dialog", None);
        let window_clone = window.clone();
        action.connect_activate(move |_, _| {
            show_about(&window_clone);
        });
        window.add_action(&action);
    }

    pub fn get_child_by_name(&self, widget: &Widget, name: &str) -> Option<Widget> {
        if widget.widget_name().as_str() == name {
            return Some(widget.clone());
        }

        if let Some(container) = widget.dynamic_cast_ref::<Container>() {
            for child in container.children() {
                if let Some(found) = self.get_child_by_name(&child, name) {
                    return Some(found);
                }
            }
        }

        None
    }
}

pub fn show_about(window: &ApplicationWindow) {
    let svg_data = include_bytes!("../../res/icons/ic_launcher.svg");
    let loader = PixbufLoader::with_type("svg").expect("Failed to create SVG loader");
    loader.write(svg_data).expect("Failed to load SVG data");
    loader.close().expect("Failed to close SVG loader");
    let icon_pixbuf = loader.pixbuf().expect("Failed to get Pixbuf from SVG");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Ethernaut")
        .version(VERSION)
        .authors(vec!["DrBrad"])
        .website_label("https://ethernaut.com")
        .website("https://ethernaut.com")
        .comments("")
        .copyright("Copyright (c) 2024 Ethernaut")
        .license("Copyright (c) 2024 Ethernaut\r\n\r\n\
        \
        Permission is hereby granted, free of charge, to any person obtaining a copy\r\n\
        of this software and associated documentation files (the \"Software\"), to deal\r\n\
        in the Software without restriction, including without limitation the rights\r\n\
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\r\n\
        copies of the Software, and to permit persons to whom the Software is\r\n\
        furnished to do so, subject to the following conditions:\r\n\r\n\
        \
        The above copyright notice and this permission notice shall be included in all\r\n\
        copies or substantial portions of the Software.\r\n\r\n\
        \
        THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\r\n\
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\r\n\
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\r\n\
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\r\n\
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\r\n\
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\r\n\
        SOFTWARE.")
        .logo(&icon_pixbuf)
        .build();

    dialog.present();
}
