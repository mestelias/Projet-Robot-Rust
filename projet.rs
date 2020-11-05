#[derive(Debug)]
enum Orientation {
    Right,
    Left,
    Move,
    Nothing
}

enum Instruction {
    let player1 : Orientation;

    match player1 {
        Orientation : R => println!("Rotation vers la droit !"),
        Orientation : L => println!("Rotation vers la gauche !"),
        Orientation : F => println!("Avancer dans le sens de l'orientation !"),
        Orientation : Nothing => println!("Ne rien faire !"),
    }
}

struct Robot {
    name : String,
    age : u32,
    orientation : Orientation,
}

fn main() {
    let robot1 = Robot {
        name : String::from("robot1"),
        age : u32,
        orientation : Orientation,
    };

    let robot2 = Robot {
        name : String::from("robot2"),
        age : u32,
        orientation : Orientation,
    };
}