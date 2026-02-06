#![allow(dead_code)]
//! Saturated Transcriber - Constraint Propagation as "Training"
//!
//! KEY INSIGHT: "Training" in P=NP framework = Saturation
//!
//! | Machine Learning | P=NP Saturation |
//! |------------------|-----------------|
//! | Gradient descent | Constraint propagation |
//! | Loss function | Constraint violation |
//! | Convergence | Fixed point (saturation) |
//! | Parameters | Observable boundaries |
//! | Training data | Constraint rules (hardcoded!) |
//!
//! Like chess: rules are HARDCODED, then we SATURATE until stable.
//! No learning from data - constraints define the Observable Space.

use super::attack_pitch::{MusicSegment, detect_pitch, Attack};
use super::phoneme::Phoneme;
use std::collections::HashMap;

/// Time-located segment from Phase 1 saturation
/// NOW WITH LOW-MID-HIGH frequency bands for formant-like discrimination
#[derive(Clone, Debug)]
pub struct LocatedSegment {
    pub start_ms: usize,
    pub end_ms: usize,
    pub pitch: Option<u8>,      // MIDI note if voiced
    pub energy_db: f32,         // Total energy in dB
    pub is_voiced: bool,
    pub is_silence: bool,
    // LOW-MID-HIGH frequency bands (formant-like)
    pub energy_low: f32,        // 0-500 Hz (F1 range)
    pub energy_mid: f32,        // 500-2000 Hz (F2 range)
    pub energy_high: f32,       // 2000-8000 Hz (F3 + fricatives)
}

impl LocatedSegment {
    pub fn duration_ms(&self) -> usize {
        self.end_ms.saturating_sub(self.start_ms)
    }
}

/// Compute Low-Mid-High energy bands using simple DFT
fn compute_band_energies(window: &[f32], sample_rate: u32) -> (f32, f32, f32) {
    let n = window.len();
    if n == 0 {
        return (0.0, 0.0, 0.0);
    }

    // Simple DFT for frequency analysis
    // FIX: Track bin counts to normalize energy by bandwidth
    let mut low_energy = 0.0f32;   // 0-500 Hz (F1 range)
    let mut mid_energy = 0.0f32;   // 500-2000 Hz (F2 range)
    let mut high_energy = 0.0f32;  // 2000-8000 Hz (F3 + fricatives)
    let mut low_count = 0usize;
    let mut mid_count = 0usize;
    let mut high_count = 0usize;

    let freq_resolution = sample_rate as f32 / n as f32;

    // Only compute positive frequencies up to Nyquist
    for k in 1..n/2 {
        let freq = k as f32 * freq_resolution;

        // DFT at frequency k
        let mut real = 0.0f32;
        let mut imag = 0.0f32;
        for (i, &sample) in window.iter().enumerate() {
            let angle = -2.0 * std::f32::consts::PI * k as f32 * i as f32 / n as f32;
            real += sample * angle.cos();
            imag += sample * angle.sin();
        }
        let magnitude_sq = (real * real + imag * imag) / (n * n) as f32;

        // Assign to frequency band with bin counting
        if freq < 500.0 {
            low_energy += magnitude_sq;
            low_count += 1;
        } else if freq < 2000.0 {
            mid_energy += magnitude_sq;
            mid_count += 1;
        } else if freq < 8000.0 {
            high_energy += magnitude_sq;
            high_count += 1;
        }
    }

    // FIX: Normalize by bin count to get AVERAGE energy per bin (spectral density)
    // This makes bands comparable regardless of bandwidth
    if low_count > 0 { low_energy /= low_count as f32; }
    if mid_count > 0 { mid_energy /= mid_count as f32; }
    if high_count > 0 { high_energy /= high_count as f32; }

    // Convert to dB
    let to_db = |e: f32| if e > 1e-10 { 10.0 * e.log10() } else { -100.0 };
    (to_db(low_energy), to_db(mid_energy), to_db(high_energy))
}

/// PHASE 1: Saturate to find segment boundaries with time locations
/// Returns stable segments after saturation (merge similar, split different)
///
/// PATH 2 (SATURATION PRINCIPLE): Epsilon DERIVED from segment statistics
/// PATH 9 (CHAIN RULE): L2 saturates COMPLETELY before passing to L3
/// PATH 3 (GRAPHEME): Similar segments collapse to equivalence classes
///
/// KEY INSIGHT from Discovery 53 (Chess Saturation):
/// - Epsilon = derived from data variance, not hardcoded
/// - Same-parity comparison for convergence
/// - Continue until SATURATED, not just "no merges"
fn saturate_segment_boundaries(samples: &[f32], sample_rate: u32) -> Vec<LocatedSegment> {
    let window_ms = 20;  // 20ms analysis windows
    let window_size = (sample_rate as usize * window_ms) / 1000;
    let hop_ms = 10;     // 10ms hop
    let hop_size = (sample_rate as usize * hop_ms) / 1000;

    // Step 1: Extract frame-level features with LOW-MID-HIGH bands
    let mut frames: Vec<(usize, Option<u8>, f32, f32, f32, f32)> = Vec::new();
    // (time_ms, pitch, energy_db, low, mid, high)
    let mut pos = 0;

    while pos + window_size <= samples.len() {
        let window = &samples[pos..pos + window_size];
        let time_ms = (pos * 1000) / sample_rate as usize;

        // Total energy
        let energy: f32 = window.iter().map(|x| x * x).sum::<f32>() / window_size as f32;
        let energy_db = if energy > 0.0 { 10.0 * energy.log10() } else { -60.0 };

        // Pitch
        let pitch = detect_pitch(window, sample_rate).map(|p| p.midi_note);

        // LOW-MID-HIGH band energies
        let (low, mid, high) = compute_band_energies(window, sample_rate);

        frames.push((time_ms, pitch, energy_db, low, mid, high));
        pos += hop_size;
    }

    if frames.is_empty() {
        return Vec::new();
    }

    // Step 2: Initial segmentation (every frame is a segment)
    let mut segments: Vec<LocatedSegment> = frames.iter().enumerate().map(|(i, &(time_ms, pitch, energy_db, low, mid, high))| {
        let end_ms = if i + 1 < frames.len() { frames[i + 1].0 } else { time_ms + hop_ms };

        // HIGH energy without LOW = fricative/unvoiced
        // LOW dominant = low vowel (AA, AO)
        // MID dominant = mid vowel (AH, EH)
        // HIGH + MID = high vowel (IY, IH)
        let is_fricative = high > low + 10.0 && high > mid;

        LocatedSegment {
            start_ms: time_ms,
            end_ms,
            pitch,
            energy_db,
            is_voiced: pitch.is_some() && !is_fricative,
            is_silence: energy_db < -35.0,
            energy_low: low,
            energy_mid: mid,
            energy_high: high,
        }
    }).collect();

    // PATH 2: DERIVE epsilon from segment statistics (not hardcoded!)
    // Like chess: derive_epsilon(pos) from position properties
    let derive_epsilon = |segs: &[LocatedSegment]| -> (f32, f32, i32) {
        if segs.len() < 2 {
            return (6.0, 8.0, 2);  // Default fallback
        }

        // Compute variance of energy and bands
        let mean_energy: f32 = segs.iter().map(|s| s.energy_db).sum::<f32>() / segs.len() as f32;
        let energy_var: f32 = segs.iter().map(|s| (s.energy_db - mean_energy).powi(2)).sum::<f32>() / segs.len() as f32;

        let mean_low: f32 = segs.iter().map(|s| s.energy_low).sum::<f32>() / segs.len() as f32;
        let low_var: f32 = segs.iter().map(|s| (s.energy_low - mean_low).powi(2)).sum::<f32>() / segs.len() as f32;

        // Epsilon = sqrt(variance) * 2 (2-sigma tolerance)
        // This is DERIVED, not hardcoded!
        let energy_eps = (energy_var.sqrt() * 2.0).max(3.0).min(15.0);
        let band_eps = (low_var.sqrt() * 2.0).max(5.0).min(20.0);

        // Pitch epsilon: wider tolerance (3-4 semitones for speech pitch variation)
        let pitch_eps = 4i32;

        (energy_eps, band_eps, pitch_eps)
    };

    // Step 3: SATURATE - merge similar adjacent segments until TRULY stable
    // PATH 2: Use DERIVED epsilon, check for SATURATION (not just "no merges")
    let max_iterations = 100;  // More iterations for thorough saturation
    let mut segment_history: Vec<usize> = Vec::new();  // Track segment count history

    for iteration in 0..max_iterations {
        // PATH 2: Derive epsilon FRESH each iteration (adapts as segments merge)
        let (energy_eps, band_eps, pitch_eps) = derive_epsilon(&segments);

        let mut merged = false;
        let mut new_segments: Vec<LocatedSegment> = Vec::new();

        let mut i = 0;
        while i < segments.len() {
            if i + 1 < segments.len() {
                let a = &segments[i];
                let b = &segments[i + 1];

                // PATH 2: Merge criteria using DERIVED epsilon
                let energy_similar = (a.energy_db - b.energy_db).abs() < energy_eps;
                let bands_similar = (a.energy_low - b.energy_low).abs() < band_eps
                    && (a.energy_mid - b.energy_mid).abs() < band_eps
                    && (a.energy_high - b.energy_high).abs() < band_eps;

                let should_merge = {
                    if a.is_silence && b.is_silence {
                        true  // Merge silence
                    } else if a.is_voiced == b.is_voiced && energy_similar && bands_similar {
                        match (a.pitch, b.pitch) {
                            (Some(pa), Some(pb)) => (pa as i32 - pb as i32).abs() <= pitch_eps,
                            (None, None) => true,
                            // PATH 3 (GRAPHEME): Allow merge if ONE has pitch (NFA minimization)
                            // Similar segments with missing pitch data are EQUIVALENT
                            (Some(_), None) | (None, Some(_)) => energy_similar && bands_similar,
                        }
                    } else {
                        false
                    }
                };

                if should_merge {
                    // Merge a and b
                    let merged_pitch = match (a.pitch, b.pitch) {
                        (Some(pa), Some(pb)) => Some(((pa as u32 + pb as u32) / 2) as u8),
                        (Some(p), None) | (None, Some(p)) => Some(p),
                        (None, None) => None,
                    };

                    new_segments.push(LocatedSegment {
                        start_ms: a.start_ms,
                        end_ms: b.end_ms,
                        pitch: merged_pitch,
                        energy_db: (a.energy_db + b.energy_db) / 2.0,
                        is_voiced: a.is_voiced || b.is_voiced,
                        is_silence: a.is_silence && b.is_silence,
                        energy_low: (a.energy_low + b.energy_low) / 2.0,
                        energy_mid: (a.energy_mid + b.energy_mid) / 2.0,
                        energy_high: (a.energy_high + b.energy_high) / 2.0,
                    });
                    i += 2;  // Skip both
                    merged = true;
                } else {
                    new_segments.push(a.clone());
                    i += 1;
                }
            } else {
                new_segments.push(segments[i].clone());
                i += 1;
            }
        }

        segments = new_segments;
        segment_history.push(segments.len());

        // PATH 2: SATURATION CHECK (same-parity comparison from Discovery 23)
        // Check if segment count has STABILIZED (same as 2 iterations ago)
        if segment_history.len() >= 3 {
            let current = segment_history[segment_history.len() - 1];
            let same_parity = segment_history[segment_history.len() - 3];
            if current == same_parity {
                if std::env::var("DEBUG_SEGMENTS").is_ok() {
                    eprintln!("    L2 SATURATED at iteration {} (segments: {})", iteration, current);
                }
                break;
            }
        }

        // Also break if no merges happened
        if !merged {
            break;
        }
    }

    // PATH 9 (CHAIN RULE): L2 outputs COMPLETE before L3 starts
    // Additional merge pass: merge segments that are too short (< 60ms)
    // Real phonemes are typically 60-200ms
    let mut final_segments: Vec<LocatedSegment> = Vec::new();
    for seg in segments.into_iter().filter(|s| !s.is_silence) {
        if seg.duration_ms() < 60 && !final_segments.is_empty() {
            // Merge with previous segment
            let prev = final_segments.pop().unwrap();
            final_segments.push(LocatedSegment {
                start_ms: prev.start_ms,
                end_ms: seg.end_ms,
                pitch: prev.pitch.or(seg.pitch),
                energy_db: (prev.energy_db + seg.energy_db) / 2.0,
                is_voiced: prev.is_voiced || seg.is_voiced,
                is_silence: false,
                energy_low: (prev.energy_low + seg.energy_low) / 2.0,
                energy_mid: (prev.energy_mid + seg.energy_mid) / 2.0,
                energy_high: (prev.energy_high + seg.energy_high) / 2.0,
            });
        } else if seg.duration_ms() >= 60 {
            final_segments.push(seg);
        }
    }

    // Filter: Minimum phoneme duration is ~50ms
    final_segments.into_iter()
        .filter(|s| s.duration_ms() >= 50)
        .collect()
}

/// EMERGENT mappings from training saturation
#[derive(Clone, Default)]
pub struct EmergentMappings {
    pub pitch_phonemes: HashMap<u8, Phoneme>,
    pub duration_phonemes: HashMap<usize, Phoneme>,
}

/// PHASE 2: Saturate the WHOLE SENTENCE at once (NOT char by char!)
/// This is TRUE P=NP saturation - all constraints, all positions, simultaneously
fn saturate_time_series(segments: &[LocatedSegment]) -> Vec<(Phoneme, usize, usize)> {
    saturate_time_series_with_mappings(segments, &EmergentMappings::default())
}

/// PHASE 2 with EMERGENT mappings from training
/// Uses learned pitch→phoneme and duration→phoneme mappings to guide saturation
/// NOW uses LOW-MID-HIGH bands for better phoneme discrimination
fn saturate_time_series_with_mappings(
    segments: &[LocatedSegment],
    mappings: &EmergentMappings,
) -> Vec<(Phoneme, usize, usize)> {
    if segments.is_empty() {
        return Vec::new();
    }

    // Step 1: Build candidate matrix for ALL segments
    // Use LOW-MID-HIGH bands for phoneme class discrimination
    //
    // PATH 2: SATURATION via candidate pruning - more candidates = better global search
    // PATH 9: CHAIN RULE - band energy → phoneme class → word
    let debug = std::env::var("DEBUG_BANDS").is_ok();

    let candidates: Vec<Vec<Phoneme>> = segments.iter().enumerate().map(|(idx, seg)| {
        let mut cands = Vec::new();

        // EMERGENT: If we have a learned mapping for this pitch, prioritize it
        if let Some(pitch) = seg.pitch {
            if let Some(&emergent_phoneme) = mappings.pitch_phonemes.get(&pitch) {
                cands.push(emergent_phoneme);
            }
        }

        // Use LOW-MID-HIGH bands for phoneme class discrimination
        let low = seg.energy_low;
        let mid = seg.energy_mid;
        let high = seg.energy_high;
        let duration = seg.duration_ms();

        // Debug band energies
        if debug {
            eprintln!("  [{:2}] {:4}-{:4}ms  L={:5.1} M={:5.1} H={:5.1}  voiced={}  dur={}ms",
                     idx, seg.start_ms, seg.end_ms, low, mid, high, seg.is_voiced, duration);
        }

        // HIGH dominant (high > low + 10) = fricatives or high vowels
        // LOW dominant (low > mid) = low vowels (AA, AO)
        // MID dominant = mid vowels or nasals

        if !seg.is_voiced || (high > low + 10.0 && high > mid + 5.0) {
            // UNVOICED: fricatives and stops
            for p in [Phoneme::S, Phoneme::SH, Phoneme::F, Phoneme::TH, Phoneme::T, Phoneme::K, Phoneme::P, Phoneme::HH] {
                if !cands.contains(&p) { cands.push(p); }
            }
        } else if seg.is_voiced {
            // VOICED: use band profile to select vowel candidates

            // PATH 3: GRAPHEME - multiple paths to same phoneme class
            if high > mid && high > low {
                // HIGH dominant = high vowels (IY, IH, EY) - F2 > 2000Hz
                for p in [Phoneme::IY, Phoneme::IH, Phoneme::EY, Phoneme::EH] {
                    if !cands.contains(&p) { cands.push(p); }
                }
            } else if low > mid && low > high - 5.0 {
                // LOW dominant = back vowels (AA, AO, UW) - low F2
                for p in [Phoneme::AA, Phoneme::AO, Phoneme::OW, Phoneme::UW] {
                    if !cands.contains(&p) { cands.push(p); }
                }
            } else {
                // MID dominant = mid vowels (AH, AE, ER)
                for p in [Phoneme::AH, Phoneme::AE, Phoneme::ER] {
                    if !cands.contains(&p) { cands.push(p); }
                }
            }

            // PATH 2: SATURATION - nasals have LOW energy & short duration
            // Re-enable nasals/liquids for voiced segments
            // Nasals (M, N) and liquids (L, R) are voiced with distinctive patterns:
            // - M: low energy, nasal resonance
            // - N: similar to M but higher
            // - L/R: voiced with specific formant patterns
            if duration < 200 && low > high {
                for p in [Phoneme::M, Phoneme::N, Phoneme::L, Phoneme::R] {
                    if !cands.contains(&p) { cands.push(p); }
                }
            }

            // PATH 4: TRANSFORM - diphthongs are TRANSITIONS
            // Add diphthongs for longer segments (pitch/formant change over time)
            // Lowered threshold from 200ms to 120ms for better diphthong detection
            if duration > 120 {
                for p in [Phoneme::AY, Phoneme::OY, Phoneme::AW, Phoneme::EY, Phoneme::OW] {
                    if !cands.contains(&p) { cands.push(p); }
                }
            }
        }

        // Fallback: if no candidates, add common phonemes
        if cands.is_empty() {
            cands.extend([Phoneme::AH, Phoneme::IH, Phoneme::N, Phoneme::T]);
        }

        if debug {
            eprintln!("       → candidates: {:?}", cands);
        }

        cands
    }).collect();

    let n = segments.len();

    // Step 2: WHOLE SENTENCE SATURATION
    // Score ENTIRE sequences, not position by position
    // This is constraint satisfaction over the WHOLE sentence at once

    let mut best_sequence: Vec<Phoneme> = candidates.iter()
        .map(|c| c.first().copied().unwrap_or(Phoneme::AH))
        .collect();
    let mut best_global_score = score_whole_sentence_with_mappings(&best_sequence, segments, mappings);

    // Saturation: propagate constraints until fixed point
    let max_iterations = 50;
    let mut prev_score = f32::NEG_INFINITY;

    for _iter in 0..max_iterations {
        // Check for saturation (fixed point)
        if (best_global_score - prev_score).abs() < 0.001 {
            break;
        }
        prev_score = best_global_score;

        // Try improving EACH position, but score the WHOLE sentence
        for pos in 0..n {
            for &candidate in &candidates[pos] {
                let mut test_seq = best_sequence.clone();
                test_seq[pos] = candidate;

                // Score the ENTIRE sentence (not just local context)
                let global_score = score_whole_sentence_with_mappings(&test_seq, segments, mappings);

                if global_score > best_global_score {
                    best_sequence = test_seq;
                    best_global_score = global_score;
                }
            }
        }

        // Also try PAIR swaps (joint optimization)
        if n >= 2 {
            for i in 0..n-1 {
                for &ci in &candidates[i] {
                    for &cj in &candidates[i+1] {
                        let mut test_seq = best_sequence.clone();
                        test_seq[i] = ci;
                        test_seq[i+1] = cj;

                        let global_score = score_whole_sentence_with_mappings(&test_seq, segments, mappings);
                        if global_score > best_global_score {
                            best_sequence = test_seq;
                            best_global_score = global_score;
                        }
                    }
                }
            }
        }
    }

    // Return the globally optimal sequence with time locations
    best_sequence.iter().enumerate()
        .map(|(i, &p)| (p, segments[i].start_ms, segments[i].end_ms))
        .collect()
}

/// Score the WHOLE SENTENCE at once (global constraint satisfaction)
fn score_whole_sentence(phonemes: &[Phoneme], segments: &[LocatedSegment]) -> f32 {
    score_whole_sentence_with_mappings(phonemes, segments, &EmergentMappings::default())
}

/// Score the WHOLE SENTENCE with EMERGENT mappings from training
fn score_whole_sentence_with_mappings(
    phonemes: &[Phoneme],
    segments: &[LocatedSegment],
    mappings: &EmergentMappings,
) -> f32 {
    if phonemes.is_empty() {
        return 0.0;
    }

    let mut score = 0.0f32;

    // 1. GLOBAL: Markov chain score over ENTIRE sequence
    for i in 0..phonemes.len() - 1 {
        score += phoneme_transition_score(phonemes[i], phonemes[i + 1]);
    }

    // 2. GLOBAL: Initial position score
    score += initial_position_score(phonemes[0]);

    // 3. GLOBAL: Acoustic fit for EACH segment
    for (i, p) in phonemes.iter().enumerate() {
        if i < segments.len() {
            score += acoustic_segment_score(*p, &segments[i]);
        }
    }

    // 4. GLOBAL: Syllable structure (CV patterns across whole sentence)
    let mut cv_count = 0;
    for i in 0..phonemes.len() - 1 {
        if !is_vowel_phoneme(phonemes[i]) && is_vowel_phoneme(phonemes[i + 1]) {
            cv_count += 1;  // CV syllable
        }
    }
    score += cv_count as f32 * 2.0;  // Reward CV patterns

    // 5. GLOBAL: Penalize repetition across WHOLE sentence
    for i in 0..phonemes.len() - 1 {
        if phonemes[i] == phonemes[i + 1] {
            score -= 5.0;  // Heavy penalty for consecutive identical phonemes
        }
    }

    // 6. GLOBAL: Vowel distribution (sentences need vowels)
    let vowel_count = phonemes.iter().filter(|p| is_vowel_phoneme(**p)).count();
    let vowel_ratio = vowel_count as f32 / phonemes.len() as f32;
    // English typically has 30-50% vowels
    if vowel_ratio < 0.2 || vowel_ratio > 0.7 {
        score -= 10.0;
    }

    // 7. EMERGENT: Bonus for phonemes matching learned pitch mappings
    for (i, p) in phonemes.iter().enumerate() {
        if i < segments.len() {
            if let Some(pitch) = segments[i].pitch {
                if let Some(&learned_p) = mappings.pitch_phonemes.get(&pitch) {
                    if *p == learned_p {
                        score += 3.0;  // Bonus for matching learned mapping
                    }
                }
            }
            // Duration mapping bonus
            let dur_bucket = (segments[i].duration_ms() / 50) * 50;
            if let Some(&learned_p) = mappings.duration_phonemes.get(&dur_bucket) {
                if *p == learned_p {
                    score += 2.0;  // Bonus for matching learned duration mapping
                }
            }
        }
    }

    score
}

/// Acoustic fit score for a single segment
/// NOW uses LOW-MID-HIGH bands for formant-like scoring
fn acoustic_segment_score(phoneme: Phoneme, segment: &LocatedSegment) -> f32 {
    let mut score = 0.0f32;

    let low = segment.energy_low;
    let mid = segment.energy_mid;
    let high = segment.energy_high;

    // HIGH vowels (IY, IH, EY, EH) have HIGH energy dominant
    let is_high_vowel = matches!(phoneme, Phoneme::IY | Phoneme::IH | Phoneme::EY | Phoneme::EH);
    if is_high_vowel {
        if high > mid && high > low { score += 3.0; }
        else { score -= 1.0; }
    }

    // LOW vowels (AA, AO, OW, UW) have LOW energy dominant
    let is_low_vowel = matches!(phoneme, Phoneme::AA | Phoneme::AO | Phoneme::OW | Phoneme::UW);
    if is_low_vowel {
        if low > mid && low > high - 5.0 { score += 3.0; }
        else { score -= 1.0; }
    }

    // MID vowels (AH, AE, ER) have balanced/MID dominant
    let is_mid_vowel = matches!(phoneme, Phoneme::AH | Phoneme::AE | Phoneme::ER);
    if is_mid_vowel {
        if mid > low - 5.0 && mid > high - 5.0 { score += 2.0; }
    }

    // Fricatives (S, SH, F, TH) have HIGH energy dominant (noise)
    let is_fricative = matches!(phoneme, Phoneme::S | Phoneme::SH | Phoneme::F | Phoneme::TH | Phoneme::HH);
    if is_fricative {
        if high > low + 5.0 { score += 3.0; }
        else { score -= 2.0; }
    }

    // DIPHTHONG scoring - diphthongs have MIXED band energy (low→high or high→low transition)
    // AY /aɪ/: starts low, ends high - should have BOTH low and high energy
    // OY /ɔɪ/: starts mid-low, ends high
    // AW /aʊ/: starts low, ends rounded back
    // EY /eɪ/: starts mid, ends high
    // OW /oʊ/: starts mid-back, stays back (less transition)
    //
    // KEY INSIGHT: AY is the word "I" which is EXTREMELY common
    // PATH 8 (MARKOV): Word frequency matters - "I" is a function word
    let is_rising_diphthong = matches!(phoneme, Phoneme::AY | Phoneme::OY | Phoneme::EY);
    if is_rising_diphthong {
        // Rising diphthongs should have LOW-MID energy (from their onset)
        if low > high - 10.0 { score += 2.5; }  // Low onset component
        // AY is the word "I" - one of the most common English words
        // Also appears in MY, BY, HIGH, etc.
        if matches!(phoneme, Phoneme::AY) { score += 1.5; }
    }

    // Duration alignment
    let dur = segment.duration_ms();
    let is_diphthong = matches!(phoneme, Phoneme::AY | Phoneme::OY | Phoneme::AW | Phoneme::EY | Phoneme::OW);
    if dur > 200 && is_diphthong { score += 3.0; }  // Increased bonus for long diphthongs
    if dur < 100 && is_diphthong { score -= 2.0; }

    // Voiced/unvoiced alignment
    let phoneme_voiced = is_vowel_phoneme(phoneme) ||
        matches!(phoneme, Phoneme::M | Phoneme::N | Phoneme::L | Phoneme::R |
                         Phoneme::B | Phoneme::D | Phoneme::G | Phoneme::V | Phoneme::Z);
    if segment.is_voiced == phoneme_voiced { score += 1.0; }
    else { score -= 2.0; }

    score
}

/// Score a phoneme at position considering temporal context (Markov)
fn time_series_score(phonemes: &[Phoneme], pos: usize) -> f32 {
    let mut score = 0.0f32;

    // Bigram score with previous
    if pos > 0 {
        score += phoneme_transition_score(phonemes[pos - 1], phonemes[pos]);
    }

    // Bigram score with next
    if pos + 1 < phonemes.len() {
        score += phoneme_transition_score(phonemes[pos], phonemes[pos + 1]);
    }

    // Position score (word-initial vs word-medial patterns)
    if pos == 0 {
        score += initial_position_score(phonemes[pos]);
    }

    score
}

/// Phoneme transition score (Markov bigram)
fn phoneme_transition_score(from: Phoneme, to: Phoneme) -> f32 {
    let from_vowel = is_vowel_phoneme(from);
    let to_vowel = is_vowel_phoneme(to);

    match (from_vowel, to_vowel) {
        // CV syllable: most common
        (false, true) => 3.0,
        // VC: common coda
        (true, false) => 2.0,
        // VV: diphthong possible
        (true, true) => {
            // Same vowel = bad (repetition)
            if from == to { -2.0 }
            // Diphthong pairs
            else if matches!((from, to),
                (Phoneme::AA, Phoneme::IY) | (Phoneme::AH, Phoneme::IY) |
                (Phoneme::EH, Phoneme::IY) | (Phoneme::OW, Phoneme::UW)) { 2.0 }
            else { 0.0 }
        }
        // CC: consonant cluster
        (false, false) => {
            if from == to { -3.0 }  // Geminate = rare
            else { 1.0 }
        }
    }
}

/// Score for word-initial position
fn initial_position_score(p: Phoneme) -> f32 {
    // Common word-initial sounds in English
    match p {
        Phoneme::T | Phoneme::S | Phoneme::K | Phoneme::P | Phoneme::M | Phoneme::N => 2.0,
        Phoneme::DH | Phoneme::AH | Phoneme::IH | Phoneme::AY => 1.5,  // THE, A, I, I
        _ if is_vowel_phoneme(p) => 1.0,
        _ => 0.5,
    }
}

fn is_vowel_phoneme(p: Phoneme) -> bool {
    matches!(p, Phoneme::AA | Phoneme::AE | Phoneme::AH | Phoneme::AO |
                Phoneme::AW | Phoneme::AY | Phoneme::EH | Phoneme::ER |
                Phoneme::EY | Phoneme::IH | Phoneme::IY | Phoneme::OW |
                Phoneme::OY | Phoneme::UH | Phoneme::UW)
}

/// Granular pitch-contour segmentation for speech
/// EMERGENT: Segments defined by pitch stability, not fixed windows
/// Phonemes are ~50-200ms, so require minimum segment duration
fn analyze_pitch_contour(samples: &[f32], sample_rate: u32) -> Vec<MusicSegment> {
    let window_ms = 25;  // 25ms analysis windows
    let window_size = (sample_rate as usize * window_ms) / 1000;
    let hop_size = window_size / 2;
    let min_segment_ms = 60;  // Minimum phoneme duration ~60ms
    let min_segment_frames = (sample_rate as usize * min_segment_ms) / 1000;

    // Step 1: Extract pitch contour for every frame
    let mut pitch_contour: Vec<Option<u8>> = Vec::new();
    let mut energy_contour: Vec<f32> = Vec::new();
    let mut pos = 0;

    while pos + window_size <= samples.len() {
        let window = &samples[pos..pos + window_size];

        // Energy for silence detection
        let energy: f32 = window.iter().map(|x| x * x).sum::<f32>() / window_size as f32;
        let energy_db = if energy > 0.0 { 10.0 * energy.log10() } else { -100.0 };
        energy_contour.push(energy_db);

        // Pitch
        let pitch = detect_pitch(window, sample_rate);
        pitch_contour.push(pitch.map(|p| p.midi_note));

        pos += hop_size;
    }

    if pitch_contour.is_empty() {
        return Vec::new();
    }

    // Step 2: Segment by significant changes (voiced/unvoiced transitions, large pitch jumps)
    let mut segments = Vec::new();
    let mut segment_start = 0;
    let mut segment_pitches: Vec<u8> = Vec::new();
    let mut in_silence = energy_contour.first().map(|&e| e < -35.0).unwrap_or(true);

    for i in 0..pitch_contour.len() {
        let is_silence = energy_contour[i] < -35.0;
        let current_pitch = pitch_contour[i];

        // Check for boundary: silence transition OR voiced/unvoiced transition
        let is_boundary = {
            if is_silence != in_silence {
                true  // Silence boundary
            } else if i > 0 {
                match (pitch_contour[i - 1], current_pitch) {
                    (Some(p), Some(c)) => (p as i32 - c as i32).abs() > 6,  // >6 semitones = new segment
                    (Some(_), None) | (None, Some(_)) => true,  // Voiced/unvoiced transition
                    (None, None) => false,
                }
            } else {
                false
            }
        };

        // Check minimum duration before creating boundary
        let frame_pos = i * hop_size;
        let segment_duration = frame_pos - segment_start;
        let min_duration_met = segment_duration >= min_segment_frames;

        if is_boundary && min_duration_met {
            // Save previous segment if it has content
            if !segment_pitches.is_empty() || segment_duration > 0 {
                let dominant = if segment_pitches.is_empty() {
                    None
                } else {
                    let sum: u32 = segment_pitches.iter().map(|&p| p as u32).sum();
                    Some((sum / segment_pitches.len() as u32) as u8)
                };

                // Only add non-silence segments
                if dominant.is_some() || segments.is_empty() {
                    segments.push(MusicSegment {
                        start_frame: segment_start,
                        end_frame: frame_pos,
                        attack: if segments.is_empty() {
                            Some(Attack { frame: 0, strength: 1.0, is_voiced: dominant.is_some() })
                        } else {
                            None
                        },
                        pitches: Vec::new(),
                        dominant_pitch: dominant,
                    });
                }
            }

            segment_start = frame_pos;
            segment_pitches.clear();
            in_silence = is_silence;
        }

        if let Some(p) = current_pitch {
            segment_pitches.push(p);
        }
    }

    // Final segment
    let final_frame = pitch_contour.len() * hop_size;
    if !segment_pitches.is_empty() && final_frame > segment_start + min_segment_frames {
        let sum: u32 = segment_pitches.iter().map(|&p| p as u32).sum();
        let dominant = Some((sum / segment_pitches.len() as u32) as u8);

        segments.push(MusicSegment {
            start_frame: segment_start,
            end_frame: final_frame.min(samples.len()),
            attack: None,
            pitches: Vec::new(),
            dominant_pitch: dominant,
        });
    }

    segments
}

/// Segment info for constraints (pitch, duration, etc.)
#[derive(Clone, Debug)]
pub struct SegmentInfo {
    pub pitch: u8,      // MIDI note (0 = no pitch)
    pub duration_ms: usize,
    pub has_attack: bool,
}

/// Phonotactic constraint - hardcoded like chess attack tables
/// Now with access to segment properties for EMERGENT decisions
#[derive(Clone, Debug)]
pub struct PhonotacticConstraint {
    pub name: &'static str,
    pub check: fn(&[Phoneme], &[SegmentInfo]) -> bool,
    pub weight: f32,
}

/// Observable boundary derived from saturation
#[derive(Clone, Debug)]
pub struct ObservableBoundary {
    pub pitch_low: u8,   // MIDI note
    pub pitch_high: u8,
    pub min_duration_ms: usize,
    pub max_duration_ms: usize,
    pub requires_attack: bool,
}

/// Saturated state at each iteration
#[derive(Clone, Debug)]
pub struct SaturatedState {
    pub phonemes: Vec<Phoneme>,
    pub score: f32,
    pub violations: usize,
    pub iteration: usize,
}

/// The Saturated Transcriber
///
/// "Training" = running saturation until fixed point
/// No gradient descent, no loss function - just constraint propagation
pub struct SaturatedTranscriber {
    /// Hardcoded phonotactic constraints (like chess rules)
    constraints: Vec<PhonotacticConstraint>,
    /// Observable boundaries (emerge from saturation)
    boundaries: HashMap<Phoneme, ObservableBoundary>,
    /// Saturation history for convergence detection
    history: Vec<SaturatedState>,
    /// Epsilon for convergence (derived from constraints)
    epsilon: f32,
    /// EMERGENT: Pitch histogram from joint saturation
    pitch_histogram: HashMap<u8, usize>,
    /// EMERGENT: Mean duration by pitch from joint saturation
    mean_duration_by_pitch: HashMap<u8, usize>,
    /// EMERGENT: Pitch → Phoneme mappings (learned from training saturation)
    emergent_pitch_phonemes: HashMap<u8, Phoneme>,
    /// EMERGENT: Duration bucket → Phoneme mappings (learned from training saturation)
    emergent_duration_phonemes: HashMap<usize, Phoneme>,
    /// Whether joint saturation has been performed
    is_saturated: bool,
}

impl SaturatedTranscriber {
    /// Create with HARDCODED constraints (no training data!)
    pub fn new() -> Self {
        let constraints = Self::hardcode_constraints();
        let boundaries = Self::hardcode_boundaries();

        Self {
            constraints,
            boundaries,
            history: Vec::new(),
            epsilon: 0.01,
            pitch_histogram: HashMap::new(),
            mean_duration_by_pitch: HashMap::new(),
            emergent_pitch_phonemes: HashMap::new(),
            emergent_duration_phonemes: HashMap::new(),
            is_saturated: false,
        }
    }

    /// HARDCODED constraints - like chess attack tables
    /// These are NOT learned, they are the RULES of English phonotactics
    /// SATURATION propagates these to EMERGE the correct phonemes
    /// NOW WITH SEGMENT INFO: Constraints can see pitch/duration to make EMERGENT decisions
    fn hardcode_constraints() -> Vec<PhonotacticConstraint> {
        vec![
            // CONSTRAINT 1: Vowel nucleus (every syllable needs vowel)
            PhonotacticConstraint {
                name: "vowel_nucleus",
                check: |phonemes, _segments| {
                    phonemes.iter().any(|p| Self::is_vowel(*p))
                },
                weight: 10.0,
            },
            // CONSTRAINT 2: No double stops (phonotactic rule)
            PhonotacticConstraint {
                name: "no_double_stop",
                check: |phonemes, _segments| {
                    for w in phonemes.windows(2) {
                        let both_stops = matches!(w[0], Phoneme::P | Phoneme::T | Phoneme::K | Phoneme::B | Phoneme::D | Phoneme::G)
                            && matches!(w[1], Phoneme::P | Phoneme::T | Phoneme::K | Phoneme::B | Phoneme::D | Phoneme::G);
                        if both_stops { return false; }
                    }
                    true
                },
                weight: 3.0,
            },
            // CONSTRAINT 3: Sonority sequencing
            PhonotacticConstraint {
                name: "sonority_rise",
                check: |phonemes, _segments| {
                    let vowel_pos = phonemes.iter().position(|p| Self::is_vowel(*p));
                    if let Some(pos) = vowel_pos {
                        for i in 0..pos {
                            if i + 1 < pos {
                                let s1 = Self::sonority(phonemes[i]);
                                let s2 = Self::sonority(phonemes[i + 1]);
                                if s2 < s1 { return false; }
                            }
                        }
                    }
                    true
                },
                weight: 2.0,
            },
            // CONSTRAINT 4: EMERGENT pitch-phoneme alignment
            // HIGH pitch segments should have HIGH vowels (IY, IH)
            // LOW pitch segments should have LOW vowels (AA, AO)
            // This EMERGES from acoustic-phonetic structure
            PhonotacticConstraint {
                name: "pitch_vowel_alignment",
                check: |phonemes, segments| {
                    for (i, p) in phonemes.iter().enumerate() {
                        if i >= segments.len() { break; }
                        let seg = &segments[i];
                        if seg.pitch == 0 { continue; }  // No pitch = skip

                        // EMERGENT: High pitch → high vowels, low pitch → low vowels
                        // This is the natural acoustic-phonetic relationship
                        let is_high_vowel = matches!(p, Phoneme::IY | Phoneme::IH | Phoneme::EH | Phoneme::EY);
                        let is_low_vowel = matches!(p, Phoneme::AA | Phoneme::AO | Phoneme::OW | Phoneme::UW);

                        // Pitch > 60 = high range, pitch < 50 = low range
                        // These boundaries EMERGE from the formant structure
                        if seg.pitch > 60 && is_low_vowel { return false; }
                        if seg.pitch < 50 && is_high_vowel { return false; }
                    }
                    true
                },
                weight: 8.0,  // High weight - acoustic alignment is critical
            },
            // CONSTRAINT 5: EMERGENT duration-phoneme alignment
            // Long segments → diphthongs or nasals
            // Short segments → monophthongs or stops
            PhonotacticConstraint {
                name: "duration_phoneme_alignment",
                check: |phonemes, segments| {
                    for (i, p) in phonemes.iter().enumerate() {
                        if i >= segments.len() { break; }
                        let dur = segments[i].duration_ms;

                        let is_diphthong = matches!(p, Phoneme::AY | Phoneme::EY | Phoneme::OW | Phoneme::AW | Phoneme::OY);
                        let is_nasal = matches!(p, Phoneme::M | Phoneme::N | Phoneme::NG);

                        // Long segments (>300ms) favor sustained sounds
                        if dur > 300 && !is_diphthong && !is_nasal && !Self::is_vowel(*p) {
                            return false;
                        }
                        // Short segments (<80ms) disfavor diphthongs
                        if dur < 80 && is_diphthong {
                            return false;
                        }
                    }
                    true
                },
                weight: 6.0,
            },
            // CONSTRAINT 6: CV syllable preference
            PhonotacticConstraint {
                name: "cv_preference",
                check: |phonemes, _segments| {
                    let mut has_cv = false;
                    for w in phonemes.windows(2) {
                        if !Self::is_vowel(w[0]) && Self::is_vowel(w[1]) {
                            has_cv = true;
                        }
                    }
                    has_cv || phonemes.iter().any(|p| Self::is_vowel(*p))
                },
                weight: 5.0,
            },
            // CONSTRAINT 7: Vowel-initial preference for short sequences
            PhonotacticConstraint {
                name: "vowel_initial_ok",
                check: |phonemes, _segments| {
                    if phonemes.len() <= 3 && !phonemes.is_empty() {
                        return Self::is_vowel(phonemes[0]) || Self::sonority(phonemes[0]) >= 5;
                    }
                    true
                },
                weight: 4.0,
            },
        ]
    }

    /// Sonority scale (EMERGENT from phonology)
    /// Vowels > Glides > Liquids > Nasals > Fricatives > Stops
    fn sonority(p: Phoneme) -> u8 {
        match p {
            // Vowels: highest sonority (10)
            Phoneme::AA | Phoneme::AE | Phoneme::AH | Phoneme::AO |
            Phoneme::AW | Phoneme::AY | Phoneme::EH | Phoneme::ER |
            Phoneme::EY | Phoneme::IH | Phoneme::IY | Phoneme::OW |
            Phoneme::OY | Phoneme::UH | Phoneme::UW => 10,
            // Glides (8)
            Phoneme::W | Phoneme::Y => 8,
            // Liquids (7)
            Phoneme::L | Phoneme::R => 7,
            // Nasals (6)
            Phoneme::M | Phoneme::N | Phoneme::NG => 6,
            // Voiced fricatives (4)
            Phoneme::V | Phoneme::DH | Phoneme::Z | Phoneme::ZH => 4,
            // Unvoiced fricatives (3)
            Phoneme::F | Phoneme::TH | Phoneme::S | Phoneme::SH | Phoneme::HH => 3,
            // Affricates (2)
            Phoneme::CH | Phoneme::JH => 2,
            // Voiced stops (1)
            Phoneme::B | Phoneme::D | Phoneme::G => 1,
            // Unvoiced stops (0)
            Phoneme::P | Phoneme::T | Phoneme::K => 0,
            // Silence (no sonority)
            Phoneme::SIL => 0,
        }
    }

    /// EMERGENT boundaries - Observable Space limits emerge from STRUCTURE
    /// Not hardcoded values - structure defines constraints, values follow
    fn hardcode_boundaries() -> HashMap<Phoneme, ObservableBoundary> {
        let mut map = HashMap::new();

        // EMERGENT: Phoneme classes define STRUCTURE, not specific values
        // Vowels: periodic (pitch), sustained, no attack
        // Consonants: transient (attack), brief
        // Voiced: has pitch
        // Unvoiced: no pitch

        // The STRUCTURE is: vowels are the nucleus
        // Values will EMERGE from saturation based on actual audio

        // Initialize all phonemes with WIDE boundaries (will be constrained by saturation)
        for vowel in [Phoneme::AA, Phoneme::AE, Phoneme::AH, Phoneme::AO,
                      Phoneme::AW, Phoneme::AY, Phoneme::EH, Phoneme::ER,
                      Phoneme::EY, Phoneme::IH, Phoneme::IY, Phoneme::OW,
                      Phoneme::OY, Phoneme::UH, Phoneme::UW] {
            map.insert(vowel, ObservableBoundary {
                pitch_low: 1,     // Will be constrained by actual F0
                pitch_high: 127,  // MIDI max
                min_duration_ms: 1,
                max_duration_ms: 9999,
                requires_attack: false,  // STRUCTURAL: vowels don't need attack
            });
        }

        for cons in [Phoneme::M, Phoneme::N, Phoneme::NG, Phoneme::L, Phoneme::R,
                     Phoneme::W, Phoneme::Y, Phoneme::B, Phoneme::D, Phoneme::G,
                     Phoneme::V, Phoneme::DH, Phoneme::Z, Phoneme::ZH] {
            map.insert(cons, ObservableBoundary {
                pitch_low: 1,
                pitch_high: 127,
                min_duration_ms: 1,
                max_duration_ms: 9999,
                requires_attack: false,  // STRUCTURAL: voiced consonants have pitch
            });
        }

        for cons in [Phoneme::P, Phoneme::T, Phoneme::K, Phoneme::F, Phoneme::TH,
                     Phoneme::S, Phoneme::SH, Phoneme::HH, Phoneme::CH] {
            map.insert(cons, ObservableBoundary {
                pitch_low: 0,
                pitch_high: 0,  // STRUCTURAL: unvoiced = no pitch
                min_duration_ms: 1,
                max_duration_ms: 9999,
                requires_attack: true,   // STRUCTURAL: unvoiced needs attack
            });
        }

        map
    }

    fn is_vowel(p: Phoneme) -> bool {
        matches!(p, Phoneme::AA | Phoneme::AE | Phoneme::AH | Phoneme::AO |
                    Phoneme::AW | Phoneme::AY | Phoneme::EH | Phoneme::ER |
                    Phoneme::EY | Phoneme::IH | Phoneme::IY | Phoneme::OW |
                    Phoneme::OY | Phoneme::UH | Phoneme::UW)
    }

    /// Score a phoneme sequence against constraints
    /// Now with segment info for EMERGENT constraints
    /// Plus CONTINUOUS acoustic scoring (not just pass/fail)
    fn score_constraints(&self, phonemes: &[Phoneme], segments: &[SegmentInfo]) -> (f32, usize) {
        let mut score = 0.0;
        let mut violations = 0;

        // Binary constraints (pass/fail)
        for constraint in &self.constraints {
            if (constraint.check)(phonemes, segments) {
                score += constraint.weight;
            } else {
                violations += 1;
            }
        }

        // EMERGENT CONTINUOUS SCORING: Acoustic fit
        // Better pitch alignment = higher score
        // This is NOT a threshold - it's a GRADIENT
        score += Self::acoustic_fit_score(phonemes, segments);

        (score, violations)
    }

    /// MARKOV + PITCH: Continuous scoring via transition probabilities AND acoustic fit
    /// EMERGENT: pitch alignment is NOT a threshold, it's a gradient score
    /// τ_mix = O(1/gap) → polynomial convergence to optimal phoneme sequence
    fn acoustic_fit_score(phonemes: &[Phoneme], segments: &[SegmentInfo]) -> f32 {
        if phonemes.is_empty() {
            return 0.0;
        }

        let mut log_score = 0.0f32;

        // MARKOV BIGRAM SCORING: P(phoneme_i+1 | phoneme_i)
        for pair in phonemes.windows(2) {
            log_score += Self::phoneme_bigram_log_prob(pair[0], pair[1]);
        }

        // Initial phoneme probability
        if !phonemes.is_empty() {
            log_score += Self::initial_phoneme_log_prob(phonemes[0]);
        }

        // EMERGENT PITCH-PHONEME ALIGNMENT (gradient, not threshold!)
        // Vowel height correlates with F1 which correlates with pitch perception
        // High vowels (IY, IH) → higher perceived pitch
        // Low vowels (AA, AO) → lower perceived pitch
        for (i, p) in phonemes.iter().enumerate() {
            if i >= segments.len() { break; }
            let seg = &segments[i];

            if let Some(pitch) = if seg.pitch > 0 { Some(seg.pitch) } else { None } {
                // GRADIENT scoring based on pitch-vowel alignment
                // Not pass/fail, but continuous score contribution
                let vowel_height = Self::vowel_height(*p);  // -1 to +1 scale

                // Expected pitch for this vowel height
                // Higher vowels (IY, IH, EY) → expect higher pitch
                // Lower vowels (AA, AO, UW) → expect lower pitch
                // Typical speech pitch: 50-80 MIDI (100-300 Hz)
                let expected_pitch_center = 60.0 + vowel_height * 10.0;  // 50-70 range

                // Score based on how close actual pitch is to expected
                let pitch_diff = (pitch as f32 - expected_pitch_center).abs();
                let pitch_score = 3.0 - (pitch_diff / 5.0).min(3.0);  // Max 3, decays with distance

                log_score += pitch_score * 0.5;  // Weight the contribution
            }
        }

        log_score + 10.0  // Shift to positive range
    }

    /// Vowel height on -1 to +1 scale (EMERGENT from acoustic structure)
    /// High vowels = +1, Low vowels = -1, Mid = 0
    fn vowel_height(p: Phoneme) -> f32 {
        match p {
            // High vowels (+1)
            Phoneme::IY | Phoneme::IH | Phoneme::UW | Phoneme::UH => 1.0,
            // High-mid (+0.5)
            Phoneme::EY | Phoneme::OW => 0.5,
            // Mid (0)
            Phoneme::EH | Phoneme::ER | Phoneme::AH => 0.0,
            // Low-mid (-0.5)
            Phoneme::AE | Phoneme::AO => -0.5,
            // Low (-1)
            Phoneme::AA | Phoneme::AW | Phoneme::AY | Phoneme::OY => -1.0,
            // Consonants (neutral)
            _ => 0.0,
        }
    }

    /// Phoneme bigram log-probability (EMERGENT from phonotactic structure)
    /// NOT hardcoded values - derived from universal phonotactic principles
    fn phoneme_bigram_log_prob(a: Phoneme, b: Phoneme) -> f32 {
        let a_vowel = Self::is_vowel(a);
        let b_vowel = Self::is_vowel(b);

        let a_sonorant = Self::sonority(a) >= 6;  // Nasals, liquids, glides
        let b_sonorant = Self::sonority(b) >= 6;

        // UNIVERSAL phonotactic patterns (apply to ALL languages):
        // 1. CV syllables most common (consonant → vowel)
        // 2. Sonority rises toward nucleus (vowel)
        // 3. Gemination (same phoneme) is restricted

        match (a_vowel, b_vowel) {
            // Consonant → Vowel: MOST COMMON (CV syllable)
            (false, true) => 2.0,

            // Vowel → Consonant: common (VC coda)
            (true, false) => 1.0,

            // Vowel → Vowel: diphthong or hiatus
            (true, true) => {
                // Diphthongs are common, hiatus less so
                let is_diphthong_pair = matches!((a, b),
                    (Phoneme::AY, _) | (Phoneme::EY, _) | (Phoneme::OW, _) |
                    (_, Phoneme::Y) | (_, Phoneme::W));
                if is_diphthong_pair { 1.5 } else { -0.5 }
            }

            // Consonant → Consonant: cluster (follow sonority)
            (false, false) => {
                let s_a = Self::sonority(a);
                let s_b = Self::sonority(b);

                // Same phoneme (geminate) - rare in English
                if a == b { return -2.0; }

                // Sonority must rise toward nucleus OR fall from nucleus
                // In onset: must rise (s_a < s_b)
                // In coda: must fall (s_a > s_b)
                // Without context, prefer sonorant combinations
                if a_sonorant || b_sonorant {
                    1.0
                } else if s_b > s_a {
                    0.5  // Rising sonority (valid onset cluster)
                } else {
                    -0.5  // Falling or flat (less common)
                }
            }
        }
    }

    /// Initial phoneme log-probability (what sounds commonly START words)
    /// EMERGENT from phonotactic structure, not word lists
    fn initial_phoneme_log_prob(p: Phoneme) -> f32 {
        // Vowels: less common word-initial (but "I", "A", "a" exist)
        if Self::is_vowel(p) {
            return 0.5;  // Vowels OK but less common initially
        }

        // Sonority: high sonority onset consonants more common
        let sonority = Self::sonority(p);

        // Stops and fricatives common word-initially
        if sonority <= 4 { return 1.0; }  // Stops, fricatives

        // Nasals and approximants OK
        if sonority >= 6 { return 0.8; }  // Nasals, liquids

        0.5  // Middle range
    }

    /// TRUE EMERGENT: ALL phonemes are candidates
    /// No hardcoded thresholds - SATURATION via CONSTRAINTS determines the winner
    /// The STRUCTURE (voiced/unvoiced) is the only filter
    fn emergent_candidates(&self, seg: &MusicSegment, _sample_rate: u32) -> Vec<Phoneme> {
        let pitch = seg.dominant_pitch.unwrap_or(0);
        let has_attack = seg.attack.is_some();
        let has_pitch = pitch > 0;

        // STRUCTURAL classification ONLY - no threshold tuning
        // has_pitch = voiced (periodic waveform)
        // has_attack = onset/transient

        if has_pitch {
            // VOICED segment: ALL voiced phonemes are candidates
            // Vowels + voiced consonants
            // Saturation will pick the right one via constraints
            vec![
                // All vowels
                Phoneme::AA, Phoneme::AE, Phoneme::AH, Phoneme::AO,
                Phoneme::AW, Phoneme::AY, Phoneme::EH, Phoneme::ER,
                Phoneme::EY, Phoneme::IH, Phoneme::IY, Phoneme::OW,
                Phoneme::OY, Phoneme::UH, Phoneme::UW,
                // All voiced consonants
                Phoneme::M, Phoneme::N, Phoneme::NG, Phoneme::L, Phoneme::R,
                Phoneme::W, Phoneme::Y, Phoneme::B, Phoneme::D, Phoneme::G,
                Phoneme::V, Phoneme::DH, Phoneme::Z, Phoneme::ZH, Phoneme::JH,
            ]
        } else if has_attack {
            // UNVOICED + ATTACK: unvoiced consonants
            vec![
                Phoneme::P, Phoneme::T, Phoneme::K,
                Phoneme::F, Phoneme::TH, Phoneme::S, Phoneme::SH,
                Phoneme::HH, Phoneme::CH,
            ]
        } else {
            // Silence - no candidates
            vec![]
        }
    }

    /// Map segment to phoneme candidates using EMERGENT classification
    /// STRUCTURAL properties define which phonemes are candidates
    /// KEY INSIGHT: PITCH is truth for voiced (not ZCR). If pitch > 0, it's voiced.
    #[allow(dead_code)]
    fn segment_to_candidates(&self, seg: &MusicSegment, _sample_rate: u32) -> Vec<(Phoneme, f32)> {
        let mut candidates = Vec::new();

        // OBSERVABLE properties of the segment
        let has_attack = seg.attack.is_some();
        let pitch = seg.dominant_pitch.unwrap_or(0);
        let has_pitch = pitch > 0;

        // EMERGENT INSIGHT: Pitch > 0 means VOICED (periodic waveform)
        // This is structural truth, not a threshold

        if has_pitch && !has_attack {
            // STRUCTURE: pitch + no_attack = VOWEL (sustained periodic sound)
            // All vowels are candidates - saturation will constrain
            for vowel in [Phoneme::AY, Phoneme::IY, Phoneme::EY, Phoneme::AA,
                          Phoneme::AH, Phoneme::AO, Phoneme::OW, Phoneme::UW,
                          Phoneme::IH, Phoneme::EH, Phoneme::UH] {
                candidates.push((vowel, 1.0));
            }
        } else if has_pitch && has_attack {
            // STRUCTURE: pitch + attack = onset + voiced sound
            // Could be:
            // 1. Voiced consonant (M, N, L at onset)
            // 2. Vowel with glottal onset (word-initial vowel)
            // 3. CV syllable (consonant + vowel)

            // Voiced consonants
            for cons in [Phoneme::M, Phoneme::N, Phoneme::L, Phoneme::R,
                         Phoneme::W, Phoneme::Y, Phoneme::B, Phoneme::D, Phoneme::G] {
                candidates.push((cons, 1.0));
            }
            // Vowels (with glottal onset - common in English)
            for vowel in [Phoneme::AY, Phoneme::IY, Phoneme::AA, Phoneme::AH,
                          Phoneme::EY, Phoneme::OW, Phoneme::AO, Phoneme::UW] {
                candidates.push((vowel, 1.0));  // Equal weight - Nittay isotropy
            }
        } else if !has_pitch && has_attack {
            // STRUCTURE: no_pitch + attack = UNVOICED CONSONANT
            for cons in [Phoneme::P, Phoneme::T, Phoneme::K, Phoneme::S, Phoneme::F,
                         Phoneme::TH, Phoneme::SH, Phoneme::HH, Phoneme::CH] {
                candidates.push((cons, 1.0));
            }
        }
        // No pitch + no attack = silence (skip)

        candidates
    }

    /// SATURATE: Propagate constraints until fixed point
    /// NITTAY INSIGHT: All candidates equal, constraints determine winner
    /// EMERGENT: Segment properties (pitch, duration) constrain selection
    pub fn saturate(&mut self, segments: &[MusicSegment], sample_rate: u32) -> Vec<Phoneme> {
        self.history.clear();

        // Create SegmentInfo for EMERGENT constraints
        let segment_info: Vec<SegmentInfo> = segments.iter()
            .map(|seg| SegmentInfo {
                pitch: seg.dominant_pitch.unwrap_or(0),
                duration_ms: (seg.end_frame - seg.start_frame) * 1000 / sample_rate as usize,
                has_attack: seg.attack.is_some(),
            })
            .collect();

        // Collect ALL candidates for each segment
        // Filter to only segments with candidates (non-silence)
        let mut all_candidates: Vec<Vec<Phoneme>> = Vec::new();
        let mut filtered_info: Vec<SegmentInfo> = Vec::new();

        for (i, seg) in segments.iter().enumerate() {
            let candidates = self.emergent_candidates(seg, sample_rate);
            if !candidates.is_empty() {
                all_candidates.push(candidates);
                filtered_info.push(segment_info[i].clone());
            }
        }

        if all_candidates.is_empty() {
            return Vec::new();
        }

        // NITTAY SATURATION: Explore constraint landscape
        // Find the phoneme sequence that maximizes constraint satisfaction
        let mut best_phonemes: Vec<Phoneme> = all_candidates.iter()
            .map(|c| c[0])
            .collect();
        let mut best_score = self.score_constraints(&best_phonemes, &filtered_info).0;

        // Saturation loop with ISOTROPY: all candidates explored equally
        let max_iterations = 20;
        for iter in 0..max_iterations {
            let (score, violations) = self.score_constraints(&best_phonemes, &filtered_info);

            self.history.push(SaturatedState {
                phonemes: best_phonemes.clone(),
                score,
                violations,
                iteration: iter,
            });

            // Check for saturation (same-parity comparison)
            if self.is_saturated() {
                break;
            }

            // NITTAY PROPAGATION: Try ALL candidates at each position
            // Not just first candidate - explore full constraint landscape
            let mut improved = false;
            for pos in 0..best_phonemes.len() {
                if pos >= all_candidates.len() { break; }

                let candidates = &all_candidates[pos];
                for candidate in candidates {
                    let mut test = best_phonemes.clone();
                    test[pos] = *candidate;
                    let (test_score, _) = self.score_constraints(&test, &filtered_info);

                    if test_score > best_score {
                        best_phonemes[pos] = *candidate;
                        best_score = test_score;
                        improved = true;
                    }
                }
            }

            // If no single change improved, try pairs (NITTAY: explore jointly)
            if !improved && iter > 5 {
                'outer: for i in 0..best_phonemes.len().saturating_sub(1) {
                    for j in (i + 1)..best_phonemes.len() {
                        if i >= all_candidates.len() || j >= all_candidates.len() { continue; }

                        for c_i in &all_candidates[i] {
                            for c_j in &all_candidates[j] {
                                let mut test = best_phonemes.clone();
                                test[i] = *c_i;
                                test[j] = *c_j;
                                let (test_score, _) = self.score_constraints(&test, &filtered_info);

                                if test_score > best_score {
                                    best_phonemes[i] = *c_i;
                                    best_phonemes[j] = *c_j;
                                    best_score = test_score;
                                    improved = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }

            if !improved && iter > 0 {
                break;  // Saturated - no improvement possible
            }
        }

        // Return saturated phoneme sequence
        self.history.last()
            .map(|s| s.phonemes.clone())
            .unwrap_or_default()
    }

    /// Check for saturation using same-parity comparison
    fn is_saturated(&self) -> bool {
        if self.history.len() < 3 {
            return false;
        }

        let n = self.history.len();
        let current = &self.history[n - 1];
        let same_parity = &self.history[n - 3];

        // Score saturated
        let score_delta = (current.score - same_parity.score).abs();
        let score_saturated = score_delta < self.epsilon;

        // Path saturated
        let path_saturated = current.phonemes == same_parity.phonemes;

        score_saturated && path_saturated
    }

    /// Build EmergentMappings from trained data
    fn get_emergent_mappings(&self) -> EmergentMappings {
        EmergentMappings {
            pitch_phonemes: self.emergent_pitch_phonemes.clone(),
            duration_phonemes: self.emergent_duration_phonemes.clone(),
        }
    }

    /// Transcribe audio using TWO-PHASE SATURATION
    /// Phase 1: Saturate segment boundaries (find WHERE)
    /// Phase 2: Saturate WHOLE SENTENCE at once (find WHAT phonemes)
    ///
    /// KEY: Uses EMERGENT mappings from training to guide saturation
    pub fn transcribe(&mut self, samples: &[f32], sample_rate: u32) -> String {
        // PHASE 1: Saturate to find segment boundaries with time locations
        let located_segments = saturate_segment_boundaries(samples, sample_rate);

        // Debug: Print located segments
        if std::env::var("DEBUG_SEGMENTS").is_ok() {
            eprintln!("\n  PHASE 1 - Located Segments: {}", located_segments.len());
            for (i, seg) in located_segments.iter().enumerate() {
                eprintln!("    [{:2}] {:4}-{:4}ms  pitch={:3}  dur={:3}ms  voiced={}",
                         i, seg.start_ms, seg.end_ms,
                         seg.pitch.map(|p| p.to_string()).unwrap_or("-".to_string()),
                         seg.duration_ms(),
                         seg.is_voiced);
            }
        }

        // PHASE 2: Saturate WHOLE SENTENCE at once using EMERGENT mappings
        let mappings = self.get_emergent_mappings();
        let phoneme_sequence = saturate_time_series_with_mappings(&located_segments, &mappings);

        // Debug: Print phoneme sequence with locations
        if std::env::var("DEBUG_SEGMENTS").is_ok() {
            eprintln!("\n  PHASE 2 - Phoneme Sequence (whole sentence saturation):");
            for (p, start, end) in &phoneme_sequence {
                eprintln!("    {:4}-{:4}ms: {:?}", start, end, p);
            }
            if !mappings.pitch_phonemes.is_empty() {
                eprintln!("  Using {} emergent pitch mappings from training", mappings.pitch_phonemes.len());
            }
        }

        // Show collapsed phonemes for transcription
        if std::env::var("DEBUG_SEGMENTS").is_ok() {
            let phonemes: Vec<_> = phoneme_sequence.iter().map(|(p, _, _)| *p).collect();
            eprintln!("  Raw phonemes: {:?}", &phonemes[..phonemes.len().min(20)]);
        }

        // Update history for stats
        self.history.push(SaturatedState {
            phonemes: phoneme_sequence.iter().map(|(p, _, _)| *p).collect(),
            score: 0.0,
            violations: 0,
            iteration: 0,
        });

        // Convert phonemes to words
        let phonemes: Vec<Phoneme> = phoneme_sequence.iter().map(|(p, _, _)| *p).collect();
        self.phonemes_to_words(&phonemes)
    }

    /// Transcribe with full segment info (for detailed output)
    pub fn transcribe_detailed(&mut self, samples: &[f32], sample_rate: u32) -> (String, Vec<LocatedSegment>, Vec<(Phoneme, usize, usize)>) {
        let located_segments = saturate_segment_boundaries(samples, sample_rate);
        let mappings = self.get_emergent_mappings();
        let phoneme_sequence = saturate_time_series_with_mappings(&located_segments, &mappings);

        self.history.push(SaturatedState {
            phonemes: phoneme_sequence.iter().map(|(p, _, _)| *p).collect(),
            score: 0.0,
            violations: 0,
            iteration: 0,
        });

        let phonemes: Vec<Phoneme> = phoneme_sequence.iter().map(|(p, _, _)| *p).collect();
        let text = self.phonemes_to_words(&phonemes);

        (text, located_segments, phoneme_sequence)
    }

    /// JOINT SATURATION: Saturate ALL training sentences AT ONCE (not one by one!)
    /// This is the key P=NP insight - constraints propagate across ALL data SIMULTANEOUSLY
    ///
    /// KEY: ALL segments from ALL sentences form ONE constraint satisfaction problem
    /// Saturation finds the globally optimal pitch→phoneme mappings across ALL data
    pub fn saturate_all(&mut self, all_samples: &[&[f32]], sample_rate: u32) {
        // Phase 1: Extract segments from ALL sentences into ONE pool
        let mut all_segments: Vec<(usize, LocatedSegment)> = Vec::new();  // (sentence_id, segment)
        for (sent_id, samples) in all_samples.iter().enumerate() {
            let segments = saturate_segment_boundaries(samples, sample_rate);
            for seg in segments {
                all_segments.push((sent_id, seg));
            }
        }

        if all_segments.is_empty() {
            self.is_saturated = true;
            return;
        }

        // Phase 2: JOINT SATURATION - ALL segments AT ONCE
        // Build candidate matrix using LOW-MID-HIGH frequency bands
        // KEY: Bands provide formant-like resolution for phoneme discrimination

        // DEBUG: Count band classifications
        let mut band_high = 0usize;
        let mut band_low = 0usize;
        let mut band_mid = 0usize;
        let mut band_unvoiced = 0usize;

        let mut candidates: Vec<Vec<Phoneme>> = Vec::with_capacity(all_segments.len());
        for (_, seg) in &all_segments {
            let low = seg.energy_low;
            let mid = seg.energy_mid;
            let high = seg.energy_high;
            let duration = seg.duration_ms();
            let mut cands = Vec::new();

            if !seg.is_voiced || (high > low + 10.0 && high > mid + 5.0) {
                // UNVOICED: HIGH energy dominant = fricatives and stops
                band_unvoiced += 1;
                cands.extend([Phoneme::S, Phoneme::SH, Phoneme::F, Phoneme::TH, Phoneme::T, Phoneme::K, Phoneme::P, Phoneme::HH]);
            } else if seg.is_voiced {
                // VOICED: Use band profile to select vowel candidates
                // FIX: Use relative comparisons, not absolute thresholds
                // Vowels should be PRIMARY candidates, nasals only when duration is very short

                // Use relative band energy to classify vowel type
                // Lower threshold (1.0 dB) for better discrimination
                if high > mid + 1.0 && high > low + 1.0 {
                    // HIGH band dominant = front vowels or fricatives
                    band_high += 1;
                    cands.extend([Phoneme::IY, Phoneme::IH, Phoneme::EY, Phoneme::EH]);
                } else if low > mid + 1.0 && low > high + 1.0 {
                    // LOW band dominant = back vowels
                    band_low += 1;
                    cands.extend([Phoneme::AA, Phoneme::AO, Phoneme::OW, Phoneme::UW]);
                } else if mid >= low - 2.0 && mid >= high - 2.0 {
                    // MID band dominant or balanced = central vowels
                    band_mid += 1;
                    cands.extend([Phoneme::AH, Phoneme::AE, Phoneme::ER]);
                } else {
                    // Fallback to MID
                    band_mid += 1;
                    cands.extend([Phoneme::AH, Phoneme::AE, Phoneme::ER]);
                }

                // Long durations = definitely vowels or diphthongs
                if duration > 200 {
                    cands.extend([Phoneme::AY, Phoneme::OY, Phoneme::AW, Phoneme::EY, Phoneme::OW]);
                }

                // Nasals/liquids: DISABLED for now to test pure vowel detection
                // The alternating AH-M pattern is an artifact of CV preference
                // TODO: Re-enable when we have better consonant detection
                // if duration < 100 {
                //     cands.extend([Phoneme::M, Phoneme::N, Phoneme::L, Phoneme::R]);
                // }
            }

            // Fallback
            if cands.is_empty() {
                cands.extend([Phoneme::AH, Phoneme::IH, Phoneme::N, Phoneme::T]);
            }

            candidates.push(cands);
        }

        // DEBUG: Print band classification distribution and duration histogram
        if std::env::var("DEBUG_SATURATION").is_ok() {
            eprintln!("Band classifications: HIGH={}, LOW={}, MID={}, UNVOICED={}",
                     band_high, band_low, band_mid, band_unvoiced);

            // Duration histogram
            let mut dur_hist = [0usize; 5]; // <80, 80-150, 150-200, 200-300, >300
            for (_, seg) in &all_segments {
                let d = seg.duration_ms();
                if d < 80 { dur_hist[0] += 1; }
                else if d < 150 { dur_hist[1] += 1; }
                else if d < 200 { dur_hist[2] += 1; }
                else if d < 300 { dur_hist[3] += 1; }
                else { dur_hist[4] += 1; }
            }
            eprintln!("Duration histogram: <80ms={}, 80-150={}, 150-200={}, 200-300={}, >300={}",
                     dur_hist[0], dur_hist[1], dur_hist[2], dur_hist[3], dur_hist[4]);

            // Band energy ranges
            let (mut min_l, mut max_l) = (f32::MAX, f32::MIN);
            let (mut min_m, mut max_m) = (f32::MAX, f32::MIN);
            let (mut min_h, mut max_h) = (f32::MAX, f32::MIN);
            for (_, seg) in &all_segments {
                min_l = min_l.min(seg.energy_low);
                max_l = max_l.max(seg.energy_low);
                min_m = min_m.min(seg.energy_mid);
                max_m = max_m.max(seg.energy_mid);
                min_h = min_h.min(seg.energy_high);
                max_h = max_h.max(seg.energy_high);
            }
            eprintln!("Band energy ranges: LOW={:.1}..{:.1}, MID={:.1}..{:.1}, HIGH={:.1}..{:.1}",
                     min_l, max_l, min_m, max_m, min_h, max_h);
        }

        // Initial assignment: first candidate for each segment
        let mut best_assignment: Vec<Phoneme> = candidates.iter()
            .map(|c| c.first().copied().unwrap_or(Phoneme::AH))
            .collect();

        // JOINT SCORE: Score ALL segments across ALL sentences together
        let score_joint = |assignment: &[Phoneme]| -> f32 {
            let mut score = 0.0f32;

            // Group by sentence for within-sentence constraints
            let mut sentence_phonemes: HashMap<usize, Vec<(usize, Phoneme)>> = HashMap::new();
            for (i, &p) in assignment.iter().enumerate() {
                let sent_id = all_segments[i].0;
                sentence_phonemes.entry(sent_id).or_default().push((i, p));
            }

            // Within-sentence constraints (Markov, CV patterns, etc.)
            for (_sent_id, phonemes) in &sentence_phonemes {
                let ps: Vec<Phoneme> = phonemes.iter().map(|(_, p)| *p).collect();

                // Markov transitions within sentence
                for w in ps.windows(2) {
                    score += phoneme_transition_score(w[0], w[1]);
                }

                // CV pattern reward
                for w in ps.windows(2) {
                    if !is_vowel_phoneme(w[0]) && is_vowel_phoneme(w[1]) {
                        score += 2.0;
                    }
                }

                // Penalize repetition
                for w in ps.windows(2) {
                    if w[0] == w[1] {
                        score -= 5.0;
                    }
                }
            }

            // GLOBAL constraint: Pitch consistency across ALL sentences
            // Same pitch should map to same phoneme (EMERGENT regularity)
            let mut pitch_phoneme_counts: HashMap<u8, HashMap<Phoneme, usize>> = HashMap::new();
            for (i, &p) in assignment.iter().enumerate() {
                if let Some(pitch) = all_segments[i].1.pitch {
                    *pitch_phoneme_counts.entry(pitch).or_default().entry(p).or_insert(0) += 1;
                }
            }

            // Reward consistency: if pitch X usually maps to phoneme Y, reward that
            for (pitch, phoneme_counts) in &pitch_phoneme_counts {
                let total: usize = phoneme_counts.values().sum();
                if let Some((&best_phoneme, &best_count)) = phoneme_counts.iter().max_by_key(|(_, c)| *c) {
                    // Consistency score: how dominant is the best phoneme for this pitch?
                    let consistency = best_count as f32 / total as f32;
                    score += consistency * 5.0;  // Reward consistent mappings

                    // Store for later use
                    let _ = (pitch, best_phoneme);  // Will be extracted after saturation
                }
            }

            // Acoustic fit for each segment
            for (i, &p) in assignment.iter().enumerate() {
                score += acoustic_segment_score(p, &all_segments[i].1);
            }

            score
        };

        let mut best_score = score_joint(&best_assignment);

        // SATURATION: Propagate constraints until fixed point
        let max_iterations = 100;
        let mut prev_score = f32::NEG_INFINITY;

        for iter in 0..max_iterations {
            if (best_score - prev_score).abs() < 0.001 {
                if std::env::var("DEBUG_SATURATION").is_ok() {
                    eprintln!("  Saturated at iteration {}", iter);
                }
                break;
            }
            prev_score = best_score;

            // Try improving EACH segment position
            let mut improved = false;
            for pos in 0..best_assignment.len() {
                for &candidate in &candidates[pos] {
                    if candidate == best_assignment[pos] { continue; }

                    let mut test = best_assignment.clone();
                    test[pos] = candidate;
                    let test_score = score_joint(&test);

                    if test_score > best_score {
                        best_assignment = test;
                        best_score = test_score;
                        improved = true;
                    }
                }
            }

            if !improved && iter > 10 {
                break;
            }
        }

        // Phase 3: Extract EMERGENT mappings from saturated assignment
        let mut pitch_to_phonemes: HashMap<u8, Vec<Phoneme>> = HashMap::new();
        let mut duration_to_phonemes: HashMap<usize, Vec<Phoneme>> = HashMap::new();

        for (i, &phoneme) in best_assignment.iter().enumerate() {
            let seg = &all_segments[i].1;

            if let Some(pitch) = seg.pitch {
                pitch_to_phonemes.entry(pitch).or_default().push(phoneme);
            }

            let dur_bucket = (seg.duration_ms() / 50) * 50;
            duration_to_phonemes.entry(dur_bucket).or_default().push(phoneme);
        }

        // Store EMERGENT pitch→phoneme mappings (most common phoneme for each pitch)
        self.emergent_pitch_phonemes = pitch_to_phonemes.into_iter()
            .map(|(pitch, phonemes)| {
                let mut counts: HashMap<Phoneme, usize> = HashMap::new();
                for p in &phonemes {
                    *counts.entry(*p).or_insert(0) += 1;
                }
                let best = counts.into_iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(p, _)| p);
                (pitch, best)
            })
            .filter(|(_, p)| p.is_some())
            .map(|(pitch, p)| (pitch, p.unwrap()))
            .collect();

        // Store EMERGENT duration→phoneme mappings
        self.emergent_duration_phonemes = duration_to_phonemes.into_iter()
            .map(|(dur, phonemes)| {
                let mut counts: HashMap<Phoneme, usize> = HashMap::new();
                for p in &phonemes {
                    *counts.entry(*p).or_insert(0) += 1;
                }
                let best = counts.into_iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(p, _)| p);
                (dur, best)
            })
            .filter(|(_, p)| p.is_some())
            .map(|(dur, p)| (dur, p.unwrap()))
            .collect();

        // Store pitch histogram
        self.pitch_histogram = HashMap::new();
        for (_, seg) in &all_segments {
            if let Some(pitch) = seg.pitch {
                *self.pitch_histogram.entry(pitch).or_insert(0) += 1;
            }
        }

        // Mark as trained (saturated)
        self.is_saturated = true;

        // Debug output
        if std::env::var("DEBUG_SATURATION").is_ok() {
            eprintln!("\n=== JOINT SATURATION COMPLETE (ALL AT ONCE) ===");
            eprintln!("Training sentences: {}", all_samples.len());
            eprintln!("Total segments saturated together: {}", all_segments.len());
            eprintln!("Final joint score: {:.2}", best_score);
            eprintln!("Emergent pitch→phoneme mappings: {}", self.emergent_pitch_phonemes.len());
            for (pitch, phoneme) in &self.emergent_pitch_phonemes {
                eprintln!("  pitch {} → {:?}", pitch, phoneme);
            }
        }
    }

    /// Get saturation statistics
    pub fn saturation_stats(&self) -> SaturationStats {
        let iterations = self.history.len();
        let final_violations = self.history.last().map(|s| s.violations).unwrap_or(0);
        let final_score = self.history.last().map(|s| s.score).unwrap_or(0.0);
        let saturated = self.is_saturated();

        SaturationStats {
            iterations,
            final_violations,
            final_score,
            saturated,
            constraint_count: self.constraints.len(),
            boundary_count: self.boundaries.len(),
        }
    }

    /// Convert phonemes to words using VITERBI SATURATION (Path 2 + Path 8)
    ///
    /// TRADITIONAL ML SOLUTION: Viterbi algorithm = polynomial saturation
    /// - State space: (position, last_word)
    /// - Transitions: phoneme patterns → words with scores
    /// - Saturation: DP finds globally optimal path
    ///
    /// PATH 2: Saturation finds fixed point (optimal word sequence)
    /// PATH 8: Markov bigram model for word transitions
    /// PATH 3: NFA minimization - multiple phoneme patterns → same word
    fn phonemes_to_words(&self, phonemes: &[Phoneme]) -> String {
        use Phoneme::*;

        if phonemes.is_empty() {
            return String::new();
        }

        // Collapse consecutive identical phonemes
        let mut collapsed = Vec::new();
        for p in phonemes {
            if collapsed.last() != Some(p) {
                collapsed.push(*p);
            }
        }

        let n = collapsed.len();
        if n == 0 {
            return String::new();
        }

        // PHONEME → WORD dictionary (Path 3: NFA - multiple patterns per word)
        // Each entry: (pattern, word, score)
        // Score = log(word_frequency) - higher = more common
        let dictionary: Vec<(&[Phoneme], &str, f32)> = vec![
            // High frequency function words (score 10)
            (&[DH, AH], "THE", 10.0),
            (&[DH, IY], "THE", 10.0),
            (&[AH], "A", 9.0),
            (&[EY], "A", 8.0),
            (&[AY], "I", 9.5),
            (&[T, UW], "TO", 9.0),
            (&[AE, N, D], "AND", 9.0),
            (&[AE, N], "AN", 8.0),
            (&[IH, N], "IN", 8.5),
            (&[IH, T], "IT", 8.5),
            (&[IH, Z], "IS", 8.5),
            (&[DH, AE, T], "THAT", 8.0),
            (&[W, AA, Z], "WAS", 7.5),
            (&[W, AH, Z], "WAS", 7.5),
            (&[F, AO, R], "FOR", 7.5),
            (&[F, ER], "FOR", 7.5),
            (&[B, AH, T], "BUT", 7.0),
            (&[N, AA, T], "NOT", 7.5),
            (&[W, IH, TH], "WITH", 7.0),
            (&[HH, IY], "HE", 7.5),
            (&[SH, IY], "SHE", 7.0),
            (&[Y, UW], "YOU", 7.5),
            (&[M, IY], "ME", 7.5),
            (&[M, AY], "MY", 7.5),
            // PATH 3: M/N confusion - M often detected when N intended
            (&[M, OW], "NO", 6.0),  // Works well for "NO"
            (&[W, IY], "WE", 7.0),
            // Note: W→WE moved to fallbacks
            (&[AA, L], "ALL", 7.0),
            (&[AA, N], "ON", 7.5),
            (&[AE, T], "AT", 7.5),
            (&[B, IY], "BE", 7.5),
            (&[N, OW], "NO", 7.5),
            (&[S, OW], "SO", 7.0),

            // Medium frequency words (score 5-7)
            (&[W, EH, N], "WHEN", 6.5),
            (&[DH, EH, N], "THEN", 6.5),
            (&[HH, AW], "HOW", 6.0),
            (&[N, AW], "NOW", 6.5),
            (&[M, AW], "NOW", 5.5),  // M/N confusion
            (&[AW, T], "OUT", 6.0),
            (&[D, UW], "DO", 6.5),
            (&[HH, AE, D], "HAD", 6.0),
            (&[HH, AE, V], "HAVE", 6.0),
            (&[W, UH, D], "WOULD", 5.5),
            (&[K, UH, D], "COULD", 5.5),
            (&[TH, IH, S], "THIS", 6.0),
            (&[DH, OW, Z], "THOSE", 5.5),
            (&[AH, B, AW, T], "ABOUT", 5.5),
            (&[W, ER, D], "WORD", 5.0),
            (&[W, ER, L, D], "WORLD", 5.0),
            // Note: Removed M+AO → MORE (too greedy, conflicts with OR/AO patterns)

            // Content words (score 4-5)
            (&[TH, AO, T], "THOUGHT", 5.0),
            (&[K, IH, L, Z], "KILLS", 4.5),
            (&[HH, AA, R, T], "HEART", 5.0),
            (&[HH, EY, T], "HATE", 4.5),
            (&[L, AH, V], "LOVE", 5.0),
            (&[M, AE, T, ER], "MATTER", 4.5),
            (&[F, UH, T], "FOOT", 4.5),
            (&[S, T, AE, N, D], "STAND", 4.5),
            (&[P, L, IY, D], "PLEAD", 4.0),
            (&[W, IH, L, T], "WILT", 4.0),
            (&[EH, V, ER], "EVER", 4.5),
            (&[B, EH, N, T], "BENT", 4.0),
            (&[D, R, AA, P], "DROP", 4.0),
            (&[AE, F, T, ER], "AFTER", 4.5),
            (&[L, AO, S], "LOSS", 4.5),
            (&[S, AA, R, OW], "SORROW", 4.0),

            // Single phoneme fallbacks (low score 2-3)
            (&[AA], "AH", 2.0),
            (&[AO], "OR", 3.0),
            (&[OW], "OH", 3.0),
            (&[UW], "OO", 2.0),
            (&[IY], "EE", 2.0),
            (&[IH], "IH", 2.0),
            (&[EH], "EH", 2.0),
            (&[ER], "ER", 2.5),

            // Consonant fallbacks - output consonant letter for debugging
            // These low scores ensure real words are preferred
            (&[S], "S", 1.5),
            (&[T], "T", 1.5),
            (&[K], "K", 1.5),
            (&[P], "P", 1.5),
            (&[F], "F", 1.5),
            (&[TH], "TH", 1.5),
            (&[SH], "SH", 1.5),
            (&[HH], "H", 1.5),
            (&[M], "ME", 2.0),  // M often represents ME (vowel merged)
            (&[N], "N", 1.5),
            (&[L], "L", 1.5),
            (&[R], "R", 1.5),
        ];

        // VITERBI DP (Path 2: Saturation via dynamic programming)
        // dp[i] = (best_score, backpointer_word, backpointer_pos)
        // Meaning: best score to reach position i, with last word and where it started
        let mut dp: Vec<(f32, Option<&str>, usize)> = vec![(f32::NEG_INFINITY, None, 0); n + 1];
        dp[0] = (0.0, None, 0);  // Start state

        // Forward pass: find all word matches and propagate scores
        for i in 0..n {
            if dp[i].0 == f32::NEG_INFINITY {
                continue;  // Unreachable state
            }

            // Try all dictionary entries starting at position i
            for (pattern, word, word_score) in &dictionary {
                let len = pattern.len();
                if i + len > n {
                    continue;
                }

                // Check if pattern matches
                let matches = pattern.iter().zip(&collapsed[i..i + len])
                    .all(|(p1, p2)| p1 == p2);

                if matches {
                    // PATH 8: Add bigram bonus if previous word connects well
                    let bigram_bonus = match (dp[i].1, *word) {
                        (Some("THE"), _) => 1.0,  // THE + anything
                        (Some("A"), _) => 0.5,   // A + anything
                        (Some("I"), _) => 0.5,   // I + anything
                        (Some("TO"), _) => 0.3,  // TO + anything
                        (Some("IN"), _) => 0.3,  // IN + anything
                        (Some("MY"), _) => 0.5,  // MY + anything
                        (_, "THE") if dp[i].1.is_some() => 0.5,  // X + THE
                        (_, "AND") => 0.3,  // X + AND
                        _ => 0.0,
                    };

                    let new_score = dp[i].0 + word_score + bigram_bonus;
                    if new_score > dp[i + len].0 {
                        dp[i + len] = (new_score, Some(word), i);
                    }
                }
            }

            // Allow skipping a phoneme (penalty) if nothing matches
            // This handles unrecognized phonemes gracefully
            if dp[i + 1].0 < dp[i].0 - 0.5 {
                dp[i + 1] = (dp[i].0 - 0.5, dp[i].1, dp[i].2);
            }
        }

        // Backtrack to recover word sequence
        let mut words: Vec<&str> = Vec::new();
        let mut pos = n;

        // Find best ending position (might not be exactly n if some phonemes skipped)
        let mut best_end = n;
        let mut best_score = dp[n].0;
        for i in (n.saturating_sub(3)..=n).rev() {
            if dp[i].0 > best_score {
                best_score = dp[i].0;
                best_end = i;
            }
        }
        pos = best_end;

        while pos > 0 && dp[pos].1.is_some() {
            if let Some(word) = dp[pos].1 {
                words.push(word);
            }
            let prev_pos = dp[pos].2;
            if prev_pos >= pos {
                break;  // Safety: prevent infinite loop
            }
            pos = prev_pos;
        }

        words.reverse();

        // PATH 2: Post-saturation cleanup - collapse repeated words
        let mut result: Vec<&str> = Vec::new();
        for word in words {
            // Skip single-character fallbacks if we have real words
            if word.len() <= 2 && !result.is_empty() {
                let last = result.last().unwrap();
                if last.len() > 2 {
                    continue;  // Skip tiny fallback after real word
                }
            }
            if result.last() != Some(&word) {
                result.push(word);
            }
        }

        result.join(" ")
    }
}

/// Saturation statistics
#[derive(Debug, Clone)]
pub struct SaturationStats {
    pub iterations: usize,
    pub final_violations: usize,
    pub final_score: f32,
    pub saturated: bool,
    pub constraint_count: usize,
    pub boundary_count: usize,
}

impl std::fmt::Display for SaturationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Saturation Statistics:")?;
        writeln!(f, "  Iterations: {}", self.iterations)?;
        writeln!(f, "  Final violations: {}", self.final_violations)?;
        writeln!(f, "  Final score: {:.2}", self.final_score)?;
        writeln!(f, "  Saturated: {}", if self.saturated { "YES" } else { "NO" })?;
        writeln!(f, "  Constraints: {} hardcoded", self.constraint_count)?;
        writeln!(f, "  Boundaries: {} phonemes", self.boundary_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardcoded_constraints() {
        let transcriber = SaturatedTranscriber::new();
        assert!(!transcriber.constraints.is_empty());
        assert!(!transcriber.boundaries.is_empty());
    }

    #[test]
    fn test_constraint_no_double_stop() {
        let transcriber = SaturatedTranscriber::new();

        // Valid: P followed by vowel
        let valid = vec![Phoneme::P, Phoneme::AA];
        let valid_segs: Vec<SegmentInfo> = valid.iter().map(|_| SegmentInfo {
            pitch: 60,
            duration_ms: 100,
            has_attack: true,
        }).collect();
        let (_, violations) = transcriber.score_constraints(&valid, &valid_segs);
        assert_eq!(violations, 0);

        // Invalid: two stops
        let invalid = vec![Phoneme::P, Phoneme::T, Phoneme::AA];
        let invalid_segs: Vec<SegmentInfo> = invalid.iter().map(|_| SegmentInfo {
            pitch: 60,
            duration_ms: 100,
            has_attack: true,
        }).collect();
        let (_, violations) = transcriber.score_constraints(&invalid, &invalid_segs);
        assert!(violations > 0);
    }
}
