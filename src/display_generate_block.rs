use rand::Rng;
use lazy_static::lazy_static;
use std::{collections::btree_map::Values, io::{self, Write}, vec};

pub enum Input {
    Coordinates(i32, i32),
    Rotate,
    Invalid,
}


lazy_static! {
    pub static ref RAND_MAPS: Vec<Vec<Vec<i32>>> = vec![
    vec![
        vec![1, 1, 1], 
        vec![0, 0, 0],
        vec![0, 0, 0]],  // 블록 1
    vec![
        vec![1, 1, 1], 
        vec![1, 0, 0], 
        vec![0, 0, 0]],  // 블록 2
    vec![
        vec![1, 1, 1], 
        vec![0, 0, 1], 
        vec![0, 0, 0]],  // 블록 3
    vec![
        vec![1, 1, 1], 
        vec![0, 1, 0], 
        vec![0, 0, 0]],  // 블록 4
    vec![
        vec![0, 1, 1], 
        vec![1, 1, 0], 
        vec![0, 0, 0]],  // 블록 5
    vec![
        vec![1, 1, 0], 
        vec![0, 1, 1], 
        vec![0, 0, 0]],  // 블록 6
    vec![
        vec![1, 1, 0], 
        vec![1, 1, 0], 
        vec![0, 0, 0]],  // 블록 7
    vec![
        vec![1, 0, 0], 
        vec![1, 1, 0], 
        vec![1, 1, 0]],  // 블록 8
    vec![
        vec![0, 1, 1], 
        vec![1, 1, 0], 
        vec![1, 0, 0]],  // 블록 9
    vec![
        vec![1, 1, 0], 
        vec![0, 1, 1], 
        vec![0, 1, 0]],  // 블록 10
    vec![
        vec![0, 1, 1], 
        vec![1, 1, 0], 
        vec![0, 1, 0]],  // 블록 11
    vec![
        vec![0, 0, 1], 
        vec![1, 1, 1], 
        vec![1, 0, 0]],  // 블록 12
    vec![
        vec![1, 0, 0], 
        vec![1, 1, 1], 
        vec![0, 0, 1]],  // 블록 13
    vec![
        vec![0, 1, 0], 
        vec![1, 1, 1], 
        vec![0, 1, 0]],  // 블록 14 
];
}

pub fn random_generate_block (rand_maps:&Vec<Vec<Vec<i32>>>)->Vec<Vec<i32>>{
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..13);
    let rand_block = &rand_maps[rand_num];
    let rand_copy= rand_block.clone();
    return rand_copy
}

pub fn laminate_block(user: bool, number_block: &Vec<Vec<i32>>) -> Vec<Vec<char>> {
    let symbol = if user { 'O' } else { '@' };

    let laminated_block: Vec<Vec<char>> = number_block
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|val| if *val == 1 { symbol } else { '_' })
                .collect::<Vec<char>>() // 각 행(row)을 Vec<char>로 변환
        })
        .collect::<Vec<Vec<char>>>(); // 최종 결과를 Vec<Vec<char>>로 변환
    
    return laminated_block;
}

pub fn display_block (block: &Vec<Vec<char>>){

    // 상단 경계선을 출력합니다.
    for _ in 0..block.len() {
        print!(" _");
    }
    println!();
    for (_, row) in block.iter().enumerate() {
        // 열을 하나씩 출력하며 `|` 틀 안에 맞춥니다.
        print!("|");  // 시작 경계
        for &cell in row {
            print!("{}|", cell);  // 각 셀을 `|`로 구분하여 출력
        }
        println!();  // 행의 끝에 줄바꿈
    }

}

pub fn get_and_parse_input() -> Input {
    print!("Put your block (r c) or Rotate (O): ");
    io::stdout().flush().unwrap(); // 출력을 플러시하여 입력 프롬프트가 바로 보이게 함

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // 양쪽 공백 제거

    // "O" 인풋 처리
    if input == "O" {
        return Input::Rotate;
    }

    // "int int" 형태의 인풋 처리
    let mut parts = input.split_whitespace();
    if let (Some(x_str), Some(y_str)) = (parts.next(), parts.next()) {
        if let (Ok(x), Ok(y)) = (x_str.parse::<i32>(), y_str.parse::<i32>()) {
            return Input::Coordinates(x, y);
        }
    }

    // 위 조건을 모두 만족하지 않으면 Invalid 리턴
    Input::Invalid
}

pub fn handle_input_and_return_block_ij(user:bool, number_block:&Vec<Vec<i32>>)->(Vec<Vec<char>>,(i32,i32)) {
    let mut rotated_block= number_block.clone();
    loop {
        match get_and_parse_input() {
            Input::Coordinates(x, y) => {
                println!("Received coordinates: ({}, {})", x, y);
                if x>9|| x< -1 || y>9 ||y< -1 {
                    println!("Invalid input. Integer must be in range -1 to 9");
                }
                else {
                    //inner board는 2~10의 범위와 외부가 매칭 되므로, x와 y에 값을 +1해 주었음.
                    // 1행,1열에 위치시킨단 입력은 실은 i=2,j=2에 위치시키겠단 뜻과 동일.
                    let i= x+1;
                    let j= y+1;
                    return (laminate_block(user, &rotated_block), (i,j))
                }
            }
            Input::Rotate => {
                println!("Received rotation command: O");
                //블록 로테이션
                rotated_block= rotate_block(&rotated_block);
                //블록 디스플레이
                display_block(&laminate_block(user,&rotated_block));
                
            }
            Input::Invalid => {
                println!("Invalid input. Please enter 'int int' or 'O'.");
            }
        }
    }

}

pub fn rotate_block(number_block: &Vec<Vec<i32>>)-> Vec<Vec<i32>>{
    //그래프의 중간을 원점에 고정했다고 가정한 '수학적 좌표'. j,i를 x,y의 좌표로 치환
    let dot_xy_list = get_xy_list(number_block);
    // 좌표를 기준으로 회전 연산=> 좌표 리스트 리턴
    let rotated_list=get_rotated_xy(dot_xy_list);

    //좌표 리스트=> 그래프의 i,j리스트로 변환.
    let dot_ij_axis_list = rotated_list
    .iter()
    .map(|&(x, y)| {
        //행렬 연산에 근거함. x좌표를 (0,1)축에, y좌표를 (-1,0)축에 변환 
        let i = (1-y) as usize;
        let j = (x +1) as usize;  
        (i, j)
    })
    .collect::<Vec<(usize, usize)>>();// 반환 타입 명시

    //ij리스트=>  회전된 그래프 리턴
    let mut  rotated_matrix= vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]];
    for (i,j) in dot_ij_axis_list{
        rotated_matrix[i][j]=1;
    }
    return  rotated_matrix;

}

pub fn get_xy_list (number_block: &Vec<Vec<i32>>)-> Vec<(i32, i32)>{
    let mut dot_xy_axis_list = Vec::new();
    //()=> 그래프의 '1'인 요소의 '좌표 리스트' 리턴
    for (i, row) in number_block.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == 1 {
                let y_axis_dot:i32= (i as i32)*(-1)+1;
                let x_axis_dot= (j as i32)-1;
                dot_xy_axis_list.push((x_axis_dot, y_axis_dot));
            }
        }
    }
    return dot_xy_axis_list;
}

pub fn get_rotated_xy (dot_xy_list:Vec<(i32,i32)>)->Vec<(i32,i32)>{
    let rotated_list= dot_xy_list
    .iter()
    .map(|&(x, y)| {
        //행렬 연산에 근거함. x좌표를 (0,1)축에, y좌표를 (-1,0)축에 변환 
        let a = x * 0 + y * -1; // a = y * -1
        let b = x * 1 + y * 0;  // b = x
        (a, b)
    })
    .collect::<Vec<(i32, i32)>>(); // 반환 타입 명시
    return rotated_list;
}

pub fn validate_coordination(board:&Vec<Vec<char>>, block: &Vec<Vec<char>>, i: i32,j: i32)->bool{
    let mut scope:Vec<Vec<char>>= vec![vec!['_','_','_'],vec!['_','_','_'],vec!['_','_','_'],];
    let i_start = i as usize;
    let i_end= (i+2) as usize;
    let j_start= j as usize;
    let j_end= (j+2) as usize;
    
    //스코프 생성하기. 스코프= 코디네이트 될 지역 가져오기
    for (i,row) in board.iter().enumerate(){
        if i<i_start||i> i_end {continue;}
        for (j, &val) in row.iter().enumerate(){
            if j<j_start || j>j_end{ continue;}
            scope[i-i_start][j-j_start]= val;
        }
    }
    //display_block(&scope);

    //스코프에 대해서 점령 검증
    let mut is_conflict= false;
    'outer: for (i, row) in block.iter().enumerate(){
        for (j,_) in row.iter().enumerate(){
            if block[i][j]!='_' && scope[i][j] != '_' {
                is_conflict=true;
                //println!("conflict with i={i}, j={j}");
                break 'outer;
            }
            }
    }
    
    let is_validate= ! is_conflict;
    return is_validate;



}

pub fn coordinate_block (board:&mut Vec<Vec<char>>, block:&Vec<Vec<char>>,i:i32, j:i32){
    let start_i=i as usize;
    let start_j=j as usize;
    for (r, row) in block.iter().enumerate(){
        for (c, v) in row.iter().enumerate(){
            //블록의 '_'부분은 다른 지역 덮어씌우면 안됨.
            // 블록의 문자 부분 대해서만 반영.
            if *v!= '_' {
                board[start_i+r][start_j+c]=*v;
            }
        }
    }

}


pub fn possibility_check (board:&Vec<Vec<char>>, number_block: &Vec<Vec<i32>>)-> bool{

    //처음엔 가능한 케이스를 발견하지 못한 상태.
    let mut is_possible= false;

    //회전을 고려한 4가지 케이스 생성
    let mut block_versions: Vec<Vec<Vec<i32>>>= Vec::new();
    let mut rotated_case= number_block.clone();
    for _ in 0..4 {
        let copied= rotated_case.clone();
        block_versions.push(copied);
        rotated_case= rotate_block(&rotated_case);
    }

    let laminated_versions: Vec<Vec<Vec<char>>>= block_versions.iter().map(
        |number_block: &Vec<Vec<i32>>| laminate_block(true, number_block)).collect();

    //각 케이스에 대해 탐색
    'outer: for (_, l_version) in laminated_versions.iter().enumerate() {
        for i in 0..10 {
            for j in 0..10{
                //한 케이스에 대해서라도 맞으면 바로 리턴
                let is_validate= validate_coordination(board, l_version, i, j);
                if is_validate {
                    is_possible=true;
                    break 'outer;
                }
            }
        }
    }
    
    return is_possible
}

#[cfg(test)]
mod tests {
    use lazy_static::initialize;

    use crate::initialize_board;

    use super::*;
    #[test]
    fn test_get_xy_list(){
        let number_block =vec![
        vec![1,0,1],
        vec![1,0,0],
        vec![0,0,0]];
        let dox_xy_list=get_xy_list(&number_block);
        assert_eq!(dox_xy_list,vec![(-1,1),(1,1),(-1,0)]);
    }
    #[test]
    fn test_get_rotated_xy (){
        let befor_xy_list=vec![(-1,1),(1,1),(-1,0)];
        let rotated_xy_list = get_rotated_xy(befor_xy_list);
        assert_eq!(rotated_xy_list,vec![(-1,-1),(-1,1),(0,-1)]);
    }

    #[test]
    fn test_validate_check (){
        let board= initialize_board();
        let block= vec![
            vec!['_','_','_'],
            vec!['_','_','_'],
            vec!['l','l','l'],
        ];
        assert_eq!(true, validate_coordination(&board, &block, -1, 2));
    }

}