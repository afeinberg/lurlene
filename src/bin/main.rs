use lurlene::Index;

#[derive(Debug, Clone)]
struct DemoApp {
    index: Index,
}

impl Default for DemoApp {
    fn default() -> Self {
        DemoApp {
            index: Index::default(),
        }
    }
}

impl DemoApp {
    pub fn add_entries(&mut self) {
        self.index.add("d1", "t1 t2 t3 t4");
        self.index.add("d2", "t2 t3");
        self.index.add("d3", "t3 t4");
        self.index.add("d4", "t4");
        self.index.add("d5", "t1 t3");
    }

    pub fn dump_index(&self) {
        println!("self: {:?}", self)
    }
}

fn main() {
    let mut app = DemoApp::default();
    app.add_entries();
    app.dump_index();
}
