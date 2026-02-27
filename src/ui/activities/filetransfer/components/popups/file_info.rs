use std::time::UNIX_EPOCH;

use bytesize::ByteSize;
use remotefs::File;
use tui_realm_stdlib::List;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};
#[cfg(posix)]
use uzers::{get_group_by_gid, get_user_by_uid};

use crate::ui::activities::filetransfer::{Msg, UiMsg};
use crate::utils::fmt::fmt_time;

#[derive(MockComponent)]
pub struct FileInfoPopup {
    component: List,
}

impl FileInfoPopup {
    pub fn new(file: &File) -> Self {
        let mut texts: TableBuilder = TableBuilder::default();
        // Abs path
        let real_path = file.metadata().symlink.as_deref();
        let path: String = match real_path {
            Some(symlink) => format!("{} -> {}", file.path().display(), symlink.display()),
            None => format!("{}", file.path().display()),
        };
        // Make texts
        texts
            .add_col(TextSpan::from("Path: "))
            .add_col(TextSpan::new(path.as_str()).fg(Color::Yellow));
        texts
            .add_row()
            .add_col(TextSpan::from("Name: "))
            .add_col(TextSpan::new(file.name()).fg(Color::Yellow));
        if let Some(filetype) = file.extension() {
            texts
                .add_row()
                .add_col(TextSpan::from("File type: "))
                .add_col(TextSpan::new(filetype).fg(Color::LightGreen));
        }
        let (bsize, size): (ByteSize, u64) = (ByteSize(file.metadata().size), file.metadata().size);
        texts
            .add_row()
            .add_col(TextSpan::from("Size: "))
            .add_col(TextSpan::new(format!("{bsize} ({size})").as_str()).fg(Color::Cyan));
        let atime: String = fmt_time(
            file.metadata().accessed.unwrap_or(UNIX_EPOCH),
            "%b %d %Y %H:%M:%S",
        );
        let ctime: String = fmt_time(
            file.metadata().created.unwrap_or(UNIX_EPOCH),
            "%b %d %Y %H:%M:%S",
        );
        let mtime: String = fmt_time(
            file.metadata().modified.unwrap_or(UNIX_EPOCH),
            "%b %d %Y %H:%M:%S",
        );
        texts
            .add_row()
            .add_col(TextSpan::from("Creation time: "))
            .add_col(TextSpan::new(ctime.as_str()).fg(Color::LightGreen));
        texts
            .add_row()
            .add_col(TextSpan::from("Last modified time: "))
            .add_col(TextSpan::new(mtime.as_str()).fg(Color::LightBlue));
        texts
            .add_row()
            .add_col(TextSpan::from("Last access time: "))
            .add_col(TextSpan::new(atime.as_str()).fg(Color::LightRed));
        // User
        #[cfg(posix)]
        let username: String = match file.metadata().uid {
            Some(uid) => match get_user_by_uid(uid) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => uid.to_string(),
            },
            None => String::from("0"),
        };
        #[cfg(win)]
        let username: String = format!("{}", file.metadata().uid.unwrap_or(0));
        // Group
        #[cfg(posix)]
        let group: String = match file.metadata().gid {
            Some(gid) => match get_group_by_gid(gid) {
                Some(group) => group.name().to_string_lossy().to_string(),
                None => gid.to_string(),
            },
            None => String::from("0"),
        };
        #[cfg(win)]
        let group: String = format!("{}", file.metadata().gid.unwrap_or(0));
        texts
            .add_row()
            .add_col(TextSpan::from("User: "))
            .add_col(TextSpan::new(username.as_str()).fg(Color::LightYellow));
        texts
            .add_row()
            .add_col(TextSpan::from("Group: "))
            .add_col(TextSpan::new(group.as_str()).fg(Color::Blue));
        Self {
            component: List::default()
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .scroll(false)
                .title(file.name(), Alignment::Left)
                .rows(texts.build()),
        }
    }
}

impl Component<Msg, NoUserEvent> for FileInfoPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseFileInfoPopup)),
            _ => None,
        }
    }
}
