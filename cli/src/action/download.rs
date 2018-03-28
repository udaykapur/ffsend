use std::sync::{Arc, Mutex};

use ffsend_api::action::download::Download as ApiDownload;
use ffsend_api::file::file::DownloadFile;
use ffsend_api::reqwest::Client;

use cmd::cmd_download::CmdDownload;
use error::ActionError;
use progress::ProgressBar;

/// A file download action.
pub struct Download<'a> {
    cmd: &'a CmdDownload<'a>,
}

impl<'a> Download<'a> {
    /// Construct a new download action.
    pub fn new(cmd: &'a CmdDownload<'a>) -> Self {
        Self {
            cmd,
        }
    }

    /// Invoke the download action.
    // TODO: create a trait for this method
    pub fn invoke(&self) -> Result<(), ActionError> {
        // Get the share URL
        let url = self.cmd.url();

        // Create a reqwest client
        let client = Client::new();

        // Parse the file based on the URL
        // TODO: handle error here
        let file = DownloadFile::parse_url(url)
            .expect("invalid share URL, could not parse file data");

        // Create a progress bar reporter
        let bar = Arc::new(Mutex::new(ProgressBar::new_download()));

        // Execute an download action
        ApiDownload::new(&file, self.cmd.password())
            .invoke(&client, bar)?;

        // TODO: open the file, or it's location
        // TODO: copy the file location

        Ok(())
    }
}