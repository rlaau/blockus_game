use Problem_1::*;


fn main() {
    let board= initialize_board();
    display_board(&board);
    println!();
    let number_block= random_generate_block(&RAND_MAPS);
    let user= true;
    let laminated_block = laminate_block(user, &number_block);
    let user_str= if user {"P1"} else {"P2"};
    println!("{user_str}'s block:");
    display_block(&laminated_block);
    handle_input_and_return_block(user, &number_block);
}
