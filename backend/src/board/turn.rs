use std::cmp::Ordering;

use crate::board::position::Position;

#[derive(Debug, Clone)]
pub struct Move {
    pub starting_position: Position,
    pub end_position: Position,
    pub moves_counter: usize,
    pub kills: Vec<Position>,
}

impl Move {
    fn step<P, O>(&mut self, position: P, to_kill: O)
    where
        P: Into<Position>,
        O: Into<Option<P>>,
    {
        self.end_position = position.into();
        self.moves_counter += 1;
        if let Some(killed) = to_kill.into() {
            self.kills.push(killed.into());
        }
    }

    /// Merges the other into the current one
    pub fn merge(&mut self, mut other: Self) {
        self.end_position = other.end_position;
        self.moves_counter += other.moves_counter;
        self.kills.append(&mut other.kills);
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.moves_counter.cmp(&other.moves_counter)
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.moves_counter == other.moves_counter
    }
}

impl Eq for Move {}

impl<P> From<P> for Move
where
    P: Into<Position>,
{
    fn from(pos: P) -> Self {
        let pos = pos.into();
        Self {
            starting_position: pos,
            end_position: pos,
            moves_counter: 0,
            kills: Vec::new(),
        }
    }
}

#[test]
fn test_creation_position() {
    let turn = Move::from(Position((3, 5)));
    assert_eq!(turn.moves_counter, 0);
    assert_eq!(turn.kills, Vec::<Position>::new());
    assert_eq!(turn.end_position, Position((3, 5)));
    assert_eq!(turn.starting_position, Position((3, 5)));
}

#[test]
fn test_creation_usize_tuple() {
    let turn = Move::from((3, 5));
    assert_eq!(turn.moves_counter, 0);
    assert_eq!(turn.kills, Vec::<Position>::new());
    assert_eq!(turn.end_position, Position((3, 5)));
    assert_eq!(turn.starting_position, Position((3, 5)));
}

#[test]
fn test_step_position() {
    let mut turn = Move::from(Position((3, 6)));
    turn.step(Position((5, 4)), Some(Position((4, 5))));

    assert_eq!(turn.moves_counter, 1);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((5, 4)));
    assert_eq!(turn.starting_position, Position((3, 6)));

    turn.step(Position((4, 3)), None);

    assert_eq!(turn.moves_counter, 2);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((4, 3)));
    assert_eq!(turn.starting_position, Position((3, 6)));
}

#[test]
fn test_step_usize_tuple() {
    let mut turn = Move::from((3, 6));
    turn.step((5, 4), Some((4, 5)));

    assert_eq!(turn.moves_counter, 1);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((5, 4)));
    assert_eq!(turn.starting_position, Position((3, 6)));

    turn.step((4, 3), None);

    assert_eq!(turn.moves_counter, 2);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((4, 3)));
    assert_eq!(turn.starting_position, Position((3, 6)));
}

#[test]
fn test_step_position_into() {
    let mut turn = Move::from(Position((3, 6)));
    turn.step((5, 4), Position((4, 5)));

    assert_eq!(turn.moves_counter, 1);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((5, 4)));
    assert_eq!(turn.starting_position, Position((3, 6)));

    turn.step((4, 3), None);

    assert_eq!(turn.moves_counter, 2);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((4, 3)));
    assert_eq!(turn.starting_position, Position((3, 6)));
}

#[test]
fn test_step_usize_tuple_into() {
    let mut turn = Move::from((3, 6));
    turn.step((5, 4), (4, 5));

    assert_eq!(turn.moves_counter, 1);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((5, 4)));
    assert_eq!(turn.starting_position, Position((3, 6)));

    turn.step((4, 3), None);

    assert_eq!(turn.moves_counter, 2);
    assert_eq!(turn.kills, vec![Position((4, 5))]);
    assert_eq!(turn.end_position, Position((4, 3)));
    assert_eq!(turn.starting_position, Position((3, 6)));
}
