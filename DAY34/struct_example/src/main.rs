pub struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    heigth: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { heigth: b.heigth / 2, ..b };
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

struct Bounds(pub usize, pub usize);

struct Ascii(Vec<u8>);

struct Onesuch;

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }
}

fn main() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        heigth: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);

    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.heigth, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.heigth, 30);
    assert_eq!(hokey2.health, 100);

    let image_bounds = Bounds(1024, 768);

    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let o = Onesuch;
}
