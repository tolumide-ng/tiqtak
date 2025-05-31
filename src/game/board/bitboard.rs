use crate::game::model::player::Player;
use crate::game::model::{action::Action, path::ActionPath};
use crate::game::traits::u32_shift::U32Ext;
use crate::{Board, Qmvs};

pub(crate) struct BitBoard {
    current: u32,
    other: u32,
    team: u32,
}

impl BitBoard {
    const LEFT: u32 = 0x08080808;
    const RIGHT: u32 = 0x10101010;
    const BOTTOM: u32 = 0x0000000F;
    const TOP: u32 = 0xF0000000;

    const TOP_LEFT_MV: u8 = 4;
    const TOP_RIGHT_MV: u8 = 5;
    const BOTTOM_LEFT_MV: u8 = 5;
    const BOTTOM_RIGHT_MV: u8 = 4;

    /// Number of rows on a checkers board (for each side)
    const NUM_ROWS: u32 = 4;
    /// On a 0 indexed board (first row as 0), the last row on the board is 7
    /// Yes, there are 8 rows, but its 0 indexed
    const MAX_ROW: u32 = 7;

    // hor_mask: horizontal mask
    fn get(&self, hor_mask: u32, shift: u8, turn: Player) -> Vec<ActionPath> {
        // vertical mask
        let v_mask = match turn {
            Player::South => Self::TOP,
            Player::North => Self::BOTTOM,
        };

        // South
        let mut pcs = (!v_mask) & self.current & (!hor_mask);

        let mut mvs = Vec::with_capacity(pcs.count_ones() as usize);

        // let bb = Board::with(pcs, 0, 0, turn, Qmvs::default());
        // println!("the board here is \n{bb} \n >>>>>>");

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;

            // println!("the src here is !!!!!!!!!!!!!! {:?}", src);
            pcs &= pcs - 1;

            let mut tgt = (1 << src).shift_by(shift, turn);
            let mut capture = false;
            let mut promoted = false;

            let self_on_target = ((self.current | self.team) & tgt) != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !v_mask & !hor_mask) != 0)
                && ((tgt.shift_by(shift, turn)) & (self.current | self.other | self.team) == 0); // ensures landing(jumping target) space is empty

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target && valid_capture {
                let new_others = self.other & !tgt;
                tgt = tgt.shift_by(shift, turn);

                promoted =
                    (tgt.trailing_zeros() / Self::NUM_ROWS) == ((turn as u32) * Self::MAX_ROW);

                // let idx = tgt.trailing_zeros();
                // promoted = match turn {
                //     Player::South => idx >= 28,
                //     Player::North => idx < 4,
                // };

                let new_team = (self.current & !(1 << src)) | (self.team & !(1 << src)) | tgt;
                capture = true;

                let kings = (promoted as u32) * tgt;

                let board = BitBoard::new(tgt, new_others, new_team);

                let parent = Action::new(src, tgt.trailing_zeros() as u8, capture, promoted);

                let result = board.moves(turn);
                result.into_iter().for_each(|mut actions| {
                    if let Some(act) = actions.peek(actions.len - 1) {
                        if act.capture {
                            actions.prepend(parent);
                            mvs.push(actions);
                        }
                    }
                });

                (kings != 0).then(|| {
                    let more = board.moves(!turn);
                    more.into_iter().for_each(|mut actions| {
                        if let Some(act) = actions.peek(actions.len - 1) {
                            if act.capture {
                                actions.prepend(parent);
                                mvs.push(actions);
                            }
                        }
                    });
                });
            }

            let tgt = tgt.trailing_zeros() as u8;

            // println!("src {:?} {:?}", src, tgt);
            promoted = (tgt / 8) == ((turn as u8) * 7);
            mvs.push(Action::from((src, tgt, capture, promoted, false)).into());
        }

        mvs
    }

    pub(crate) fn moves(&self, play_as: Player) -> Vec<ActionPath> {
        // let b = Board::with(Self::BOTTOM, 0, 0, Player::North, Qmvs::default());
        // println!("{}", b);

        let (mut left, right) = match play_as {
            Player::South => (self.top_left(), self.top_right()),
            Player::North => (self.bottom_left(), self.bottom_right()),
        };

        left.reserve(right.len());
        left.extend(right);

        left
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 8 (top row)
    /// pieces that are safe to move top-left
    fn top_left(&self) -> Vec<ActionPath> {
        self.get(Self::LEFT, Self::TOP_LEFT_MV, Player::South)
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> Vec<ActionPath> {
        self.get(Self::RIGHT, Self::TOP_RIGHT_MV, Player::South)
    }

    fn bottom_right(&self) -> Vec<ActionPath> {
        let xx = self.get(Self::RIGHT, Self::BOTTOM_RIGHT_MV, Player::North);
        // println!("bottom_right");
        // for x in 0..xx.len() {
        //     let ax = xx[x];
        //     for act in 0..ax.len {
        //         print!("{} -->> ", Action::from(ax[act]))
        //     }
        //     println!("||||");
        // }
        // println!("xx:::: {:?}", xx);
        xx
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    pub(crate) fn bottom_left(&self) -> Vec<ActionPath> {
        let xx = self.get(Self::LEFT, Self::BOTTOM_LEFT_MV, Player::North);
        // println!("bottom_left");
        // for x in 0..xx.len() {
        //     let ax = xx[x];
        //     for act in 0..ax.len {
        //         print!("{} -->> ", Action::from(ax[act]))
        //     }
        //     println!("||||");
        // }
        // println!("xx:::: {:?}", xx);
        xx
    }

    pub(super) fn new(current: u32, other: u32, team: u32) -> Self {
        Self {
            current,
            other,
            team,
        }
    }
}

impl From<(u32, u32, u32)> for BitBoard {
    fn from(value: (u32, u32, u32)) -> Self {
        Self {
            current: value.0,
            other: value.1,
            team: value.2,
        }
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

        let board = Board::with(north, south, 0, Player::North, Qmvs::default());
        println!("{board}");

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

    // #[test]
    // fn should_return_all_possible_moves_for_south_player() {
    //     let north = 0x520000a00000000u64;
    //     let south = 0x40014200000u64;

    //     let board = Board::with(north, south, 0, Player::South, Qmvs::default());
    //     let received = board.options(Player::South);

    //     let expected = get_path(vec![
    //         vec![(21u8, 30u8, false, false)],
    //         vec![(26u8, 44u8, true, false), (44u8, 62u8, true, true)],
    //         vec![(26u8, 44u8, true, false)],
    //         vec![(26u8, 40u8, true, false)],
    //         vec![(28u8, 37u8, false, false)],
    //         vec![(42u8, 49u8, false, false)],
    //         vec![(42u8, 51u8, false, false)],
    //     ]);

    //     assert_eq!(received.len(), expected.len());
    //     expected.iter().for_each(|x| assert!(received.contains(&x)));
    // }

    // #[test]
    // fn should_return_all_south_moves_including_kings() {
    //     let north = 0x520000a00000000u64;
    //     let south = 0x40014200000u64;

    //     let kings = 1 << 42;

    //     let board = Board::with(north, south, kings, Player::South, Qmvs::default());
    //     let received = board.options(Player::South);

    //     let expected = get_path(vec![
    //         vec![(26u8, 40u8, true, false)],
    //         vec![(26, 44, true, false)],
    //         vec![(26, 44, true, false), (44, 62, true, true)],
    //         vec![(28, 37, false, false)],
    //         vec![(21, 30, false, false)],
    //         vec![(42, 24, true, false)],
    //         vec![(42, 49, false, false)],
    //         vec![(42, 51, false, false)],
    //     ]);

    //     assert_eq!(received.len(), expected.len());

    //     expected
    //         .iter()
    //         .for_each(|mv| assert!(received.contains(&mv)));
    // }

    // // should return all mulitple moves (for a single piece) in one go for a regular player test (bottom-left -->> bottom-right)
    // // same as above, but testing for kings
    // #[test]
    // fn should_return_all_multiples_moves_by_one_piece() {
    //     let south = 0x200008000801u64;
    //     let north = 0x40000000000000u64;

    //     let kings = 1 << 42;

    //     let board = Board::with(north, south, kings, Player::North, Qmvs::default());
    //     let received = board.options(Player::North);

    //     // received.sort();

    //     received.iter().for_each(|x| println!("{}", x.to_string()));

    //     let expected = get_path(vec![
    //         vec![
    //             (54u8, 36u8, true, false),
    //             (36, 18, true, false),
    //             (18, 4, true, true),
    //         ],
    //         vec![(54u8, 36u8, true, false), (36, 18, true, false)],
    //         vec![(54u8, 36u8, true, false)],
    //         vec![(54u8, 47u8, false, false)],
    //     ]);

    //     assert_eq!(received.len(), expected.len());

    //     expected
    //         .iter()
    //         .for_each(|mv| assert!(received.contains(mv)));
    // }

    // // should_return_all_possible_moves_in_the_start_position
    // #[test]
    // fn should_return_all_possible_moves_in_the_base_position() {
    //     let board = Board::new();
    //     let received = board.options(Player::South);
    //     assert_eq!(received.len(), 7);
    //     assert_eq!(board.options(Player::South).len(), 7);

    //     let expected = get_path(vec![
    //         vec![(16, 25, false, false)],
    //         vec![(18, 25, false, false)],
    //         vec![(18, 27, false, false)],
    //         vec![(20, 27, false, false)],
    //         vec![(20, 29, false, false)],
    //         vec![(22, 29, false, false)],
    //         vec![(22, 31, false, false)],
    //     ]);

    //     assert_eq!(received.len(), expected.len());
    //     expected
    //         .iter()
    //         .for_each(|mv| assert!(received.contains(&mv)));
    // }

    // // should convert a regular to a king after they reach the opponents base
    // #[test]
    // fn should_convert_a_regular_to_king_if_they_touch_the_opponents_base() {
    //     let south = 0x20000000000u64;
    //     let north = 0x14000008000000u64;

    //     let board = Board::with(north, south, 0, Player::South, Qmvs::default());
    //     // println!("{board}");
    //     let received = board.options(Player::South);

    //     received.iter().for_each(|x| println!("{}", x.to_string()));

    //     let expected = get_path(vec![
    //         vec![(41, 59, true, true), (59, 45, true, false)],
    //         vec![(41, 59, true, true)],
    //         vec![(41, 48, false, false)],
    //     ]);

    //     assert_eq!(received.len(), expected.len());
    //     expected
    //         .iter()
    //         .for_each(|mv| assert!(received.contains(&mv)));
    // }

    // #[test]
    // fn a_king_should_never_overwrite_its_teammates() {
    //     let north = 0x244u64;
    //     let south = 0xaa00000000000000u64;

    //     let kings = 1 << 2 | 1 << 6 | 1 << 57 | 1 << 59 | 1 << 61 | 1 << 63;

    //     let board = Board::with(north, south, kings, Player::North, Qmvs::default());
    //     let received = board.options(Player::North);

    //     let expected = get_path(vec![
    //         vec![(6, 15, false, false)],
    //         vec![(6, 13, false, false)],
    //         vec![(2, 11, false, false)],
    //         vec![(9, 0, false, true)],
    //     ]);

    //     // expected.iter().for_each(|x| {
    //     //     x.mvs[..x.len]
    //     //         .iter()
    //     //         .for_each(|xx| print!("{} -->", Action::from(*xx).to_string()));
    //     //     println!("\n");
    //     // });

    //     expected.iter().for_each(|x| assert!(received.contains(&x)));
    //     assert_eq!(expected.len(), received.len());
    // }

    // #[test]
    // fn king_piece_should_never_be_demoted_when_it_leaves_the_opponents_base() {
    //     let north = 0x244u64;
    //     let south = 0xaa00000000000000u64;

    //     let kings = 1 << 2 | 1 << 6 | 1 << 57 | 1 << 59 | 1 << 61 | 1 << 63;

    //     let board = Board::with(north, south, kings, Player::North, Qmvs::default());
    //     println!("{board}");

    //     assert_eq!(board.kings.count_ones(), 6);
    //     assert_eq!((board.south & board.kings).count_ones(), 4);
    //     assert_eq!(board.north.count_ones(), 3);
    //     assert_eq!((board.north & (!board.kings)).count_ones(), 1);
    //     assert_eq!((board.north & board.kings).count_ones(), 2);
    //     assert!((board.kings & (1 << 2)) != 0);
    //     assert!((board.kings & (1 << 11)) == 0);

    //     let action = ActionPath::from(Action::from((2, 11, false, false)));

    //     let new_board = board.play(action).unwrap();

    //     assert_eq!(new_board.kings.count_ones(), 6);
    //     assert_eq!((new_board.south & board.kings).count_ones(), 4);
    //     assert_eq!(new_board.north.count_ones(), 3);
    //     assert!((new_board.kings & (1 << 2)) == 0);
    //     assert!((new_board.kings & (1 << 11)) != 0);
    //     assert_eq!((new_board.north & (!new_board.kings)).count_ones(), 1);
    //     assert_eq!((new_board.north & new_board.kings).count_ones(), 2);
    // }

    // #[test]
    // fn should_be_able_to_play_a_jump_game() {}
}
