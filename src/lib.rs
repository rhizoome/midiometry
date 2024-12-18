mod editor;

use editor::create_editor;
use nih_plug::prelude::*;
use rtrb::{Producer, RingBuffer};
use std::sync::Arc;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct PluginStruct {
    params: Arc<PluginStructParams>,
    input_buffer: Option<Producer<NoteEvent<()>>>,
}

#[derive(Params, Default)]
struct PluginStructParams {}

impl Default for PluginStruct {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginStructParams::default()),
            input_buffer: None,
        }
    }
}

impl Plugin for PluginStruct {
    const NAME: &'static str = "MIDIOMETRY";
    const VENDOR: &'static str = "dvub";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "dvubdevs@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            // NOT SURE IF THIS IS NEEDED...
            for sample in channel_samples {
                *sample *= 1.0;
            }
            // TODO:
            // does it matter where this goes?
            if let Some(event) = context.next_event() {
                if let Some(buffer) = &mut self.input_buffer {
                    buffer.push(event).unwrap();
                }
                // im fairly sure this is necessary
                context.send_event(event);
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        //
        // TODO:
        // the capacity is important
        // this is the max number of midi events we can pass through from audio -> GUI at one time
        //
        let (producer, consumer) = RingBuffer::new(20);

        self.input_buffer = Some(producer);
        Some(Box::new(create_editor(consumer)))
    }
}

impl ClapPlugin for PluginStruct {
    const CLAP_ID: &'static str = "com.your-domain.midiometry";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A cool plugin for vis midi input");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for PluginStruct {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(PluginStruct);
nih_export_vst3!(PluginStruct);
