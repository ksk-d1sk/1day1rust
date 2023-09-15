use std::ops::Drop;

struct Appellation {
    name: String,
    nicknames: Vec<String>
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);

        if !self.nicknames.is_empty() {
            print!( " (AKA {})", self.nicknames.join(", "));
        }
        println!();
    }
}

fn complicated_condition() -> bool {
    false
}

fn main() {
    {
        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };
    
        println!("before assignment");
        a = Appellation { name: "Hera".to_string(), nicknames: vec![] };
        println!("at end of block");
    }

    let p;
    {
        let q = Appellation {
            name: "Cardamine hirsuta".to_string(),
            nicknames: vec![
                "shotweed".to_string(),
                "bittercress".to_string(),
            ],
        };

        if complicated_condition() {
            p = q;
        }
    }
    println!("sproing! What was that?");
}
