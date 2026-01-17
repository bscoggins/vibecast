#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Number of frequency bins for visualization
pub const NUM_BINS: usize = 32;

/// Represents audio spectrum data for visualization
#[derive(Clone)]
pub struct SpectrumData {
    /// Normalized frequency bin values (0.0 to 1.0)
    pub bins: [f32; NUM_BINS],
    /// Peak values with decay
    pub peaks: [f32; NUM_BINS],
    /// RMS level (0.0 to 1.0)
    pub rms: f32,
    /// Peak level (0.0 to 1.0)
    pub peak: f32,
    /// Whether we have real audio data
    pub has_audio: bool,
    /// Timestamp of last update
    pub last_update: Instant,
}

impl Default for SpectrumData {
    fn default() -> Self {
        Self {
            bins: [0.0; NUM_BINS],
            peaks: [0.0; NUM_BINS],
            rms: 0.0,
            peak: 0.0,
            has_audio: false,
            last_update: Instant::now(),
        }
    }
}

impl SpectrumData {
    /// Apply decay to values for smooth animation
    pub fn decay(&mut self, factor: f32) {
        for bin in &mut self.bins {
            *bin *= factor;
        }
        for peak in &mut self.peaks {
            *peak *= factor * 0.98; // Peaks decay slower
        }
        self.rms *= factor;
        self.peak *= factor;
    }

    /// Generate spectrum based on RMS/peak audio levels
    /// Note: We only have overall levels from mpv, not per-frequency FFT data,
    /// so all bars respond uniformly to the audio energy
    pub fn simulate_from_levels(&mut self, rms: f32, peak: f32) {
        self.rms = rms;
        self.peak = peak;
        self.has_audio = true;
        self.last_update = Instant::now();

        // Mix RMS and peak for the energy level
        let energy = (rms * 0.5 + peak * 0.5).clamp(0.0, 1.0);

        for i in 0..NUM_BINS {
            // All bins respond directly to audio level - no artificial variation
            let value = energy;

            // Smooth transitions - fast attack, slow visible decay
            let smoothing = if value > self.bins[i] { 0.3 } else { 0.85 };
            self.bins[i] = self.bins[i] * smoothing + value * (1.0 - smoothing);

            // Update peaks with slow decay
            if self.bins[i] > self.peaks[i] {
                self.peaks[i] = self.bins[i];
            } else {
                self.peaks[i] *= 0.99;
            }
        }
    }

    /// Decay spectrum when no audio data is available (or paused/stopped)
    pub fn animate(&mut self, playing: bool, paused: bool) {
        // When not playing or paused, just decay existing values
        // No artificial animation - only real audio should drive the spectrum
        if !playing || paused {
            self.decay(0.85);
        } else {
            // If playing but no audio data, still decay slowly
            // This handles the case where mpv hasn't provided data yet
            self.decay(0.92);
        }
        self.last_update = Instant::now();
    }
}

/// Analyzer that processes audio data and produces spectrum information
pub struct SpectrumAnalyzer {
    data: Arc<RwLock<SpectrumData>>,
    active: Arc<AtomicBool>,
}

impl SpectrumAnalyzer {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(SpectrumData::default())),
            active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get a clone of the current spectrum data
    pub async fn get_data(&self) -> SpectrumData {
        self.data.read().await.clone()
    }

    /// Update spectrum with audio levels (from mpv metadata)
    pub async fn update_from_levels(&self, rms_db: f32, peak_db: f32) {
        // Convert dB to linear (0-1 range)
        // Typical audio range: -60dB (silent) to 0dB (max)
        let rms = db_to_linear(rms_db.clamp(-60.0, 0.0));
        let peak = db_to_linear(peak_db.clamp(-60.0, 0.0));

        let mut data = self.data.write().await;
        data.simulate_from_levels(rms, peak);
        self.active.store(true, Ordering::Relaxed);
    }

    /// Animate the spectrum when no real audio data
    pub async fn animate(&self, playing: bool, paused: bool) {
        let mut data = self.data.write().await;
        data.animate(playing, paused);
    }

    /// Check if we're receiving real audio data
    pub fn has_audio(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Mark as inactive (no audio data)
    pub fn set_inactive(&self) {
        self.active.store(false, Ordering::Relaxed);
    }
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert decibels to linear scale (0.0 to 1.0)
fn db_to_linear(db: f32) -> f32 {
    // Map -60dB to 0.0 and 0dB to 1.0
    let linear = 10.0f32.powf(db / 20.0);
    linear.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_to_linear() {
        assert!((db_to_linear(0.0) - 1.0).abs() < 0.01);
        assert!((db_to_linear(-6.0) - 0.5).abs() < 0.1);
        assert!(db_to_linear(-60.0) < 0.01);
    }
}
