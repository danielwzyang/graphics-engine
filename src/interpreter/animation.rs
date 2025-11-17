use super::parser::Command;
use std::{
    error::Error,
    collections::HashMap,
};

pub fn first_pass(commands: &Vec<Command>) -> Result<(usize, String), Box<dyn Error>> {
    let mut frames: usize = 0;
    let mut basename = String::new();

    let mut contains_frames = false;
    let mut contains_vary = false;
    let mut contains_basename = false;
    let mut contains_tween = false;

    for command in commands {
        match command {
            Command::SetBaseName { name } => { basename = name.clone(); contains_basename = true; }
            Command::Tween { .. } => { contains_tween = true; }
            Command::SetFrames { num_frames } => { frames = *num_frames; contains_frames = true; }
            Command::VaryKnob { .. } => { contains_vary = true; }
            _ => {}
        }
    }

    if (contains_vary || contains_tween || contains_basename) && !contains_frames {
        Err("Animation was detected but the number of frames wasn't set.".into())
    } else if contains_frames && !contains_basename {
        Err("Number of frames was set but basename wasn't.".into())
    } else {
        Ok((frames, basename))
    }
}

pub fn second_pass(commands: &Vec<Command>, frames: &usize) -> Result<Vec<HashMap<String, f32>>, Box<dyn Error>> {
    let mut frame_knobs: Vec<HashMap<String, f32>> = vec![HashMap::new(); *frames];
    let mut saved_knobs: HashMap<String, HashMap<String, f32>> = HashMap::new();

    for command in commands {
        match command {
            Command::VaryKnob { knob, start_frame, end_frame, start_val, end_val } => {
                if *start_frame >= *frames || *end_frame >= *frames {
                    return Err(format!("Vary command has frames outside range: {} to {}.", start_frame, end_frame).into());
                }
                
                if start_frame > end_frame {
                    return Err(format!("Vary command has start_frame > end_frame: {} > {}.", start_frame, end_frame).into());
                }

                let num_frames = (end_frame - start_frame) as f32;
                let delta = (end_val - start_val) / num_frames;

                for frame in *start_frame..=*end_frame {
                    let value = start_val + delta * ((frame - start_frame) as f32);
                    frame_knobs[frame].insert(knob.clone(), value);
                }
            }

            Command::SaveKnobList { name } => {
                if !frame_knobs.is_empty() {
                    saved_knobs.insert(name.clone(), frame_knobs[0].clone());
                }
            }

            Command::Tween { start_frame, end_frame, knoblist0, knoblist1 } => {
                if *start_frame >= *frames || *end_frame >= *frames {
                    return Err(format!("Tween command has frames outside range: {} to {}.", start_frame, end_frame).into());
                }
                
                if start_frame > end_frame {
                    return Err(format!("Tween command has start_frame > end_frame: {} > {}.", start_frame, end_frame).into());
                }

                let knobs0 = saved_knobs.get(knoblist0)
                    .ok_or_else(|| format!("Knoblist '{}' not found", knoblist0))?;
                let knobs1 = saved_knobs.get(knoblist1)
                    .ok_or_else(|| format!("Knoblist '{}' not found", knoblist1))?;

                let num_frames = (end_frame - start_frame) as f32;

                let mut all_knobs: std::collections::HashSet<String> = std::collections::HashSet::new();
                for knob in knobs0.keys() {
                    all_knobs.insert(knob.clone());
                }
                for knob in knobs1.keys() {
                    all_knobs.insert(knob.clone());
                }

                for knob_name in all_knobs {
                    let start_val = *knobs0.get(&knob_name).unwrap_or(&0.0);
                    let end_val = *knobs1.get(&knob_name).unwrap_or(&0.0);
                    let delta = (end_val - start_val) / num_frames;

                    for frame in *start_frame..=*end_frame {
                        let value = start_val + delta * ((frame - start_frame) as f32);
                        frame_knobs[frame].insert(knob_name.clone(), value);
                    }
                }
            }

            _ => {}
        }
    }

    Ok(frame_knobs)
}

