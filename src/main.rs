extern crate cairo;
extern crate gtk;

use cairo::{
    ImageSurface,
    Format,
    Context,
};

use gtk::prelude::*;
use gtk::{
    DrawingArea,
    Button,
    Window,
    WindowType,
};

use std::fs::File;

use std::f64::consts::PI;


// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

struct Agent {
    x: f64,
    y: f64,
    r: f64,
}

impl Agent {
    fn draw(&self, context: &Context) {
        context.set_source_rgb(1., 1., 1.);
        context.arc(
            self.x,
            self.y,
            self.r,
            0.,
            2. * PI
        );
        context.set_line_width(3.);
        context.stroke();
    }
}


fn main() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Title");
    window.set_default_size(800, 600);

    // let button = Button::new_with_label("Button");
    // window.add(&button);
    // button.connect_clicked(|_| {
    //     println!("clicked");
    // });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    let area = DrawingArea::new(); 
    window.add(&area);


    let agents = [
        Agent { x: 32., y: 32., r: 16. },
        Agent { x: 16., y: 32., r: 16. },
        Agent { x: 32., y: 16., r: 16. },
    ];
    area.connect_draw(move |_, context| {
        context.set_source_rgb(0., 0., 0.);
        context.paint();

        for agent in agents.iter() {
            agent.draw(&context);
        }

        Inhibit(false)
    });

    gtk::timeout_add(
        100,
        move || {
            println!("."); 
            Continue(true)
        }
    );
    window.show_all();
    gtk::main(); 

/*
    let (width, height) = (512.0, 512.0);
    let surface = ImageSurface::create(
        Format::ARgb32,
        width as i32,
        height as i32
    ).expect("Unable to create ImageSurface");

    let context = Context::new(&surface);

    context.set_source_rgb(1.0, 0.0, 0.0);
    context.paint();

    let mut file = File::create("output.png")
        .expect("Can't create output PNG");

    surface.write_to_png(&mut file)
        .expect("Can't write to output PNG");

*/  

}
