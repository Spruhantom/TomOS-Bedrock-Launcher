//! Read-only JSON service consumed by the Qt desktop shell.
//! This process only inspects host metadata and runtime executable presence.

use std::fmt::Write;

use tomos_bedrock_android::AndroidBackend;
use tomos_bedrock_core::{
    backend::{Backend, BackendProbe},
    dashboard::DashboardSnapshot,
    host::detect_host,
};
use tomos_bedrock_windows::WindowsBackend;

fn json_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                let _ = write!(output, "\\u{:04x}", character as u32);
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn dashboard_json() -> String {
    let host = detect_host();
    let backends: [&dyn Backend; 2] = [&WindowsBackend, &AndroidBackend];
    let probes: Vec<BackendProbe> = backends
        .iter()
        .map(|backend| backend.probe(&host))
        .collect();
    let snapshot = DashboardSnapshot::build(&host, &probes);

    let mut output = String::new();
    let _ = write!(
        output,
        "{{\"distribution\":{},\"architecture\":{},\"session_type\":{},\"backends\":[",
        json_string(&snapshot.distribution),
        json_string(&snapshot.architecture),
        json_string(&snapshot.session_type)
    );

    for (index, (card, probe)) in snapshot.backends.iter().zip(probes.iter()).enumerate() {
        if index > 0 {
            output.push(',');
        }
        let kind = match card.kind {
            tomos_bedrock_core::BackendKind::Windows => "windows",
            tomos_bedrock_core::BackendKind::Android => "android",
        };
        let _ = write!(
            output,
            "{{\"kind\":{},\"name\":{},\"available\":{},\"status\":{},\"diagnostics\":[",
            json_string(kind),
            json_string(card.name),
            card.available,
            json_string(if card.available {
                "Runtime executable detected; game compatibility not yet verified"
            } else {
                card.status
            })
        );
        for (diagnostic_index, diagnostic) in probe.diagnostics.iter().enumerate() {
            if diagnostic_index > 0 {
                output.push(',');
            }
            output.push_str(&json_string(&diagnostic.message));
        }
        output.push_str("]}");
    }
    output.push_str("]}");
    output
}

fn main() {
    println!("{}", dashboard_json());
}

#[cfg(test)]
mod tests {
    use super::json_string;

    #[test]
    fn escapes_json_control_characters() {
        assert_eq!(
            json_string("line 1\n\"quoted\""),
            "\"line 1\\n\\\"quoted\\\"\""
        );
    }
}
