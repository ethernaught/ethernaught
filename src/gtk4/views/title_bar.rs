use gtk4::{gio, ApplicationWindow, Builder, Button, HeaderBar, Image, Label, PackType, PopoverMenuBar, Widget, WindowControls};
use gtk4::gio::SimpleAction;
use gtk4::prelude::{ActionMapExt, BoxExt, GtkWindowExt, NativeExt, ObjectExt, WidgetExt};

#[derive(Clone)]
pub struct TitleBar {
    //pub header_bar: HeaderBar,
    pub root: gtk4::Box,
    pub back: Button,
    pub next: Button,
    pub network_type_icon: Image,
    pub network_type_label: Label,
    pub app_options: gtk4::Box,
    pub start: Button,
    pub stop: Button
}

impl TitleBar {

    pub fn new(window: &ApplicationWindow) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/title_bar.ui");

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in title_bar.ui");

        /*
        let header_bar = HeaderBar::new();
        header_bar.set_height_request(40);
        #[cfg(target_os = "macos")]
        header_bar.set_property("use-native-controls", &true);
        header_bar.set_title_widget(Some(&root));
        //header_bar.set_show_title_buttons(false);
        header_bar.show();*/


        let handle = gtk4::WindowHandle::new();
        handle.set_child(Some(&root));

        let window_controls = WindowControls::new(PackType::Start);
        #[cfg(target_os = "macos")]
        window_controls.set_property("use-native-controls", &true);
        window_controls.set_vexpand(true);
        root.insert_child_after(&window_controls, None::<&Widget>);
        //root.append(&window_controls);
        //handle.set_child(Some(&window_controls));



        #[cfg(any(target_os = "linux", target_os = "windows"))]
        {
            let menubar: PopoverMenuBar = builder
                .object("menubar")
                .expect("Couldn't find 'menubar' in title_bar.ui");

            let navigation_buttons: gtk4::Box = builder
                .object("navigation_buttons")
                .expect("Couldn't find 'navigation_buttons' in ethernaught_ui.xml");

            //.connect_deactivate
            menubar.connect_cursor_notify({
                let navigation_menubar = menubar.clone();
                let navigation_buttons = navigation_buttons.clone();
                move |_| {
                    navigation_menubar.hide();
                    navigation_buttons.show();
                }
            });

            let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
            let model: gio::MenuModel = builder
                .object("main_window_menu")
                .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");
            menubar.set_menu_model(Some(&model));
            menubar.show();
            menubar.hide();

            let action = SimpleAction::new("menu", None);
            action.connect_activate({
                let navigation_buttons = navigation_buttons.clone();
                move |_, _| {
                    navigation_buttons.hide();
                    menubar.show();
                    //menubar.select_first(true);
                }
            });
            window.add_action(&action);
        }

        let back: Button = builder
            .object("back")
            .expect("Couldn't find 'back' in ethernaught_ui.xml");

        let next: Button = builder
            .object("next")
            .expect("Couldn't find 'next' in ethernaught_ui.xml");

        let network_type_icon: Image = builder
            .object("network_type_icon")
            .expect("Couldn't find 'network_type_icon' in title_bar.ui");

        let network_type_label: Label = builder
            .object("network_type_label")
            .expect("Couldn't find 'network_type_label' in title_bar.ui");

        let app_options: gtk4::Box = builder
            .object("app_options")
            .expect("Couldn't find 'app_options' in title_bar.ui");

        let start: Button = builder
            .object("start")
            .expect("Couldn't find 'start' in title_bar.ui");

        let stop: Button = builder
            .object("stop")
            .expect("Couldn't find 'stop' in title_bar.ui");

        window.set_titlebar(Some(&handle));

        Self {
            //header_bar,
            root,
            back,
            next,
            network_type_icon,
            network_type_label,
            app_options,
            start,
            stop
        }
    }
}
