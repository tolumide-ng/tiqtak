use crate::{
    Action, ActionPath, Board, Player,
    game::{
        model::bits::Bits,
        traits::{u8_ops::U8Ext, u32_shift::U32Ext},
    },
};

use super::scale::Scale::{self, *};

impl Board {
    const ROW_8_MASK: u32 = 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;

    pub(crate) fn movers(&self, turn: Player) -> Vec<Action> {
        let empty = !(self.north | self.south);
        let king = self[turn] & self.kings;

        let mut mvs = vec![];

        for src in Bits::new((empty.shift_by(4, turn)) & self[turn]) {
            mvs.push(Action::new(src, src.move_by(4, turn), false, false, U32));
        }

        for src in Bits::new(((empty & turn.s3()).shift_by(3, turn)) & self[turn]) {
            mvs.push(Action::new(src, src.move_by(3, turn), false, false, U32));
        }

        for src in Bits::new(((empty & turn.s5()).shift_by(5, turn)) & self[turn]) {
            mvs.push(Action::new(src, src.move_by(3, turn), false, false, U32));
        }

        if king != 0 {
            Bits::new((empty.shift_by(4, !turn)) & king)
                .into_iter()
                .for_each(|src| {
                    mvs.push(Action::new(src, src.move_by(4, !turn), false, false, U32))
                });

            Bits::new(((empty & (!turn).s3()).shift_by(3, !turn)) & king)
                .into_iter()
                .for_each(|src| {
                    mvs.push(Action::new(src, src.move_by(3, !turn), false, false, U32))
                });

            Bits::new(((empty & (!turn).s5()).shift_by(5, !turn)) & king)
                .into_iter()
                .for_each(|src| {
                    mvs.push(Action::new(src, src.move_by(3, !turn), false, false, U32))
                });
        }

        return mvs;
    }

    pub(crate) fn jumpers(&self, turn: Player) -> Vec<Action> {
        let empty = !(self.north | self.south);
        let king = self[turn] & self.kings;
        let mut mvs = Vec::new();

        let temp = empty.shift_by(4, turn) & self[!turn];
        let jump = (((temp & turn.s3()).shift_by(3, turn))
            | ((temp & turn.s5()).shift_by(5, turn)))
            & self[turn];

        for src in Bits::new(jump) {
            let right = ((turn.s3().shift_by(src, !turn)) & 1) != 0;
            let dir = if right { 5 } else { 3 };
            let tgt = src.move_by(dir, turn).move_by(4, turn);

            let promoted = ((1u32).shift_by(tgt, turn) & turn.opponent_base()) != 0;
            mvs.push(Action::new(src, tgt, true, promoted, Scale::U32));
        }

        let temp = (((empty & turn.s3()).shift_by(3, turn))
            | ((empty & turn.s5()).shift_by(5, turn)))
            & self[!turn];
        for src in Bits::new((temp.shift_by(4, turn)) & self[turn]) {
            let left = (turn.s3().shift_by(src.move_by(4, turn), !turn)) & 1 != 0;
            let dir = if left { 5 } else { 3 };
            let tgt = src.move_by(dir, turn).move_by(4, turn);
            let promoted = ((1u32).shift_by(tgt, turn) & turn.opponent_base()) != 0;
            mvs.push(Action::new(src, tgt, true, promoted, Scale::U32));
        }

        if king != 0 {
            let temp = (empty.shift_by(4, !turn)) & self[!turn];
            let jump = (((temp & turn.s3()).shift_by(3, !turn))
                | ((temp & turn.s5()).shift_by(5, !turn)))
                & king;

            for src in Bits::new(jump) {
                let right = (turn.s3().shift_by(src, !turn)) & 1 != 0;
                let dir = if right { 3 } else { 5 };
                let tgt = src.move_by(dir + 4, turn);
                mvs.push(Action::new(src, tgt, true, true, Scale::U32));
            }

            let temp = (((empty & turn.s3()).shift_by(3, !turn))
                | ((empty & turn.s5()).shift_by(5, !turn)))
                & self[!turn];
            for src in Bits::new((temp.shift_by(4, !turn)) & king) {
                let right = (turn.s3().shift_by(src + 4, !turn)) & 1 != 0;
                let dir = if right { 3 } else { 5 };
                let tgt = src.move_by(dir + 4, turn);
                mvs.push(Action::new(src, tgt, true, true, Scale::U32));
            }
        }
        mvs
    }

    fn get(&self, turn: Player) {
        let mut regulars = self.movers(turn);
        let mut jumpers = self.jumpers(turn);

        for mv in regulars.iter_mut().chain(jumpers.iter_mut()) {
            let board = self.play((*mv).into()).unwrap();
            let regs = board.get(!turn);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Player,
        game::{board::state::Board, utils::Qmvs},
    };

    use super::*;

    // fn get_path<T>(input: Vec<Vec<T>>) -> Vec<ActionPath>
    // where
    //     Action: From<T>,
    //     T: Copy,
    // {
    //     input
    //         .into_iter()
    //         .map(|a| {
    //             ActionPath::try_from(
    //                 a.iter()
    //                     .map(|ac| Action::from(*ac).into())
    //                     .collect::<Vec<u16>>()
    //                     .as_slice(),
    //             )
    //             .unwrap()
    //         })
    //         .collect::<Vec<_>>()
    // }

    #[test]
    fn should_return_regular_black_moves() {
        // let north = 1 << 22 | 1 << 15;
        // let south = 1 << 0;

        let north = 1 << 22;
        let south = 1 << 19 | 1 << 18;

        let board = Board::with(north, south, 1 << 22, Player::South, Qmvs::default());
        println!("{board}");

        let mvs = board.movers(Player::South);
        let expected = vec![
            Action::new(38, 47, false, false, U64),
            Action::new(36, 43, false, false, U64),
        ];

        assert_eq!(mvs.len(), expected.len());
        expected
            .iter()
            .for_each(|a| assert!(mvs.contains(&a.transcode())));
    }

    #[test]
    fn should_make_return_valid_moves_for_simple_pieces() {
        // let north = 1 << 22 | 1 << 15;
        // let south = 1 << 0;

        let north = 1 << 22;
        let south = 1 << 19 | 1 << 18;

        let board = Board::with(north, south, 1 << 22, Player::North, Qmvs::default());
        let jumps = board.jumpers(Player::North);
        let mvs = board.movers(Player::North);

        let expected = vec![
            Action::new(45, 31, true, false, Scale::U64),
            Action::new(45, 27, true, false, Scale::U64),
            Action::new(45, 50, false, false, Scale::U64),
            Action::new(45, 52, false, false, Scale::U64),
        ];

        assert_eq!(mvs.iter().chain(jumps.iter()).count(), expected.len());
        expected.iter().for_each(|a| {
            assert!(
                mvs.iter()
                    .chain(jumps.iter())
                    .collect::<Vec<_>>()
                    .contains(&&a.transcode())
            )
        });

        // received.iter().for_each(|path| {
        //     for i in 0..path.len {
        //         let act = Action::from(path[i]);
        //         println!("abc sssss {:?}", act);
        //         println!("xx >>> {} \n\n", act);
        //     }
        // });

        // // let received = board.options(Player::North);

        // let expected = get_path(vec![
        //     vec![(22u8, 26u8, false, false, true)],
        //     vec![(22u8, 27u8, false, false, true)],
        // ]);

        // assert_eq!(expected.len(), received.len());

        // println!("received:: {:?}", received);
        // println!("expected:: {:?}", expected);

        // expected.iter().for_each(|x| {
        //     // let rr = received.iter().for_each(|a| {
        //     //     let abx = Action::from(a[0]).transcode();
        //     //     println!("the x here is {:?} {:?}", abx, abx.to_string());
        //     // });
        //     assert!(received.contains(&x))
        // });

        // // expected.iter().for_each(|x| assert!(received.contains(&x)));
        // assert_eq!(received.len(), expected.len());
    }

    #[test]
    fn should_return_correct_actions_for_jumps_and_mvs_on_even_numbered_rows() {
        let north = 1 << 11;
        let south = 1 << 6;

        let board = Board::with(north, south, 0, Player::North, Qmvs::default());

        let mvs = board.movers(Player::North);
        let jumps = board.jumpers(Player::North);

        let expected_mvs = vec![Action::new(22, 15, false, false, Scale::U64)];
        let expected_jumps = vec![Action::new(22, 4, true, false, Scale::U64)];

        assert_eq!(mvs.len(), expected_mvs.len());
        assert_eq!(jumps.len(), expected_jumps.len());

        expected_jumps
            .into_iter()
            .for_each(|j| assert!(jumps.contains(&j.transcode())));

        println!("{}", board);

        expected_mvs
            .into_iter()
            .for_each(|m| assert!(mvs.contains(&m.transcode())));
    }

    // #[test]
    // fn should_return_correct_moves_for_kings() {
    //     let north = 1 << 11 | 1 << 10 | 1 << 17 | 1 << 18;
    //     let south = 1 << 15;

    //     let board = Board::with(north, south, 0, Player::North, Qmvs::default());

    //     println!("{}", board);

    //     assert!(false);
    // }
}
