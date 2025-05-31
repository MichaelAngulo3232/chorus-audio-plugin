use nih_plug::prelude::*;
use std::sync::Arc;
use nih_plug::params::range::FloatRange;

// structure for chorus params
#[derive(Params)]
struct ChorusParams {
    #[id = "rate"]
    pub rate: FloatParam,

    #[id = "depth"]
    pub depth: FloatParam, 

    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for ChorusParams {

        fn default() -> Self {

            let rate_default = 0.25;
            let rate_min = 0.01;
            let rate_max = 5.0;

            let depth_default = 0.01;
            let depth_min = 0.0;
            let depth_max = 0.02;

            let mix_default = 0.5;
            let mix_min = 0.0;
            let mix_max = 1.0;

            Self {
                
                rate: FloatParam::new("Rate", rate_default, FloatRange::Linear{min: rate_min, max: rate_max})
                    .with_unit("Hz") // UI Unit for Rate
                    .with_value_to_string(Arc::new(|val| format!("{:.1}", val)))
                    .with_smoother(SmoothingStyle::Linear(40.0)), // snappier than 50.0
                
                depth: FloatParam::new("Depth", depth_default, FloatRange::Linear{min: depth_min, max: depth_max})
                    .with_unit("ms") // UI Unit for Depth
                    .with_value_to_string(Arc::new(|val| format!("{:.0}", val *1000.0)))
                    .with_smoother(SmoothingStyle::Linear(50.0)), // 40.0 not worth trade off - will introduce artifacts

                mix: FloatParam::new("Mix", mix_default, FloatRange::Linear{min: mix_min, max: mix_max})
                    .with_unit("%")
                    .with_value_to_string(Arc::new(|val| format!("{:.0}", val *100.0)))
                    .with_smoother(SmoothingStyle::Linear(50.0)),
        }
    }
}

// structure for chorus plugin
struct Chorus {
    delay_buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    sample_rate: f32,
    params: Arc<ChorusParams>,
}

impl Default for Chorus {

    fn default() -> Self {
        let max_delay_samples = (0.02 * 48000.0) as usize;
        Self {
            delay_buffer: vec![0.0; max_delay_samples],
            write_pos: 0,
            lfo_phase: 0.0,
            sample_rate: 48000.0,
            params: Arc::new(ChorusParams::default()),
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
        self.params.clone()
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

        let delay_buffer_len = self.delay_buffer.len();

        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {

                let rate = self.params.rate.smoothed.next();
                let depth = self.params.depth.smoothed.next();
                let mix = self.params.mix.smoothed.next();
                // chorus execution logic lives here
                let lfo_increment = rate * std::f32::consts::TAU / self.sample_rate;
                let lfo = self.lfo_phase.sin();

                // mod_delay ranges from 0 to depth * sample_rate (e.g., 0 to 960 samples if depth = 0.02 and SR = 48kHz)
                let mod_delay = depth * self.sample_rate * (0.5 * (lfo + 1.0));
                let read_pos = (self.write_pos as f32 + delay_buffer_len as f32 - mod_delay) % delay_buffer_len as f32;
                let index_a = read_pos.floor() as usize;
                let index_b = (index_a + 1) % delay_buffer_len;
                let frac = read_pos - index_a as f32;
                let delayed_sample = (1.0 - frac) * self.delay_buffer[index_a] + frac * self.delay_buffer[index_b];

                // write the current position
                self.delay_buffer[self.write_pos] = *sample;

                // Mix dry/wet
                let wet = delayed_sample;
                let dry = *sample;
                *sample = (mix * wet + (1.0 - mix) * dry).clamp(-1.0, 1.0);

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