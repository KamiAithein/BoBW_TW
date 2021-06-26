mod descr_battle;
mod deserializer;
mod serialize;

use descr_battle::{*};
use std::path::{Path, PathBuf};
use serialize::serialize;

fn main() {
    println!("Hello, world!");
    let teut = Alliance{
        factions: vec![
            DescrFaction{
                faction: Faction::TeutonicOrder,
                settlement: None,
                army: Army {
                    leader: Leader {
                        name: "Athalwolf von_Lubeck",
                        gender: Gender::Male,
                        age: 25,
                        pos: (14, 95)
                    },
                    soldiers: vec![
                        Unit{
                            unit_type: UnitType::TOLateBodyguard,
                            pos: (148.927, 708.470, -166.),
                            formation: Formation { formation_type: FormationType::Square, formation_width: 39.099 },
                            count: 40,
                            exp: 0,
                            armour: 0,
                            weapon_lvl: 0
                        },
                        Unit{
                            unit_type: UnitType::TOLateBodyguard,
                            pos: (148.927, 708.470, -166.),
                            formation: Formation { formation_type: FormationType::Square, formation_width: 39.099 },
                            count: 40,
                            exp: 0,
                            armour: 0,
                            weapon_lvl: 0
                        }
                    ],
                    sieging: true
                },
                deployment_area_points: vec![
                    (672.973, 807.287),
                    (-91.195, 802.618),
                    (-140.922, 803.873),
                    (-137.871, 695.340),
                    (570.182, 710.661),
                    (589.937, 140.352),
                    (690.719, 143.428),
                ],
                playable: true
            },
        ],
        side: AllianceSide::Attacking,
        objective: Objective::CaptureMajorSettlement
    };
    let lith = Alliance{
        factions: vec![
            DescrFaction{
                faction: Faction::Lithuania,
                settlement: Some(Settlement{
                    level: SettlementLevel::Village,
                    tile: (14, 95),
                    year_founded: 1,
                    turns_owned: 10,
                    fortification: 0,
                    fortification_style: FortificationStyle::SouthernEuropean,
                    num_towers: 0,
                    population: 1090,
                    plan_set: PlanSet::DefaultSet,
                    faction_creator: Faction::Spain,
                    package_path: PathBuf::from("settlements/South_European/Settlements/Village/south_european_village_A.world")
                }),
                army: Army {
                    leader: Leader {
                        name: "Alexander",
                        gender: Gender::Male,
                        age: 22,
                        pos: (14, 95)
                    },
                    soldiers: vec![
                        Unit{
                            unit_type: UnitType::EELateBodyguard,
                            pos: (-239.457, -187.192, -138.),
                            formation: Formation { formation_type: FormationType::Square, formation_width: 45.810 },
                            count: 20,
                            exp: 0,
                            armour: 0,
                            weapon_lvl: 0
                        }
                    ],
                    sieging: false
                },
                deployment_area_points: vec![
                    (31.302, -110.000),
                    (40.451, -213.133),
                    (-274.316, -207.440),
                    (-273.507, 139.014),
                    (-191.068, 140.221),
                    (-189.669, -97.748)
                ],
                playable: true
            }
        ],
        side: AllianceSide::Defending,
        objective: Objective::DestroyOrRoutEnemy
    };

    let battle = DescrBattle {
        name: "(2v2)_custom_generated",
        start_date: Date{ year: 1410, season: Season::Summer },
        end_date: Date{ year: 1410, season: Season::Summer },
        battle: (14, 95),
        battle_time: (8.0, 12.0),
        weather: (Visibility::Clear, Forecast::Arid),
        home_faction: Faction::Lithuania,
        alliances: vec![
            teut,
            lith,
        ]
    };
    serialize(battle, Path::new("descr_battle.txt"));
}
