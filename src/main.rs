// Compile me with `cargo build`
// Execute me with `cargo run`
// ;)

use std::str::FromStr;

// TODO: Remove this and read in the file without
// compile time constants.
/// Maximum size in Y
const X_MAX: isize = 5;
/// Maximum size in X
const Y_MAX: isize = 5;

#[derive(Debug, PartialEq)]
// enumeration Orientation avec les 4 cas possibles
pub enum Orientation {
    Right,
    Left,
    North,
    South,
}
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Forward,
    Right,
    Left,
}

// fonction qui lit un caractères char et renvoie une instruction
pub fn parse_orientation(instruction: char) -> Orientation {
    match instruction {
        'R' => Orientation::Right,
        'L' => Orientation::Left,
        'N' => Orientation::North,
        'S' => Orientation::South,
        unknow_char => panic!("Can't handle <{}>", unknow_char),
    }
}

fn parse_instruction(instruction: char) -> Instruction {
    unimplemented!("Maurice faut coder maintenant ! le bouchon toussa.");
}

#[derive(Debug, PartialEq)]
// structure du robot
pub struct Robot {
    pub id: i32,
    pub position_en_x: usize,
    pub position_en_y: usize,
    pub orientation: Orientation,
    pub instruction: Vec<Instruction>,
}

impl Robot {
    fn new(
        id: i32,
        x: usize,
        y: usize,
        orientation: Orientation,
        instruction: Vec<Instruction>,
    ) -> Robot {
        Robot {
            id,
            position_en_x: x,
            position_en_y: y,
            orientation,
            instruction,
        }
    }
}

//REMIX: On a rajouté les cas de collisions avec les bordures du plateau
// fonction qui gère les cas de collisions avec un autre robot et affiche le plateau,
// en indiquant les coordonnées x et y de la collision
pub fn collisions(
    grid: &Vec<Vec<char>>,
    robot_vecteur: &Vec<Robot>,
    plateau_size: &Vec<usize>,
) -> bool {
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
                return true;
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
    false
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
        assert_eq!(parse_instruction('R'), Instruction::Right);
        assert_eq!(parse_instruction('F'), Instruction::Forward);
        assert_eq!(parse_instruction('L'), Instruction::Left);
    }

    #[test]
    //test d'instructions
    fn test_orientation_parsing() {
        assert_eq!(parse_orientation('L'), Orientation::Left);
        assert_eq!(parse_orientation('N'), Orientation::North);
    }

    #[test]
    // test de collisions
    fn test_collisions() {
        let mut plateau_size: Vec<usize> = vec![5, 5];
        let mut state = vec![vec!['.'; plateau_size[0]]; plateau_size[1]];
        let mut robot_vecteur: Vec<Robot> = Vec::new();
        use Instruction as I;
        let robot1 = Robot::new(
            1,
            1,
            0,
            Orientation::South,
            vec![
                I::Forward,
                I::Left,
                I::Left,
                I::Forward,
                I::Right,
                I::Forward,
            ],
        );
        robot_vecteur.push(robot1);
        let collision = collisions(&mut state, &robot_vecteur, &plateau_size);
        assert_eq!(collision);
    }

    #[test]
    // creation des robots
    fn test_autres_Robots() {
        use Instruction as I;
        let a = Robot::new(
            1,
            1,
            1,
            Orientation::South,
            vec![I::Right, I::Left, I::Forward],
        );
        let robot_test = Robot::new(
            1,
            1,
            1,
            Orientation::North,
            vec![I::Right, I::Left, I::Forward],
        );
        assert_eq!(a, robot_test);
    }
}
