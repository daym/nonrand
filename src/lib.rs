pub trait Rng {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;

}

pub struct ThreadRng {
}

pub fn thread_rng() -> ThreadRng {
    ThreadRng { }
}

impl Rng for ThreadRng {
    fn next_u32(&mut self) -> u32 {
        1u32
    }

    fn next_u64(&mut self) -> u64 {
        1u64
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

