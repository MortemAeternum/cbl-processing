use super::*;

#[test]
fn stack_size() {
    println!(
        "std::mem::size_of::<CharacterBuild>() == {}",
        std::mem::size_of::<CharacterBuild>()
    );
}

#[test]
fn wartrapper_test() {
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

    let mut enhancements = Enhancements::default();
    let rank3 = Some(NonZeroU8::new(3).unwrap());
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        0,
        Enhancement::new(
            "Skaldic".to_owned(),
            Some("Constitution".to_owned()),
            None,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        0,
        Enhancement::new("Weapon Training".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        0,
        Enhancement::new("Song of Heroism".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        0,
        Enhancement::new("Fighting Spirit".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        1,
        Enhancement::new("Poetic Edda".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        1,
        Enhancement::new("Enchant Weapon".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        1,
        Enhancement::new("Rough and Ready".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        2,
        Enhancement::new("Words of Encouragement".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        2,
        Enhancement::new("Arcane Shield Chant".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        2,
        Enhancement::new("Iced Edges".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        3,
        Enhancement::new("Ironskin Chant".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        3,
        Enhancement::new("Obstinance".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        3,
        Enhancement::new(
            "High Spirits".to_owned(),
            None,
            Some(NonZeroU8::new(1).unwrap()),
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        4,
        Enhancement::new("Reckless Chant".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        4,
        Enhancement::new("Armorer".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        5,
        Enhancement::new(
            "Movement Booster".to_owned(),
            Some("Expeditious Chant".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        5,
        Enhancement::new("Chant of Power".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        5,
        Enhancement::new("Howl of the North".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Warchanter),
        5,
        Enhancement::new("Kingly Recovery".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        0,
        Enhancement::new("Toughness".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        0,
        Enhancement::new("Stalwart Defense".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        1,
        Enhancement::new(
            "Improved Stalwart Defense".to_owned(),
            Some("Durable Defense".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        1,
        Enhancement::new("Stalwart Defensive Mastery".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        2,
        Enhancement::new(
            "Improved Stalwart Defense".to_owned(),
            Some("Resilient Defense".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        2,
        Enhancement::new("Armor Expertise".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        3,
        Enhancement::new(
            "Greater Stalwart Defense".to_owned(),
            Some("Tenacious Defense".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        3,
        Enhancement::new("Shield Expertise".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        4,
        Enhancement::new(
            "Greater Stalwart Defense".to_owned(),
            Some("Hardy Defense".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::StalwartDefender),
        4,
        Enhancement::new(
            "Reinforced Defense".to_owned(),
            Some("Reinforced Armor".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Swashbuckler),
        0,
        Enhancement::new("Confidence".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Swashbuckler),
        1,
        Enhancement::new("Tavern Shanties".to_owned(), None, rank3),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Spellsinger),
        0,
        Enhancement::new("Spellsinger".to_owned(), None, None),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Spellsinger),
        1,
        Enhancement::new(
            "Studies".to_owned(),
            Some("Magical".to_owned()),
            rank3,
        ),
    );
    enhancements.insert(
        EnhancementTreeName::Class(ClassEnhancementTree::Spellsinger),
        1,
        Enhancement::new(
            "Lingering Songs".to_owned(),
            None,
            Some(NonZeroU8::new(2).unwrap()),
        ),
    );

    let c = CharacterBuild {
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
                    "Cleave".to_owned(),
                ),
                Feat::new(
                    SecondaryFeatType::Class(Class::Fighter),
                    2,
                    "Improved Critical: Slashing".to_owned(),
                ),
                Feat::new(
                    SecondaryFeatType::Class(Class::Fighter),
                    4,
                    "Improved Two Handed Fighting".to_owned(),
                ),
            ],
            race_feats:     Vec::new(),
        },
        spells: Spells {
            spellbooks: [
                Some(Spellbook {
                    class:           Class::Bard,
                    spells_by_level: vec![
                        vec![
                            Some("Cure Light Wounds".to_owned()),
                            Some("Focusing Chant".to_owned()),
                            Some("Expeditious Retreat".to_owned()),
                            Some("Remove Fear".to_owned()),
                        ],
                        vec![
                            Some("Blur".to_owned()),
                            Some("Cure Moderate Wounds".to_owned()),
                            Some("Heroism".to_owned()),
                            Some("Invisibility".to_owned()),
                        ],
                        vec![
                            Some("Cure Serious Wounds".to_owned()),
                            Some("Displacement".to_owned()),
                            Some("Haste".to_owned()),
                            Some("Good Hope".to_owned()),
                        ],
                        vec![
                            Some("Cure Critical Wounds".to_owned()),
                            Some("Freedom of Movement".to_owned()),
                            Some("Break Enchantment".to_owned()),
                            Some("Dimension Door".to_owned()),
                        ],
                        vec![
                            Some("Greater Heroism".to_owned()),
                            Some("Mass Cure Light Wounds".to_owned()),
                            Some("Shadow Walk".to_owned()),
                        ],
                    ],
                }),
                None,
                None,
            ],
        },
        tier_five: Some(EnhancementTreeName::Class(
            ClassEnhancementTree::Warchanter,
        )),
        enhancements,
    };

    let parsed = parse(&mut buf_reader).unwrap();

    assert_eq!(parsed, c);
}
