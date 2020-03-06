use rust_hit_and_blow::*;

fn main() {
    const GAME_DIGIT: usize = 4;
    let model = get_random_numbers(GAME_DIGIT);

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

        let reply = string_to_u8vec(&s);
        let hit = count_hit(&model, &reply);
        let blow = count_blow(&model, &reply);

        println!("HIT: {}, BLOW: {}", hit, blow);

        if hit == GAME_DIGIT {
            game_clear = true;
        }
    }

    println!("ゲームクリア！");
}
