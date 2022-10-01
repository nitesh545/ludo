use rand::Rng;
use std::{thread, time::Duration};
use std::io;
use std::io::Read;
use std::string::ToString;
use colored::Colorize;


fn main() {
    let mut input_string: String = String::new();
    let mut quit: String = String::from("quit");
    
    // 4 players 4 paths
    let mut vec1: Vec<u8> = Vec::new();
    let mut vec2: Vec<u8> = Vec::new();
    let mut vec3: Vec<u8> = Vec::new();
    let mut vec4: Vec<u8> = Vec::new();
    
    let mut victory_message: String = String::from("Victory!!");
    
    init_path(&mut vec1);
    init_path(&mut vec2);
    init_path(&mut vec3);
    init_path(&mut vec4);
    
    // which players turn
    let mut turn: u8 = 0;
    
    let mut current_position_p1: usize = 1;
    let mut current_position_p2: usize = 1;
    let mut current_position_p3: usize = 1;
    let mut current_position_p4: usize = 1;
    
    while true {
        input_string = wait_for_response();
        if input_string.trim() == quit {
            break;
        }
        
        if turn % 4 == 0 {
            let mut moved: u8 = 0;
            let steps = get_step_count();
            move_steps_pawn(&mut vec1, steps, &mut moved, &mut current_position_p1, "player 1".to_string());
            if current_position_p1 >= vec1.len()-1 {
                println!("player 1 {}", victory_message);
                break;
            }
            if current_position_p1 == current_position_p4 {
                vec4[current_position_p4] = 0;
                vec4[0] = 1;
            }
            if current_position_p1 == current_position_p2 {
                vec2[current_position_p2] = 0;
                vec2[0] = 1;
            }
            if current_position_p1 == current_position_p3 {
                vec3[current_position_p3] = 0;
                vec3[0] = 1;
            }
        }
        else if turn % 4 == 1 {
            let mut moved: u8 = 0;
            let steps = get_step_count();
            move_steps_pawn(&mut vec2, steps, &mut moved, &mut current_position_p2, "player 2".to_string());
            if current_position_p2 >= vec2.len()-1 {
                println!("player 2 {}", victory_message);
                break;
            }
            if current_position_p2 == current_position_p1 {
                vec1[current_position_p1] = 0;
                vec1[0] = 1;
            }
            if current_position_p2 == current_position_p4 {
                vec4[current_position_p4] = 0;
                vec4[0] = 1;
            }
            if current_position_p2 == current_position_p3 {
                vec3[current_position_p3] = 0;
                vec3[0] = 1;
            }
        }
        else if turn % 4 == 2 {
            let mut moved: u8 = 0;
            let steps = get_step_count();
            move_steps_pawn(&mut vec3, steps, &mut moved, &mut current_position_p3, "player 3".to_string());
            if current_position_p3 >= vec3.len()-1 {
                println!("player 3 {}", victory_message);
                break;
            }
            if current_position_p3 == current_position_p1 {
                vec1[current_position_p1] = 0;
                vec1[0] = 1;
            }
            if current_position_p3 == current_position_p2 {
                vec2[current_position_p2] = 0;
                vec2[0] = 1;
            }
            if current_position_p3 == current_position_p4 {
                vec4[current_position_p4] = 0;
                vec4[0] = 1;
            }
        }
        else if turn % 4 == 3 {
            let mut moved: u8 = 0;
            let steps = get_step_count();
            move_steps_pawn(&mut vec4, steps, &mut moved, &mut current_position_p4, "player 4".to_string());
            if current_position_p4 >= vec4.len()-1 {
                println!("player 4 {}", victory_message);
                break;
            }
            if current_position_p4 == current_position_p1 {
                vec1[current_position_p1] = 0;
                vec1[0] = 1;
            }
            if current_position_p4 == current_position_p2 {
                vec2[current_position_p2] = 0;
                vec2[0] = 1;
            }
            if current_position_p4 == current_position_p3 {
                vec3[current_position_p3] = 0;
                vec3[0] = 1;
            }
        }
        
        turn += 1;
    }
    println!("Total turns: {}", turn);
}

// create vector to show movements of 1 on zeroes
fn init_path(vec: &mut Vec<u8>) {
    for i in 0..56 {
        vec.push(0);
    }
    vec[0] = 1;
}

// generate a random number between 1 and 12
fn get_step_count() -> usize {
    let mut rng = rand::thread_rng();
    let mut steps: usize = rng.gen_range(1..=12);
    steps
}

// move 1 on zeroes
fn move_steps_pawn(vec: &mut Vec<u8>, steps: usize, moved: &mut u8, cur_pos: &mut usize, player_name: String) {
    if steps + *cur_pos < vec.len() {
        for i in 0..steps {
            thread::sleep(Duration::from_millis(10));
            let clr: String = "\x1B[2J\x1B[1;1H".to_string();
            println!("{}", clr);
            let pos = vec.iter().position(|&x| x == 1).expect("1 not found");
            *cur_pos = pos+1;
            vec[pos] = 0;
            vec[pos + 1] = 1;
            *moved += 1;
            println!("steps: {}, moved: {}, current position: {}, {}", steps, moved, cur_pos, player_name.yellow().bold());
            println!("field: {:?}", vec);
        }
    }
    else {
        println!("{}, Dice roll: {}, current position: {}, can take {} steps", player_name.red().bold(), steps, cur_pos, vec.len() - *cur_pos);
    }
}

// takes in input
fn wait_for_response() -> String{
    let mut input_string: String = String::new();
    io::stdin().read_line(&mut input_string).expect("failed to read line");
    input_string
}