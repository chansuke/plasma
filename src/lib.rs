use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::plugin_main;
use vst::event::Event;
use vst::api::Events;
use rand::random;

#[derive(Default)]
struct Plasma {
    notes: u8
};

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
}

plugin_main!(Plasma);
