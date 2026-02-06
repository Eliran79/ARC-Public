//! Riemann Hypothesis Compression Test
//!
//! Applies the Two Randomness Theorem to prime number gaps.
//! This is the diagnostic test for attacking the Riemann Hypothesis.
//!
//! Key insight: If prime gaps are physics-level compressible (15-92%),
//! they have hidden bounded discrete structure that the zeta zeros encode.
//! If bit-level incompressible (<5%), the attack path should be abandoned.
//!
//! Run: cargo run --release --bin riemann_compression_test

use std::io::Cursor;
use std::time::Instant;

fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     RIEMANN HYPOTHESIS - TWO RANDOMNESS COMPRESSION TEST         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Millennium Prize Problem #2: Do all non-trivial zeros of the     ║");
    println!("║ Riemann zeta function have real part 1/2?                        ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Attack Vector: Two Randomness Theorem (PATH_20)                  ║");
    println!("║   Bit-Level:     K(x) ≈ |x|, compression <5% (crypto keys)       ║");
    println!("║   Physics-Level: K(x) ≤ α|x|, compression 15-92% (structure!)   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Comparison baselines from PATH_20
    println!("=== BASELINE COMPARISONS (from Two Randomness Theorem) ===");
    println!("  Crypto keys (CSPRNG):     -0.04% compressible (bit-level, true random)");
    println!("  CPU temperature noise:     35.0% compressible (physics-level)");
    println!("  White noise audio:         91.6% compressible (physics-level)");
    println!();

    // Test prime gaps at multiple scales
    println!("=== PRIME GAP COMPRESSION ANALYSIS ===");
    println!();

    let test_sizes = [10_000, 100_000, 1_000_000, 10_000_000];
    let mut results = Vec::new();

    for &n in &test_sizes {
        let result = test_prime_gaps(n);
        results.push(result);
    }

    // Summary
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                        SUMMARY                                   ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");

    let avg_compression: f64 = results.iter().map(|r| r.compression_ratio).sum::<f64>() / results.len() as f64;
    let avg_entropy: f64 = results.iter().map(|r| r.entropy).sum::<f64>() / results.len() as f64;

    println!("║ Average compression ratio: {:5.1}%                                ║", avg_compression * 100.0);
    println!("║ Average entropy per byte:  {:5.3}                                 ║", avg_entropy);
    println!("╠══════════════════════════════════════════════════════════════════╣");

    // Decision
    let decision = if avg_compression < 0.05 {
        ("BIT-LEVEL RANDOMNESS", "ABANDON this Riemann attack path", "❌")
    } else if avg_compression < 0.40 {
        ("PHYSICS-LEVEL STRUCTURE", "PROCEED to full Riemann attack", "✅")
    } else {
        ("STRONG PATTERN DETECTED", "ACCELERATE - major discovery!", "🚀")
    };

    println!("║ Classification: {}                          ║", decision.0);
    println!("║ Decision: {}                      {}║", decision.1, decision.2);
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Nittay Limit check
    println!("=== NITTAY LIMIT (√2) CHECK ===");
    check_nittay_in_primes(&results);

    // Gap distribution analysis
    println!();
    println!("=== GAP DISTRIBUTION ANALYSIS ===");
    analyze_gap_distribution(10_000_000);
}

struct CompressionResult {
    #[allow(dead_code)]
    n: usize,
    compression_ratio: f64,
    entropy: f64,
    mean_gap: f64,
    std_gap: f64,
}

/// Test compression of prime gaps for first N primes
fn test_prime_gaps(n: usize) -> CompressionResult {
    let start = Instant::now();

    // Generate primes using Sieve of Eratosthenes
    let limit = estimate_nth_prime_bound(n);
    let primes = sieve_of_eratosthenes(limit);
    let primes: Vec<u64> = primes.into_iter().take(n).collect();

    // Extract gaps
    let gaps: Vec<u64> = primes.windows(2).map(|w| w[1] - w[0]).collect();

    // Statistics
    let mean_gap = gaps.iter().sum::<u64>() as f64 / gaps.len() as f64;
    let variance = gaps.iter().map(|g| {
        let diff = *g as f64 - mean_gap;
        diff * diff
    }).sum::<f64>() / gaps.len() as f64;
    let std_gap = variance.sqrt();

    // Convert to bytes for compression (using variable-length encoding for efficiency)
    let bytes: Vec<u8> = gaps.iter()
        .flat_map(|g| {
            // Use 1-2 bytes for small gaps, more for larger
            if *g < 256 {
                vec![*g as u8]
            } else {
                g.to_le_bytes().to_vec()
            }
        })
        .collect();

    // Compress using zstd at maximum level
    let compressed = compress_zstd(&bytes);
    let compression_ratio = 1.0 - (compressed.len() as f64 / bytes.len() as f64);

    // Calculate entropy
    let entropy = calculate_entropy(&bytes);

    let elapsed = start.elapsed();

    // Classification
    let class = if compression_ratio < 0.05 {
        "BIT-LEVEL ❌"
    } else if compression_ratio < 0.40 {
        "PHYSICS ✅"
    } else {
        "STRONG 🚀"
    };

    println!("N = {:>10}: compression = {:5.1}% | entropy = {:.3} | mean_gap = {:6.2} | std = {:6.2} | {} | {:?}",
             n, compression_ratio * 100.0, entropy, mean_gap, std_gap, class, elapsed);

    CompressionResult {
        n,
        compression_ratio,
        entropy,
        mean_gap,
        std_gap,
    }
}

/// Estimate upper bound for nth prime using prime number theorem
fn estimate_nth_prime_bound(n: usize) -> usize {
    if n < 6 {
        return 15;
    }
    let nf = n as f64;
    let ln_n = nf.ln();
    let ln_ln_n = ln_n.ln();
    // Upper bound: n * (ln(n) + ln(ln(n)))
    (nf * (ln_n + ln_ln_n + 2.0)) as usize
}

/// Sieve of Eratosthenes - generate all primes up to limit
fn sieve_of_eratosthenes(limit: usize) -> Vec<u64> {
    if limit < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter(|(_, &is_p)| is_p)
        .map(|(i, _)| i as u64)
        .collect()
}

/// Compress using zstd at maximum level
fn compress_zstd(data: &[u8]) -> Vec<u8> {
    let mut encoder = zstd::stream::Encoder::new(Vec::new(), 19).unwrap();
    std::io::copy(&mut Cursor::new(data), &mut encoder).unwrap();
    encoder.finish().unwrap()
}

/// Calculate Shannon entropy per byte
fn calculate_entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Check if Nittay Limit (√2) appears in prime gap statistics
fn check_nittay_in_primes(results: &[CompressionResult]) {
    let sqrt_2 = std::f64::consts::SQRT_2;

    println!("Looking for √2 ≈ {:.6} in prime statistics...", sqrt_2);
    println!();

    for result in results {
        let ratio_mean_std = result.mean_gap / result.std_gap;
        let ratio_std_mean = result.std_gap / result.mean_gap;

        let diff_mean_std = (ratio_mean_std - sqrt_2).abs();
        let diff_std_mean = (ratio_std_mean - sqrt_2).abs();

        println!("N = {:>10}:", result.n);
        println!("  mean/std = {:.6} (diff from √2: {:.6})", ratio_mean_std, diff_mean_std);
        println!("  std/mean = {:.6} (diff from √2: {:.6})", ratio_std_mean, diff_std_mean);

        if diff_mean_std < 0.1 || diff_std_mean < 0.1 {
            println!("  ⚡ CLOSE TO √2 - Potential Nittay connection!");
        }
    }
}

/// Detailed gap distribution analysis
fn analyze_gap_distribution(n: usize) {
    println!("Analyzing gap distribution for N = {}...", n);

    let limit = estimate_nth_prime_bound(n);
    let primes = sieve_of_eratosthenes(limit);
    let primes: Vec<u64> = primes.into_iter().take(n).collect();
    let gaps: Vec<u64> = primes.windows(2).map(|w| w[1] - w[0]).collect();

    // Gap frequency distribution
    let mut gap_counts = std::collections::HashMap::new();
    for gap in &gaps {
        *gap_counts.entry(*gap).or_insert(0usize) += 1;
    }

    let mut sorted_gaps: Vec<_> = gap_counts.iter().collect();
    sorted_gaps.sort_by_key(|(gap, _)| *gap);

    println!();
    println!("Most frequent gaps:");
    let mut by_count: Vec<_> = gap_counts.iter().collect();
    by_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (gap, count) in by_count.iter().take(15) {
        let pct = **count as f64 / gaps.len() as f64 * 100.0;
        println!("  Gap {:>4}: {:>8} occurrences ({:5.2}%)", gap, count, pct);
    }

    // Check if gap distribution is "physics-level" structured
    let top_10_pct: f64 = by_count.iter().take(10).map(|(_, c)| **c as f64).sum::<f64>() / gaps.len() as f64;

    println!();
    println!("Top 10 gaps account for {:.1}% of all gaps", top_10_pct * 100.0);

    if top_10_pct > 0.5 {
        println!("⚡ HIGHLY STRUCTURED - only a few gap values dominate!");
        println!("   This is strong evidence of physics-level compressibility.");
    }
}
