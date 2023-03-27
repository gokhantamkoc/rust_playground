// A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.

// Example: Animal is defined as a group of method

struct Sheep {
    naked: bool,
    name: &'static str,
}

pub trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &str;
    fn noise(&self) -> &str;

    fn talk(&self) {
        println!("{} {}", self.name(), self.noise());
    }
}

impl Sheep {
    pub fn is_naked(&self) -> bool {
        self.naked
    }

    pub fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked!", self.name());
        } else {
            println!("{} is sheared!", self.name());
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &str {
        self.name
    }

    fn noise(&self) -> &str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn run() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
