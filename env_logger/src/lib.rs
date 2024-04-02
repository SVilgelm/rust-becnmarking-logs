#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate test;

    static ONCE: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        ONCE.call_once(|| {
            env_logger::Builder::new()
                .filter_level(log::LevelFilter::max())
                .target(env_logger::Target::Pipe(Box::new(std::io::empty())))
                .init()
        });
    }

    #[bench]
    fn bench_stat(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| print_logger::stat());
    }

    #[bench]
    fn bench_format(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            print_logger::format(name);
        });
    }

    #[bench]
    fn bench_kv(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            print_logger::kv(name);
        });
    }

    #[bench]
    fn bench_kv_with_error(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            print_logger::kv_with_error(name);
        });
    }

    #[bench]
    fn bench_kv_10(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            print_logger::kv_10(name);
        });
    }
}
