use crate::detections::print::MessageNotation;
use crate::detections::utils::check_command;
use crate::models::event;
use std::collections::HashMap;

pub struct Sysmon {
    checkunsigned: u16,
}

impl Sysmon {
    pub fn new() -> Sysmon {
        Sysmon { checkunsigned: 0 }
    }

    pub fn detection(
        &mut self,
        event_id: String,
        system: &event::System,
        event_data: HashMap<String, String>,
    ) {
        self.check_command_lines(&event_id, &event_data, &system.time_created.system_time);
        self.check_for_unsigned_files(&event_id, &event_data, &system.time_created.system_time);
    }

    fn check_command_lines(
        &mut self,
        event_id: &String,
        event_data: &HashMap<String, String>,
        system_time: &String,
    ) {
        if event_id != "1" {
            return;
        }

        if let Some(_command_line) = event_data.get("CommandLine") {
            let default = "".to_string();
            let _creater = event_data.get("ParentImage").unwrap_or(&default);

            check_command(1, _command_line, 1000, 0, "", _creater, &system_time);
        }
    }

    fn check_for_unsigned_files(
        &mut self,
        event_id: &String,
        event_data: &HashMap<String, String>,
        system_time: &String,
    ) {
        if event_id != "7" {
            return;
        }

        if self.checkunsigned == 1 {
            let default = "".to_string();
            let _signed = event_data.get("Signed").unwrap_or(&default);
            if _signed == "false" {
                let _image = event_data.get("Image").unwrap_or(&default);
                let _command_line = event_data.get("ImageLoaded").unwrap_or(&default);
                let stdout = std::io::stdout();
                let mut stdout = stdout.lock();
                MessageNotation::info_noheader(&mut stdout, format!("Date : {}", system_time)).ok();
                MessageNotation::info_noheader(
                    &mut stdout,
                    format!("Message : Unsigned Image (DLL)"),
                )
                .ok();
                MessageNotation::info_noheader(
                    &mut stdout,
                    format!("Result  : Loaded by: {}", _image),
                )
                .ok();
                MessageNotation::info_noheader(&mut stdout, format!("Command : {}", _command_line))
                    .ok();
            }
        };
    }
}
