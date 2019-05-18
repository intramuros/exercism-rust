extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails }
    }

    fn get_indices(rails: i32, length: usize) -> impl Iterator<Item = i32> {
        (0..rails)
            .flat_map(move |i| {
                (0..(length as i32) + (rails * 2 - 3))
                    .step_by((rails * 2 - 2) as usize)
                    .flat_map(move |v| {
                        if i == 0 || i == rails - 1 {
                            vec![v + i]
                        } else {
                            vec![v - i, v + i]
                        }
                    })
            })
            .filter(move |&elem| elem >= 0 && elem < length as i32)
    }

    pub fn encode(&self, text: &str) -> String {
        let graphemes = UnicodeSegmentation::graphemes(text, true).collect::<Vec<&str>>();
        Self::get_indices(self.rails as i32, graphemes.len())
            .map(|idx| graphemes[idx as usize])
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut res = cipher
            .chars()
            .zip(Self::get_indices(self.rails as i32, cipher.len()))
            .collect::<Vec<(char, i32)>>();
        res.sort_by_key(|&(_, i)| i);
        res.iter().map(|&(c, _)| c).collect()
    }
}
