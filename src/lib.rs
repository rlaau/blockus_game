mod display_generate_block;
pub use display_generate_block::RAND_MAPS; 
pub use display_generate_block::*;
use std::io::{self, Write};
use std::io::Cursor;

pub use display_generate_block::{random_generate_block, display_block}; 
pub fn initialize_board ()-> Vec<Vec<char>>{
    let board = vec![
    vec!['X', 'X', 'X', 'X', 'X', 'X','X', 'X', 'X','X', 'X', 'X','X'],
    vec!['X', 'X', 'X', 'X', 'X', 'X','X', 'X', 'X','X', 'X', 'X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', '_', '_', '_', '_','_', '_', '_','_','_','X','X'],
    vec!['X', 'X', 'X', 'X', 'X', 'X','X', 'X', 'X','X', 'X', 'X','X'],
    vec!['X', 'X', 'X', 'X', 'X', 'X','X', 'X', 'X','X', 'X', 'X','X'],
];
    return board; 
}

pub fn display_board (board: &Vec<Vec<char>>){
    let size = board.len();

    // 열 번호를 출력합니다.
    print!("   ");
    for col in 1..=size-4 {
        print!("{} ", col);
    }
    println!();

    // 상단 경계선을 출력합니다.
    print!("  ");
    for _ in 0..size-4 {
        print!(" _");
    }
    println!();

    // 각 행을 출력합니다.
    for (i, row) in board.iter().enumerate() {
        if i<2 || i> 10{
            continue;
        }
        // 행 번호 출력
        print!("{} ", i -1);

        // 각 셀을 출력하며 `|` 틀 안에 맞춥니다.
        print!("|");  // 시작 경계
        let mut col=-1;
        for &cell in row {
            col=col+1;
            if col <2 || col>10 {
                continue;
            }

            print!("{}|", cell);  // 각 셀을 `|`로 구분하여 출력
        }
        println!();  // 행의 끝에 줄바꿈
    }
}








pub fn do_turns(board:Vec<Vec<char>>){
    let user:bool=true;



}






#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use std::io::Cursor;

    #[test]
    fn test_initialize_board() {
        let board = initialize_board();
        assert_eq!(board.len(), 13);  // 행의 수가 13인지 확인
        assert_eq!(board[0].len(), 13);  // 열의 수가 13인지 확인

        // 특정 위치의 값 확인 (예: 테두리와 내부의 값)
        assert_eq!(board[0][0], 'X');
        assert_eq!(board[2][2], '_');
    }



    #[test]
    fn test_do_turns() {
        let board = initialize_board();
        do_turns(board);  // 실제 게임 턴이 진행되는지 확인
        // 함수 실행이 오류 없이 완료되는지 확인하기 위함
    }
}