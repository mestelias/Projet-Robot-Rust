// Compile me with `cargo build`
// Execute me with `cargo run`
// ;)

use std::str::FromStr;

// TODO: Remove this and read in the file without
// compile time constants.
/// Maximum size in Y
const X_MAX:isize = 5;
/// Maximum size in X
const Y_MAX:isize = 5;

#[derive(Debug, PartialEq)]
// enumeration Orientation avec les 4 cas possibles
pub enum Orientation {
    Right,
    Left,
    North,
    South,
    Move,
    Nothing,
}

// fonction qui lit un caractères char et renvoie une instruction
pub fn function_instruction(instruction: char) -> Orientation {
    match instruction {
        'R' => Orientation::Right,
        'L' => Orientation::Left,
        'N' => Orientation::North,
        'S' => Orientation::South,
        'F' => Orientation::Move,
        _ => Orientation::Nothing,
    }
}

#[derive(Debug, PartialEq)]
// structure du robot
pub struct Robot {
    pub id: i32,
    pub position_en_x: usize,
    pub position_en_y: usize,
    pub instruction: String,
    pub command: Vec<char>,
}

// création d'autres robots
fn autres_Robots(
    id: i32,
    position_en_x: usize,
    position_en_y: usize,
    instruction: String,
    command: Vec<char>,
) -> Robot {
    Robot {
        id: id,
        position_en_x: position_en_x,
        position_en_y: position_en_y,
        instruction: instruction,
        command: command,
    }
}

//REMIX: On a rajouté les cas de collisions avec les bordures du plateau
// fonction qui gère les cas de collisions avec un autre robot et affiche le plateau,
// en indiquant les coordonnées x et y de la collision
pub fn collisions(
    grid: &Vec<Vec<char>>,
    robot_vecteur: &Vec<Robot>,
    plateau_size: &Vec<usize>,
) {
    for i in 0..robot_vecteur.len() {
        for j in 0..robot_vecteur.len() {
            //on vérifie d'abord que les robots ne sont pas les memes
            if (robot_vecteur[i].id != robot_vecteur[j].id)
                //Puis on vérifie les coordonnées en x et en y
                && (robot_vecteur[i].position_en_x == robot_vecteur[j].position_en_x)
                && (robot_vecteur[i].position_en_y == robot_vecteur[j].position_en_y)
                // Check si on touche les bords 0 0 / X_MAX Y_MAX
            {
                println!(
                    "Robot ID<{}> Collision en ({}, {})",
                    robot_vecteur[i].id,
                    robot_vecteur[i].position_en_x,
                    robot_vecteur[i].position_en_y
                );
            }
        }
    }
    // affiche le plateau
    for i in 0..plateau_size[0] {
        for j in 0..plateau_size[1] {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
    println!("===============");
}


/// Entry point of your rust program.
fn main() -> std::io::Result<()> {
    unimplemented!("Let's code nowwww");
    Ok(()) // don't touch this line it's like return 0 in c ;)
}

//ici se passe tout les tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //test d'instructions
    fn test_function_instruction() {
        assert_eq!(function_instruction('R'), Orientation::Right);
        assert_eq!(function_instruction('L'), Orientation::Left);
        assert_eq!(function_instruction('F'), Orientation::Move);
        assert_eq!(function_instruction('_'), Orientation::Nothing);
    }

    #[test]
    // test de collisions
    fn test_collisions() {
        let mut plateau_size: Vec<usize> = vec![5, 5];
        let mut state = vec![vec!['.'; plateau_size[0]]; plateau_size[1]];
        let mut robot_vecteur: Vec<Robot> = Vec::new();
        let robot1 = Robot {
            id: 1,
            position_en_x: 1,
            position_en_y: 0,
            instruction: "S".to_string(),
            command: vec!['F', 'L', 'L', 'F', 'R', 'F'],
        };
        robot_vecteur.push(robot1);
        collisions(&mut state, &robot_vecteur, &plateau_size);
        assert_eq!(state[0][1]);
    }

    #[test]
    // creation des robots
    fn test_autres_Robots() {
        let a = autres_Robots(1, 'r'.to_string(), 'r', 1, 1, vec!['R', 'L', 'F']);
        let robot_test: Robot = Robot {
            id: 1,
            position_en_x: 1,
            position_en_y: 1,
            orientation: 'r'.to_string(),
            command: vec!['R', 'L', 'F'],
        };
        assert_eq!(a, robot_test);
    }
}
