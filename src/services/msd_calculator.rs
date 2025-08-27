use minacalc_rs::{Calc, MsdForAllRates, Note};
use om_fast_parser::{HitObjectType, OsuParser};

/// Converts X position of a note to bitflag for 4K
fn get_columns(x: i32) -> Result<u32, String> {
    match x {
        64 => Ok(1),  // bit flag 0b0001
        192 => Ok(2), // bit flag 0b0010
        320 => Ok(4), // bit flag 0b0100
        448 => Ok(8), // bit flag 0b1000
        _ => Err(format!("not supported columns {x}")),
    }
}

/// Converts a HitObject to Note for MinaCalc
fn hit_object_to_note(hit_object: &om_fast_parser::HitObject) -> Result<Note, String> {
    let time = (hit_object.time as f32) / 1000.0; // Convert ms to seconds
    match hit_object.object_type {
        HitObjectType::Circle => Ok(Note {
            notes: get_columns(hit_object.x)?,
            row_time: time,
        }),
        HitObjectType::Hold => Ok(Note {
            notes: get_columns(hit_object.x)?,
            row_time: time,
        }),
    }
}

/// Merges notes that have the same time by adding their bitflags
fn merge_notes_at_same_time(mut raw_notes: Vec<Note>) -> Vec<Note> {
    raw_notes.sort_by(|a, b| a.row_time.partial_cmp(&b.row_time).unwrap());

    let mut notes = Vec::new();
    let mut current_time = -1.0;
    let mut current_notes = 0u32;

    for note in &raw_notes {
        if note.row_time == current_time {
            // Same time: add bitflags
            current_notes |= note.notes;
        } else {
            // New time: save previous note and start a new one
            if current_time >= 0.0 {
                notes.push(Note {
                    notes: current_notes,
                    row_time: current_time,
                });
            }
            current_time = note.row_time;
            current_notes = note.notes;
        }
    }

    // Don't forget the last note
    if current_time >= 0.0 {
        notes.push(Note {
            notes: current_notes,
            row_time: current_time,
        });
    }

    notes
}

pub fn osu_to_notes(content: &str) -> Result<Vec<Note>, String> {
    // Load and parse the .osu file with our parser
    let mut parser = OsuParser::new();
    parser
        .parse_content(content)
        .map_err(|e| format!("Failed to parse file: {:?}", e))?;

    // Check that it's a 4K Mania map
    if parser.mode != 3 {
        return Err("Map is not mania".into());
    }

    // Convert HitObjects to Notes
    let mut raw_notes = Vec::new();
    for hit_object in &parser.hit_objects {
        match hit_object_to_note(hit_object) {
            Ok(note) => raw_notes.push(note),
            Err(e) => return Err(format!("Error converting hit object: {}", e)),
        }
    }

    // Merge notes that have the same time
    let notes = merge_notes_at_same_time(raw_notes);

    Ok(notes)
}

pub fn calculate_etterna_rating(notes: &Vec<Note>, calc: &Calc) -> Result<MsdForAllRates, String> {
    match calc.calc_msd(&notes) {
        Ok(msd) => Ok(msd.into()),
        Err(_) => Err("Failed to calculate MSD".to_string()),
    }
}
