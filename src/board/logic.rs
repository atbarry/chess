use super::{Board, BoardPos, PieceType, Side, Piece};

#[derive(Clone, Copy)]
enum Dir{
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    UpRight(i32),
    UpLeft(i32),
    DownRight(i32),
    DownLeft(i32),
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
            Dir::Up(steps) => convert_to_option(x, y + steps),
            Dir::Down(steps) => convert_to_option(x, y - steps),
            Dir::Left(steps) => convert_to_option(x - steps, y),
            Dir::Right(steps) => convert_to_option(x + steps, y),
            Dir::UpRight(steps) => convert_to_option(x + steps, y + steps),
            Dir::UpLeft(steps) => convert_to_option(x - steps, y + steps),
            Dir::DownRight(steps) => convert_to_option(x + steps, y - steps),
            Dir::DownLeft(steps) => convert_to_option(x - steps, y - steps),
            Dir::Custom(x_steps, y_steps) => convert_to_option(x + x_steps, y + y_steps),
        }
    }
}


fn slide(dir: Dir, side: Side, start: BoardPos, board: &Board) -> Vec<BoardPos> {
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
                    moves.push(current_square);
                }
                break;
            },
            None => moves.push(current_square),
        }
    }

    moves
}



impl Board {
    pub fn is_valid_move(&self, start: BoardPos, target: BoardPos) -> bool {
        self.get_valid_moves(start).contains(&target)
    }

    pub fn get_valid_moves(&self, selected_square: BoardPos) -> Vec<BoardPos>{
        let piece = self.get_piece(selected_square).unwrap(); // if this fails we have a bug
        let side = piece.side;

        // not the correct turn then no moves
        if !self.is_turn(side) {
            return Vec::new();
        }
        
        match &piece.piece_type {
            PieceType::King => self.get_king_moves(selected_square, side),
            PieceType::Queen => self.get_queen_moves(selected_square, side),
            PieceType::Rook => self.get_rook_moves(selected_square, side),
            PieceType::Bishop => self.get_bishop_moves(selected_square, side),
            PieceType::Knight => self.get_knight_moves(selected_square, side),
            PieceType::Pawn => self.get_pawn_moves(selected_square, piece),
        }
    }

    fn get_king_moves(&self, selected_square: BoardPos, side: Side) -> Vec<BoardPos>{
        let mut moves = Vec::new();
        
        let mut add_move = |dir: Dir| {
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return,
            };
            
            match self.get_piece(square) {
                Some(target_piece) => {
                    if side.is_enemy(&target_piece.side) {
                        moves.push(square);
                    }
                },
                None => moves.push(square),
            }
        };
        
        add_move(Dir::Up(1));
        add_move(Dir::Down(1));
        add_move(Dir::Left(1));
        add_move(Dir::Right(1));
        add_move(Dir::UpRight(1));
        add_move(Dir::UpLeft(1));
        add_move(Dir::DownRight(1));
        add_move(Dir::DownLeft(1));


        moves
    }

    fn get_pawn_moves(&self, selected_square: BoardPos, piece: Piece) -> Vec<BoardPos>{
        let mut moves = Vec::new();
        let side = piece.side;

        let mut add_move = |dir: Dir| {
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return,
            };

            match self.get_piece(square) {
                Some(target_piece) =>  {
                    match &dir {
                        Dir::Up(_) | Dir::Down(_) => (),
                        _ => {
                            if side.is_enemy(&target_piece.side) {
                                moves.push(square);
                            }
                        }
                        
                    }
                },
                None => match &dir {
                    Dir::Up(_) => moves.push(square),
                    Dir::Down(_) => moves.push(square),
                    _ => (),
                },
            }
        };

        let check_en_passant = |dir: Dir| -> bool{
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return false,
            };

            if let Some(target_piece) = self.get_piece(square) {
                if side.is_enemy(&target_piece.side) {
                    if let PieceType::Pawn = target_piece.piece_type {
                        dbg!(target_piece.distance_moved);
                        if target_piece.distance_moved == 2 {
                            return true;
                        }
                    }
                }
            }

            false
        };


        match side {
            Side::White => {
                add_move(Dir::Up(1));
                add_move(Dir::UpRight(1));
                add_move(Dir::UpLeft(1));

                if piece.num_moves == 0 {
                    add_move(Dir::Up(2));
                }

                // check for en passant
                if check_en_passant(Dir::Right(1)) {
                    moves.push(selected_square.square_in_dir(Dir::UpRight(1)).unwrap());
                }

                if check_en_passant(Dir::Left(1)) {
                    moves.push(selected_square.square_in_dir(Dir::UpLeft(1)).unwrap());
                }
            },
            Side::Black => {
                add_move(Dir::Down(1));
                add_move(Dir::DownRight(1));
                add_move(Dir::DownLeft(1));

                if piece.num_moves == 0 {
                    add_move(Dir::Down(2));
                }

                // check for en passant
                if check_en_passant(Dir::Right(1)) {
                    moves.push(selected_square.square_in_dir(Dir::DownRight(1)).unwrap());
                }

                if check_en_passant(Dir::Left(1)) {
                    moves.push(selected_square.square_in_dir(Dir::DownLeft(1)).unwrap());
                }
            },
        }

        moves
    }

    fn get_queen_moves(&self, selected_square: BoardPos, side: Side) -> Vec<BoardPos>{
        let mut moves = Vec::new();

        let mut add_move = |dir: Dir| {
            let slide_moves = slide(dir, side, selected_square, self);
            moves.extend(slide_moves);
        };

        add_move(Dir::Up(1));
        add_move(Dir::Down(1));
        add_move(Dir::Left(1));
        add_move(Dir::Right(1));
        add_move(Dir::UpRight(1));
        add_move(Dir::UpLeft(1));
        add_move(Dir::DownRight(1));
        add_move(Dir::DownLeft(1));

        moves
    }

    fn get_rook_moves(&self, selected_square: BoardPos, side: Side) -> Vec<BoardPos>{
        let mut moves = Vec::new();

        let mut add_move = |dir: Dir| {
            let slide_moves = slide(dir, side, selected_square, self);
            moves.extend(slide_moves);
        };

        add_move(Dir::Up(1));
        add_move(Dir::Down(1));
        add_move(Dir::Left(1));
        add_move(Dir::Right(1));

        moves
    }

    fn get_bishop_moves(&self, selected_square: BoardPos, side: Side) -> Vec<BoardPos>{
        let mut moves = Vec::new();

        let mut add_move = |dir: Dir| {
            let slide_moves = slide(dir, side, selected_square, self);
            moves.extend(slide_moves);
        };

        add_move(Dir::UpRight(1));
        add_move(Dir::UpLeft(1));
        add_move(Dir::DownRight(1));
        add_move(Dir::DownLeft(1));

        moves
    }

    fn get_knight_moves(&self, selected_square: BoardPos, side: Side) -> Vec<BoardPos>{
        let mut moves = Vec::new();

        let mut add_move = |dir: Dir| {
            let square = match selected_square.square_in_dir(dir) {
                Some(square) => square,
                None => return,
            };

            match self.get_piece(square) {
                Some(target_piece) => {
                    if side.is_enemy(&target_piece.side) {
                        moves.push(square);
                    }
                },
                None => moves.push(square),
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
