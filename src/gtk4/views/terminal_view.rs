use gtk4::Builder;

#[derive(Clone)]
pub struct TerminalView {
    pub root: gtk4::Box
}

impl TerminalView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/terminal_view.ui");

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in terminal_view.ui");

        Self {
            root
        }
    }
}
