use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Estimates {
    mean: Estimate,
}

#[derive(Deserialize)]
struct Estimate {
    point_estimate: f64,
}

#[derive(Deserialize)]
struct BenchmarkJson {
    throughput: Option<ThroughputData>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ThroughputData {
    Bytes { Bytes: u64 },
    Array(Vec<ThroughputEntry>),
}

#[derive(Deserialize)]
struct ThroughputEntry {
    per_iteration: u64,
}

#[derive(Deserialize)]
struct TokenCounts {
    #[serde(flatten)]
    languages: HashMap<String, HashMap<String, HashMap<String, u64>>>,
}

fn load_token_counts() -> TokenCounts {
    let json_path = "pkgs/benchmarks/token_counts.json";
    let content = fs::read_to_string(json_path).expect(
        "Failed to read token_counts.json. Make sure you're running from the project root.",
    );
    serde_json::from_str(&content).expect("Failed to parse token_counts.json")
}

fn format_time(nanos: f64) -> String {
    if nanos < 1_000.0 {
        format!("{:.2} ns", nanos)
    } else if nanos < 1_000_000.0 {
        format!("{:.2} µs", nanos / 1_000.0)
    } else if nanos < 1_000_000_000.0 {
        format!("{:.2} ms", nanos / 1_000_000.0)
    } else {
        format!("{:.2} s", nanos / 1_000_000_000.0)
    }
}

fn format_throughput(bytes: u64, nanos: f64) -> String {
    if bytes == 0 {
        return "N/A".to_string();
    }
    let seconds = nanos / 1_000_000_000.0;
    let bytes_per_sec = bytes as f64 / seconds;
    let mb_per_sec = bytes_per_sec / (1024.0 * 1024.0);
    format!("{:.2} MiB/s", mb_per_sec)
}

fn format_ops_per_sec(nanos: f64) -> String {
    let ops = 1_000_000_000.0 / nanos;
    if ops >= 1_000_000.0 {
        format!("{:.1} M ops/s", ops / 1_000_000.0)
    } else if ops >= 1_000.0 {
        format!("{:.1} K ops/s", ops / 1_000.0)
    } else {
        format!("{:.1} ops/s", ops)
    }
}

fn format_token_throughput(tokens: u64, nanos: f64) -> String {
    if tokens == 0 {
        return "N/A".to_string();
    }
    let seconds = nanos / 1_000_000_000.0;
    let tokens_per_sec = tokens as f64 / seconds;

    if tokens_per_sec >= 1_000_000.0 {
        format!("{:.2} M tokens/s", tokens_per_sec / 1_000_000.0)
    } else if tokens_per_sec >= 1_000.0 {
        format!("{:.2} K tokens/s", tokens_per_sec / 1_000.0)
    } else {
        format!("{:.0} tokens/s", tokens_per_sec)
    }
}

fn parse_benchmark(
    base_dir: &Path,
    parser: &str,
    size: &str,
    token_counts: &TokenCounts,
    bench_name: &str,
) -> Option<(f64, u64, u64)> {
    let bench_path = base_dir.join(parser).join(size);

    if !bench_path.exists() {
        return None;
    }

    // Read estimates
    let estimates_path = bench_path.join("new").join("estimates.json");
    if !estimates_path.exists() {
        return None;
    }

    let content = fs::read_to_string(estimates_path).ok()?;
    let estimates: Estimates = serde_json::from_str(&content).ok()?;

    // Try to read throughput
    let benchmark_path = bench_path.join("new").join("benchmark.json");
    let bytes = if benchmark_path.exists() {
        let content = fs::read_to_string(benchmark_path).ok()?;
        let benchmark: BenchmarkJson = serde_json::from_str(&content).ok()?;
        match benchmark.throughput {
            Some(ThroughputData::Bytes { Bytes }) => Bytes,
            Some(ThroughputData::Array(ref arr)) => {
                arr.first().map(|e| e.per_iteration).unwrap_or(0)
            }
            None => 0,
        }
    } else {
        0
    };

    // Get token count from JSON
    let tokens = token_counts
        .languages
        .get(bench_name)?
        .get(parser)?
        .get(size)
        .copied()
        .unwrap_or(0);

    Some((estimates.mean.point_estimate, bytes, tokens))
}

struct BenchResult<'a> {
    time: f64,
    bytes: u64,
    tokens: u64,
    parser: &'a str,
}

struct ParserResult {
    name: String,
    time: f64,
    bytes: u64,
    tokens: u64,
}

fn main() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 VOLUMEN PARSER BENCHMARKS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let criterion_dir = Path::new("target/criterion");
    if !criterion_dir.exists() {
        println!("❌ No benchmark results found. Run 'cargo bench' first.\n");
        return;
    }

    // Load token counts
    let token_counts = load_token_counts();

    // Collect all results for cross-language comparison
    let mut all_small: Vec<ParserResult> = Vec::new();
    let mut all_medium: Vec<ParserResult> = Vec::new();
    let mut all_large: Vec<ParserResult> = Vec::new();

    let languages = vec![
        ("go_parsers", "Go Parser", vec!["Tree-sitter"]),
        ("java_parsers", "Java Parser", vec!["Tree-sitter"]),
        ("csharp_parsers", "C# Parser", vec!["Tree-sitter"]),
        ("php_parsers", "PHP Parser", vec!["Tree-sitter"]),
        ("ruby_parsers", "Ruby Parser", vec!["Tree-sitter"]),
        (
            "typescript_parsers",
            "TypeScript Parser",
            vec!["Oxc", "Tree-sitter"],
        ),
        (
            "python_parsers",
            "Python Parser",
            vec!["Ruff", "RustPython", "Tree-sitter"],
        ),
    ];

    for (bench_name, display_name, parsers) in languages {
        let bench_dir = criterion_dir.join(bench_name);
        if !bench_dir.exists() {
            continue;
        }

        println!("Running \"{}\" suite...", display_name);
        println!("  Progress: 100%\n");

        let sizes = vec!["small", "medium", "large"];
        let mut all_results: HashMap<&str, Vec<BenchResult>> = HashMap::new();

        // Collect results for each parser and size
        for parser in &parsers {
            for size in &sizes {
                if let Some((time, bytes, tokens)) =
                    parse_benchmark(&bench_dir, parser, size, &token_counts, bench_name)
                {
                    all_results
                        .entry(size)
                        .or_insert_with(Vec::new)
                        .push(BenchResult {
                            time,
                            bytes,
                            tokens,
                            parser,
                        });

                    // Also collect for cross-language comparison
                    let parser_name = if parsers.len() > 1 && *parser != "Tree-sitter" {
                        format!("{} ({})", display_name.replace(" Parser", ""), parser)
                    } else {
                        display_name.replace(" Parser", "")
                    };

                    match *size {
                        "small" => all_small.push(ParserResult {
                            name: parser_name.clone(),
                            time,
                            bytes,
                            tokens,
                        }),
                        "medium" => all_medium.push(ParserResult {
                            name: parser_name.clone(),
                            time,
                            bytes,
                            tokens,
                        }),
                        "large" => all_large.push(ParserResult {
                            name: parser_name.clone(),
                            time,
                            bytes,
                            tokens,
                        }),
                        _ => {}
                    }
                }
            }
        }

        if all_results.is_empty() {
            println!("  ⚠️  No results found\n");
            continue;
        }

        let mut total_cases = 0;

        // Display results by size
        for size in &sizes {
            if let Some(results) = all_results.get(size) {
                if results.is_empty() {
                    continue;
                }

                // Find fastest for this size
                let fastest_time = results
                    .iter()
                    .map(|r| r.time)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();

                let slowest_time = results
                    .iter()
                    .map(|r| r.time)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();

                println!("  Parse {} code:", size);

                for result in results {
                    let time_str = format_time(result.time);
                    let throughput_str = format_throughput(result.bytes, result.time);
                    let token_throughput_str = format_token_throughput(result.tokens, result.time);
                    let ops_str = format_ops_per_sec(result.time);

                    let mut tags = Vec::new();
                    if results.len() > 1 {
                        if result.parser != "Tree-sitter" {
                            tags.push(result.parser.to_string());
                        }
                    }

                    if result.time == fastest_time {
                        tags.push("fastest".to_string());
                    } else if result.time == slowest_time && results.len() > 2 {
                        let pct_slower = ((result.time / fastest_time) - 1.0) * 100.0;
                        tags.push(format!("slowest, {:.1}% slower", pct_slower));
                    } else if results.len() > 1 {
                        let pct_slower = ((result.time / fastest_time) - 1.0) * 100.0;
                        tags.push(format!("{:.1}% slower", pct_slower));
                    }

                    let tag_str = if tags.is_empty() {
                        String::new()
                    } else {
                        format!("   | {}", tags.join(", "))
                    };

                    println!(
                        "    {} ({}, {} | {}){}",
                        time_str, ops_str, throughput_str, token_throughput_str, tag_str
                    );

                    total_cases += 1;
                }
                println!();
            }
        }

        println!("Finished {} cases!", total_cases);

        if parsers.len() > 1 {
            // Show fastest parser
            let mut parser_avg_times: HashMap<&str, (f64, usize)> = HashMap::new();
            for results in all_results.values() {
                for result in results {
                    let entry = parser_avg_times.entry(result.parser).or_insert((0.0, 0));
                    entry.0 += result.time;
                    entry.1 += 1;
                }
            }

            let fastest_parser = parser_avg_times
                .iter()
                .min_by(|(_, (time1, count1)), (_, (time2, count2))| {
                    (time1 / *count1 as f64)
                        .partial_cmp(&(time2 / *count2 as f64))
                        .unwrap()
                })
                .map(|(parser, _)| *parser);

            if let Some(fastest) = fastest_parser {
                println!("  Fastest: {}", fastest);
            }
        }

        println!("  Results saved to: target/criterion/{}/\n", bench_name);
    }

    // Display cross-language comparison
    if !all_small.is_empty() || !all_medium.is_empty() || !all_large.is_empty() {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🏆 CROSS-LANGUAGE COMPARISON");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let print_ranking = |size: &str, results: &mut Vec<ParserResult>| {
            if results.is_empty() {
                return;
            }

            // Sort by token throughput (highest first)
            results.sort_by(|a, b| {
                let tokens_per_sec_a = a.tokens as f64 / (a.time / 1_000_000_000.0);
                let tokens_per_sec_b = b.tokens as f64 / (b.time / 1_000_000_000.0);
                tokens_per_sec_b.partial_cmp(&tokens_per_sec_a).unwrap()
            });

            println!("{}:", size.to_uppercase());
            println!();
            println!("│ Rank │ Parser                   │ Time     │ Byte Throughput │ Token Throughput │");
            println!("├──────┼──────────────────────────┼──────────┼─────────────────┼──────────────────┤");

            for (i, result) in results.iter().enumerate() {
                let rank = i + 1;
                let time_str = format_time(result.time);
                let byte_throughput_str = format_throughput(result.bytes, result.time);
                let token_throughput_str = format_token_throughput(result.tokens, result.time);

                // Pad the parser name to 24 chars
                let mut name = result.name.clone();
                if name.len() > 24 {
                    name.truncate(21);
                    name.push_str("...");
                } else {
                    while name.len() < 24 {
                        name.push(' ');
                    }
                }

                println!(
                    "│ {:>4} │ {} │ {:>8} │ {:>15} │ {:>16} │",
                    rank, name, time_str, byte_throughput_str, token_throughput_str
                );
            }
            println!();
        };

        print_ranking("Small Code Samples", &mut all_small);
        print_ranking("Medium Code Samples", &mut all_medium);
        print_ranking("Large Code Samples", &mut all_large);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ All benchmarks complete!\n");
    println!("📄 View detailed report: BENCHMARK_REPORT.md");
    println!("🌐 View HTML reports: target/criterion/report/index.html");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
