use Problem_1::*;


fn main() {
    //! 최종적으론 이거 싹다 lib에 넣기!!

    //이니셜라이즈
    let mut board= initialize_board();
    display_board(&board);
    println!();
    let mut user= true;

    //한 턴 시작 (보드 프린트-> 블록 생성 -> 검증 및 코디네이트(될 때까지 루프)-> 유저체인지 )
    'game: while true {
        let number_block= random_generate_block(&RAND_MAPS);
        let laminated_block = laminate_block(user, &number_block);
        let user_str= if user {"P1"} else {"P2"};
        let counter= if user {"P2"} else {"P1"};
        println!("{user_str}'s block:");
        display_block(&laminated_block);
        let is_possible= possibility_check(&board, &number_block);
        if !is_possible {
            println!("{user_str} fails to put the block. {counter} wins!");
            break 'game;
        }

    //입력-> 코디네이트
        'coordinate: while true {
            let (block, (i,j))= handle_input_and_return_block_ij(user, &number_block);
            let is_validate=validate_coordination(&board, &block, i, j);
            if is_validate {
                coordinate_block(&mut board, &block, i, j);
                println!("coordinated!");
                //유저 체인지
                user=!user;
                display_board(&board);
                break 'coordinate;
            }
            else {
                println!("{user_str}'s block:");
                println!("it's not validate! try again");
                display_block(&laminated_block);}
    }
    
}
}
