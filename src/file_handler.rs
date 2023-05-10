pub mod file_handler_mod {
    use std::fs::File;
    use std::io::Write;
    use std::time::SystemTime;

    use serde_json::json;

    use crate::bmi::Bmi;

    fn now() -> u64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
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
        let output_string = json!({
            "bmi": bmi.value(),
            "height": bmi.height().0,
            "weight": bmi.weight().0,
            "time": now(),
        });
        writeln!(&mut file, "{}", output_string).unwrap();
    }
}
