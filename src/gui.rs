use std::sync::Mutex;
use std::rc::Rc;

use gtk;
use error::*;
use gtk::prelude::*;
use gtk::{
    ColorButton,
    Window,
    WindowType,
    ScrolledWindow,
    Label,
    Grid
};

pub struct Gui {
    keys: Rc<Mutex<Vec<[bool; 5]>>>,
}

impl Gui {
    pub fn new() -> Result<Gui> {
        let mut keys = Rc::new(Mutex::new(Vec::new()));

        gtk::init().map_err(|_| Error::from("cannot init GTK"))?;

        let window = Window::new(WindowType::Toplevel);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);
        let scrolled_window = ScrolledWindow::new(None, None);

        let grid = Grid::new();
        grid.attach(&Label::new(Some("Frame")), 0, 0, 1, 1);
        grid.attach(&Label::new(Some("↑")), 1, 0, 1, 1);
        grid.attach(&Label::new(Some("↓")), 2, 0, 1, 1);
        grid.attach(&Label::new(Some("←")), 3, 0, 1, 1);
        grid.attach(&Label::new(Some("→")), 4, 0, 1, 1);
        grid.attach(&Label::new(Some("踊")), 5, 0, 1, 1);
        for i in 1..10 {
            keys.lock().unwrap().push([false, false, false, false, false]);
            grid.attach(&Label::new(Some(&format!("{}", i))), 0, i, 1, 1);
            for j in 1..6 {
                let button = ColorButton::new();
                let keys = keys.clone();
                button.connect_clicked(move |self| {
                    let lock = keys.lock().unwrap();
                    let mut enabled = &lock[i as usize-1][j as usize - 1];
                    *enabled ^= *enabled;
                    if enabled {
                        self.set_rgba(RGBA { red: 0., green: 1., blue: 0., alpha: 0. });
                    } else {
                        self.set_rgba(RGBA { red: 1., green: 1., blue: 1., alpha: 0. });
                    }
                    println!("{:?}", *keys.lock().unwrap());
                });
                grid.attach(&button, j, i, 1, 1);
            }
        }
        scrolled_window.add(&grid);

        window.add(&scrolled_window);
        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        Ok(Gui {
            keys: keys,
        })
    }

    pub fn main(&self) {
        gtk::main()
    }
}
