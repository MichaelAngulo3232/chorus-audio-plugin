use nih_plug::prelude::*;
use std::sync::Arc;

struct Chorus {
    delay_buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    sample_rate: f32
}

impl Default for Chorus {

    fn default() -> Self {
        let max_delay_samples = (0.02 * 48000.0) as usize;
        Self {
            delay_buffer: vec![0.0; max_delay_samples],
            write_pos: 0,
            lfo_phase: 0.0,
            sample_rate: 48000.0,
        }
    }
}

impl Plugin for Chorus {
    
    const NAME: &'static str = "Crimson";
    const VENDOR: &'static str = "Pyfessional";
    const URL: &'static str = "https://pyfessional.tech";
    const EMAIL: &'static str = "N/A";






   // End of Plugin block 
}