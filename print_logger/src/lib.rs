#![feature(test)]

use std::{error::Error, fmt};

pub fn stat() {
    log::info!("stat");
}

pub fn format(name: String) {
    log::info!("hello {}", name);
}

pub fn kv(name: String) {
    log::info!(name = name; "hello");
}

pub fn kv_with_error(name: String) {
    let e = TestError;
    log::info!(name = name, error:err = e; "hello");
}

pub fn kv_10(name: String) {
    log::info!(name0 = name, name1 = name, name2 = name, name3 = name, name4 = name, name5 = name, name6 = name, name7 = name, name8 = name, name9 = name; "hello");
}

struct TestError;

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}

impl fmt::Debug for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug error")
    }
}

impl Error for TestError {}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use std::io::Write;

    struct KVVisitor<'a> {
        buf: &'a mut String,
        visited: bool,
    }

    impl<'kvs> log::kv::VisitSource<'kvs> for KVVisitor<'_> {
        fn visit_pair(
            &mut self,
            key: log::kv::Key<'kvs>,
            value: log::kv::Value<'kvs>,
        ) -> Result<(), log::kv::Error> {
            if self.visited {
                self.buf.push_str(", ");
            } else {
                self.visited = true;
            }
            self.buf.push_str(format!("{}={}", key, value).as_str());
            Ok(())
        }
    }

    struct PrintLogger;
    impl log::Log for PrintLogger {
        fn enabled(&self, _: &log::Metadata) -> bool {
            true
        }

        fn log(&self, record: &log::Record) {
            let mut out = std::io::empty();
            let pairs = record.key_values();
            if pairs.count() == 0 {
                out.write_fmt(format_args!("{} - {}", record.level(), record.args()))
                    .unwrap();
                return;
            }

            let mut buf = String::new();
            let mut visitor = KVVisitor {
                buf: &mut buf,
                visited: false,
            };
            pairs.visit(&mut visitor).unwrap();

            out.write_fmt(format_args!(
                "{} - {}: {}",
                record.level(),
                record.args(),
                buf
            ))
            .unwrap();
        }

        fn flush(&self) {}
    }

    static LOGGER: PrintLogger = PrintLogger;
    static ONCE: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        ONCE.call_once(|| {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(log::LevelFilter::Info))
                .unwrap();
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

    #[bench]
    fn bench_kv(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            kv(name);
        });
    }

    #[bench]
    fn bench_kv_with_error(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            kv_with_error(name);
        });
    }

    #[bench]
    fn bench_kv_10(b: &mut test::Bencher) {
        init_logger();
        b.iter(|| {
            let name = String::from("foo");
            kv_10(name);
        });
    }
}
