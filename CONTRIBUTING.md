# Contributing

Before contributing to this repository, please first discuss the change you wish to make via issue of this repository before making a change.
Please note we have a [code of conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

- [Contributing](#contributing)
  - [Preferred contributions](#preferred-contributions)
  - [Pull Request Process](#pull-request-process)
  - [Developer contributions guide](#developer-contributions-guide)
    - [How TermSCP works](#how-termscp-works)
      - [Activities](#activities)
    - [Implementing File Transfers](#implementing-file-transfers)

---

## Preferred contributions

At the moment, these kind of contributions are more appreciated and should be preferred:

- Fix for issues described in [Known Issues](./README.md#known-issues) or [issues reported by the community](https://github.com/veeso/termscp/issues)
- New file transfers: for further details see [Implementing File Transfer](#implementing-file-transfers)
- Code optimizations: any optimization to the code is welcome

For any other kind of contribution, especially for new features, please submit an issue first.

## Pull Request Process

Let's make it simple and clear:

1. Open an issue with an **appropriate label** (e.g. bug, enhancement, ...).
2. Write a **properly documentation** compliant with **rustdoc** standard.
3. Write tests for your code. This doesn't apply necessarily for implementation regarding the user-interface module (`ui`) and (if a test server is not available) for file transfers.
4. Report changes to the issue you opened, writing a report of what you changed and what you have introduced.
5. Update the `CHANGELOG.md` file with details of changes to the application. In changelog report changes under a chapter called `PR{PULL_REQUEST_NUMBER}` (e.g. PR12).
6. Request maintainers to merge your changes.

## Developer contributions guide

Welcome to the contributions guide for TermSCP. This chapter DOESN'T contain the documentation for TermSCP, which can instead be found on Rust Docs at <https://docs.rs/termscp>
This chapter describes how TermSCP works and the guide lines to implement stuff such as file transfers and add features to the user interface.

### How TermSCP works

TermSCP is basically made up of 4 components:

- the **filetransfer**: the filetransfer takes care of managing the remote file system; it provides function to establish a connection with the remote, operating on the remote server file system (e.g. remove files, make directories, rename files, ...), read files and write files. The FileTransfer, as we'll see later, is actually a trait, and for each protocol a FileTransfer must be implement the trait.
- the **host**: the host module provides functions to interact with the local host file system.
- the **ui**: this module contains the implementation of the user interface, as we'll see in the next chapter, this is achieved through **activities**.
- the **activity_manager**: the activity manager takes care of managing activities, basically it runs the activities of the user interface, and chooses, based on their state, when is the moment to terminate the current activity and which activity to run after the current one.

#### Activities

Just a little paragraph about activities. Really, read the code and the documentation to have a clear idea of how the ui works.
I think there are many ways to implement a user interface; for termscp, I decided to go for a **Android-like** approach.
Android works with Activity, each activity represents a view and there is a context shared between them, and so it works here.

Just a little note about activities, activities work with a `Context`, the context is a data holder for different data, which are shared and common between the activities.

I've implemented a Trait called `Activity`, which, is a very very reduced version of the Android activity of course.
This trait provides only 3 methods:

- `on_create`: this method must initialize the activity; the context is passed to the activity, which will be the only owner of the Context, until the activity terminates.
- `on_draw`: this method must be called each time you want to perform an update of the user interface. This is basically the run method of the activity. This method also cares about handling input events. The developer shouldn't draw the interface on each call of this method (consider that this method might be called hundreds of times per second), but only when actually something has changed (for example after an input event has been raised).
- `on_destroy`: this method finalizes the activity and drops it; this method returns the Context to the caller (the activity manager).

---

### Implementing File Transfers

This chapter describes how to implement a file transfer in TermSCP. A file transfer is a module which implements the `FileTransfer` trait. The file transfer provides different modules to interact with a remote server, which in addition to the most obvious methods, used to download and upload files, provides also methods to list files, delete files, create directories etc.

In the following steps I will describe how to implement a new file transfer, in this case I will be implementing the SCP file transfer (which I'm actually implementing the moment I'm writing this lines).

1. Add the Scp protocol to the `FileTransferProtocol` enum.

    Move to `src/filetransfer/mod.rs` and add `Scp` to the `FileTransferProtocol` enum

    ```rs
    /// ## FileTransferProtocol
    ///
    /// This enum defines the different transfer protocol available in TermSCP
    #[derive(std::cmp::PartialEq, std::fmt::Debug, std::clone::Clone)]
    pub enum FileTransferProtocol {
        Sftp,
        Ftp(bool), // Bool is for secure (true => ftps)
        Scp, // <-- here
    }
    ```

    In this case Scp is a "plain" enum type. If you need particular options, follow the implementation of `Ftp` which uses a boolean flag for indicating if using FTPS or FTP.

2. Implement the FileTransfer struct

    Create a file at `src/filetransfer/mytransfer.rs`

    Declare your file transfer struct

    ```rs
    /// ## ScpFileTransfer
    ///
    /// SFTP file transfer structure
    pub struct ScpFileTransfer {
        session: Option<Session>,
        sftp: Option<Sftp>,
        wrkdir: PathBuf,
    }
    ```

3. Implement the `FileTransfer` trait for it

    You'll have to implement the following methods for your file transfer:

    - connect: connect to remote server
    - disconnect: disconnect from remote server
    - is_connected: returns whether the file transfer is connected to remote
    - pwd: get working directory
    - change_dir: change working directory.
    - list_dir: get files and directories at a certain path
    - mkdir: make a new directory. Return an error in case the directory already exists
    - remove: remove a file or a directory. In case the protocol doesn't support recursive removing of directories you MUST implement this through a recursive algorithm
    - rename: rename a file or a directory
    - stat: returns detail for a certain path
    - send_file: opens a stream to a remote path for write purposes (write a remote file)
    - recv_file: opens a stream to a remote path for read purposes (write a local file)
    - on_sent: finalize a stream when writing a remote file. In case it's not necessary just return `Ok(())`
    - on_recv: fianlize a stream when reading a remote file. In case it's not necessary just return `Ok(())`

    In case the protocol you're working on doesn't support any of this features, just return `Err(FileTransferError::new(FileTransferErrorType::UnsupportedFeature))`

4. Add your transfer to filetransfers:

    Move to `src/filetransfer/mod.rs` and declare your file transfer:

    ```rs
    // Transfers
    pub mod ftp_transfer;
    pub mod scp_transfer; // <-- here
    pub mod sftp_transfer;
    ```

5. Handle FileTransfer in `FileTransferActivity::new`

    Move to `src/ui/activities/filetransfer_activity/mod.rs` and add the new protocol to the client match

    ```rs
    client: match protocol {
        FileTransferProtocol::Sftp => Box::new(SftpFileTransfer::new()),
        FileTransferProtocol::Ftp(ftps) => Box::new(FtpFileTransfer::new(ftps)),
        FileTransferProtocol::Scp => Box::new(ScpFileTransfer::new()), // <--- here
    },
    ```

6. Handle right/left input events in `AuthActivity`:

    Move to `src/ui/activities/auth_activity.rs` and handle the new protocol in `handle_input_event_mode_text` for `KeyCode::Left` and `KeyCode::Right`.
    Consider that the order they "rotate" must match the way they will be drawned in the interface.
    For newer protocols, please put them always at the end of the list. In this list I won't, because Scp is more important than Ftp imo.

    ```rs
    KeyCode::Left => {
        // If current field is Protocol handle event... (move element left)
        if self.selected_field == InputField::Protocol {
            self.protocol = match self.protocol {
                FileTransferProtocol::Sftp => FileTransferProtocol::Ftp(true), // End of list (wrap)
                FileTransferProtocol::Scp => FileTransferProtocol::Sftp,
                FileTransferProtocol::Ftp(ftps) => match ftps {
                    false => FileTransferProtocol::Scp,
                    true => FileTransferProtocol::Ftp(false),
                }
            };
        }
    }
    KeyCode::Right => {
        // If current field is Protocol handle event... ( move element right )
        if self.selected_field == InputField::Protocol {
            self.protocol = match self.protocol {
                FileTransferProtocol::Sftp => FileTransferProtocol::Scp,
                FileTransferProtocol::Scp => FileTransferProtocol::Ftp(false),
                FileTransferProtocol::Ftp(ftps) => match ftps {
                    false => FileTransferProtocol::Ftp(true),
                    true => FileTransferProtocol::Sftp, // End of list (wrap)
                }
            };
        }
    }
    ```

7. Add your new file transfer to the protocol input field

    Move to `AuthActivity::draw_protocol_select` method.
    Here add your new protocol to the `Spans` vector and to the match case, which chooses which element to highlight.

    ```rs
    let protocols: Vec<Spans> = vec![Spans::from("SFTP"), Spans::from("SCP"), Spans::from("FTP"), Spans::from("FTPS")];
    let index: usize = match self.protocol {
        FileTransferProtocol::Sftp => 0,
        FileTransferProtocol::Scp => 1,
        FileTransferProtocol::Ftp(ftps) => match ftps {
            false => 2,
            true => 3,
        }
    };
    ```

---

Thank you for any contribution!  
Christian Visintin
