use nih_plug::prelude::*;
use std::sync::Arc;

// structure for chorus plugin

struct Chorus {
    delay_buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    sample_rate: f32
}


// structure for the chorus params
#[derive(Params)]
struct ChorusParams {
    #[id = "rate"]
    pub rate: FloatParam,

    #[id = "depth"]
    pub depth: FloatParam, 

    #[id = "mix"]
    pub mix: FloatParam,
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

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],

        ..AudioIOLayout::const_default()
    }
    ];


    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        Arc::new(ChorusParams::default())
    }

    fn initialize(
        &mut self, 
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
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
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>

    ) -> ProcessStatus {

        let rate = 0.25;
        let depth = 0.01;
        let mix = 0.5;

        let delay_buffer_len = self.delay_buffer.len();
        let lfo_increment = rate * std::f32::consts::TAU / self.sample_rate;

        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                
                // chorus execution logic lives here
                let lfo = self.lfo_phase.sin();
                let mod_delay = (depth * self.sample_rate * (0.5 * (lfo + 1.0))) as usize;
                let read_pos = (self.write_pos + delay_buffer_len - mod_delay) % delay_buffer_len;
                let delayed_sample = self.delay_buffer[read_pos];

                // write the current position
                self.delay_buffer[self.write_pos] = *sample;

                // Mix dry/wet
                let wet = delayed_sample;
                let dry = *sample;
                *sample = mix * wet + (1.0 - mix) * dry ;

                // advance the write head and LFO
                self.write_pos = (self.write_pos + 1) % delay_buffer_len;
                self.lfo_phase = (self.lfo_phase + lfo_increment) % std::f32::consts::TAU;

            }
            // End of Samples block
        }
        // End of Channels block
        ProcessStatus::Normal

    }
    // End of Process block 
}

// End of Plugin block

impl ClapPlugin for Chorus {
    const CLAP_ID: &'static str = "pyfessional.tech/crimson";
    const CLAP_DESCRIPTION: Option<&'static str> = None;
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[];
}

// EXPORTS
nih_export_clap!(Chorus);
// nih_export_vst3!(Chorus);