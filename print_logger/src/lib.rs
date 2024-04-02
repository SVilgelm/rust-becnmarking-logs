#![feature(test)]

fn stat() {
    log::info!("stat");
}

fn format(name: String) {
    log::info!("hello {}", name);
}

fn kv(name: String) {
    log::info!(name = name; "hello");
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use std::io::Write;

    // formtatting extra key value pairs from kv_unstable feature
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
}
