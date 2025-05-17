use nih_plug::prelude::*;
use std::sync::Arc;

struct chorus {
    delay_buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    sample_rate: f32
}