// First, I will define a structure Character
struct Character {
    name: String,
    house: House,
}

// Next, I will define an enumeration for the different Houses
// Able to take in different types. We will only be using String
// and None (null)
enum House {
    Stark,
    Lannister,
    Targaryen,
    None, // Night King
}

// Implement a method for the Character structure
impl Character {
    fn new(name: &str, house: House) -> Character {
        Character {
            name: name.to_string(),
            house,
        }
    }
// Matching House to String (name of each house)
    fn print(&self) {
        match self.house {
            House::Stark => println!("{} of House Stark", self.name),
            House::Lannister => println!("{} of House Lannister", self.name),
            House::Targaryen => println!("{} of House Targaryen", self.name),
            House::None => println!("{} does not belong to any house", self.name),
        }
    }
}

// Main function
fn main() {
    let jon = Character::new("Jon Snow", House::Stark); // Bastard of the Starks, legend
    let cersei = Character::new("Cersei", House::Lannister); // Teeth clencher, scary woman
    let daenerys = Character::new("Daenerys", House::Targaryen); // Smokin hot Targaryen babe
    let night_king = Character::new("Night King", House::None); //  Leader of the White Walkers

    jon.print();
    cersei.print();
    daenerys.print();
    night_king.print();
}
