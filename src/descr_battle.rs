use std::path::{Path, PathBuf};

pub trait Representation {
    fn repr(&self) -> String;
}

pub enum PlanSet {
    DefaultSet
}

impl Representation for PlanSet {
    fn repr(&self) -> String {
        match self {
            PlanSet::DefaultSet => "default_set"
        }.to_string()
    }
}

pub enum SettlementLevel {
    Village
}

impl Representation for SettlementLevel {
    fn repr(&self) -> String {
        match self {
            SettlementLevel::Village => "village"
        }.to_string()
    }
}

pub enum FortificationStyle {
    SouthernEuropean
}
impl Representation for FortificationStyle {
    fn repr(&self) -> String {
        match self {
            FortificationStyle::SouthernEuropean => "southern_european"
        }.to_string()
    }
}

pub struct Settlement {
    pub level: SettlementLevel,
    pub tile: (u32, u32),
    pub year_founded: u32,
    pub turns_owned: u32,
    pub fortification: u32, //TODO DONT UNDERSTAND!
    pub fortification_style: FortificationStyle,
    pub num_towers: u32,
    pub population: u32,
    pub plan_set: PlanSet,
    pub faction_creator: Faction,
    pub package_path: PathBuf,
}

pub enum Gender {
    Male
}

impl Representation for Gender {
    fn repr(&self) -> String {
        match self {
            Gender::Male => "male"
        }.to_string()
    }
}

pub struct Leader<'leader> {
    pub name: &'leader str,
    pub gender: Gender,
    pub age: u32,
    pub pos: (i32, i32)
}

pub enum UnitType {
    Ritterbruder,
    Knechten,
    PrussianArchers,
    TartarLancers,
    TOLateBodyguard,
    EELateBodyguard,
}

impl Representation for UnitType {
    fn repr(&self) -> String {
        match self {
            UnitType::Ritterbruder => "Ritterbruder",
            UnitType::Knechten => "Knechten",
            UnitType::PrussianArchers => "Prussian Archers",
            UnitType::TartarLancers => "Tartar Lancers",
            UnitType::TOLateBodyguard => "TO Late Bodyguard",
            UnitType::EELateBodyguard => "EE Late Bodyguard"
        }.to_string()
    }
}

pub enum FormationType {
    Square
}

impl Representation for FormationType {
    fn repr(&self) -> String {
        match self {
            FormationType::Square => "square",
        }.to_string()
    }
}

pub struct Formation {
    pub formation_type: FormationType,
    pub formation_width: f32
}
pub struct Unit {
    pub unit_type: UnitType,
    pub pos: (f32, f32, f32),
    pub formation: Formation,
    pub count: u32,
    pub exp: u32,
    pub armour: u32,
    pub weapon_lvl: u32
}
pub struct Army<'army> {
    pub leader: Leader<'army>,
    pub soldiers: Vec<Unit>,
    pub sieging: bool,
}
#[derive(Eq, PartialEq, Hash)]
pub enum Faction {
    TeutonicOrder,
    Lithuania,
    Poland,
    Spain,
}

impl Representation for Faction {
    fn repr(&self) -> String {
        match self {
            Faction::TeutonicOrder => "teutonic_order",
            Faction::Lithuania => "lithuania",
            Faction::Poland => "poland",
            Faction::Spain => "spain",
        }.to_string()
    }
}

pub struct DescrFaction<'faction> {
    pub faction: Faction,
    pub settlement: Option<Settlement>,
    pub army: Army<'faction>,
    pub deployment_area_points: Vec<(f32, f32)>,
    pub playable: bool,
}

pub enum Season {
    Summer
}

impl Representation for Season {
    fn repr(&self) -> String {
        match self {
            Season::Summer => "summer",
        }.to_string()
    }
}

pub struct Date {
    pub year: u32,
    pub season: Season,
}

pub enum Visibility {
    Clear
}

impl Representation for Visibility {
    fn repr(&self) -> String {
        match self {
            Visibility::Clear => "clear"
        }.to_string()
    }
}

pub enum Forecast {
    Arid
}

impl Representation for Forecast {
    fn repr(&self) -> String {
        match self {
            Forecast::Arid => "arid"
        }.to_string()
    }
}

pub enum AllianceSide {
    Attacking,
    Defending,
}

impl Representation for AllianceSide {
    fn repr(&self) -> String {
        match self {
            AllianceSide::Attacking => "attacking",
            AllianceSide::Defending => "defending",
        }.to_string()
    }
}

pub enum Objective {
    DestroyOrRoutEnemy,
    CaptureMajorSettlement,
}

impl Representation for Objective {
    fn repr(&self) -> String {
        match self {
            Objective::DestroyOrRoutEnemy => "destroy_or_rout_enemy",
            Objective::CaptureMajorSettlement => "capture_major_settlement",
        }.to_string()
    }
}

pub struct Alliance<'alliance> {
    pub factions: Vec<DescrFaction<'alliance>>,
    pub side: AllianceSide,
    pub objective: Objective
}

pub struct DescrBattle<'battle> {
    pub name: &'battle str,
    pub start_date: Date,
    pub end_date: Date,

    pub battle: (u32, u32), //TODO no idea what this is
    pub battle_time: (f32, f32),
    pub weather: (Visibility, Forecast),
    pub home_faction: Faction,
    pub alliances: Vec<Alliance<'battle>>,
}