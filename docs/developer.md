# Developer Manual

Document audience: developers
Revision: 2022-01-05

- [Developer Manual](#developer-manual)
  - [How termscp works](#how-termscp-works)
  - [Activities](#activities)
    - [The Context](#the-context)
  - [Achieving an abstract file transfer client](#achieving-an-abstract-file-transfer-client)

Welcome to the developer manual for termscp. This chapter DOESN'T contain the documentation for termscp modules, which can instead be found on Rust Docs at <https://docs.rs/termscp>
This chapter describes how termscp works and the guide lines to implement stuff such as file transfers and add features to the user interface.

---

## How termscp works

termscp is basically made up of 3 core modules:

- the **host**: the host module provides functions to interact with the local host file system.
- the **ui**: this module contains the implementation of the user interface, as we'll see in the next chapter, this is achieved through **activities**.
- the **activity_manager**: the activity manager takes care of managing activities, basically it runs the activities of the user interface, and chooses, based on their state, when is the moment to terminate the current activity and which activity to run after the current one.

In addition to the 3 core modules, other have been added through the time:

- **config**: this module provides the configuration schema and serialization methods for it.
- **explorer**: this modules exposes the explorer structures, which are used to handle the file explorer in the ui. So, basically they store the current directory model and the view states (e.g. sorting, whether to display hidden files, ...).
- **system**: the system module provides a way to interact with the configuration, with the ssh key storage and with the bookmarks.
- **utils**: contains the utilities used by pretty much all of the project.

## Activities

Just a little paragraph about activities. Really, read the code and the documentation to have a clear idea of how the ui works.
I think there are many ways to implement a user interface and I've worked with different languages and frameworks in my career, so for this project I've decided to get what I like the most from different frameworks to implement it.

My approach was this:

- **Activities on top**: each "view" is an Activity and an `Activity Manager` handles them. I got inspired by Android for this case. I think that's a good way to implement the ui in case like this, where you have different views, each one with their view, their components and their logic. Activities work with the `Context`, which is a data holder to share data between the activities.
- **Activities display Applications**: Each activity can show different **Applications**. An application, contains a **View** is basically a list of **components**, each one with its properties. The view is a facade to the components and also handles the focus, which is the current active component. You cannot have more than one component active, so you need to handle this; but at the same time you also have to give focus to the previously active component if the current one is destroyed. So basically **Application** takes care of all this stuff. If you're interested on how this works, you can read more on <https://github.com/veeso/tui-realm>.
- **Components**: I've decided to write around `tui` in order to re-use widgets. To do so I've implemented the `Component` trait. To implement traits I got inspired by [React](https://reactjs.org/). Each component has its *Properties* and can have its *States*. Then each component must be able to handle input events and to be updated with new properties. Last but not least, each component must provide a method to **render** itself. At the beginning this was implemented inside of termscp, but now this has been moved to [tui-realm](https://github.com/veeso/tui-realm).
- **Messages: an Elm based approach**: I was really satisfied with my implementation choices; the problem at this point was solving one of the biggest teardrops I've ever had with this project: **events**. Input events were really a pain to handle, since I had to handle states in the activity to handle which component was enabled etc. To solve this I got inspired by a wonderful language I had recently studied, which is [Elm](https://elm-lang.org/). Basically in Elm you implement your ui using three basic functions: **update**, **view** and **init**. View and init were pretty much already implemented here, but at this point I decided to implement also something like the **elm update function**. I came out with a huge match case to handle messages inside a recursive function, which you can basically find in the `update.rs` file inside each activity. This match case handles the messages produced by the components in front of an incoming input event. It matches the messages causing the activity to change its states *et voilà*.

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
- The **Terminal**: the terminal is used to view the tui on the terminal

---

## Achieving an abstract file transfer client

When I started to implement termscp, in december 2020, the file transfer was at the core of my implementation focus, since, for obvious reasons, it is at the heart of termscp.
The first implementation consisted of a `filetransfer` module, which exposed a trait called `FileTransfer`, which exposed different methods to generically interact with the remote file system.
This thing has changed over the last year, since different users has asked me to implement a dedicated library to implement this.
So in the last quarter of 2021, I dedicated part of my time in implementing an abstract library to work with remote device file systems, and this is how [remotefs](https://github.com/veeso/remotefs-rs) was born.
Remotefs provides a `RemoteFs` trait which exposes all of the core file-system functionalities and this has since 0.8.0 version, replaced the `FileTransfer` trait.

The file transfer module, still exists though, but its only task is to create a builder from the "file transfer parameters" into the `RemoteFs` client implementation.
