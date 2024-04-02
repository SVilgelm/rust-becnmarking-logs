#![feature(test)]

fn stat() {
    log::info!("stat");
}

fn format(name: String) {
    log::info!("hello {}", name);
}

#[cfg(feature = "kv")]
fn kv(name: String) {
    log::info!(name = name; "hello");
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    static ONCE: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        ONCE.call_once(|| {
            structured_logger::Builder::with_level(log::Level::Info.as_str())
                .with_default_writer(structured_logger::json::new_writer(std::io::empty()))
                .init()
        });
    }

    #[bench]
    fn bench_stat(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| stat());
    }

    #[bench]
    fn bench_format(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            format(name);
        });
    }

    #[cfg(feature = "kv")]
    #[bench]
    fn bench_kv(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            kv(name);
        });
    }
}
