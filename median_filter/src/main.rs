
struct median_filter_t<'a> {
    window: &'a mut u8,
    input: &'a mut usize,
    hole: &'a mut i64,
    max: &'a mut i64,
}

fn median_init(f: median_filter_t, w_size: isize) -> i64 {
    *f.max = 0;
    *f.hole = 0;

    return 0;
}

fn main() {
    println!("Hello, world!");
}
