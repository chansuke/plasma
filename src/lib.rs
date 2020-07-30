#[macro_use]
extern crate vst;

use crate::vst::plugin::{Info, Plugin, Category};

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
}

plugin_main!(Plasma);
