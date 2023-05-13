use clap::Parser;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Parser, Debug, Copy, Clone)]
pub struct Height {
    #[clap(short, long)]
    value: f64
}

impl Height {
    pub fn new(height: f64) -> Height {
        Height {
            value: height
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
