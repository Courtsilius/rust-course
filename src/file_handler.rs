pub mod file_handler_mod {
    use std::time::SystemTime;
    use std::fs::File;
    use std::io::Write;

    use serde_json::json;

    use crate::bmi::Bmi;

    fn now() -> u64{
        let r = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        r
    }

    pub fn save(bmi: Bmi) {
        let mut file = match File::options()
            .create(true)
            .append(true)
            .open("example.log")
        {
            Ok(file) => {
                log::debug!("Opened or created file successfully");
                file
            }
            Err(e) => {
                log::error!("error creating or opening file: {e:?}");
                std::process::exit(1)
            }
        };
        let output_string = serde_json::to_string(&bmi);
        writeln!(&mut file, "{}", output_string.unwrap());
    }
}
