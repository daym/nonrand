pub trait Rng {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;

}

pub struct ThreadRng {
    seed : u32
}

pub fn thread_rng() -> ThreadRng {
    ThreadRng { seed: 0u32 }
}

impl Rng for ThreadRng {
    fn next_u32(&mut self) -> u32 {
        self.seed += 1u32;
        return self.seed;
    }

    fn next_u64(&mut self) -> u64 {
        self.seed += 1u32;
        return self.seed as u64;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

