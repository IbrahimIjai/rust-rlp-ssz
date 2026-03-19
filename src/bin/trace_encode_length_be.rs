fn to_hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "(empty)".to_string();
    }
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn trace_encode_length_be(mut n: usize) -> Vec<u8> {
    println!("start n = {} (0x{:x})", n, n);

    let mut bytes = Vec::new();
    let mut step = 1;

    while n > 0 {
        let low = n as u8;
        println!("step {}:", step);
        println!("  current n         = {} (0x{:x})", n, n);
        println!("  low byte pushed   = {} (0x{:02x})", low, low);
        bytes.push(low);
        println!("  bytes so far      = {:?} | hex [{}]", bytes, to_hex(&bytes));
        n >>= 8;
        println!("  n after >> 8      = {} (0x{:x})", n, n);
        step += 1;
    }

    println!("before reverse      = {:?} | hex [{}]", bytes, to_hex(&bytes));
    bytes.reverse();
    println!("after reverse       = {:?} | hex [{}]", bytes, to_hex(&bytes));
    println!();

    bytes
}

fn trace_case(label: &str, n: usize) {
    println!("============================================================");
    println!("{}", label);
    let out = trace_encode_length_be(n);
    println!("returned bytes      = {:?} | hex [{}]", out, to_hex(&out));
    println!();
}

fn main() {
    trace_case("case: 0", 0);
    trace_case("case: 1", 1);
    trace_case("case: 55", 55);
    trace_case("case: 56 (first long-string length)", 56);
    trace_case("case: 255", 255);
    trace_case("case: 256", 256);
    trace_case("case: 1024", 1024);

    let short = "dog";
    let long = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    let thousand_as = "a".repeat(1024);

    trace_case(&format!("string length: {:?} -> {}", short, short.len()), short.len());
    trace_case(&format!("string length: lorem ipsum -> {}", long.len()), long.len());
    trace_case(
        &format!("string length: 1024 x 'a' -> {}", thousand_as.len()),
        thousand_as.len(),
    );
}
