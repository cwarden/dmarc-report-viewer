use crate::config::Configuration;
use crate::imap::get_mails;
use crate::parser::{extract_xml_files, parse_xml_file};
use crate::state::AppState;
use crate::summary::Summary;
use crate::xml_error::XmlError;
use anyhow::{Context, Result};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

pub fn start_bg_task(
    config: Configuration,
    state: Arc<Mutex<AppState>>,
    mut stop_signal: Receiver<()>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!(
            "Started background task with check interval of {} secs",
            config.imap_check_interval
        );
        loop {
            match bg_update(&config, &state).await {
                Ok(..) => info!("Finished update cycle without errors"),
                Err(err) => error!("Failed updated cycle: {err:#}"),
            };
            let duration = Duration::from_secs(config.imap_check_interval);
            tokio::select! {
                _ = tokio::time::sleep(duration) => {},
                _ = stop_signal.recv() => { break; },
            }
        }
    })
}

async fn bg_update(config: &Configuration, state: &Arc<Mutex<AppState>>) -> Result<()> {
    info!("Starting background update cycle");
    let mut mails = get_mails(config).await.context("Failed to get mails")?;

    let mut xml_files = Vec::new();
    for mail in &mut mails {
        if mail.body.is_some() {
            match extract_xml_files(mail) {
                Ok(mut files) => xml_files.append(&mut files),
                Err(err) => warn!("Failed to extract XML files from mail: {err:#}"),
            }
        }
    }
    info!("Extracted {} XML files from mails", xml_files.len());

    let mut xml_errors = Vec::new();
    let mut reports = Vec::new();
    for xml_file in &xml_files {
        match parse_xml_file(xml_file) {
            Ok(report) => reports.push(report),
            Err(err) => {
                let error = format!("{err:#}");
                xml_errors.push(XmlError {
                    error,
                    xml: String::from_utf8_lossy(xml_file).to_string(),
                });
            }
        }
    }
    info!("Parsed {} DMARC reports successfully", reports.len());
    if !xml_errors.is_empty() {
        warn!(
            "Failed to parse {} XML file as DMARC reports",
            xml_errors.len()
        );
    }

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("Failed to get Unix time stamp")?
        .as_secs();

    let summary = Summary::new(mails.len(), xml_files.len(), &reports, timestamp);

    {
        let mut locked_state = state.lock().expect("Failed to lock app state");
        locked_state.mails = mails;
        locked_state.xml_files = xml_files.len();
        locked_state.summary = summary;
        locked_state.reports = reports;
        locked_state.last_update = timestamp;
        locked_state.xml_errors = xml_errors;
    }
    info!("Finished updating shared state");

    Ok(())
}
