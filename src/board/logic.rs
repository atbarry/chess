use crate::constants::BOARD_HEIGHT;

use super::{Board, BoardPos, PieceType, Side, Piece, BChange, BChange::{Move, MoveDestroy, BothMove}};

#[derive(Clone, Copy)]
enum Dir{
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Custom(i32, i32),
}

impl BoardPos{
    fn square_in_dir(&self, dir: Dir) -> Option<BoardPos> {
        let convert_to_option = |x, y| -> Option<BoardPos> {
            if x < 0 || y < 0 {
                return None;
            }
            let (x, y) = (x as usize, y as usize);
            match BoardPos::new(x, y) {
                Ok(pos) => Some(pos),
                Err(_) => None,
            }
        };

        let x = self.x as i32;
        let y = self.y as i32;
        match dir {
            Dir::Up => convert_to_option(x, y + 1),
            Dir::Down => convert_to_option(x, y - 1),
            Dir::Left => convert_to_option(x - 1, y),
            Dir::Right => convert_to_option(x + 1, y),
            Dir::UpRight => convert_to_option(x + 1, y + 1),
            Dir::UpLeft => convert_to_option(x - 1, y + 1),
            Dir::DownRight => convert_to_option(x + 1, y - 1),
            Dir::DownLeft => convert_to_option(x - 1, y - 1),
            Dir::Custom(x_steps, y_steps) => convert_to_option(x + x_steps, y + y_steps),
        }
    }
}


fn slide(dir: Dir, side: Side, start: BoardPos, board: &Board) -> Vec<BChange> {
    let mut moves = Vec::new();
    let mut current_square = start;
    loop {
        current_square = match current_square.square_in_dir(dir) {
            Some(square) => square,
            None => break,
        };

        match board.get_piece(current_square) {
            Some(target_piece) => {
                if side.is_enemy(&target_piece.side) {
                    let board_change = MoveDestroy { start, end: current_square, target: current_square };
                    moves.push(board_change);
                }
                break;
            },
            None => moves.push(Move { start, end: current_square}),
        }
    }

    moves
}

impl Board {
    pub fn check_valid_change(&self, start: BoardPos, click_pos: BoardPos) -> Option<BChange> {
        for change in self.get_possible_moves(start) {
            if change.click_pos_to_activate_change() == click_pos{
                return Some(change);
            }
        }

        None
    }

    pub fn get_possible_moves(&self, selected_square: BoardPos) -> Vec<BChange>{
        let piece = self.get_piece(selected_square).unwrap(); // if this fails we have a bug

        // not the correct turn then no moves
        if !self.is_turn(piece.side) {
            return Vec::new();
        }
        
        match &piece.piece_type {
            PieceType::King => self.get_king_moves(selected_square, piece),
            PieceType::Queen => self.get_queen_moves(selected_square, piece),
            PieceType::Rook => self.get_rook_moves(selected_square, piece),
            PieceType::Bishop => self.get_bishop_moves(selected_square, piece),
            PieceType::Knight => self.get_knight_moves(selected_square, piece),
            PieceType::Pawn => self.get_pawn_moves(selected_square, piece),
        }
    }

    fn get_king_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BChange>{
        let mut moves = Vec::new();
        let side = piece.side;
        
        let mut add_move = |dir: Dir| {
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return,
            };
            
            match self.get_piece(square) {
                Some(target_piece) => {
                    if side.is_enemy(&target_piece.side) {
                        moves.push(MoveDestroy { start: piece.board_pos, end: square, target: square });
                    }
                },
                None => moves.push(Move { start: piece.board_pos, end: square }),
            };
        };

        let check_castling = || -> Vec<BChange> {
            let mut changes = Vec::new();

            if piece.num_moves > 0 {return changes}

            // Check if the king can castle to the right
            let mut open_squares = true;
            for x in 1..3 {
                let square = match selected_square.square_in_dir(Dir::Custom(x, 0)) {
                    Some(square) => square,
                    None => break,
                };

                if self.is_occupied(square){
                    open_squares = false;
                    break;
                }
            }

            if open_squares {
                let rook_square = selected_square.square_in_dir(Dir::Custom(3, 0)).unwrap();

                if let Some(rook) = self.get_piece(rook_square) {
                    if rook.piece_type == PieceType::Rook && rook.num_moves == 0 {
                        changes.push(BothMove { 
                            start1: selected_square, 
                            end1: selected_square.square_in_dir(Dir::Custom(2, 0)).unwrap(), 
                            start2: rook_square, 
                            end2: selected_square.square_in_dir(Dir::Right).unwrap(),
                        });
                    }
                }
            }

            // Check if the king can castle to the left
            open_squares = true;
            for x in 1..4 {
                let square = match selected_square.square_in_dir(Dir::Custom(-x, 0)) {
                    Some(square) => square,
                    None => break,
                };

                if self.is_occupied(square) {
                    open_squares = false;
                    break;
                }
            }

            if open_squares {
                let rook_square = selected_square.square_in_dir(Dir::Custom(-4, 0)).unwrap();
                
                if let Some(rook) = self.get_piece(rook_square) {
                    if rook.piece_type == PieceType::Rook && rook.num_moves == 0 {
                        changes.push(BothMove { 
                            start1: selected_square, 
                            end1: selected_square.square_in_dir(Dir::Custom(-2, 0)).unwrap(), 
                            start2: rook_square, 
                            end2: selected_square.square_in_dir(Dir::Left).unwrap(),
                        });
                    }
                }
            }

            changes
        };
        
        add_move(Dir::Up);
        add_move(Dir::Down);
        add_move(Dir::Left);
        add_move(Dir::Right);
        add_move(Dir::UpRight);
        add_move(Dir::UpLeft);
        add_move(Dir::DownRight);
        add_move(Dir::DownLeft); 

        moves.append(&mut check_castling());
        moves
    }

    fn get_pawn_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BChange>{
        let mut moves = Vec::new();
        let side = piece.side;

        let mut add_normal_move = |dir: Dir| {
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return,
            };

            match self.get_piece(square) {
                Some(target_piece) =>  {
                    // checks for diagonal attack
                    match &dir {
                        Dir::Up | Dir::Down => (),
                        _ => {
                            if side.is_enemy(&target_piece.side) {
                                moves.push(MoveDestroy { start: selected_square, end: square, target: square });
                            }
                        }
                        
                    }
                },
                None => match &dir {
                    // check if it can move forward
                    Dir::Up => moves.push(Move { start: selected_square, end: square }),
                    Dir::Down => moves.push(Move { start: selected_square, end: square }),
                    _ => (),
                },
            }
        };

        let check_double_space_move = |dir: Dir| -> Option<BChange>{
            if piece.num_moves != 0 {
                return None;
            }

            let one_square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return None,
            };

            let two_square = match one_square.square_in_dir(dir) {
                Some(square) => square,
                None => return None,
            };

            if self.is_occupied(one_square) || self.is_occupied(two_square) {
                return None;
            }

            Some(Move{start: selected_square, end: two_square})
        };

        let check_en_passant = |dir: Dir| -> bool{
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return false,
            };

            if let Some(target_piece) = self.get_piece(square) {
                if side.is_enemy(&target_piece.side) 
                    && target_piece.piece_type == PieceType::Pawn
                    && target_piece.num_moves == 1
                    && target_piece.turns_since_last_move(self.turn_num) == 1 
                {
                    return true;
                }
            }

            false
        };

        match side {
            Side::White => {
                add_normal_move(Dir::Up);
                add_normal_move(Dir::UpRight);
                add_normal_move(Dir::UpLeft);

                // check if the pawn can be promoted
                if piece.board_pos.y == BOARD_HEIGHT - 2 {
                    return moves.iter().map(|f| f.convert_to_promotion()).collect();
                }

                if let Some(change) = check_double_space_move(Dir::Up) {
                    moves.push(change);
                }

                // check for en passant
                if check_en_passant(Dir::Right) {
                    let board_change = MoveDestroy { 
                        start: piece.board_pos, 
                        end: selected_square.square_in_dir(Dir::UpRight).unwrap(),
                        target: selected_square.square_in_dir(Dir::Right).unwrap(),
                    };
                    moves.push(board_change);
                }

                if check_en_passant(Dir::Left) {
                    let board_change = MoveDestroy { 
                        start: piece.board_pos, 
                        end: selected_square.square_in_dir(Dir::UpLeft).unwrap(),
                        target: selected_square.square_in_dir(Dir::Left).unwrap(),
                    };
                    moves.push(board_change);
                }
            },
            Side::Black => {
                add_normal_move(Dir::Down);
                add_normal_move(Dir::DownRight);
                add_normal_move(Dir::DownLeft);

                // check if the pawn can be promoted
                if piece.board_pos.y == 1 {
                    return moves.iter().map(|f| f.convert_to_promotion()).collect();
                }

                if let Some(change) = check_double_space_move(Dir::Down) {
                    moves.push(change);
                }

                // check for en passant
                if check_en_passant(Dir::Right) {
                    let board_change = MoveDestroy { 
                        start: piece.board_pos, 
                        end: selected_square.square_in_dir(Dir::DownRight).unwrap(),
                        target: selected_square.square_in_dir(Dir::Right).unwrap(),
                    };
                    moves.push(board_change);
                }

                if check_en_passant(Dir::Left) {
                    let board_change = MoveDestroy { 
                        start: piece.board_pos, 
                        end: selected_square.square_in_dir(Dir::DownLeft).unwrap(),
                        target: selected_square.square_in_dir(Dir::Left).unwrap(),
                    };
                    moves.push(board_change);
                }
            },
        }

        moves
    }

    fn get_queen_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BChange>{
        let mut moves = Vec::new();
        let side = piece.side;

        let mut add_move = |dir: Dir| {
            let slide_moves = slide(dir, side, selected_square, self);
            moves.extend(slide_moves);
        };

        add_move(Dir::Up);
        add_move(Dir::Down);
        add_move(Dir::Left);
        add_move(Dir::Right);
        add_move(Dir::UpRight);
        add_move(Dir::UpLeft);
        add_move(Dir::DownRight);
        add_move(Dir::DownLeft);

        moves
    }

    fn get_rook_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BChange>{
        let mut moves = Vec::new();
        let side = piece.side;

        let mut add_move = |dir: Dir| {
            let slide_moves = slide(dir, side, selected_square, self);
            
            // check for push move 
            let last_square = match slide_moves.len() {
                0 => selected_square,
                _ => slide_moves.last().unwrap().click_pos_to_activate_change(),
            };

            // this means that it has to be an enemy
            if self.is_occupied(last_square) && slide_moves.len() > 0 {
                moves.extend(slide_moves);
                return;
            }

            if let Some(next_square) = last_square.square_in_dir(dir) {
                if self.is_occupied_and_friendly(next_square, piece.side) {
                    if let Some(push_move) = next_square.square_in_dir(dir){
                        if !self.is_occupied(push_move) {
                            let mut new_move = BothMove{
                                start1: selected_square,
                                start2: next_square,
                                end1: next_square,
                                end2: push_move,
                            };
                            
                            let Some(pushed_piece) = self.get_piece(next_square) else {
                                panic!("This should not panic")
                            };

                            if pushed_piece.piece_type == PieceType::Pawn {
                                match pushed_piece.side {
                                    Side::White => {
                                        if pushed_piece.board_pos.y == BOARD_HEIGHT - 2 {
                                            new_move = new_move.convert_to_promotion();
                                        }
                                    },
                                    Side::Black => {
                                        if pushed_piece.board_pos.y == 1 {
                                            new_move = new_move.convert_to_promotion();
                                        }
                                    },
                                }
                            }
                            
                            
                            
                            moves.push(new_move);
                        }
                    }
                }
            }

            moves.extend(slide_moves);
        };

        add_move(Dir::Up);
        add_move(Dir::Down);
        add_move(Dir::Left);
        add_move(Dir::Right);

        moves
    }

    fn get_bishop_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BChange>{
        let mut moves = Vec::new();
        let side = piece.side;

        let mut add_move = |dir: Dir| {
            let slide_moves = slide(dir, side, selected_square, self);
            moves.extend(slide_moves);
        };

        add_move(Dir::UpRight);
        add_move(Dir::UpLeft);
        add_move(Dir::DownRight);
        add_move(Dir::DownLeft);

        moves
    }

    fn get_knight_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BChange>{
        let mut moves = Vec::new();
        let side = piece.side;

        let mut add_move = |dir: Dir| {
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return,
            };

            match self.get_piece(square) {
                Some(target_piece) => {
                    if side.is_enemy(&target_piece.side) {
                        moves.push(
                            MoveDestroy { start: selected_square, end: square, target: square }
                        );
                    }
                },
                None => moves.push(Move { start: selected_square, end: square }),
            }
        };

        add_move(Dir::Custom(1, 2));
        add_move(Dir::Custom(2, 1));
        add_move(Dir::Custom(1, -2));
        add_move(Dir::Custom(2, -1));
        add_move(Dir::Custom(-1, 2));
        add_move(Dir::Custom(-2, 1));
        add_move(Dir::Custom(-1, -2));
        add_move(Dir::Custom(-2, -1));

        moves
    }
}
