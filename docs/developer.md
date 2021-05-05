# Developer Manual

Document audience: developers

- [Developer Manual](#developer-manual)
  - [How termscp works](#how-termscp-works)
  - [Activities](#activities)
    - [The Context](#the-context)
  - [Tests fails due to receivers](#tests-fails-due-to-receivers)
  - [Implementing File Transfers](#implementing-file-transfers)

Welcome to the developer manual for termscp. This chapter DOESN'T contain the documentation for termscp modules, which can instead be found on Rust Docs at <https://docs.rs/termscp>
This chapter describes how termscp works and the guide lines to implement stuff such as file transfers and add features to the user interface.

## How termscp works

termscp is basically made up of 4 components:

- the **filetransfer**: the filetransfer takes care of managing the remote file system; it provides function to establish a connection with the remote, operating on the remote server file system (e.g. remove files, make directories, rename files, ...), read files and write files. The FileTransfer, as we'll see later, is actually a trait, and for each protocol a FileTransfer must be implement the trait.
- the **host**: the host module provides functions to interact with the local host file system.
- the **ui**: this module contains the implementation of the user interface, as we'll see in the next chapter, this is achieved through **activities**.
- the **activity_manager**: the activity manager takes care of managing activities, basically it runs the activities of the user interface, and chooses, based on their state, when is the moment to terminate the current activity and which activity to run after the current one.

In addition to the 4 main components, other have been added through the time:

- **config**: this module provides the configuration schema and serialization methods for it.
- **fs**: this modules exposes the FsEntry entity and the explorers. The explorers are structs which hold the content of the current directory; they also they take of filtering files up to your preferences and format file entries based on your configuration.
- **system**: the system module provides a way to actually interact with the configuration, the ssh key storage and with the bookmarks.
- **utils**: contains the utilities used by pretty much all the project.

## Activities

Just a little paragraph about activities. Really, read the code and the documentation to have a clear idea of how the ui works.
I think there are many ways to implement a user interface and I've worked with different languages and frameworks in my career, so for this project I've decided to get what I like the most from different frameworks to implement it.

My approach was this:

- **Activities on top**: each "page" is an Activity and an `Activity Manager` handles them. I got inspired by Android for this case. I think that's a good way to implement the ui in case like this, where you have different pages, each one with their view, their components and their logics. Activities work with the `Context`, which is a data holder for different data, which are shared and common between the activities.
- **Activities display Views**: Each activity can show different views. A view is basically a list of **components**, each one with its properties. The view is a facade to the components and also handles the focus, which is the current active component. You cannot have more than one component active, so you need to handle this; but at the same time you also have to give focus to the previously active component if the current one is destroyed. So basically view takes care of all this stuff.
- **Components**: I've decided to write around `tui` in order to re-use widgets. To do so I've implemented the `Component` trait. To implement traits I got inspired by [React](https://reactjs.org/). Each component has its *Properties* and can have its *States*. Then each component must be able to handle input events and to be updated with new properties. Last but not least, each component must provide a method to **render** itself.
- **Messages: an Elm based approach**: I was really satisfied with my implementation choices; the problem at this point was solving one of the biggest teardrops I've ever had with this project: **events**. Input events were really a pain to handle, since I had to handle states in the activity to handle which component was enabled etc. To solve this I got inspired by a wonderful language I had recently studied, which is [Elm](https://elm-lang.org/). Basically in Elm you implement your ui using three basic functions: **update**, **view** and **init**. View and init were pretty much already implemented here, but at this point I decided to implement also something like the **elm update function**. I came out with a huge match case to handle events inside a recursive function, which you can basically find in the `update.rs` file inside each activity. This match case handles a tuple, made out of the **component id** and the **input event** received from the view. It matches the two propeties against the input event we want to handle for each component *et voilà*.

I've implemented a Trait called `Activity`, which, is a very very reduced version of the Android activity of course.
This trait provides only 3 methods:

- `on_create`: this method must initialize the activity; the context is passed to the activity, which will be the only owner of the Context, until the activity terminates.
- `on_draw`: this method must be called each time you want to perform an update of the user interface. This is basically the run method of the activity. This method also cares about handling input events. The developer shouldn't draw the interface on each call of this method (consider that this method might be called hundreds of times per second), but only when actually something has changed (for example after an input event has been raised).
- `will_umount`: this method was added in 0.4.0 and returns whethere the activity should be destroyed. If so returns an ExitReason, which indicates why the activity should be terminated. Based on the reason, the activity manager chooses whether to stop the execution of termscp or to start a new activity and which one.
- `on_destroy`: this method finalizes the activity and drops it; this method returns the Context to the caller (the activity manager).

### The Context

The context is a structure which holds data which must be shared between activities. Everytime an Activity starts, the Context is taken by the activity, until it is destroyed, where finally the context is returned to the activity manager.
The context basically holds the following data:

- The **Localhost**: the local host structure
- The **File Transfer Params**: the current parameters set to connect to the remote
- The **Config Client**: the configuration client is a structure which provides functions to access the user configuration
- The **Store**: the store is a key-value storage which can hold any kind of data. This can be used to store states to share between activities or to keep persistence for heavy/slow tasks (such as checking for updates).
- The **Input handler**: the input handler is used to read input events from the keyboard
- The **Terminal**: the terminal is used to view the tui on the terminal

---

## Tests fails due to receivers

Yes. This happens quite often and is related to the fact that I'm using public SSH/SFTP/FTP server to test file receivers and sometimes this server go down for even a day or more. If your tests don't pass due to this, don't worry, submit the pull request and I'll take care of testing them by myself.

---

## Implementing File Transfers

This chapter describes how to implement a file transfer in termscp. A file transfer is a module which implements the `FileTransfer` trait. The file transfer provides different modules to interact with a remote server, which in addition to the most obvious methods, used to download and upload files, provides also methods to list files, delete files, create directories etc.

In the following steps I will describe how to implement a new file transfer, in this case I will be implementing the SCP file transfer (which I'm actually implementing the moment I'm writing this lines).

1. Add the Scp protocol to the `FileTransferProtocol` enum.

    Move to `src/filetransfer/mod.rs` and add `Scp` to the `FileTransferProtocol` enum

    ```rs
    /// ## FileTransferProtocol
    ///
    /// This enum defines the different transfer protocol available in termscp
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
