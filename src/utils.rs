// Shared utility helpers across the project.

pub mod random {
    #[cfg(all(target_arch = "wasm32", feature = "wasm"))]
    use js_sys::Math;

    #[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
    use rand::Rng;

    /// Generate a floating-point number in [0.0, 1.0).
    #[inline]
    fn unit_f64() -> f64 {
        #[cfg(all(target_arch = "wasm32", feature = "wasm"))]
        {
            Math::random()
        }

        #[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
        {
            rand::thread_rng().gen::<f64>()
        }
    }

    /// Return true with the provided probability (clamped between 0.0 and 1.0).
    pub fn hit(probability: f64) -> bool {
        let p = probability.clamp(0.0, 1.0);
        unit_f64() <= p
    }

    /// Sample an f64 within the half-open range [start, end).
    pub fn range_f64(start: f64, end: f64) -> f64 {
        if end <= start {
            return start;
        }
        let span = end - start;
        start + span * unit_f64()
    }

    /// Sample an f64 within the closed range [start, end].
    pub fn range_f64_inclusive(start: f64, end: f64) -> f64 {
        if end <= start {
            return start;
        }
        // Math::random never returns exactly 1.0, so reusing the half-open helper is fine.
        range_f64(start, end)
    }

    /// Sample a usize within the half-open interval [start, end).
    pub fn range_usize(start: usize, end: usize) -> usize {
        if end <= start {
            return start;
        }
        let span = (end - start) as f64;
        start + (unit_f64() * span).floor() as usize
    }

    /// Sample a usize within the closed interval [start, end].
    pub fn range_usize_inclusive(start: usize, end: usize) -> usize {
        if end <= start {
            return start;
        }
        let span = (end - start + 1) as f64;
        start + (unit_f64() * span).floor() as usize
    }

    /// Sample a u32 within the closed interval [start, end].
    pub fn range_u32_inclusive(start: u32, end: u32) -> u32 {
        if end <= start {
            return start;
        }
        let span = (end - start + 1) as f64;
        start + (unit_f64() * span).floor() as u32
    }
}
