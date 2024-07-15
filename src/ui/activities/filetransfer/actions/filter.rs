use std::str::FromStr;

use regex::Regex;
use remotefs::File;
use wildmatch::WildMatch;

use crate::ui::activities::filetransfer::lib::browser::FileExplorerTab;
use crate::ui::activities::filetransfer::FileTransferActivity;

#[derive(Clone, Debug)]
pub enum Filter {
    Regex(Regex),
    Wildcard(WildMatch),
}

impl FromStr for Filter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // try as regex
        if let Ok(regex) = Regex::new(s) {
            Ok(Self::Regex(regex))
        } else {
            Ok(Self::Wildcard(WildMatch::new(s)))
        }
    }
}

impl Filter {
    fn matches(&self, s: &str) -> bool {
        debug!("matching '{s}' with {:?}", self);
        match self {
            Self::Regex(re) => re.is_match(s),
            Self::Wildcard(wm) => wm.matches(s),
        }
    }
}

impl FileTransferActivity {
    pub fn filter(&self, filter: &str) -> Vec<File> {
        let filter = Filter::from_str(filter).unwrap();

        match self.browser.tab() {
            FileExplorerTab::Local => self.browser.local().iter_files(),
            FileExplorerTab::Remote => self.browser.remote().iter_files(),
            _ => return vec![],
        }
        .filter(|f| filter.matches(&f.name()))
        .cloned()
        .collect()
    }
}
