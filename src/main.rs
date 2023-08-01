pub mod enums;

use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;


struct Player {
    health: i32,
    attack: i32,
    defense: i32,
}

struct Monster {
    health: i32,
    attack: i32,
    defense: i32,
}

fn main() {
    let mut player = Player {
        health: 100,
        attack: 10,
        defense: 5,
    };

    loop {
        println!("请选择操作：");
        println!("1. 显示玩家状态");
        println!("2. 购买攻击装备");
        println!("3. 购买防御装备");
        println!("4. 寻找怪物打怪");
        println!("5. 退出游戏");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取用户输入时出错");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效的选项，请重新输入");
                continue;
            }
        };

        match choice {
            1 => show_player_status(&player),
            2 => buy_attack_equipment(&mut player),
            3 => buy_defense_equipment(&mut player),
            4 => {
                let mut monster = generate_monster();
                show_monster_status(&monster);

                loop {
                    println!("请选择操作：");
                    println!("1. 显示玩家状态");
                    println!("2. 显示怪物状态");
                    println!("3. 逃跑");
                    println!("4. 攻击");

                    let mut inner_input = String::new();
                    io::stdin()
                        .read_line(&mut inner_input)
                        .expect("读取用户输入时出错");

                    let inner_choice: u32 = match inner_input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("无效的选项，请重新输入");
                            continue;
                        }
                    };

                    match inner_choice {
                        1 => show_player_status(&player),
                        2 => show_monster_status(&monster),
                        3 => {
                            println!("你逃跑了！");
                            break;
                        }
                        4 => {
                            let result = attack_monster(&mut player, &mut monster);
                            if monster.health <= 0 {
                                println!("你击败了怪物！");
                                break;
                            } else if !result {
                                println!("你被怪物击败！");
                                thread::sleep(Duration::from_secs(5));
                                println!("游戏结束");
                                return;
                            }
                        }
                        _ => {
                            println!("无效的选项，请重新输入");
                        }
                    }
                }
            }
            5 => {
                println!("游戏结束");
                return;
            }
            _ => {
                println!("无效的选项，请重新输入");
            }
        }
    }
}

fn show_player_status(player: &Player) {
    println!("玩家状态：");
    println!("体力: {}", player.health);
    println!("攻击力: {}", player.attack);
    println!("防御力: {}", player.defense);
}

fn show_monster_status(monster: &Monster) {
    println!("怪物状态：");
    println!("血量: {}", monster.health);
    println!("攻击力: {}", monster.attack);
    println!("防御力: {}", monster.defense);
}

fn buy_attack_equipment(player: &mut Player) {
    // 假设购买攻击装备增加攻击力 5
    player.attack += 5;
    println!("购买攻击装备成功，攻击力增加！");
}

fn buy_defense_equipment(player: &mut Player) {
    // 假设购买防御装备增加防御力 3
    player.defense += 3;
    println!("购买防御装备成功，防御力增加！");
}

fn generate_monster() -> Monster {
    let mut rng = rand::thread_rng();

    Monster {
        health: rng.gen_range(50..100),
        attack: rng.gen_range(5..15),
        defense: rng.gen_range(2..10),
    }
}

fn attack_monster(player: &mut Player, monster: &mut Monster) -> bool {
    let player_damage = player.attack - monster.defense;
    let monster_damage = monster.attack - player.defense;

    if player_damage > 0 {
        println!("你攻击了怪物，造成 {} 点伤害", player_damage);
        monster.health -= player_damage;
    } else {
        println!("你的攻击对怪物无效！");
    }

    if monster_damage > 0 {
        println!("怪物攻击了你，造成 {} 点伤害", monster_damage);
        player.health -= monster_damage;
    } else {
        println!("怪物的攻击对你无效！");
    }

    if player.health <= 0 {
        return false;
    }

    true
}
