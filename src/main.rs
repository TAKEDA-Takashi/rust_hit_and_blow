use rust_hit_and_blow::*;
use rust_hit_and_blow::validation;
use std::str::FromStr;

fn main() {
    const GAME_DIGIT: usize = 4;
    let model = Numbers::new(GAME_DIGIT);

    let mut game_clear = false;

    println!("{}桁の数字を当てよう！。", GAME_DIGIT);

    while !game_clear {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let s = input.trim();

        if !validation::is_match_length(&s, GAME_DIGIT) {
            println!("{}桁で入力してください。", GAME_DIGIT);
            continue;
        }

        if !validation::is_num_string(&s) {
            println!("数字以外が入力されています。");
            continue;
        }

        if validation::is_duplicate(&s) {
            println!("重複した数字が入力されています。");
            continue;
        }

        if let Ok(reply) = Numbers::from_str(&s) {
            let hit = model.count_hit(&reply).unwrap();
            let blow = model.count_blow(&reply).unwrap();

            println!("HIT: {}, BLOW: {}", hit, blow);

            if hit == GAME_DIGIT {
                game_clear = true;
            }
        }
    }

    println!("ゲームクリア！");
}
