pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut s = String::new();
        for i in 0..self.rails {
            let mut ind = 0;
        }
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
    }
}
