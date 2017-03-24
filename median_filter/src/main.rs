
struct median_filter_t {
    window: Vec<u8>,
    input: Vec<u8>,
    hole: i64,
    max: i64,
}

fn median_close_hole(f: &mut median_filter_t) {
    while f.hole < f.max - 1 {
        let i: usize = f.hole as usize;
        f.window[i] = f.window[i + 1];
        f.hole += 1;
    }

    if f.hole == f.max - 1 {
        f.max = f.max - 1;
    }
}

fn media_remove(f: &mut median_filter_t, v: u8) {
    median_close_hole(f);

    let mut lo: i64 = 0;
    let mut hi: i64 = f.max - 1;

    f.hole = (hi / 2) as i64;

    while lo <= hi {
        let i: usize = f.hole as usize;
        if v > f.window[i] {
            lo = f.hole + 1;
        } else if v < f.window[i] {
            hi = f.hole - 1;
        } else {
            break;
        }
        f.hole = (lo + hi) / 2;
    }
}

fn median_add(f: &mut median_filter_t, v: u8) {
    if f.hole == f.max {
        f.max += 1;
    }

    let mut i: usize = f.hole as usize;
    while (f.hole > 0) && (f.window[i] > v) {
        f.window[i] = f.window[i-1];
        f.hole -= 1;
        i = f.hole as usize;
    }

    i = f.hole as usize;

    while (f.hole < f.max - 1) && (f.window[i+1] < v) {
        f.window[i] = f.window[i+1];
        f.hole += 1;
        i = f.hole as usize;
    }

    f.window[i] = v;
    f.hole = f.max;
}

fn median_get(f: &mut median_filter_t) -> u8 {
    median_close_hole(f);

    if f.max == 0 {
        return 0;
    }

    if (f.max & 1i64) == 1 {
        let i: usize = f.max as usize;
        return f.window[(i-1)/2];
    } else {
        let i: usize = f.max as usize;
        return (f.window[(i-1)/2] + f.window[i/2]) / 2;
    }
}

fn median_filter(input: &mut Vec<u8>, size: usize, w_size: usize) -> i32 {
    let mut f: median_filter_t = median_filter_t{ max: 0, hole: 0, window: vec![0; w_size], input: vec![0, (w_size/2) as u8]};
    let mut i: usize = 0;
    let mut start: i64 = -(w_size as i64);
    let mut end: usize = 0;
    let tmp: i64 = w_size as i64;
    let mut pos: i64 = -(tmp - (tmp/2)) + 1;

    while pos < size as i64 {
        if start >= 0 {
            let temp: u8 = f.input[i];
            media_remove(&mut f, temp);
        }

        if end < size {
            median_add(&mut f, input[end]);
        }

        if pos >= 0 {
            f.input[i] = input[pos as usize];
            if !((i+1) < (w_size / 2)) {
                i = 0;
            }
            input[pos as usize] = median_get(&mut f);
        }
        start += 1;
        end += 1;
        pos += 1;
    }

    return 0;
}

fn main() {
    println!("Hello, world!");
}
