use crate::{Action, Board, game::model::bits::Bits};

use super::scale::Scale;

impl Board {
    const L3_MASK: u32 = 0xE0E0E0E;
    const L5_MASK: u32 = 0x707070;
    const R3_MASK: u32 = 0x70707070;
    const R5_MASK: u32 = 0xE0E0E00;

    const ROW_8_MASK: u32 = 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;

    pub(crate) fn white_movers(&self) -> Vec<Action> {
        let empty = !(self.north | self.south);
        let wk = self.north & self.kings;

        let mut mvs = vec![];

        for src in Bits::new((empty << 4) & self.north) {
            mvs.push(Action::new(src, src + 4, false, false, Scale::U32));
        }

        for src in Bits::new(((empty & Self::L3_MASK) << 3) & self.north) {
            mvs.push(Action::new(src, src + 3, false, false, Scale::U32));
        }

        for src in Bits::new(((empty & Self::L5_MASK) << 5) & self.north) {
            mvs.push(Action::new(src, src + 3, false, false, Scale::U32));
        }

        if wk != 0 {
            Bits::new((empty >> 4) & wk)
                .into_iter()
                .for_each(|src| mvs.push(Action::new(src, src - 4, false, false, Scale::U32)));

            Bits::new(((empty & Self::R3_MASK) >> 3) & wk)
                .into_iter()
                .for_each(|src| mvs.push(Action::new(src, src - 3, false, false, Scale::U32)));

            Bits::new(((empty & Self::R5_MASK) >> 5) & wk)
                .into_iter()
                .for_each(|src| mvs.push(Action::new(src, src - 3, false, false, Scale::U32)));
        }

        return mvs;
    }

    pub(crate) fn white_jumpers(&self) -> Vec<Action> {
        let empty = !(self.north | self.south);
        let wk = self.north & self.kings;
        let mut mvs = Vec::new();

        let mut movers = 0;
        let temp = (empty << 4) & self.south;

        let jump = (((temp & Self::L3_MASK) << 3) | ((temp & Self::L5_MASK) << 5)) & self.north;
        for src in Bits::new(jump) {
            let up = ((Self::L3_MASK >> src) & 1) != 0;
            let dir = if up { 3 } else { 5 };
            let tgt = src + dir + 4;
            let promoted = (1 << tgt & Self::ROW_8_MASK) != 0;
            mvs.push(Action::new(src, tgt, true, promoted, Scale::U32));
        }

        let temp = (((empty & Self::L3_MASK) << 3) | ((empty & Self::L5_MASK) << 5)) & self.south;
        for src in Bits::new((temp << 4) & self.north) {
            let left = (Self::L3_MASK >> (src - 4)) & 1 != 0;
            let dir = if left { 3 } else { 5 };
            let tgt = src + dir + 4;
            let promoted = (1 << tgt & Self::ROW_8_MASK) != 0;
            mvs.push(Action::new(src, tgt, true, promoted, Scale::U32));
        }

        if wk != 0 {
            let temp = (empty >> 4) & self.south;
            let jump = (((temp & Self::R3_MASK) >> 3) | ((temp & Self::R5_MASK) >> 5)) & wk;
            for src in Bits::new(jump) {
                let right = (Self::R3_MASK >> src) & 1 != 0;
                let dir = if right { 3 } else { 5 };
                let tgt = src - (dir + 4);
                mvs.push(Action::new(src, tgt, true, true, Scale::U32));
            }
            let temp =
                (((empty & Self::R3_MASK) >> 3) | ((empty & Self::R5_MASK) >> 5)) & self.south;
            for src in Bits::new((temp >> 4) & wk) {
                let right = (Self::R3_MASK >> (src + 4)) & 1 != 0;
                let dir = if right { 3 } else { 5 };
                let tgt = src - (4 + dir);
                mvs.push(Action::new(src, tgt, true, true, Scale::U32));
            }
        }
        mvs
    }

    fn get(&self) {}
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
    fn should_make_return_valid_moves_for_simple_pieces() {
        // let north = 1 << 22 | 1 << 15;
        // let south = 1 << 0;

        // let north = 1 << 22;
        // let south = 1 << 19;

        let north = 1 << 19;
        let south = 1 << 15;

        let bb = Board::with(north, south, 0, Player::North, Qmvs::default());
        // let bb = Board::new();
        println!("{bb}");

        // let board = Bitty::new(south, 0, north, 0);
        // let received = board.get_mvs(Player::South);

        // let board = Bitty::new(north, 0, south, 0);
        // let received = board.get_mvs(Player::North);

        let received = bb.white_jumpers();

        // println!(
        //     "the received ones [[[[[[[[[[[[[[[[[[[[[[[[[[[[------------]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] {} \n {} \n",
        //     received,
        //     Board::with(received, 0, 0, Player::North, Qmvs::default())
        // );

        assert!(false);

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
}
