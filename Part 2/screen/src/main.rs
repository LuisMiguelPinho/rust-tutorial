// Attribute to hide warnings for unused code and variables.
// #![allow(dead_code)]
// #![allow(unused_variables)] // could add prefix _ to parameters name instead

trait Draw {
    fn draw(&self);
}
    
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing button {}", self.label);
    }
}
    
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing Select Box with options {:?}", self.options);
   
    }
}


struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
    components: vec![   Box::new(SelectBox {width: 75, height: 10, options: vec![String::from("Yes"), String::from("Maybe"),String::from("No"),],}),
                        Box::new(Button {width: 50, height: 10, label: String::from("OK"),}),
                    ], };


    screen.run();

}
            