use std::hint::black_box;
use std::time::Instant;

use windy::lexer::Lexer;

fn make_repetitive_source(lines: usize) -> String {
    let mut s = String::with_capacity(lines * 16);
    for i in 0..lines {
        s.push_str("var");
        s.push_str(&i.to_string());
        s.push_str(" = ");
        s.push_str(&(i * 37).to_string());
        s.push_str(";\n");
    }
    s
}

fn make_mixed_source(lines: usize) -> String {
    let mut s = String::with_capacity(lines * 24);
    let mut x: u64 = 0x1234_5678_9abc_def0;

    for i in 0..lines {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);

        let a = (x % 10_000) as usize;
        let b = ((x >> 16) % 10_000) as usize;

        match i % 4 {
            0 => {
                s.push_str("let x");
                s.push_str(&a.to_string());
                s.push_str(" = ");
                s.push_str(&b.to_string());
                s.push_str(";\n");
            }
            1 => {
                s.push_str("foo");
                s.push_str(&a.to_string());
                s.push_str(" = bar");
                s.push_str(&b.to_string());
                s.push_str(";\n");
            }
            2 => {
                s.push_str("if x");
                s.push_str(&a.to_string());
                s.push_str(" == ");
                s.push_str(&b.to_string());
                s.push_str(" {\n}\n");
            }
            _ => {
                s.push_str("value");
                s.push_str(&a.to_string());
                s.push_str(" = ");
                s.push_str(&b.to_string());
                s.push_str(";\n");
            }
        }
    }

    s
}

fn bench_case(name: &str, source: &str, iterations: usize) {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::with_capacity(100_000);

    // warm up
    for _ in 0..10 {
        tokens.clear();
        lexer.lex_into_and_reset(&mut tokens);
        black_box(tokens.len());
    }

    let start = Instant::now();
    let mut total_tokens = 0usize;

    for _ in 0..iterations {
        tokens.clear();

        lexer.lex_into_and_reset(&mut tokens);

        total_tokens += black_box(tokens.len());
        black_box(&tokens);
    }

    let elapsed = start.elapsed();
    let nanos = elapsed.as_nanos() as f64;
    let bytes = source.len() as f64 * iterations as f64;

    let ns_per_iter = nanos / iterations as f64;
    let ns_per_byte = nanos / bytes.max(1.0);
    let mb_per_sec = (bytes / elapsed.as_secs_f64()) / (1024.0 * 1024.0);

    println!("=== {} ===", name);
    println!("input bytes      : {}", source.len());
    println!("iterations       : {}", iterations);
    println!("total tokens     : {}", total_tokens);
    println!("total time       : {:?}", elapsed);
    println!("ns / iter        : {:.2}", ns_per_iter);
    println!("ns / byte        : {:.4}", ns_per_byte);
    println!("throughput       : {:.2} MB/s", mb_per_sec);
    println!();
}

fn main() {
    let small = make_repetitive_source(1_000);
    let medium = make_mixed_source(1_000);
    let large = make_mixed_source(10_000);

    bench_case("repetitive-1k", &small, 1_000);
    bench_case("mixed-1k", &medium, 500);
    bench_case("mixed-10k", &large, 100);
}