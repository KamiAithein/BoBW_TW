use crate::descr_battle::{DescrBattle, Faction, Representation, AllianceSide, Settlement};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::collections::HashSet;

pub fn serialize(battle: DescrBattle, output: &Path) -> Result<(),std::io::Error> {
    let mut out = File::create(output)?;
    out.write(b"; Custom battle script generated Mayo's Battle Scenario Generator\r\n\r\n")?;

    out.write(format!("battle\t\t{name}\t\tmultiplayer\r\n", name=battle.name).as_bytes())?;

    let mut playable = HashSet::<&Faction>::new();
    let mut unplayable = HashSet::<&Faction>::new();
    for alliance in &battle.alliances {
        for faction in &alliance.factions {
            if faction.playable {
                playable.insert(&faction.faction);
            } else {
                unplayable.insert(&faction.faction);
            }
        }
    }

    out.write(b"playable\r\n");
    for p in playable {
        out.write(format!("\t{pname}\r\n", pname=p.repr()).as_bytes());
    }
    out.write(b"end\r\n");

    out.write(b"nonplayable\r\n");
    for up in unplayable {
        out.write(format!("\t{upname}\r\n", upname=up.repr()).as_bytes());
    }
    out.write(b"end\r\n");

    out.write(b"\r\n\r\n");

    out.write(format!("start_date\t{year} {season}\r\n", year=battle.start_date.year, season=battle.start_date.season.repr()).as_bytes());
    out.write(format!("end_date\t{year} {season}\r\n", year=battle.end_date.year, season=battle.end_date.season.repr()).as_bytes());

    out.write(b"\r\n\r\n");
    out.write(b";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;\r\n");
    out.write(b"; >>>> start of factions section <<<<\r\n\r\n");

    for alliance in &battle.alliances {
        for faction in &alliance.factions {
            out.write(format!("faction {faction}\r\n", faction=faction.faction.repr()).as_bytes());

            match &faction.settlement {
                None => {}
                Some(settlement) => {
                    out.write(b"settlement\r\n{\r\n");
                    out.write(format!("\tlevel {level}\r\n", level=settlement.level.repr()).as_bytes());
                    out.write(format!("\ttile {x} {y}\r\n\r\n", x=settlement.tile.0, y=settlement.tile.1).as_bytes());
                    out.write(format!("\tyear_founded {year}\r\n\r\n", year=settlement.year_founded).as_bytes());
                    out.write(format!("\tturns_owned {turns}\r\n", turns=settlement.turns_owned).as_bytes());
                    out.write(format!("\tfortification {fort} {style} towers {towers}\r\n",
                                      fort=settlement.fortification, style=settlement.fortification_style.repr(), towers=settlement.num_towers).as_bytes());
                    out.write(format!("\tpopulation {pop}\r\n", pop=settlement.population).as_bytes());
                    out.write(format!("\tplan_set {plan}\r\n", plan=settlement.plan_set.repr()).as_bytes());
                    out.write(format!("\tfaction_creator {fc}\r\n", fc=settlement.faction_creator.repr()).as_bytes());
                    out.write(format!("package_path {packpath}\r\n", packpath=settlement.package_path.display()).as_bytes());

                    out.write(b"}\r\n");
                }
            }

            let leader = &faction.army.leader;
            out.write(format!("character\t{character}, general, {gender}, age {age}, x {x}, y {y}\r\n",
                                    character=leader.name, gender=leader.gender.repr(), age=leader.age, x=leader.pos.0, y=leader.pos.1).as_bytes());
            out.write(b"army\r\n");
            for unit in &faction.army.soldiers {
                out.write(format!("unit\t\t{unit}\t\t\t\t", unit=unit.unit_type.repr()).as_bytes());
                out.write(format!("soldiers {soldiers} exp {exp} armour {armour} weapon_lvl {weapon_lvl}\r\n",
                                    soldiers=unit.count, exp=unit.exp, armour=unit.armour, weapon_lvl=unit.weapon_lvl).as_bytes());
            }
            if faction.army.sieging {
                out.write(b"sieging\r\n");
            }
            out.write(b"\r\n\r\n");
        }
    }
    out.write(b"\r\n");
    out.write(b";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;\r\n");
    out.write(b";>>>> start of battle section <<<<\r\n\r\n");

    out.write(format!("battle\t{b1}, {b2}\r\n", b1=battle.battle.0, b2=battle.battle.1).as_bytes());
    out.write(format!("battle_time\t{start}\t{end}\r\n", start=format!("{:.2}",battle.battle_time.0), end=format!("{:.2}",battle.battle_time.1)).as_bytes());
    out.write(format!("weather\t{vis} {fore}\r\n\r\n", vis=battle.weather.0.repr(), fore=battle.weather.1.repr()).as_bytes());

    out.write(format!("home_faction\t{faction}\r\n", faction=battle.home_faction.repr()).as_bytes());
    for alliance in &battle.alliances {
        out.write(format!("alliance\tcan_deploy\tcan_view\t").as_bytes());
        for faction in &alliance.factions{
            out.write(format!("{faction}, ", faction=faction.faction.repr()).as_bytes());
        }
        out.write(format!("{side}\r\n", side=alliance.side.repr()).as_bytes());
    }

    out.write(b"\r\n");

    for alliance in &battle.alliances {
        for (i, faction) in alliance.factions.iter().enumerate() {
            out.write(format!("army\t{faction}, {army_num}, no_withdraw, supporting_armies 0\r\n\r\n", faction=faction.faction.repr(), army_num=i).as_bytes());
            for depl_point in &faction.deployment_area_points {
                out.write(format!("deployment_area_point\t{x}, {y}\r\n", x=depl_point.0, y=depl_point.1).as_bytes());
            }
            out.write(b"\r\n");
            let mut i = 0;
            for unit in &faction.army.soldiers {
                out.write(format!("unit\t\t\t{i}, {x}, {y}, {z}, formation_type {form}, formation_width {form_w}\r\n",
                                  i=&i, x=unit.pos.0, y=unit.pos.1, z=unit.pos.2, form=unit.formation.formation_type.repr(), form_w=unit.formation.formation_width).as_bytes());
                i += 1;
            }
            out.write(b"\r\n");
        }
    }

    out.write(b";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;\r\n");
    out.write(b"; >>>> start of objectives scripting section <<<<\r\n\r\n");

    out.write(b"objectives\r\n\r\n");

    for (i, alliance) in battle.alliances.iter().enumerate() {
        out.write(format!("alliance {alliance_n}\r\n", alliance_n=i).as_bytes());
        out.write(format!("condition {condition}\r\n\r\n", condition=alliance.objective.repr()).as_bytes());
    }


    Ok(())
}