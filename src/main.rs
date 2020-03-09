use rust_hit_and_blow::*;
use std::str::FromStr;

fn main() {
    const GAME_DIGIT: usize = 4;
    let model = Numbers::new(GAME_DIGIT);

    let mut game_clear = false;

    while !game_clear {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        let s = input.trim();

        if !is_match_length(&s, GAME_DIGIT) {
            println!("{}桁で入力してください。", GAME_DIGIT);
            continue;
        }

        if !is_num_string(&s) {
            println!("数字以外が入力されています。");
            continue;
        }

        if is_duplicate(&s) {
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
