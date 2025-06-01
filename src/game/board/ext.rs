// //! this mod would eventually be removed and placed into bitboard.rs directly

// use crate::{Action, ActionPath, Board, Player, Qmvs, game::model::bits::Bits};

// use super::bitboard::BitBoard;

// impl Board {
//     pub(crate) const BOTTOM: u32 = 0x0000000F;
//     pub(crate) const TOP: u32 = 0xF0000000;

//     fn is_promotion(&self, turn: Player, tgt: u8) -> bool {
//         match turn {
//             Player::North => (Self::BOTTOM & 1 << tgt) != 0,
//             Player::South => (Self::TOP & 1 << tgt) != 0,
//         }
//     }

//     fn get_mvs(&self, turn: Player) -> Vec<ActionPath> {
//         // we would eventually change this such that bitboard contains -> north, south, kings only
//         let empty = !(self[turn] | self[!turn]);

//         let mut mvs = Vec::new();

//         for src in Bits::from(self[turn]) {
//             let src_mask = 1 << src;
//             let is_king = (src_mask & self.kings) != 0;

//             // normal moves
//             let directions: Vec<i8> = match turn {
//                 Player::North if is_king => vec![4, 5, -4, -5],
//                 Player::North if !is_king => vec![4, 5],
//                 Player::South if is_king => vec![-4, -5, 4, 5],
//                 Player::South if !is_king => vec![-4, -5],
//                 _ => vec![],
//             };

//             for &dir in &directions {
//                 let tgt = src as i8 + dir;
//                 if tgt >= 0 && tgt < 32 {
//                     let to_mask = 1 << tgt as u8;
//                     if (to_mask & empty) != 0 {
//                         let tgt = tgt as u8;
//                         mvs.push(Action::new(src, tgt, false, self.is_promotion(turn, tgt)));
//                     }
//                 }
//             }

//             let jump_dirs = match turn {
//                 Player::North if is_king => vec![(4, 8), (5, 10), (-4, -8), (-5, -10)],
//                 Player::North if !is_king => vec![(4, 8), (5, 10)],
//                 Player::South if is_king => vec![(-4, -8), (-5, -10), (4, 8), (5, 10)],
//                 Player::North if !is_king => vec![(-4, -8), (-5, -10)],
//                 _ => vec![],
//             };

//             for &(cp, to) in &jump_dirs {
//                 let capture = src as i8 + cp;
//                 let tgt = src as i8 + to;
//                 let valid_range = 0..32;

//                 if valid_range.contains(&capture) && valid_range.contains(&tgt) {
//                     let capture_mask = 1 << capture;
//                     let tgt_mask = 1 << tgt;

//                     if (capture_mask & self[!turn]) != 0 && (tgt_mask & empty) != 0 {
//                         let tgt = tgt as u8;
//                         let promoted = self.is_promotion(turn, tgt);

//                         let mv = Action::new(src, tgt, true, promoted);

//                         let mut new_team = self[turn] & !(1 << src);
//                         let mut new_opponent = self[!turn] & !(1 << capture);
//                         let new_kings =
//                             self.kings & !(1 << src) | ((is_king || promoted) as u32) << tgt;

//                         let new_board = match turn {
//                             Player::North => {
//                                 Board::with(new_team, new_opponent, new_kings, turn, Qmvs::new())
//                             }
//                             Player::South => {
//                                 Board::with(new_opponent, new_team, new_kings, turn, Qmvs::new())
//                             }
//                         };

//                         // let result = new_board.get_mvs(turn);

//                         mvs.push(mv);
//                     }
//                 }
//             }
//         }

//         mvs
//     }
// }
