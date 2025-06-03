use crate::Board;

impl Board {
    const L3_MASK: u32 = 0xE0E0E0E;
    const L5_MASK: u32 = 0x707070;
    const R3_MASK: u32 = 0x70707070;
    const R5_MASK: u32 = 0xE0E0E00;

    pub(crate) fn white_movers(&self) -> u32 {
        let empty = !(self.north | self.south);
        let wk = self.north & self.kings;

        let mut movers = (empty << 4) & self.north;
        movers |= ((empty & Self::L3_MASK) << 3) & self.north;
        movers |= ((empty & Self::L5_MASK) << 5) & self.north;

        if wk != 0 {
            movers |= (empty >> 4) & wk;
            movers |= ((empty & Self::R3_MASK) >> 3) & wk;
            movers |= ((empty & Self::R5_MASK) >> 5) & wk;
        }

        movers
        // let empty = !(self.north | self.south);
        // let wk = self.north & self.kings;

        // let mut movers = 0;

        // let mut temp = (empty << 4) & self.south;
        // if temp != 0 {
        //     movers |= (((temp & Self::L3_MASK) << 3) | ((temp & Self::L5_MASK) << 5)) & self.north;
        // }

        // temp = (((empty & Self::L3_MASK) << 3) | ((empty & Self::L5_MASK) << 5)) & self.south;
        // movers |= (temp << 4) & self.north;

        // if self.north != 0 {
        //     temp = (empty >> 4) & self.north;
        //     if temp != 0 {
        //         movers |= (((temp & Self::R3_MASK) >> 3) | ((temp & Self::R5_MASK) >> 5)) & wk;
        //     }
        //     temp = (((empty & Self::R3_MASK) >> 3) | ((empty & Self::R5_MASK) >> 5)) & self.south;
        //     if temp != 0 {
        //         movers |= (temp >> 4) & self.north;
        //     }
        // }

        // movers
    }

    pub(crate) fn white_jumpers(&self) -> u32 {
        let empty = !(self.north | self.south);
        let wk = self.north & self.kings;

        let mut movers = 0;
        let mut temp = (empty << 4) & self.south;
        if temp != 0 {
            movers |= (((temp & Self::L3_MASK) << 3) | ((temp & Self::L5_MASK) << 5)) & self.north;
        }

        temp = (((empty & Self::L3_MASK) << 3) | ((empty & Self::L5_MASK) << 5)) & self.south;
        movers |= (temp << 4) & self.north;

        if wk != 0 {
            temp = (empty >> 4) & self.south;
            if temp != 0 {
                movers |= (((temp & Self::R3_MASK) >> 3) | ((temp & Self::R5_MASK) >> 5)) & wk;
            }
            temp = (((empty & Self::R3_MASK) >> 3) | ((empty & Self::R5_MASK) >> 5)) & self.south;
            if temp != 0 {
                movers |= (temp >> 4) & wk;
            }
        }
        movers
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

        println!(
            "the received ones [[[[[[[[[[[[[[[[[[[[[[[[[[[[------------]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] {} \n {} \n",
            received,
            Board::with(received, 0, 0, Player::North, Qmvs::default())
        );

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
