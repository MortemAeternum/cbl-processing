#![warn(clippy::all)]
#![allow(clippy::cognitive_complexity)]

extern crate fxhash;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use fxhash::FxHashMap as HashMap;
use regex::Regex;
use std::{
    self,
    fmt,
    io::{self, prelude::*},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterBuild {
    // [Overview]
    name:       String,
    race:       Race,
    alignment:  Alignment,
    max_levels: u8,
    levels:     Vec<Class>,
    // [Stats]
    preferred_build_type: BuildType,
    adventurer_stats:     Option<Stats>,
    champion_stats:       Option<Stats>,
    hero_stats:           Option<Stats>,
    legend_stats:         Option<Stats>,
    stat_tomes:           Stats,
    stat_levelups:        [Option<Ability>; 7],
    // [Skills]
    skills: Skills,
    // [Feats]
    feats: Feats,
    // [Spells]
    spells: Spells,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Race {
    Aasimar,
    AasimarScourge,
    Bladeforged,
    DeepGnome,
    Dragonborn,
    Drow,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    Halfing,
    HalfOrc,
    Human,
    Morninglord,
    PurpleDragonKnight,
    ShadarKai,
    Tiefling,
    TieflingScoundrel,
    Warforged,
    WoodElf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    NeutralGood,
    TrueNeutral,
    ChaoticGood,
    ChaoticNeutral,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    FavoredSoul,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildType {
    Adventurer,
    Champion,
    Hero,
    Legend,
}

/// The struct stores the number of build points spent on each ability, **not**
/// the ability score itself. Or, in the case of tomes, it stores the obvious
/// values.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    str_pts: u8,
    dex_pts: u8,
    con_pts: u8,
    int_pts: u8,
    wis_pts: u8,
    cha_pts: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ability {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Skill {
    Balance,
    Bluff,
    Concentration,
    Diplomacy,
    DisableDevice,
    Haggle,
    Heal,
    Hide,
    Intimidate,
    Jump,
    Listen,
    MoveSilently,
    OpenLock,
    Perform,
    Repair,
    Search,
    Spellcraft,
    Spot,
    Swim,
    Tumble,
    UseMagicDevice,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Skills {
    /// Stores number of **points** put into each skill at each level, not the
    /// number of ranks. A missing entry means that no points were spent in
    /// that skill.
    skill_table: HashMap<Skill, [u8; 20]>,
    /// Stores skill tome values. A missing entry means no tomes for that skill
    /// were eaten.
    skill_tomes: HashMap<Skill, u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BaseFeatType {
    Standard,
    Legend,
    Class,
    Race,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecondaryFeatType {
    Heroic,
    Epic,
    Destiny,
    Legend,
    Class(Class),
    Race(Race),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Feat {
    pub secondary_type: SecondaryFeatType,
    /// **NOTE:** For class feats, this level is the level of the class when
    /// you get the feat, **not** your character level when you get the feat.
    pub level: u8,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Feats {
    pub standard_feats: Vec<Feat>,
    pub legend_feats:   Option<Feat>,
    pub class_feats:    Vec<Feat>,
    pub race_feats:     Vec<Feat>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spellbook {
    pub class:           Class,
    pub spells_by_level: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spells {
    pub spellbooks: [Option<Spellbook>; 3],
}

#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    UnknownHeader(String),
    DataBeforeFirstHeader,
    UnknownRace(String),
    NoRace,
    UnknownAlignment(String),
    NoAlignment,
    WrongLevelNumber(u8, usize),
    InvalidMaxLevel(u8),
    TooManyClasses,
    UnknownClass(String),
    InvalidLevelNum(u8),
    LevelsOutOfOrder,
    UndeclaredClass(Class),
    UnknownBuildType(String),
    BadLevelupLevel(usize),
    UnknownAbility(String),
    UnknownSkill(String),
    UnknownBaseFeatType(String),
    UnknownSecondaryFeatType(String),
    InvalidFeatLevel(u8),
    MultipleLegendFeats,
    TooManySpellbooks,
}

enum Heading {
    None,
    Overview,
    Stats,
    Skills,
    Feats,
    Spells,
    Enhancements,
}

impl std::str::FromStr for Race {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Aasimar" => Ok(Self::Aasimar),
            "Aasimar Scourge" => Ok(Self::AasimarScourge),
            "Bladeforged" => Ok(Self::Bladeforged),
            "Deep Gnome" => Ok(Self::DeepGnome),
            "Dragonborn" => Ok(Self::Dragonborn),
            "Drow" => Ok(Self::Drow),
            "Dwarf" => Ok(Self::Dwarf),
            "Elf" => Ok(Self::Elf),
            "Gnome" => Ok(Self::Gnome),
            "Half-Elf" => Ok(Self::HalfElf),
            "Halfing" => Ok(Self::Halfing),
            "Half-Orc" => Ok(Self::HalfOrc),
            "Human" => Ok(Self::Human),
            "Morninglord" => Ok(Self::Morninglord),
            "Purple Dragon Knight" => Ok(Self::PurpleDragonKnight),
            "Shadar-kai" => Ok(Self::ShadarKai),
            "Tiefling" => Ok(Self::Tiefling),
            "Tiefling Scoundrel" => Ok(Self::TieflingScoundrel),
            "Warforged" => Ok(Self::Warforged),
            "Wood Elf" => Ok(Self::WoodElf),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for Alignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lawful Good" => Ok(Self::LawfulGood),
            "Lawful Neutral" => Ok(Self::LawfulNeutral),
            "Neutral Good" => Ok(Self::NeutralGood),
            "True Neutral" => Ok(Self::TrueNeutral),
            "Chaotic Good" => Ok(Self::ChaoticGood),
            "Chaotic Neutral" => Ok(Self::ChaoticNeutral),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for Class {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Artificer" => Ok(Self::Artificer),
            "Barbarian" => Ok(Self::Barbarian),
            "Bard" => Ok(Self::Bard),
            "Cleric" => Ok(Self::Cleric),
            "Druid" => Ok(Self::Druid),
            "Favored Soul" => Ok(Self::FavoredSoul),
            "Fighter" => Ok(Self::Fighter),
            "Monk" => Ok(Self::Monk),
            "Paladin" => Ok(Self::Paladin),
            "Ranger" => Ok(Self::Ranger),
            "Rogue" => Ok(Self::Rogue),
            "Sorcerer" => Ok(Self::Sorcerer),
            "Warlock" => Ok(Self::Warlock),
            "Wizard" => Ok(Self::Wizard),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for BuildType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Adventurer" => Ok(Self::Adventurer),
            "Champion" => Ok(Self::Champion),
            "Hero" => Ok(Self::Hero),
            "Legend" => Ok(Self::Legend),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for Ability {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STR" | "Strength" => Ok(Ability::Str),
            "DEX" | "Dexterity" => Ok(Ability::Dex),
            "CON" | "Constitution" => Ok(Ability::Con),
            "INT" | "Intelligence" => Ok(Ability::Int),
            "WIS" | "Wisdom" => Ok(Ability::Wis),
            "CHA" | "Charisma" => Ok(Ability::Cha),
            _ => Err(()),
        }
    }
}

impl std::ops::Index<Ability> for Stats {
    type Output = u8;

    fn index(&self, ability: Ability) -> &Self::Output {
        match ability {
            Ability::Str => &self.str_pts,
            Ability::Dex => &self.dex_pts,
            Ability::Con => &self.con_pts,
            Ability::Int => &self.int_pts,
            Ability::Wis => &self.wis_pts,
            Ability::Cha => &self.cha_pts,
        }
    }
}

impl std::ops::IndexMut<Ability> for Stats {
    fn index_mut(&mut self, ability: Ability) -> &mut Self::Output {
        match ability {
            Ability::Str => &mut self.str_pts,
            Ability::Dex => &mut self.dex_pts,
            Ability::Con => &mut self.con_pts,
            Ability::Int => &mut self.int_pts,
            Ability::Wis => &mut self.wis_pts,
            Ability::Cha => &mut self.cha_pts,
        }
    }
}

impl Skill {
    fn from_7_chars(chars: &str) -> Option<Self> {
        match &chars[..3] {
            "Bal" => Some(Self::Balance),
            "Blu" => Some(Self::Bluff),
            "Con" => Some(Self::Concentration),
            "Dip" => Some(Self::Diplomacy),
            "Dis" => Some(Self::DisableDevice),
            "Hag" => Some(Self::Haggle),
            "Hea" => Some(Self::Heal),
            "Hid" => Some(Self::Hide),
            "Int" => Some(Self::Intimidate),
            "Jum" => Some(Self::Jump),
            "Lis" => Some(Self::Listen),
            "Mov" => Some(Self::MoveSilently),
            "Ope" => Some(Self::OpenLock),
            "Per" => Some(Self::Perform),
            "Rep" => Some(Self::Repair),
            "Sea" => Some(Self::Search),
            "Spe" => Some(Self::Spellcraft),
            "Spo" => Some(Self::Spot),
            "Swi" => Some(Self::Swim),
            "Tum" => Some(Self::Tumble),
            "UMD" => Some(Self::UseMagicDevice),
            _ => None,
        }
    }
}

impl Skills {
    /// Returns an array of points spent in the given skill for levels 1
    /// through 20. A return value of `None` means that no points were spent at
    /// any level.
    pub fn points_in_skill(&self, skill: Skill) -> Option<&[u8; 20]> {
        self.skill_table.get(&skill)
    }

    /// Returns how many skill points were spent on the given skill at the
    /// given level.
    pub fn points_in_skill_at_level(&self, skill: Skill, level: usize) -> u8 {
        self.skill_table
            .get(&skill)
            .map(|points| points[level])
            .unwrap_or(0)
    }

    /// Gets the value of the skill tome eaten for the given skill. `0` if no
    /// tome is eaten.
    pub fn tome(&self, skill: Skill) -> u8 {
        *self.skill_tomes.get(&skill).unwrap_or(&0)
    }
}

impl std::str::FromStr for BaseFeatType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Standard" => Ok(Self::Standard),
            "Legend" => Ok(Self::Legend),
            "Class" => Ok(Self::Class),
            "Race" => Ok(Self::Race),
            _ => Err(()),
        }
    }
}

impl Feat {
    pub fn new(
        secondary_type: SecondaryFeatType,
        level: u8,
        name: String,
    ) -> Self {
        Self {
            secondary_type,
            level,
            name,
        }
    }
}

impl Spells {
    /// Returns `Err` iff the spells are already full, i.e. there are already
    /// spellbooks for 3 separate classes and you've tried to insert a spell of
    /// a 4th class.
    ///
    /// This method uses debug-only assertions to check that `1 <= level <= 9`.
    pub fn insert(
        &mut self,
        class: Class,
        level: usize,
        name: String,
    ) -> Result<(), ()> {
        debug_assert!(1 <= level);
        debug_assert!(level <= 9);

        self.spellbooks
            .iter_mut()
            .find(|msb| {
                if let Some(sb) = msb {
                    sb.class == class
                } else {
                    true
                }
            })
            .and_then(|msb| {
                let sb = msb.get_or_insert_with(|| Spellbook {
                    class,
                    spells_by_level: Vec::with_capacity(5),
                });

                if sb.spells_by_level.len() < level {
                    sb.spells_by_level.resize_with(level, Vec::new);
                }
                sb.spells_by_level[level - 1].push(name);

                Some(())
            })
            .ok_or(())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::IoError(ioe) => ioe.fmt(f),
            Self::UnknownHeader(h) => write!(f, "Unknown header: [{}]", h),
            Self::DataBeforeFirstHeader =>
                f.write_str("Data found before first header"),
            Self::UnknownRace(r) => write!(f, "Unknown race: {}", r),
            Self::NoRace => f.write_str("No race specified"),
            Self::UnknownAlignment(a) => write!(f, "Unknown alignment: {}", a),
            Self::NoAlignment => f.write_str("No alignment specified"),
            Self::WrongLevelNumber(e, g) => write!(
                f,
                "Wrong number of class levels; expected {}, got {}",
                e, g,
            ),
            Self::InvalidMaxLevel(m) => write!(f, "Invalid max level: {}", m),
            Self::TooManyClasses =>
                f.write_str("More than three classes specified"),
            Self::UnknownClass(c) => write!(f, "Unknown class: {}", c),
            Self::InvalidLevelNum(l) =>
                write!(f, "Invalid level number: {}", l),
            Self::LevelsOutOfOrder =>
                f.write_str("Levels are listed out of order"),
            Self::UndeclaredClass(c) => write!(f, "Undeclared class: {:?}", c),
            Self::UnknownBuildType(b) =>
                write!(f, "Unknown build type: {}", b),
            Self::BadLevelupLevel(l) =>
                write!(f, "Bad ability score increase level: {}", l),
            Self::UnknownAbility(a) => write!(f, "Unknown ability: {}", a),
            Self::UnknownSkill(s) => write!(f, "Unknown skill: {}", s),
            Self::UnknownBaseFeatType(b) =>
                write!(f, "Unknown base feat type: {}", b),
            Self::UnknownSecondaryFeatType(s) =>
                write!(f, "Unknown secondary feat type: {}", s),
            Self::InvalidFeatLevel(l) =>
                write!(f, "Invalid feat level: {}", l),
            Self::MultipleLegendFeats => f.write_str("Multiple Legend feats"),
            Self::TooManySpellbooks => f.write_str(
                "Too many spellbooks (a build can have -- at most -- three)",
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl std::convert::From<io::Error> for ParseError {
    fn from(ioe: io::Error) -> Self {
        Self::IoError(ioe)
    }
}

pub fn parse<R: BufRead>(input: &mut R) -> Result<CharacterBuild, ParseError> {
    lazy_static! {
        // General regexps //
        static ref BLANK_RE: Regex = Regex::new(r"^\s*(;.*)?$").unwrap();
        static ref HEADING_RE: Regex =
            Regex::new(r"^\[([A-Z][A-Za-z]*)\]$").unwrap();

        // [Overview] regexps //
        static ref NAME_RE: Regex = Regex::new(r"^Name:\s+(.+)$").unwrap();
        static ref RACE_RE: Regex =
            Regex::new(r"^Race:\s+([A-Za-z -]+)$").unwrap();
        static ref ALIGNMENT_RE: Regex =
            Regex::new(r"^Alignment:\s+([A-Za-z ]+)$").unwrap();
        static ref MAX_LEVELS_RE: Regex =
            Regex::new(r"^MaxLevels:\s+([0-9]{1,2})$").unwrap();
        static ref CLASS_RE: Regex =
            Regex::new(r"^Class:\s+([A-Za-z ]+)$").unwrap();
        static ref LEVEL_RE: Regex =
            Regex::new(r"^Level:\s+([0-9]{1,2})\s+([A-Za-z ]+)$").unwrap();

        // [Stats] regexps //
        static ref PREFERRED_RE: Regex =
            Regex::new(r"^Preferred:\s+([A-Z][A-Za-z]+)$").unwrap();
        static ref NO_RE: Regex =
            Regex::new(r"^(Adventurer|Champion|Hero|Legend):\s+No$").unwrap();
        static ref ABILITY_RE: Regex =
            Regex::new(
                r"^(STR|DEX|CON|INT|WIS|CHA):  (  | [1-9]|[1-9][0-9])    (  | [1-9]|[1-9][0-9])    (  | [1-9]|[1-9][0-9])    (  | [1-9]|[1-9][0-9])     ( |[1-9])$"
            ).unwrap();
        static ref LEVELUP_RE: Regex =
            Regex::new(r"^Levelup:\s+([0-9]{1,2})\s+([A-Z][a-z]+)$").unwrap();

        // [Skills] regexps //
        static ref SKILL_RE: Regex =
            Regex::new(r"^([A-Za-z ]{7}):((  [1-9] | [1-9][0-9] |    )+)$")
                .unwrap();

        // [Feats] regexps //
        static ref FEAT_RE: Regex =
            Regex::new(
                r"^([A-Z][a-z]+):\s+([A-Z][A-Za-z -]*[A-Za-z])\s+([1-9][0-9]?)\s+(.+)$"
            ).unwrap();

        // [Spells] regexps //
        static ref SPELL_RE: Regex =
            Regex::new(
                r"^Spell:\s+([A-Z][A-Za-z ]*[A-Za-z])\s+([1-9])\s+(.+)$"
            ).unwrap();
    }

    ////////////////////////////////////////////////////////////////
    // [Overview]
    let mut name = String::new();
    let mut race: Option<Race> = None;
    let mut alignment: Option<Alignment> = None;
    let mut max_levels = 20;
    let mut classes: [Option<Class>; 3] = [None, None, None];
    let mut levels = Vec::with_capacity(20);

    // [Stats]
    let mut preferred_build_type = BuildType::Adventurer;
    let mut adventurer_stats = Some(Stats::default());
    let mut champion_stats = Some(Stats::default());
    let mut hero_stats = Some(Stats::default());
    let mut legend_stats = Some(Stats::default());
    let mut stat_tomes = Stats::default();
    let mut stat_levelups = [None, None, None, None, None, None, None];

    // [Skills]
    let mut skills = Skills::default();

    // [Feats]
    let mut feats = Feats::default();
    feats.standard_feats.reserve(8);

    // [Spells]
    let mut spells = Spells::default();
    ////////////////////////////////////////////////////////////////

    let mut heading = Heading::None;

    for line in input.lines() {
        let line = line?;

        if BLANK_RE.is_match(&line) {
            continue;
        }

        if let Some(heading_caps) = HEADING_RE.captures(&line) {
            match &heading_caps[1] {
                "Overview" => heading = Heading::Overview,
                "Stats" => heading = Heading::Stats,
                "Skills" => heading = Heading::Skills,
                "Feats" => heading = Heading::Feats,
                "Spells" => heading = Heading::Spells,
                "Enhancements" => heading = Heading::Enhancements,
                h => return Err(ParseError::UnknownHeader(h.to_owned())),
            }

            continue;
        }

        match heading {
            Heading::None => return Err(ParseError::DataBeforeFirstHeader),
            Heading::Overview =>
                if let Some(name_caps) = NAME_RE.captures(&line) {
                    name = name_caps[1].to_owned();
                } else if let Some(race_caps) = RACE_RE.captures(&line) {
                    let race_str = &race_caps[1];
                    race = Some(race_str.parse().map_err(|_| {
                        ParseError::UnknownRace(race_str.to_owned())
                    })?);
                } else if let Some(alignment_caps) =
                    ALIGNMENT_RE.captures(&line)
                {
                    let alignment_str = &alignment_caps[1];
                    alignment = Some(alignment_str.parse().map_err(|_| {
                        ParseError::UnknownAlignment(alignment_str.to_owned())
                    })?);
                } else if let Some(max_levels_caps) =
                    MAX_LEVELS_RE.captures(&line)
                {
                    // Unwrapping the `.parse()` since the regular expression
                    // guarantees the result to be `Ok`
                    max_levels = max_levels_caps[1].parse().unwrap();
                    if max_levels < 1 || max_levels > 30 {
                        return Err(ParseError::InvalidMaxLevel(max_levels));
                    }
                } else if let Some(class_caps) = CLASS_RE.captures(&line) {
                    let class_str = &class_caps[1];
                    let class = class_str.parse().map_err(|_| {
                        ParseError::UnknownClass(class_str.to_owned())
                    })?;

                    classes
                        .iter_mut()
                        .find(|c| c.is_none())
                        .ok_or(ParseError::TooManyClasses)?
                        .get_or_insert(class);
                } else if let Some(level_caps) = LEVEL_RE.captures(&line) {
                    // Unwrapping the `.parse()` since the regular expression
                    // guarantees the result to be `Ok`
                    let level_num = level_caps[1].parse().unwrap();
                    if level_num < 1 || level_num > 20 {
                        return Err(ParseError::InvalidLevelNum(level_num));
                    }
                    if level_num != (levels.len() + 1) as u8 {
                        return Err(ParseError::LevelsOutOfOrder);
                    }

                    let level_class_str = &level_caps[2];
                    let level_class =
                        level_class_str.parse().map_err(|_| {
                            ParseError::UnknownClass(
                                level_class_str.to_owned(),
                            )
                        })?;
                    if classes
                        .iter()
                        .find(|&&c| c == Some(level_class))
                        .is_none()
                    {
                        return Err(ParseError::UndeclaredClass(level_class));
                    }

                    levels.push(level_class);
                },
            Heading::Stats =>
                if let Some(preferred_caps) = PREFERRED_RE.captures(&line) {
                    let preferred_str = &preferred_caps[1];
                    preferred_build_type =
                        preferred_str.parse().map_err(|_| {
                            ParseError::UnknownBuildType(
                                preferred_str.to_owned(),
                            )
                        })?;
                } else if let Some(no_caps) = NO_RE.captures(&line) {
                    match no_caps[1].as_bytes()[0] {
                        b'A' => adventurer_stats = None,
                        b'C' => champion_stats = None,
                        b'H' => hero_stats = None,
                        b'L' => legend_stats = None,
                        _ => unreachable!(), // Unreachable due to regexp
                    }
                } else if let Some(ability_caps) = ABILITY_RE.captures(&line) {
                    // Unwrapping because the regexp ensures successful parse
                    let ability = ability_caps[1].parse().unwrap();

                    (&mut [
                        adventurer_stats.as_mut(),
                        champion_stats.as_mut(),
                        hero_stats.as_mut(),
                        legend_stats.as_mut(),
                        Some(&mut stat_tomes),
                    ])
                        .iter_mut()
                        .zip(2..)
                        .filter_map(|(maybe_stats, cg)| match maybe_stats {
                            Some(s) => Some((s, cg)),
                            _ => None,
                        })
                        .for_each(|(stats, cap_grp)| {
                            stats[ability] = ability_caps[cap_grp]
                                .trim_start()
                                .parse()
                                .unwrap_or(0); // CBL uses whitespace to mean 0
                        });
                } else if let Some(levelup_caps) = LEVELUP_RE.captures(&line) {
                    // Unwrapping because the regexp ensures successful parse
                    let levelup_level =
                        levelup_caps[1].parse::<usize>().unwrap();
                    if levelup_level == 0 {
                        // CBL stores "level 0" ability increases
                        continue;
                    }
                    if levelup_level % 4 != 0 {
                        return Err(ParseError::BadLevelupLevel(
                            levelup_level,
                        ));
                    }
                    let levelup_index = levelup_level / 4 - 1;

                    let levelup_ability_str = &levelup_caps[2];
                    stat_levelups[levelup_index] =
                        Some(levelup_ability_str.parse().map_err(|_| {
                            ParseError::UnknownAbility(
                                levelup_ability_str.to_owned(),
                            )
                        })?);
                },
            Heading::Skills =>
                if let Some(skill_caps) = SKILL_RE.captures(&line) {
                    let skill_str = &skill_caps[1];
                    let skill =
                        Skill::from_7_chars(skill_str).ok_or_else(|| {
                            ParseError::UnknownSkill(skill_str.to_owned())
                        })?;

                    let points_str = &skill_caps[2];
                    let mut points_array = [0; 20];
                    let mut array_is_empty = true;
                    for (i, bs) in
                        points_str.as_bytes().chunks_exact(4).enumerate()
                    {
                        // We can do this unchecked because the regexp ensures
                        // correctness
                        let s = unsafe { std::str::from_utf8_unchecked(bs) };

                        if let Ok(points) = s.trim().parse() {
                            if i < points_array.len() {
                                points_array[i] = points;
                                array_is_empty = false;
                            } else {
                                skills.skill_tomes.insert(skill, points);
                            }
                        }
                    }

                    if !array_is_empty {
                        skills.skill_table.insert(skill, points_array);
                    }
                },
            Heading::Feats =>
                if let Some(feat_caps) = FEAT_RE.captures(&line) {
                    let base_type_str = &feat_caps[1];
                    let base_type = base_type_str.parse().map_err(|_| {
                        ParseError::UnknownBaseFeatType(
                            base_type_str.to_owned(),
                        )
                    })?;

                    let secondary_type_str = &feat_caps[2];
                    let secondary_type = match base_type {
                        BaseFeatType::Standard => match secondary_type_str {
                            "Heroic" => Some(SecondaryFeatType::Heroic),
                            "Epic" => Some(SecondaryFeatType::Epic),
                            "Destiny" => Some(SecondaryFeatType::Destiny),
                            _ => None,
                        },
                        BaseFeatType::Legend =>
                            if secondary_type_str == "Legend" {
                                Some(SecondaryFeatType::Legend)
                            } else {
                                None
                            },
                        BaseFeatType::Class => secondary_type_str
                            .parse()
                            .ok()
                            .map(SecondaryFeatType::Class),
                        BaseFeatType::Race => secondary_type_str
                            .parse()
                            .ok()
                            .map(SecondaryFeatType::Race),
                    }
                    .ok_or_else(|| {
                        ParseError::UnknownSecondaryFeatType(
                            secondary_type_str.to_owned(),
                        )
                    })?;

                    // Unwrapping parse since regexp ensures success
                    let feat_level = feat_caps[3].parse().unwrap();
                    if feat_level < 1 || feat_level > 30 {
                        return Err(ParseError::InvalidFeatLevel(feat_level));
                    }

                    let feat_name = feat_caps[4].to_owned();

                    let feat =
                        Feat::new(secondary_type, feat_level, feat_name);
                    match base_type {
                        BaseFeatType::Standard =>
                            feats.standard_feats.push(feat),
                        BaseFeatType::Legend =>
                            if feats.legend_feats.is_none() {
                                feats.legend_feats = Some(feat);
                            } else {
                                return Err(ParseError::MultipleLegendFeats);
                            },
                        BaseFeatType::Class => feats.class_feats.push(feat),
                        BaseFeatType::Race => feats.race_feats.push(feat),
                    }
                },
            Heading::Spells =>
                if let Some(spell_caps) = SPELL_RE.captures(&line) {
                    let class_str = &spell_caps[1];
                    let class: Class = class_str.parse().map_err(|_| {
                        ParseError::UnknownClass(class_str.to_owned())
                    })?;

                    // Unwrapping parse since regexp guarantees success
                    let spell_level = spell_caps[2].parse().unwrap();

                    let spell_name = spell_caps[3].to_owned();

                    spells
                        .insert(class, spell_level, spell_name)
                        .map_err(|_| ParseError::TooManySpellbooks)?;
                },
            _ => {},
        }
    }

    Ok(CharacterBuild {
        // [Overview]
        name,
        race: race.ok_or(ParseError::NoRace)?,
        alignment: alignment.ok_or(ParseError::NoAlignment)?,
        max_levels,
        levels: if levels.len() == max_levels.min(20) as usize {
            levels
        } else if levels.is_empty()
            && classes.iter().filter(|c| c.is_some()).count() == 1
        {
            let single_class = classes[0].unwrap();

            vec![single_class; max_levels as usize]
        } else {
            return Err(ParseError::WrongLevelNumber(
                max_levels.min(20),
                levels.len(),
            ));
        },

        // [Stats]
        preferred_build_type,
        adventurer_stats,
        champion_stats,
        hero_stats,
        legend_stats,
        stat_tomes,
        stat_levelups,

        // [Skills]
        skills,

        // [Feats]
        feats,

        // [Spells]
        spells,
    })
}

mod tests {
    use super::*;

    #[test]
    pub fn wartrapper_test() {
        let file = std::fs::File::open(
            "./test/brd15_ftr4_rog1_-_drw_-_trapper_-_WC_StD_-_THFs_none_BPlat_-_Wartrapper.build"
        ).unwrap();
        let mut buf_reader = std::io::BufReader::new(file);

        let mut skills = Skills::default();
        let mut set_and_forget = [0; 20];
        set_and_forget[0] = 4;
        skills.skill_table.insert(Skill::Balance, set_and_forget);
        skills
            .skill_table
            .insert(Skill::Concentration, set_and_forget);
        let mut dd = [2; 20];
        dd[0] = 4;
        skills.skill_table.insert(Skill::DisableDevice, dd);
        skills.skill_table.insert(
            Skill::Heal,
            [4, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2, 2, 1],
        );
        skills.skill_table.insert(Skill::Jump, set_and_forget);
        skills.skill_table.insert(Skill::OpenLock, set_and_forget);
        skills.skill_table.insert(
            Skill::Perform,
            [4, 3, 0, 2, 1, 1, 1, 1, 1, 1, 0, 0, 1, 3, 0, 2, 1, 1, 1, 1],
        );
        skills.skill_table.insert(Skill::Search, dd);
        skills.skill_table.insert(
            Skill::Spot,
            [4, 2, 1, 3, 2, 2, 2, 2, 2, 2, 1, 1, 4, 2, 1, 3, 2, 2, 2, 2],
        );
        skills.skill_table.insert(Skill::Tumble, set_and_forget);
        skills.skill_table.insert(
            Skill::UseMagicDevice,
            [4, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );

        assert_eq!(
            parse(&mut buf_reader).unwrap(),
            CharacterBuild {
                name: "Wartrapper".to_owned(),
                race: Race::Drow,
                alignment: Alignment::TrueNeutral,
                max_levels: 20,
                levels: vec![
                    Class::Rogue,
                    Class::Bard,
                    Class::Fighter,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Fighter,
                    Class::Fighter,
                    Class::Bard,
                    Class::Bard,
                    Class::Fighter,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                    Class::Bard,
                ],
                preferred_build_type: BuildType::Adventurer,
                adventurer_stats: Some(Stats {
                    str_pts: 10,
                    dex_pts: 0,
                    con_pts: 6,
                    int_pts: 6,
                    wis_pts: 0,
                    cha_pts: 6,
                }),
                champion_stats: None,
                hero_stats: None,
                legend_stats: None,
                stat_tomes: Stats::default(),
                stat_levelups: [
                    Some(Ability::Str),
                    Some(Ability::Str),
                    Some(Ability::Str),
                    Some(Ability::Str),
                    Some(Ability::Str),
                    Some(Ability::Str),
                    Some(Ability::Str),
                ],
                skills,
                feats: Feats {
                    standard_feats: vec![
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            1,
                            "Power Attack".to_owned(),
                        ),
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            3,
                            "Force of Personality".to_owned(),
                        ),
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            6,
                            "Great Cleave".to_owned(),
                        ),
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            9,
                            "Extend Spell".to_owned(),
                        ),
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            12,
                            "Quicken Spell".to_owned(),
                        ),
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            15,
                            "Two Handed Fighting".to_owned(),
                        ),
                        Feat::new(
                            SecondaryFeatType::Heroic,
                            18,
                            "Improved Bardic Music".to_owned(),
                        ),
                    ],
                    legend_feats:   None,
                    class_feats:    vec![
                        Feat::new(
                            SecondaryFeatType::Class(Class::Fighter),
                            1,
                            "Cleave".to_owned()
                        ),
                        Feat::new(
                            SecondaryFeatType::Class(Class::Fighter),
                            2,
                            "Improved Critical: Slashing".to_owned()
                        ),
                        Feat::new(
                            SecondaryFeatType::Class(Class::Fighter),
                            4,
                            "Improved Two Handed Fighting".to_owned()
                        )
                    ],
                    race_feats:     Vec::new(),
                },
                spells: Spells {
                    spellbooks: [
                        Some(Spellbook {
                            class:           Class::Bard,
                            spells_by_level: vec![
                                vec![
                                    "Cure Light Wounds".to_owned(),
                                    "Focusing Chant".to_owned(),
                                    "Expeditious Retreat".to_owned(),
                                    "Remove Fear".to_owned(),
                                ],
                                vec![
                                    "Blur".to_owned(),
                                    "Cure Moderate Wounds".to_owned(),
                                    "Heroism".to_owned(),
                                    "Invisibility".to_owned(),
                                ],
                                vec![
                                    "Cure Serious Wounds".to_owned(),
                                    "Displacement".to_owned(),
                                    "Haste".to_owned(),
                                    "Good Hope".to_owned(),
                                ],
                                vec![
                                    "Cure Critical Wounds".to_owned(),
                                    "Freedom of Movement".to_owned(),
                                    "Break Enchantment".to_owned(),
                                    "Dimension Door".to_owned(),
                                ],
                                vec![
                                    "Greater Heroism".to_owned(),
                                    "Mass Cure Light Wounds".to_owned(),
                                    "Shadow Walk".to_owned(),
                                ],
                            ],
                        }),
                        None,
                        None,
                    ],
                }
            }
        );
    }
}
