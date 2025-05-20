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
    const VERSION: &'static str = "0.1.0";

    type SysExMessage = ();

    fn params(&self) -> Arc<dyn Params> {
        Arc::new(EmptyParams)
    }

    fn initialize(
        &mut self, 
        buffer_config: &BufferConfig,
        _config: &mut impl InitContext<self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate
        let max_delay_samples = (0.02 * self.sample_rate) as usize;
        self.delay_buffer = vec![0.0; max_delay_samples];
        true
    }

    fn reset(&mut self) {
        self.write_pos = 0;
        self.lfo_phase = 0.0;
        self.delay_buffer.fill(0.0);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxilaryBuffers
    ) -> ProcessStatus {

        let rate = 0.25;
        let depth = 0.01;
        let mix = 0.5;

        let delay_buffer_len = self.delay_buffer.len();
        






    }
   // End of Plugin block 
}