# Developer manual

Welcome to the developer manual for termscp. This chapter does NOT contain the
documentation for termscp modules, which can instead be found on Rust Docs at
<https://docs.rs/termscp>. This chapter describes how termscp works and the
guidelines to implement features such as file transfers and additions to the
user interface.

termscp is written in Rust (edition 2024, MSRV 1.89.0). The user interface is
built with [tuirealm](https://github.com/veeso/tui-realm) v3, which runs on top
of [crossterm](https://github.com/crossterm-rs/crossterm).

## How termscp works

termscp is basically made up of 3 core modules:

- The **host**: the host module provides functions to interact with file
  systems. It exposes the `HostBridge` trait, which abstracts file operations
  over both the local host (`Localhost`) and the remote host (`RemoteBridged`).
- The **ui**: this module contains the implementation of the user interface. As
  shown in the next chapter, this is achieved through **activities**.
- The **activity_manager**: the activity manager takes care of managing
  activities. It runs the activities of the user interface and chooses, based on
  their state, when to terminate the current activity and which activity to run
  next.

In addition to the 3 core modules, others have been added over time:

- **config**: provides the configuration schema and its serialization methods.
- **explorer**: exposes the explorer structures, which are used to handle the
  file explorer in the ui. They store the current directory model and the view
  states (e.g. sorting, whether to display hidden files, the transfer queue).
- **filetransfer**: defines the `FileTransferProtocol` enum and the
  `RemoteFsBuilder`, which constructs the appropriate `RemoteFs` client from the
  connection parameters.
- **system**: provides a way to interact with the configuration, the ssh key
  storage and the bookmarks.
- **utils**: contains the utilities used by pretty much all of the project.

termscp supports the following protocols: SFTP, SCP, FTP/FTPS, Kube, S3, SMB and
WebDAV.

## Activities

This paragraph gives a short overview of activities. Read the code and the
documentation for a clear idea of how the ui works.

There are many ways to implement a user interface. This project borrows what
works best from different frameworks:

- **Activities on top**: each "view" is an Activity, and an `Activity Manager`
  handles them. This approach is inspired by Android. It fits a ui that has
  different views, each one with its own components and logic. Activities work
  with the `Context`, which is a data holder used to share data between
  activities.
- **Activities display Applications**: each activity can show different
  **Applications**. An application contains a **View**, which is basically a list
  of **components**, each one with its properties. The view is a facade to the
  components and also handles the focus, which is the current active component.
  You cannot have more than one active component, so this must be handled; at the
  same time, focus must be given back to the previously active component if the
  current one is destroyed. The **Application** takes care of all this. To learn
  more, read <https://github.com/veeso/tui-realm>.
- **Components**: components are built around tui in order to reuse widgets. This
  is achieved through the `Component` trait, inspired by
  [React](https://reactjs.org/). Each component has its *Properties* and can have
  its *States*. Each component must handle input events, accept new properties,
  and provide a method to **render** itself. This logic now lives in
  [tui-realm](https://github.com/veeso/tui-realm).
- **Messages: an Elm-based approach**: input events are handled with an approach
  inspired by [Elm](https://elm-lang.org/). In Elm you implement your ui using
  three basic functions: **update**, **view** and **init**. termscp implements
  the equivalent of the Elm update function as a large match case inside a
  recursive function, which you can find in the `update.rs` file inside each
  activity. This match case handles the messages produced by the components in
  response to incoming input events and causes the activity to change its state.

termscp implements a trait called `Activity`, a much reduced version of the
Android activity. This trait provides these methods:

- `on_create`: initializes the activity. The context is passed to the activity,
  which becomes the only owner of the Context until the activity terminates.
- `on_draw`: called each time the user interface should be updated. This is
  basically the run method of the activity, and it also handles input events. The
  interface should not be drawn on every call (this method may be called hundreds
  of times per second), but only when something has actually changed (for example
  after an input event).
- `will_umount`: returns whether the activity should be destroyed. If so, it
  returns an `ExitReason`, which indicates why the activity should terminate.
  Based on the reason, the activity manager chooses whether to stop the execution
  of termscp or to start a new activity, and which one.
- `on_destroy`: finalizes the activity and drops it. This method returns the
  Context to the caller (the activity manager).

### The Context

The context is a structure that holds data shared between activities. Every time
an Activity starts, the Context is taken by the activity, until it is destroyed,
where the context is finally returned to the activity manager. The context holds
the following data:

- The **Localhost**: the local host structure.
- The **File Transfer Params**: the current parameters used to connect to the
  remote.
- The **Config Client**: a structure that provides functions to access the user
  configuration.
- The **Store**: a key-value storage that can hold any kind of data. It can be
  used to share state between activities or to keep persistence for heavy or slow
  tasks (such as checking for updates).
- The **Terminal**: used to render the tui on the terminal.

## Achieving an abstract file transfer client

When the implementation of termscp started, in December 2020, file transfer was
at the core of the design, since it is at the heart of termscp. The first
implementation consisted of a `filetransfer` module that exposed a trait called
`FileTransfer`, which provided methods to generically interact with the remote
file system.

This changed over time, as different users asked for a dedicated library. In the
last quarter of 2021, [remotefs](https://github.com/veeso/remotefs-rs) was born:
an abstract library to work with remote device file systems. remotefs provides a
`RemoteFs` trait that exposes all of the core file-system functionalities, and
since version 0.8.0 it has replaced the `FileTransfer` trait.

The file transfer module still exists, but its only task is to build a
`RemoteFs` client implementation from the file transfer parameters through the
`RemoteFsBuilder`.
