use crate::{Action, ActionPath, Board, Player, Qmvs, game::model::bits::Bits};

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

    fn get_mvs(&self, turn: Player) -> Vec<ActionPath> {
        // we would eventually change this such that bitboard contains -> north, south, kings only
        let empty = !(self.current | self.team | self.opponent | self.kings);

        let mut mvs = Vec::new();

        for src in Bits::from(self.current) {
            let src_mask = 1 << src;
            let is_king = (src_mask & self.kings) != 0;

            // normal moves
            let directions: Vec<i8> = match turn {
                Player::North if is_king => vec![4, 5, -4, -5],
                Player::North if !is_king => vec![4, 5],
                Player::South if is_king => vec![-4, -5, 4, 5],
                Player::South if !is_king => vec![-4, -5],
                _ => vec![],
            };

            for &dir in &directions {
                let tgt = src as i8 + dir;
                if tgt >= 0 && tgt < 32 {
                    let to_mask = 1 << tgt as u8;
                    if (to_mask & empty) != 0 {
                        let tgt = tgt as u8;
                        mvs.push(
                            Action::new_32(src, tgt, false, self.is_promotion(turn, tgt)).into(),
                        );
                    }
                }
            }

            let jump_dirs = match turn {
                Player::North if is_king => vec![(4, 8), (5, 10), (-4, -8), (-5, -10)],
                Player::North if !is_king => vec![(4, 8), (5, 10)],
                Player::South if is_king => vec![(-4, -8), (-5, -10), (4, 8), (5, 10)],
                Player::North if !is_king => vec![(-4, -8), (-5, -10)],
                _ => vec![],
            };

            for &(cp, to) in &jump_dirs {
                let capture = src as i8 + cp;
                let tgt = src as i8 + to;
                let valid_range = 0..32;

                if valid_range.contains(&capture) && valid_range.contains(&tgt) {
                    let capture_mask = 1 << capture;
                    let tgt_mask = 1 << tgt;

                    if (capture_mask & self.opponent) != 0 && (tgt_mask & empty) != 0 {
                        let tgt = tgt as u8;
                        let promoted = self.is_promotion(turn, tgt);

                        let parent = Action::new_32(src, tgt, true, promoted);

                        let curr = 1 << tgt;
                        let team = self.team | (self.current & !(1 << src));
                        let opp = self.opponent & !(1 << capture);
                        let kings =
                            self.kings & !(1 << src) | ((is_king || promoted) as u32) << tgt;

                        let nb = Bitty::new(curr, team, opp, kings);
                        let mut result = nb.get_mvs(turn);

                        result.retain_mut(|path| {
                            if matches!(path.peek(path.len -1), Some(act) if act.capture) {
                                path.prepend(parent);
                                return true;
                            };
                            return false;
                        });

                        mvs.reserve(result.len());
                        mvs.extend(result);
                    }
                }
            }
        }

        mvs
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        convert64bits_to_32bits::getax,
        game::{board::state::Board, utils::Qmvs},
    };

    use super::*;

    fn get_path<T>(input: Vec<Vec<T>>) -> Vec<ActionPath>
    where
        Action: From<T>,
        T: Copy,
    {
        input
            .into_iter()
            .map(|a| {
                ActionPath::from(
                    a.iter()
                        .map(|ac| Action::from(*ac).into())
                        .collect::<Vec<u16>>()
                        .as_slice(),
                )
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn should_make_only_valid_moves() {
        let xxx = 0x11200000;
        let b = Board::with(xxx, 0, 0, Player::North, Qmvs::default());
        println!("{}", b);

        // let north = 0x11200000;
        // let south = 0x26000;
        let north = 1 << 22;
        let south = 1 << 19;

        let bb = Board::with(north, south, 0, Player::North, Qmvs::default());
        println!("{bb}");

        let board = Bitty::new(north, 0, south, 0);
        let mvs = board.get_mvs(Player::North);

        mvs.iter().for_each(|path| {
            for i in 0..path.len {
                let act = Action::from(path[i]);
                println!("xx >>> {}", act);
            }
        });

        assert!(false);

        // let received = board.options(Player::North);

        // let expected = get_path(vec![
        //     vec![(54u8, 47u8, false, false, true)],
        //     vec![(45u8, 38u8, false, false, true)],
        // ]);

        // println!(
        //     ":first :::: : {:?}",
        //     getax(Action::from((54u8, 47u8, false, false)))
        // );

        // expected.iter().for_each(|x| {
        //     let rr = received.iter().for_each(|a| {
        //         let abx = Action::from(a[0]).transcode();
        //         println!("the x here is {:?} {:?}", abx, abx.to_string());
        //     });
        //     assert!(received.contains(&x))
        // });

        // // expected.iter().for_each(|x| assert!(received.contains(&x)));
        // assert_eq!(received.len(), expected.len());
    }
}
