use vst::plugin::{Info, Plugin, PluginParameters, Category};
use vst::buffer::AudioBuffer;
use vst::plugin_main;
use vst::event::Event;
use vst::api::Events;
use rand::random;
use std::sync::Arc;

#[derive(Default)]
struct Plasma {
    params: Arc<PlasmaParameters>,
    notes: u8
}

struct PlasmaParameters {
    volume: vst::util::AtomicFloat,
}

impl Default for PlasmaParameters {
    fn default() -> Self {
        Self {
            volume: vst::util::AtomicFloat::new(1.0),
        }
    }
}

impl PluginParameters for PlasmaParameters {
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "x".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.3}", self.volume.get()),
            _ => format!(""),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "volume".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.volume.get(),
            _ => 0.0,
        }
    }
    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.volume.set(value),
            _ => (),
        }
    }
}


impl Plugin for Plasma {
    fn get_info(&self) -> Info {
        Info {
            name: "Plasma".to_string(),
            unique_id: 7777,
            inputs: 0,
            category: Category::Synth,
            ..Default::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => {
                    match ev.data[0] {
                        144 => self.notes += 1u8,
                        128 => self.notes -= 1u8,
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut outputs) = buffer.split();

        for output_channel in outputs.into_iter() {
            for output_sample in output_channel {
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

plugin_main!(Plasma);
