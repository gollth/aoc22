use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Result};
use arr_macro::arr;
use enum_iterator::{all, Sequence};
use itertools::Itertools;
use ndarray::{s, Array2};
use termion::color::{Fg, Reset, Rgb};

use crate::{coord, Cell, Coord, Direction, Rotation, State, Wrappable};

#[derive(Debug, PartialEq, Eq, Sequence)]
enum CubeNet {
    ///  0
    ///  1
    /// 253
    ///   4
    // UpsideDownTWithDash,

    ///   2
    /// 015
    ///   34
    SidewaysTAndL,
    // There are 10 more cube nets (excluding rotated ones) which are not yet implemented
}

impl CubeNet {
    fn starting_face(&self) -> usize {
        match self {
            CubeNet::SidewaysTAndL => 2,
        }
    }
    /// Generate a list of the origins (top left corner) of each face of this cubemap
    fn face_origins(&self, a: usize) -> [[usize; 2]; 6] {
        match self {
            // CubeNet::UpsideDownTWithDash => [
            //     [0, a],
            //     [a, a],
            //     [2 * a, 0],
            //     [2 * a, 2 * a],
            //     [3 * a, 2 * a],
            //     [2 * a, a],
            // ],
            CubeNet::SidewaysTAndL => [
                [a, 0],
                [a, a],
                [0, 2 * a],
                [2 * a, 2 * a],
                [2 * a, 3 * a],
                [a, 2 * a],
            ],
        }
    }

    fn neighbor(&self, face: usize, dir: &Direction) -> (usize, Rotation) {
        use Direction::*;
        match self {
            CubeNet::SidewaysTAndL => match (face, *dir) {
                (0, Right) => (1, Rotation::None),
                (0, Left) => (4, Rotation::Clockwise),
                (0, Up) => (2, Rotation::Clockwise),
                (0, Down) => (3, Rotation::CounterClockwise),

                (1, Right) => (5, Rotation::None),
                (1, Left) => (1, Rotation::None),
                (1, Up) => (2, Rotation::Clockwise),
                (1, Down) => (4, Rotation::CounterClockwise),

                (2, Right) => (4, Rotation::OneEighty),
                (2, Left) => (1, Rotation::CounterClockwise),
                (2, Up) => (0, Rotation::OneEighty),
                (2, Down) => (5, Rotation::None),

                (3, Right) => (4, Rotation::None),
                (3, Left) => (1, Rotation::Clockwise),
                (3, Up) => (5, Rotation::None),
                (3, Down) => (0, Rotation::OneEighty),

                (4, Right) => (2, Rotation::OneEighty),
                (4, Left) => (3, Rotation::None),
                (4, Up) => (5, Rotation::CounterClockwise),
                (4, Down) => (0, Rotation::CounterClockwise),

                (5, Right) => (4, Rotation::Clockwise),
                (5, Left) => (1, Rotation::None),
                (5, Up) => (2, Rotation::None),
                (5, Down) => (3, Rotation::None),

                (x, _) => panic!("Cubes only have 6 faces, index {} not supported", x),
            },
        }
    }

    fn from(sidelength: usize, map: &Array2<Cell>) -> Option<Self> {
        all::<Self>()
            .filter(|net| {
                net.face_origins(sidelength)
                    .iter()
                    .all(|x| map[*x] != Cell::Void)
            })
            .next()
    }

    fn faces(&self, sidelength: usize, map: &Array2<Cell>) -> [Array2<Cell>; 6] {
        let origins = self.face_origins(sidelength);

        let mut faces = arr![Array2::from_elem((sidelength, sidelength), Cell::Void); 6];
        for i in 0..6 {
            let [a, b] = origins[i];
            let side = map.slice(s![a..(a + sidelength), b..(b + sidelength)]);
            faces[i] = side.to_owned();
        }
        faces
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    path: Vec<State>,
    sidelength: i32,
    faces: [Array2<Cell>; 6],
    current_face: usize,
    starting: Coord,
    rotation: Rotation,
    net: CubeNet,
    origins: [[usize; 2]; 6],
}

impl Cube {
    // fn origin(&self) -> Coord {
    //     let o = self.origins[self.current_face];
    //     coord(o[0] as i32, o[1] as i32)
    // }

    fn face(&self) -> &Array2<Cell> {
        &self.faces[self.current_face]
    }

    fn cell(&self, coord: Coord) -> Option<Cell> {
        if coord.x < 0 || coord.x >= self.sidelength || coord.y < 0 || coord.y >= self.sidelength {
            None
        } else {
            Some(self.face()[[coord.y as usize, coord.x as usize]])
        }
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grey = Fg(Rgb(100, 100, 100));
        write!(f, "{}┼", grey)?;
        for _ in 0..self.sidelength {
            write!(f, "─")?;
        }
        writeln!(f, "┼")?;

        for y in 0..self.sidelength {
            write!(f, "│{}", Fg(Reset))?;
            for x in 0..self.sidelength {
                // let [oy, ox] = self.origins[self.current_face];
                let c = self.cell(coord(x, y)).unwrap_or(Cell::Void);
                write!(f, "{}", c)?;
            }
            writeln!(f, "{}│", grey)?;
        }

        write!(f, "┼")?;
        for _ in 0..self.sidelength {
            write!(f, "─")?;
        }
        writeln!(f, "┼{}", Fg(Reset))?;
        Ok(())
    }
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let a = lines
            .iter()
            .map(|line| line.len())
            .max()
            .ok_or(anyhow!("Empty input"))?;
        let b = lines.len();
        let mut array = Array2::from_elem((b, a), Cell::Void);
        for l in 0..b {
            for c in 0..a {
                array[[l, c]] = Cell::from(lines[l].chars().nth(c).unwrap_or(' '));
            }
        }
        // if b > a {
        //     array = array.reversed_axes();
        // }
        println!("{}", array);

        let sidelength = array.dim().0.max(array.dim().1) / 4;
        let net = CubeNet::from(sidelength, &array).ok_or(anyhow!("Unknown cube net"))?;
        println!("Detected cube net: {:?}", net);
        let faces = net.faces(sidelength, &array);

        // face of starting position
        let current_face = net.starting_face();

        let starting = coord(0, 0);

        Ok(Self {
            sidelength: sidelength as i32,
            faces,
            current_face,
            starting,
            origins: net.face_origins(sidelength),
            net,
            rotation: Rotation::None,
            path: vec![State::new(starting, Direction::Right)],
        })
    }
}

impl Wrappable for Cube {
    fn tip(&self) -> State {
        self.path.last().expect("empty path").clone()
    }

    fn tip_mut(&mut self) -> &mut State {
        self.path.last_mut().expect("empty path")
    }

    fn advance(&mut self) -> bool {
        use Direction::*;

        println!("{}", self);
        let state = self.tip();
        let n = state.coord.clone() + Coord::from(state.dir);

        println!(
            "Looking for coord {:?} + {:?} * {:?} = {:?}",
            state.coord, state.dir, self.rotation, n
        );
        if let Some(c) = self.cell(n) {
            // neighbor still on this face
            if c == Cell::Free {
                self.path.push(State::new(n, state.dir));
            }
            return c == Cell::Free;
        }

        // Neighbor on other face
        let (new_face, new_dir) = self.net.neighbor(self.current_face, &state.dir);
        println!(
            "Neighbor {:?} is not anymore on face {} but on {:?}",
            n,
            self.current_face + 1,
            new_face + 1
        );

        self.current_face = new_face;
        let a = self.sidelength as i32 - 1;
        let tip = self.tip().clone();
        // tip.dir = new_dir;
        use Rotation::*;
        let tip = match (state.dir, new_dir) {
            (_, None) => tip,

            // Clockwise
            (Left, Clockwise) => State::new(Coord::new(tip.coord.y, 0), Down),
            (Down, Clockwise) => State::new(Coord::new(0, a - tip.coord.x), Right),
            (Right, Clockwise) => State::new(Coord::new(tip.coord.y, a), Up),
            (Up, Clockwise) => State::new(Coord::new(a, a - tip.coord.x), Left),

            // Counter-Clockwise
            (Right, CounterClockwise) => State::new(Coord::new(a - tip.coord.y, 0), Up),
            (Up, CounterClockwise) => State::new(Coord::new(0, tip.coord.x), Left),
            (Left, CounterClockwise) => State::new(Coord::new(a - tip.coord.y, a), Down),
            (Down, CounterClockwise) => State::new(Coord::new(a, tip.coord.x), Left),

            // 180 degree rotation
            (Left, OneEighty) => State::new(),
        };

        let c = self.cell(tip.coord).unwrap();
        if c == Cell::Free {
            self.path.push(tip);
        }
        return c == Cell::Free;
    }

    fn turn_left(&mut self) {
        todo!()
    }

    fn turn_right(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use itertools::Itertools;

    use crate::{coord, Direction::*};

    use super::*;

    fn cube_from_sample() -> Result<Cube> {
        let sample = std::fs::read_to_string("sample.txt")?;
        let (a, _) = sample
            .split_terminator("\n\n")
            .collect_tuple()
            .ok_or(anyhow!("no empty line detected"))?;
        Ok(Cube::from_str(a)?)
    }
    #[test]
    fn sample_from_str() -> Result<()> {
        let cube = cube_from_sample();
        assert!(cube.is_ok(), "{:?}", cube);
        Ok(())
    }

    #[test]
    fn sample_starting_position() -> Result<()> {
        let cube = cube_from_sample()?;
        assert_eq!(cube.starting, coord(0, 0), "{}", cube);
        assert_eq!(cube.current_face, 2, "{}", cube);
        Ok(())
    }

    #[test]
    fn sample_advance_from_starting_position() -> Result<()> {
        let mut cube = cube_from_sample()?;
        *cube.tip_mut() = State::new(cube.starting, Right);

        cube.advance();

        assert_eq!(
            cube.tip(),
            State::new(cube.starting + coord(1, 0), Right),
            "{}",
            cube
        );
        Ok(())
    }

    #[test]
    fn sample_advance_doesnt_move_into_walls() -> Result<()> {
        let mut cube = cube_from_sample()?;
        *cube.tip_mut() = State::new(cube.starting + coord(2, 0), Right);

        cube.advance();

        assert_eq!(
            cube.tip(),
            State::new(cube.starting + coord(2, 0), Right),
            "{}",
            cube
        );
        Ok(())
    }

    #[test]
    fn sample_neighbor_wrapping_from_a_to_b() -> Result<()> {
        let mut cube = cube_from_sample()?;
        // A on face 6
        cube.current_face = 5;
        *cube.tip_mut() = State::new(coord(3, 1), Right);

        cube.advance();

        // B on face 5
        assert_eq!(cube.current_face, 4);
        assert_eq!(cube.tip(), State::new(coord(2, 0), Down), "{}", cube);

        cube.advance();
        assert_eq!(cube.current_face, 4);
        assert_eq!(cube.tip(), State::new(coord(2, 1), Down), "{}", cube);
        Ok(())
    }

    #[test]
    fn sample_neighbor_wrapping_from_c_to_d() -> Result<()> {
        let mut cube = cube_from_sample()?;

        // C on face 4
        cube.current_face = 3;
        *cube.tip_mut() = State::new(coord(2, 3), Down);

        cube.advance();

        // D on face 1
        assert_eq!(cube.current_face, 0);
        assert_eq!(cube.tip(), State::new(coord(1, 3), Up), "{}", cube);
        Ok(())
    }
}
