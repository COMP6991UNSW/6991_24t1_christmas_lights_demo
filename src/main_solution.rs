struct Light {
    r: u8,
    g: u8,
    b: u8,
    next: Option<Box<Light>>
}

impl Light {
    fn new(r: u8, g: u8, b: u8) -> Light {
        Light { r: r, g: g, b: b, next: None }
    }

    fn add_to_chain(&mut self, r: u8, g: u8, b: u8) {
        let new_light = Light::new(r, g, b);
        match self.next {
            Some(next) => next.add_to_chain(r, g, b),
            None => self.next = Some(Box::new(new_light))
        }
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
        match current_light.next {
            Some(ref next) => current_light = next,
            None => break
        }
    }
}
