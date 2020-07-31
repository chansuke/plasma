use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::plugin_main;
use rand::random;

#[derive(Default)]
struct Plasma;

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
