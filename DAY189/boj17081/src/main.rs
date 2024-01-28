use std::cell::Cell;
use std::collections::HashSet;
use std::fmt::{Display, Write};
use std::io::*;

use GameObject::*;
use Item::*;
use Accessories::*;

#[derive(PartialEq)]
enum GameObject {
    Player,
    Empty,
    Wall,
    Chest(Item),
    Trap,
    Enemy(Monster),
}

impl Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Player   => write!(f, "@"),
                Empty    => write!(f, "."),
                Wall     => write!(f, "#"),
                Chest(_) => write!(f, "B"),
                Trap     => write!(f, "^"),
                Enemy(boss) if boss.is_boss => write!(f, "M"),
                Enemy(_) => write!(f, "&"),
            }
    }
}

#[derive(PartialEq)]
enum Item {
    None,
    Weapon(i32),
    Armor(i32),
    Accessory(Accessories),
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Accessories {
    HPRegeneration,
    Reincarnation,
    Courage,
    Experience,
    Dexterity,
    Hunter,
    Cursed,
}

#[derive(PartialEq, Debug)]
struct Monster {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
    exp: u32,
    is_boss: bool,
}

impl Monster {
    fn new(is_boss: bool) -> Self {
        Self {
            name: String::new(),
            hp: 0,
            atk: 0,
            def: 0,
            exp: 0,
            is_boss,
        }
    }

    fn create_normal() -> Self {
        Self::new(false)
    }

    fn create_boss() -> Self {
        Self::new(true)
    }

    fn init(&mut self, name: &str, atk: i32, def: i32, hp: i32, exp: u32) {
        self.name.push_str(name);
        self.atk = atk;
        self.def = def;
        self.hp = hp;
        self.exp = exp;
    }
}

#[derive(Debug)]
struct Player {
    level: u32,
    hp: i32,
    max_hp: i32,
    atk: i32,
    weapon_atk: i32,
    def: i32,
    armor_def: i32,
    exp: u32,
    accesssories: HashSet<Accessories>,
}

impl Player {
    fn new() -> Self {
        Self {
            level: 1,
            hp: 20,
            max_hp: 20,
            atk: 2,
            weapon_atk: 0,
            def: 2,
            armor_def: 0,
            exp: 0,
            accesssories: HashSet::with_capacity(4),
        }
    }

    fn level_up(&mut self) {
        self.exp = 0;
        self.level += 1;
        self.atk += 2;
        self.def += 2;
        self.max_hp += 5;
        self.hp = self.max_hp;
    }

    fn has(&self, accessory: Accessories) -> bool {
        self.accesssories.contains(&accessory)
    }

    fn take_off(&mut self, accessory: Accessories) {
        self.accesssories.remove(&accessory);
    }

    fn attacked(&mut self, damage: i32) -> bool {
        self.hp -= damage;
        if self.hp < 0 { self.hp = 0 }
        self.hp != 0
    }

    fn attacked_by_spike_trap(&mut self) -> bool {
        if self.has(Dexterity) {
            self.attacked(1)
        } else {
            self.attacked(5)
        }
    }

    fn battle(&mut self, monster: &Monster) -> bool {
        let mut battle_turn = 0_usize;

        let mut monster_hp = monster.hp;
        let mut player_alive = true;
        let mut monster_alive = true;

        let monster_damage = (monster.atk - self.def - self.armor_def).max(1);

        if monster.is_boss && self.has(Hunter) {
            self.hp = self.max_hp;
        }

        while player_alive && monster_alive {
            let player_damage = (
                if battle_turn == 0 && self.has(Courage) {
                    if self.has(Dexterity) { 3 } else { 2 }
                } else { 1 } * (self.atk + self.weapon_atk) - monster.def
            ).max(1);

            monster_hp -= player_damage;
            monster_alive = monster_hp > 0;

            if monster_alive && !(monster.is_boss && battle_turn == 0 && self.has(Hunter)) {
                player_alive = self.attacked(monster_damage);
            }

            battle_turn += 1;
        }

        if player_alive {
            self.exp += monster.exp * if self.has(Experience) { 12 } else { 10 } / 10;

            if self.exp >= self.level * 5 {
                self.level_up();
            }

            if self.has(HPRegeneration) {
                self.hp = self.max_hp.min(self.hp + 3);
            }
        }

        player_alive
    }

    fn set_weapon(&mut self, atk: i32) {
        self.weapon_atk = atk;
    }

    fn set_armor(&mut self, def: i32) {
        self.armor_def = def;
    }

    fn set_accessory(&mut self, other: Accessories) {
        if self.accesssories.len() < 4 {
            self.accesssories.insert(other);
        }
    }

    fn info(&self) -> String {
        format!("LV : {}\n\
                 HP : {}/{}\n\
                 ATT : {}+{}\n\
                 DEF : {}+{}\n\
                 EXP : {}/{}",
                 self.level,
                 self.hp, self.max_hp,
                 self.atk, self.weapon_atk,
                 self.def, self.armor_def,
                 self.exp, self.level * 5)
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) }
    }

    let (n, m) = next!(usize, usize);
    let mut player = Player::new();
    let mut player_x = 0;
    let mut player_y = 0;
    let mut enemy_count = 0;
    let mut chest_count = 0;

    let mut grid: Vec<_> = (0..n)
        .map(|i| {
            let mut v = Vec::with_capacity(m);

            for (j, c) in next!().bytes().enumerate() {
                v.push(
                    match c {
                        b'.' => Empty,
                        b'#' => Wall,
                        b'^' => Trap,
                        b'@' => {
                            player_x = i;
                            player_y = j;
                            Empty
                        },
                        b'&' => {
                            enemy_count += 1;
                            Enemy(Monster::create_normal())
                        },
                        b'M' => {
                            enemy_count += 1;
                            Enemy(Monster::create_boss())
                        },
                        b'B' => {
                            chest_count += 1;
                            Chest(None)
                        },
                        _    => unreachable!(),
                    }
                )
            }

            v
        })
        .collect();

    let command = next!().bytes();

    for _ in 0..enemy_count {
        let (r, c) = next!(usize, usize);
        let s = next!();
        let (w, a, h, e) = next!(i32, i32, i32, u32);

        if let Enemy(monster) = &mut grid[r - 1][c - 1] {
            monster.init(s, w, a, h, e);
        }
    }

    for _ in 0..chest_count {
        let (r, c) = next!(usize, usize);
        let t = next!();

        if let Chest(item) = &mut grid[r - 1][c - 1] {
            *item = match t {
                "W" => {
                    let s = next!(i32);
                    Weapon(s)
                },
                "A" => {
                    let s = next!(i32);
                    Armor(s)
                },
                "O" => {
                    let s = next!();
                    Accessory(
                        match s {
                            "HR" => HPRegeneration,
                            "RE" => Reincarnation,
                            "CO" => Courage,
                            "EX" => Experience,
                            "DX" => Dexterity,
                            "HU" => Hunter,
                            "CU" => Cursed,
                            _    => unreachable!(),
                        }
                    )
                },
                _ => unreachable!(),
            }
        }
    }

    let home_x = player_x;
    let home_y = player_y;
    let player_alive = Cell::new(true);
    let boss_alive = Cell::new(true);
    let mut game_turn = 0_usize;
    let mut end_message = "Press any key to continue.".to_string();

    for c in command.take_while(|_| player_alive.get() && boss_alive.get()) {
        let mut nx = player_x;
        let mut ny = player_y;

        match c {
            b'L' => ny -= 1,
            b'R' => ny += 1,
            b'U' => nx -= 1,
            b'D' => nx += 1,
            _    => unreachable!(),
        }

        if nx >= n || ny >= m || grid[nx][ny] == Wall {
            nx = player_x;
            ny = player_y;
        }

        match &grid[nx][ny] {
            Empty => {},
            Chest(item) => {
                match item {
                    Weapon(atk) => player.set_weapon(*atk),
                    Armor(def) => player.set_armor(*def),
                    Accessory(accessories) => player.set_accessory(*accessories),
                    None => unreachable!(),
                }

                grid[nx][ny] = Empty;
            },
            Trap => {
                if !player.attacked_by_spike_trap() {
                    player_alive.set(false);

                    if !player.has(Reincarnation) {
                        end_message = "YOU HAVE BEEN KILLED BY SPIKE TRAP..".to_string();
                    }
                }
            },
            Enemy(monster) => {
                if player.battle(monster) {
                    if monster.is_boss {
                        boss_alive.set(false);
                        end_message = "YOU WIN!".to_string();
                    } else {
                        grid[nx][ny] = Empty;
                    }
                } else {
                    player_alive.set(false);
                    
                    if !player.has(Reincarnation) {
                        end_message = format!("YOU HAVE BEEN KILLED BY {}..", monster.name);
                    }
                }
            },
            _ => unreachable!(),
        }

        if !player_alive.get() && player.has(Reincarnation) {
            player.take_off(Reincarnation);
            player_alive.set(true);
            player.hp = player.max_hp;
            player_x = home_x;
            player_y = home_y;
        } else {
            player_x = nx;
            player_y = ny;
        }

        game_turn += 1;
    }

    if player_alive.get() {
        grid[player_x][player_y] = Player;
    }

    let output_grid = grid
        .into_iter()
        .map(|v| {
            let mut buf = String::with_capacity(m);
            for obj in v {
                let _ = write!(buf, "{obj}");
            }
            buf
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!(
        "{}\nPassed Turns : {}\n{}\n{}",
        output_grid,
        game_turn,
        player.info(),
        end_message,
    );
}
