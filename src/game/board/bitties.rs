use crate::{Action, ActionPath, Board, Player, Qmvs, game::model::bits::Bits};

use super::scale::Scale;

struct Bitty {
    /// The current bit(s) that would be playing (for which we should get moves)
    current: u32,
    /// All other team member's excluding the bits in current
    team: u32,
    opponent: u32,
    kings: u32,
}

impl Bitty {
    pub(crate) const BOTTOM: u32 = 0x0000000F;
    pub(crate) const TOP: u32 = 0xF0000000;

    pub(crate) fn new(current: u32, team: u32, opponent: u32, kings: u32) -> Self {
        Self {
            current,
            team,
            opponent,
            kings,
        }
    }

    /// Whether or not moving to this target results in a promotion
    fn is_promotion(&self, turn: Player, tgt: u8) -> bool {
        match turn {
            Player::North => (Self::BOTTOM & 1 << tgt) != 0,
            Player::South => (Self::TOP & 1 << tgt) != 0,
        }
    }

    pub(crate) fn get_mvs(&self, turn: Player) -> Vec<ActionPath> {
        // we would eventually change this such that bitboard contains -> north, south, kings only
        let empty = !(self.current | self.team | self.opponent | self.kings);

        let mut mvs = Vec::new();

        for src in Bits::from(self.current) {
            let src_mask = 1 << src;
            let is_king = (src_mask & self.kings) != 0;

            // normal moves
            // let directions: Vec<i8> = match turn {
            //     Player::South if is_king => vec![4, 5, -4, -5],
            //     Player::South if !is_king => vec![4, 5],
            //     Player::North if is_king => vec![-4, -5, 4, 5],
            //     Player::North if !is_king => vec![-4, -5],
            //     _ => vec![],
            // };
            // // let directions: Vec<i8> = match turn {
            // //     Player::North if is_king => vec![4, 5, -4, -5],
            // //     Player::North if !is_king => vec![4, 5],
            // //     Player::South if is_king => vec![-4, -5, 4, 5],
            // //     Player::South if !is_king => vec![-4, -5],
            // //     _ => vec![],
            // // };

            // for &dir in &directions {
            //     let tgt = src as i8 + dir;
            //     if tgt >= 0 && tgt < 32 {
            //         let to_mask = 1 << tgt as u8;
            //         if (to_mask & empty) != 0 {
            //             let tgt = tgt as u8;
            //             mvs.push(
            //                 Action::new(src, tgt, false, self.is_promotion(turn, tgt), Scale::U32)
            //                     .into(),
            //             );
            //         }
            //     }
            // }

            // // let jump_dirs = match turn {
            // //     Player::North if is_king => vec![(4, 8), (5, 10), (-4, -8), (-5, -10)],
            // //     Player::North if !is_king => vec![(4, 8), (5, 10)],
            // //     Player::South if is_king => vec![(-4, -8), (-5, -10), (4, 8), (5, 10)],
            // //     Player::South if !is_king => vec![(-4, -8), (-5, -10)],
            // //     _ => vec![],
            // // };

            // let jump_dirs = match turn {
            //     Player::South if is_king => vec![(4, 8), (5, 10), (-4, -8), (-5, -10)],
            //     Player::South if !is_king => vec![(4, 8), (5, 10)],
            //     Player::North if is_king => vec![(-4, -8), (-5, -10), (4, 8), (5, 10)],
            //     Player::North if !is_king => vec![(-4, -8), (-5, -10)],
            //     _ => vec![],
            // };

            // for &(cp, to) in &jump_dirs {
            //     let capture = src as i8 + cp;
            //     let tgt = src as i8 + to;
            //     let valid_range = 0..32;

            //     if valid_range.contains(&capture) && valid_range.contains(&tgt) {
            //         let capture_mask = 1 << capture;
            //         let tgt_mask = 1 << tgt;
            //         println!(
            //             "----------------------------------------_______>>>>>>>>>>>src : {src} || capture: {capture} tgt: {tgt} \n {:#0b} {:#0b} ",
            //             capture_mask, tgt_mask
            //         );

            //         if (capture_mask & self.opponent) != 0 && (tgt_mask & empty) != 0 {
            //             let tgt = tgt as u8;
            //             let promoted = self.is_promotion(turn, tgt);

            //             let parent = Action::new(src, tgt, true, promoted, Scale::U32);

            //             let curr = 1 << tgt;
            //             let team = self.team | (self.current & !(1 << src));
            //             let opp = self.opponent & !(1 << capture);
            //             let kings =
            //                 self.kings & !(1 << src) | ((is_king || promoted) as u32) << tgt;

            //             let nb = Bitty::new(curr, team, opp, kings);
            //             let mut result = nb.get_mvs(turn);

            //             result.retain_mut(|path| {
            //                 if matches!(path.peek(path.len -1), Some(act) if act.capture) {
            //                     path.prepend(parent).unwrap();
            //                     return true;
            //                 };
            //                 return false;
            //             });

            //             mvs.reserve(result.len());
            //             mvs.extend(result);
            //         }
            //     }
            // }
        }

        mvs
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{board::state::Board, utils::Qmvs};

    use super::*;

    fn get_path<T>(input: Vec<Vec<T>>) -> Vec<ActionPath>
    where
        Action: From<T>,
        T: Copy,
    {
        input
            .into_iter()
            .map(|a| {
                ActionPath::try_from(
                    a.iter()
                        .map(|ac| Action::from(*ac).into())
                        .collect::<Vec<u16>>()
                        .as_slice(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn should_make_return_valid_moves_for_simple_pieces() {
        let north = 1 << 22;
        let south = 1 << 19;

        let bb = Board::with(north, south, 0, Player::North, Qmvs::default());
        // let bb = Board::new();
        println!("{bb}");

        // let board = Bitty::new(south, 0, north, 0);
        // let received = board.get_mvs(Player::South);

        let board = Bitty::new(north, 0, south, 0);
        let received = board.get_mvs(Player::North);

        received.iter().for_each(|path| {
            for i in 0..path.len {
                let act = Action::from(path[i]);
                println!("abc sssss {:?}", act);
                println!("xx >>> {} \n\n", act);
            }
        });

        // let received = board.options(Player::North);

        let expected = get_path(vec![
            vec![(22u8, 26u8, false, false, true)],
            vec![(22u8, 27u8, false, false, true)],
        ]);

        assert_eq!(expected.len(), received.len());

        println!("received:: {:?}", received);
        println!("expected:: {:?}", expected);

        expected.iter().for_each(|x| {
            // let rr = received.iter().for_each(|a| {
            //     let abx = Action::from(a[0]).transcode();
            //     println!("the x here is {:?} {:?}", abx, abx.to_string());
            // });
            assert!(received.contains(&x))
        });

        // // expected.iter().for_each(|x| assert!(received.contains(&x)));
        // assert_eq!(received.len(), expected.len());
    }
}
