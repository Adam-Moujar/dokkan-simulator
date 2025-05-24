#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
enum Condition {
    NONE,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
enum BuffClass {
    HP,
    ATK,
    DEF,
}
#[derive(Clone)]
struct Buff {
    condition: Condition,
    class: BuffClass,
    boost: f64,
}
#[derive(Clone)]
struct Unit {
    name: String,

    base_hp: u32,
    base_atk: u32,
    base_def: u32,

    hp: f64,
    atk: f64,
    def: f64,

    leader_skill: Vec<Buff>,
}

impl Unit {
    fn load_goku() -> Self {
        Unit {
            name: "UR PHY GOKU".to_string(),

            base_hp: 14243,
            base_atk: 13017,
            base_def: 9648,

            hp: 14243_f64,
            atk: 13017_f64,
            def: 9648_f64,

            leader_skill: vec![
                Buff {
                    condition: Condition::NONE,
                    class: BuffClass::HP,
                    boost: 0.5,
                },
                Buff {
                    condition: Condition::NONE,
                    class: BuffClass::ATK,
                    boost: 0.5,
                },
                Buff {
                    condition: Condition::NONE,
                    class: BuffClass::DEF,
                    boost: 0.5,
                },
            ],
        }
    }
    fn load_saibaman() -> Self {
        Unit {
            name: "N PHY Saibaman".to_string(),

            base_hp: 507,
            base_atk: 312,
            base_def: 290,

            hp: 507_f64,
            atk: 312_f64,
            def: 290_f64,

            leader_skill: vec![],
        }
    }

    fn load_yamcha() -> Self {
        Unit {
            name: "R TEQ Yamcha".to_string(),

            base_hp: 3120,
            base_atk: 2242,
            base_def: 2348,

            hp: 3120_f64,
            atk: 2242_f64,
            def: 2348_f64,

            leader_skill: vec![],
        }
    }

    fn print(&self) {
        println!("--------------------------");
        println!("Unit name is: {:?}", self.name);
        println!("------- Base Stats -------");
        println!("Base Hp : {}", self.base_hp);
        println!("Base Atk: {}", self.base_atk);
        println!("Base Def: {}", self.base_def);
        println!("------ Current Stats -----");
        println!("Hp : {}", self.hp);
        println!("Atk: {}", self.atk);
        println!("Def: {}", self.def);
        println!("------ Leader Skill ------");
        self.print_leader_skill();
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn print_leader_skill(&self) {
        if self.leader_skill.is_empty() {
            println!("None");
            return;
        }
        for line in &self.leader_skill {
            match line.condition {
                Condition::NONE => print!("ALL TYPES "),
            }
            match line.class {
                BuffClass::HP => {
                    println!("HP UP BY {} PERCENT", (line.boost * 100_f64) as u32);
                }
                BuffClass::ATK => {
                    println!("ATK UP BY {} PERCENT", (line.boost * 100_f64) as u32);
                }
                BuffClass::DEF => {
                    println!("DEF UP BY {} PERCENT", (line.boost * 100_f64) as u32);
                }
            }
        }
    }

    fn apply_leader_skill(&self, team: &Team) -> Unit {
        let leader = team.get(0).unwrap();
        let sub_leader = team.get(1).unwrap();
        let mut res = self.clone();
        res.hp = res.base_hp as f64;
        res.atk = res.base_atk as f64;
        res.def = res.base_def as f64;
        for line in [&leader.leader_skill[..], &sub_leader.leader_skill[..]].concat() {
            match line.condition {
                Condition::NONE => (),
            }
            match line.class {
                BuffClass::HP => res.hp += line.boost * res.base_hp as f64,
                BuffClass::ATK => res.atk += line.boost * res.base_atk as f64,
                BuffClass::DEF => res.def += line.boost * res.base_def as f64,
            }
        }
        res
    }
}

type Team = Vec<Unit>;

fn load_team() -> Team {
    let mut team: Team = vec![];
    team.push(Unit::load_goku());
    team.push(Unit::load_goku());
    for _ in 2..7 {
        team.push(Unit::load_saibaman());
    }
    team
}

fn use_leader_skill(team: &Team) -> Team {
    team.iter()
        .map(|unit| unit.apply_leader_skill(team))
        .collect()
}

fn print_team(team: &Team) {
    for (i, unit) in team.iter().enumerate() {
        println!("{}.", i + 1);
        unit.print();
    }
}

fn print_total_health(team: &Team) {
    let total_hp = team.iter().fold(0_f64, |acc, unit| acc + unit.hp);
    println!("-------------------");
    println!("Total HP: {}", total_hp as u32);
}
fn main() {
    let team = load_team();
    let team = use_leader_skill(&team);
    print_team(&team);
    print_total_health(&team);
}
