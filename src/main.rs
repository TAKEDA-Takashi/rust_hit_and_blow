use rust_hit_and_blow::input_read_line;
use rust_hit_and_blow::numbers::Numbers;
use rust_hit_and_blow::validation::ValidationError;
use rust_hit_and_blow::validation::Validator;
use std::str::FromStr;

fn main() {
    const GAME_DIGIT: usize = 4;
    let validator = Validator::new(GAME_DIGIT);
    let model = Numbers::new(GAME_DIGIT);

    let mut game_clear = false;

    println!("{}桁の数字を当てよう！。", GAME_DIGIT);

    while !game_clear {
        let s = input_read_line().expect("Failed to read line");

        if let Err(e) = validator.validate(&s) {
            match e {
                ValidationError::MatchLength => println!("{}桁で入力してください。", GAME_DIGIT),
                ValidationError::NumString => println!("数字以外が入力されています。"),
                ValidationError::NotDuplicate => println!("重複した数字が入力されています。"),
            }
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
