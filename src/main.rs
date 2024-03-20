struct Light {
    r: u8,
    g: u8,
    b: u8,
    next: todo!(),
}

impl Light {
    fn new(r: u8, g: u8, b: u8) -> Light {
        Light { r: r, g: g, b: b, next: None }
    }

    fn add_to_chain(&mut self, r: u8, g: u8, b: u8) {
        todo!()
    }
}

fn main() {
    let mut light = Light::new(255, 0, 0);
    light.add_to_chain(0, 255, 0);
    light.add_to_chain(0, 0, 255);
    light.add_to_chain(255, 255, 0);
    light.add_to_chain(0, 255, 255);
    light.add_to_chain(255, 0, 255);

    let mut current_light = &light;
    loop {
        println!("R: {}, G: {}, B: {}", current_light.r, current_light.g, current_light.b);
        // how to go next?
    }
}
