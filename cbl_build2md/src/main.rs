extern crate cbl_build_parser;

use cbl_build_parser::Class;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let mut in_filename = None;
    let mut out_filename = None;
    for (i, arg) in env::args().enumerate() {
        match i {
            1 => in_filename = Some(arg),
            2 => out_filename = Some(arg),
            _ => (),
        }
    }

    let in_file = File::open(in_filename.unwrap()).unwrap();
    let mut in_file_buf = BufReader::new(in_file);
    let build = cbl_build_parser::parse(&mut in_file_buf).unwrap();
    let markdown = build_to_markdown(&build);

    let mut out_file = File::create(out_filename.unwrap()).unwrap();
    out_file.write_all(markdown.as_bytes()).unwrap();
}

fn build_to_markdown(build: &cbl_build_parser::CharacterBuild) -> String {
    let mut out = String::with_capacity(1_024);

    write_overview(build, &mut out);
    write_level_order(build, &mut out);

    out
}

fn write_overview(build: &cbl_build_parser::CharacterBuild, out: &mut String) {
    out.push_str("# ");
    out.push_str(&build.name);
    out.push_str("\n\n");

    let mut split = [None, None, None];
    for class in build.levels.iter() {
        split
            .iter_mut()
            .find(|mcl| {
                if let Some((c, _)) = mcl {
                    c == class
                } else {
                    true
                }
            })
            .and_then(|mcl| {
                if let Some((_, l)) = mcl {
                    *l += 1;
                } else {
                    *mcl = Some((*class, 1u8));
                }

                Some(())
            })
            .unwrap()
    }
    split.sort_by_key(|mcl| {
        if let Some((_, l)) = mcl {
            0xFF - l
        } else {
            0xFF
        }
    });
    out.push_str(&match split {
        [Some((c0, l0)), Some((c1, l1)), Some((c2, l2))] =>
            format!("{}/{}/{} {}/{}/{}\n\n", l0, l1, l2, c0, c1, c2),
        [Some((c0, l0)), Some((c1, l1)), _] =>
            format!("{}/{} {}/{}\n\n", l0, l1, c0, c1),
        [Some((c, l)), _, _] => format!("{} {}\n\n", c, l),
        _ => unimplemented!(),
    });

    out.push_str(&format!("{} {}\n", build.alignment, build.race));
}

fn write_level_order(
    build: &cbl_build_parser::CharacterBuild,
    out: &mut String,
) {
    out.push_str("\nLevel order\n\n");

    let class_levels = build.max_levels.min(20);
    let cols = class_levels / 5 + (class_levels % 5).min(1);
    let mut max_class_widths = [0u8; 4];
    for (i, class_slice) in build.levels.as_slice().chunks(5).enumerate() {
        max_class_widths[i] = class_slice.iter().fold(0, |accu, c| {
            let width = match c {
                Class::Artificer => 9,
                Class::Barbarian => 9,
                Class::Bard => 4,
                Class::Cleric => 6,
                Class::Druid => 5,
                Class::FavoredSoul => 12,
                Class::Fighter => 7,
                Class::Monk => 4,
                Class::Paladin => 7,
                Class::Ranger => 6,
                Class::Rogue => 5,
                Class::Sorcerer => 8,
                Class::Warlock => 7,
                Class::Wizard => 6,
            };

            width.max(accu)
        });
    }

    out.push_str("|    ");
    for _ in 0..max_class_widths[0] {
        out.push(' ');
    }
    for &w in max_class_widths[1..].iter().filter(|&&w| w != 0) {
        out.push_str(" |     ");
        for _ in 0..w {
            out.push(' ');
        }
    }
    out.push_str(" |\n");

    // ...
}
