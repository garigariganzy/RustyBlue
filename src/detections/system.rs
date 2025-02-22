use crate::detections::print::MessageNotation;
use crate::detections::utils;
use crate::models::event;
use std::collections::HashMap;

pub struct System {}

impl System {
    pub fn new() -> System {
        System {}
    }

    pub fn detection(
        &mut self,
        event_id: String,
        system: &event::System,
        event_data: HashMap<String, String>,
    ) {
        self.system_log_clear(&event_id, &system.time_created.system_time);
        self.windows_event_log(&event_id, &event_data, &system.time_created.system_time);
        self.new_service_created(&event_id, &event_data, &system.time_created.system_time);
        self.interactive_service_warning(&event_id, &event_data, &system.time_created.system_time);
        self.suspicious_service_name(&event_id, &event_data, &system.time_created.system_time);
    }

    fn new_service_created(
        &mut self,
        event_id: &String,
        event_data: &HashMap<String, String>,
        system_time: &String,
    ) {
        if event_id != "7045" {
            return;
        }

        let default = String::from("");
        let servicename = &event_data.get("ServiceName").unwrap_or(&default);
        let commandline = &event_data.get("ImagePath").unwrap_or(&default);
        let text = utils::check_regex(&servicename, 1);
        if !text.is_empty() {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            MessageNotation::info_noheader(&mut stdout, format!("Date    : {}", system_time)).ok();
            MessageNotation::info_noheader(&mut stdout, format!("Message : New Service Created"))
                .ok();
            MessageNotation::info_noheader(&mut stdout, format!("Command : {}", commandline)).ok();
            MessageNotation::info_noheader(
                &mut stdout,
                format!("Results : Service name: {}", servicename),
            )
            .ok();
            MessageNotation::info_noheader(&mut stdout, format!("Results : {}", text)).ok();
        }
        if !commandline.is_empty() {
            utils::check_command(7045, &commandline, 1000, 0, &servicename, &"", &system_time);
        }
    }

    fn interactive_service_warning(
        &mut self,
        event_id: &String,
        event_data: &HashMap<String, String>,
        system_time: &String,
    ) {
        if event_id != "7030" {
            return;
        }

        let default = String::from("");
        let servicename = &event_data.get("param1").unwrap_or(&default);
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        MessageNotation::info_noheader(&mut stdout, format!("Date    : {}", system_time)).ok();
        MessageNotation::info_noheader(
            &mut stdout,
            format!("Message : Interactive service warning"),
        )
        .ok();
        MessageNotation::info_noheader(
            &mut stdout,
            format!("Results : Service name: {}", servicename),
        )
        .ok();
        MessageNotation::info_noheader(
            &mut stdout,
            format!("Results : Service name: {}", servicename),
        )
        .ok();

        MessageNotation::info_noheader(
            &mut stdout,
            format!("Results : Malware (and some third party software) trigger this warning"),
        )
        .ok();
        MessageNotation::info_noheader(
            &mut stdout,
            format!("{}", utils::check_regex(&servicename, 1)),
        )
        .ok();
    }

    fn suspicious_service_name(
        &mut self,
        event_id: &String,
        event_data: &HashMap<String, String>,
        system_time: &String,
    ) {
        if event_id != "7036" {
            return;
        }

        let default = String::from("");
        let servicename = &event_data.get("param1").unwrap_or(&default);
        let text = utils::check_regex(&servicename, 1);
        if !text.is_empty() {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            MessageNotation::info_noheader(&mut stdout, format!("Date    : {}", system_time)).ok();
            MessageNotation::info_noheader(
                &mut stdout,
                format!("Message : Suspicious Service Name"),
            )
            .ok();
            MessageNotation::info_noheader(
                &mut stdout,
                format!("Results : Service name: {}", servicename),
            )
            .ok();
            MessageNotation::info_noheader(&mut stdout, format!("Results : {}", text)).ok();
        }
    }

    fn system_log_clear(&mut self, event_id: &String, system_time: &String) {
        if event_id != "104" {
            return;
        }
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        MessageNotation::info_noheader(&mut stdout, format!("Date : {}", system_time)).ok();
        MessageNotation::info_noheader(&mut stdout, format!("Message : System Log Clear")).ok();
        MessageNotation::info_noheader(
            &mut stdout,
            format!("Results : The System log was cleared."),
        )
        .ok();
    }

    fn windows_event_log(
        &mut self,
        event_id: &String,
        event_data: &HashMap<String, String>,
        system_time: &String,
    ) {
        if event_id != "7040" {
            return;
        }

        if let Some(_param1) = event_data.get("param1") {
            if _param1 == "Windows Event Log" {
                let stdout = std::io::stdout();
                let mut stdout = stdout.lock();
                MessageNotation::info_noheader(&mut stdout, format!("Date : {}", system_time)).ok();
                MessageNotation::info_noheader(&mut stdout, format!("Service name : {}", _param1))
                    .ok();
                if let Some(_param2) = event_data.get("param2") {
                    if _param2 == "disabled" {
                        MessageNotation::info_noheader(
                            &mut stdout,
                            format!("Message : Event Log Service Stopped"),
                        )
                        .ok();
                        MessageNotation::info_noheader(
                            &mut stdout,
                            format!(
                                "Results : Selective event log manipulation may follow this event."
                            ),
                        )
                        .ok();
                    } else if _param2 == "auto start" {
                        MessageNotation::info_noheader(
                            &mut stdout,
                            format!("Message : Event Log Service Started"),
                        )
                        .ok();
                        MessageNotation::info_noheader(
                            &mut stdout,
                            format!("Results : Selective event log manipulation may precede this event."),
                        )
                        .ok();
                    }
                }
            }
        }
    }
}
