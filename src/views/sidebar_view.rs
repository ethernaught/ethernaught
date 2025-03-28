use gtk::Builder;
use gtk::prelude::BuilderExtManual;

#[derive(Clone)]
pub struct SidebarView {
    pub root: gtk::Box
}

impl SidebarView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/sidebar_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in sidebar_view.ui");

        Self {
            root
        }
    }
}
