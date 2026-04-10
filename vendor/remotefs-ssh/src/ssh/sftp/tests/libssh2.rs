use std::io::{Cursor, sink};
use std::time::SystemTime;

use pretty_assertions::assert_eq;
use remotefs::fs::FileType;
use ssh2_config::ParseRule;

use super::*;
use crate::mock::ssh as ssh_mock;
use crate::ssh::container::OpensshServer;

#[test]
fn should_initialize_sftp_filesystem() {
    let mut client = SftpFs::libssh2(SshOpts::new("127.0.0.1"));
    assert!(client.session.is_none());
    assert!(client.sftp.is_none());
    assert_eq!(client.wrkdir, PathBuf::from("/"));
    assert_eq!(client.is_connected(), false);
}

#[test]
fn should_append_to_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert_eq!(
        client
            .create_file(p, &Metadata::default().size(10), Box::new(reader))
            .ok()
            .unwrap(),
        10
    );
    // Verify size
    assert_eq!(client.stat(p).ok().unwrap().metadata().size, 10);
    // Append to file
    let file_data = "Hello, world!\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert_eq!(
        client
            .append_file(p, &Metadata::default().size(14), Box::new(reader))
            .ok()
            .unwrap(),
        14
    );
    assert_eq!(client.stat(p).ok().unwrap().metadata().size, 24);
    finalize_client(client);
}

#[test]
fn should_not_append_to_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("/tmp/aaaaaaa/hbbbbb/a.txt");
    // Append to file
    let file_data = "Hello, world!\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .append_file(p, &Metadata::default(), Box::new(reader))
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_change_directory() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    let pwd = client.pwd().ok().unwrap();
    assert!(client.change_dir(Path::new("/tmp")).is_ok());
    assert!(client.change_dir(pwd.as_path()).is_ok());
    finalize_client(client);
}

#[test]
fn should_not_change_directory() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    assert!(
        client
            .change_dir(Path::new("/tmp/sdfghjuireghiuergh/useghiyuwegh"))
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_copy_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    assert!(client.copy(p, Path::new("b.txt")).is_ok());
    assert!(client.stat(p).is_ok());
    assert!(client.stat(Path::new("b.txt")).is_ok());
    finalize_client(client);
}

#[test]
fn should_not_copy_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    assert!(client.copy(p, Path::new("aaa/bbbb/ccc/b.txt")).is_err());
    finalize_client(client);
}

#[test]
fn should_create_directory() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // create directory
    assert!(
        client
            .create_dir(Path::new("mydir"), UnixPex::from(0o755))
            .is_ok()
    );
    finalize_client(client);
}

#[test]
fn should_not_create_directory_cause_already_exists() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // create directory
    assert!(
        client
            .create_dir(Path::new("mydir"), UnixPex::from(0o755))
            .is_ok()
    );
    assert_eq!(
        client
            .create_dir(Path::new("mydir"), UnixPex::from(0o755))
            .err()
            .unwrap()
            .kind,
        RemoteErrorType::DirectoryAlreadyExists
    );
    finalize_client(client);
}

#[test]
fn should_not_create_directory() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // create directory
    assert!(
        client
            .create_dir(
                Path::new("/tmp/werfgjwerughjwurih/iwerjghiwgui"),
                UnixPex::from(0o755)
            )
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_create_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert_eq!(
        client
            .create_file(p, &Metadata::default().size(10), Box::new(reader))
            .ok()
            .unwrap(),
        10
    );
    // Verify size
    assert_eq!(client.stat(p).ok().unwrap().metadata().size, 10);
    finalize_client(client);
}

#[test]
fn should_create_big_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = vec![1; 2 * 1024 * 1024]; // 2MB
    let mut metadata = Metadata::default();
    metadata.size = file_data.len() as u64;
    let reader = Cursor::new(file_data);
    assert_eq!(
        client
            .create_file(p, &metadata, Box::new(reader))
            .ok()
            .unwrap(),
        2 * 1024 * 1024
    );
    // Verify size
    assert_eq!(
        client.stat(p).ok().unwrap().metadata().size,
        2 * 1024 * 1024
    );
    finalize_client(client);
}

#[test]
fn should_read_big_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = vec![1; 2 * 1024 * 1024]; // 2MB
    let mut metadata = Metadata::default();
    metadata.size = file_data.len() as u64;
    let reader = Cursor::new(file_data);
    assert_eq!(
        client
            .create_file(p, &metadata, Box::new(reader))
            .ok()
            .unwrap(),
        2 * 1024 * 1024
    );
    // Verify size
    assert_eq!(
        client.stat(p).ok().unwrap().metadata().size,
        2 * 1024 * 1024
    );

    // read file
    let dest = sink();
    assert_eq!(
        client
            .open_file(p, Box::new(dest))
            .expect("Cannot read file"),
        2 * 1024 * 1024
    );

    finalize_client(client);
}

#[test]
fn should_not_create_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("/tmp/ahsufhauiefhuiashf/hfhfhfhf");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_exec_command() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    assert_eq!(
        client.exec("echo 5").ok().unwrap(),
        (0, String::from("5\n"))
    );
    finalize_client(client);
}

#[test]
fn should_tell_whether_file_exists() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    // Verify size
    assert_eq!(client.exists(p).ok().unwrap(), true);
    assert_eq!(client.exists(Path::new("b.txt")).ok().unwrap(), false);
    assert_eq!(
        client.exists(Path::new("/tmp/ppppp/bhhrhu")).ok().unwrap(),
        false
    );
    assert_eq!(client.exists(Path::new("/tmp")).ok().unwrap(), true);
    finalize_client(client);
}

#[test]
fn should_list_dir() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let wrkdir = client.pwd().ok().unwrap();
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default().size(10), Box::new(reader))
            .is_ok()
    );
    // Verify size
    let file = client
        .list_dir(wrkdir.as_path())
        .ok()
        .unwrap()
        .first()
        .unwrap()
        .clone();
    assert_eq!(file.name().as_str(), "a.txt");
    let mut expected_path = wrkdir;
    expected_path.push(p);
    assert_eq!(file.path.as_path(), expected_path.as_path());
    assert_eq!(file.extension().as_deref().unwrap(), "txt");
    assert_eq!(file.metadata.size, 10);
    assert_eq!(file.metadata.mode.unwrap(), UnixPex::from(0o644));
    finalize_client(client);
}

#[test]
fn should_not_list_dir() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    assert!(client.list_dir(Path::new("/tmp/auhhfh/hfhjfhf/")).is_err());
    finalize_client(client);
}

#[test]
fn should_move_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    // Verify size
    let dest = Path::new("b.txt");
    assert!(client.mov(p, dest).is_ok());
    assert_eq!(client.exists(p).ok().unwrap(), false);
    assert_eq!(client.exists(dest).ok().unwrap(), true);
    finalize_client(client);
}

#[test]
fn should_not_move_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    // Verify size
    let dest = Path::new("/tmp/wuefhiwuerfh/whjhh/b.txt");
    assert!(client.mov(p, dest).is_err());
    assert!(
        client
            .mov(Path::new("/tmp/wuefhiwuerfh/whjhh/b.txt"), p)
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_open_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default().size(10), Box::new(reader))
            .is_ok()
    );
    // Verify size
    let buffer: Box<dyn std::io::Write + Send> = Box::new(Vec::with_capacity(512));
    assert_eq!(client.open_file(p, buffer).ok().unwrap(), 10);
    finalize_client(client);
}

#[test]
fn should_not_open_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Verify size
    let buffer: Box<dyn std::io::Write + Send> = Box::new(Vec::with_capacity(512));
    assert!(
        client
            .open_file(Path::new("/tmp/aashafb/hhh"), buffer)
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_print_working_directory() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    assert!(client.pwd().is_ok());
    finalize_client(client);
}

#[test]
fn should_remove_dir_all() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create dir
    let mut dir_path = client.pwd().ok().unwrap();
    dir_path.push(Path::new("test/"));
    assert!(
        client
            .create_dir(dir_path.as_path(), UnixPex::from(0o775))
            .is_ok()
    );
    // Create file
    let mut file_path = dir_path.clone();
    file_path.push(Path::new("a.txt"));
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(file_path.as_path(), &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    // Remove dir
    assert!(client.remove_dir_all(dir_path.as_path()).is_ok());
    finalize_client(client);
}

#[test]
fn should_not_remove_dir_all() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Remove dir
    assert!(
        client
            .remove_dir_all(Path::new("/tmp/aaaaaa/asuhi"))
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_remove_dir() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create dir
    let mut dir_path = client.pwd().ok().unwrap();
    dir_path.push(Path::new("test/"));
    assert!(
        client
            .create_dir(dir_path.as_path(), UnixPex::from(0o775))
            .is_ok()
    );
    assert!(client.remove_dir(dir_path.as_path()).is_ok());
    finalize_client(client);
}

#[test]
fn should_not_remove_dir() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create dir
    let mut dir_path = client.pwd().ok().unwrap();
    dir_path.push(Path::new("test/"));
    assert!(
        client
            .create_dir(dir_path.as_path(), UnixPex::from(0o775))
            .is_ok()
    );
    // Create file
    let mut file_path = dir_path.clone();
    file_path.push(Path::new("a.txt"));
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(file_path.as_path(), &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    // Remove dir
    assert!(client.remove_dir(dir_path.as_path()).is_err());
    finalize_client(client);
}

#[test]
fn should_remove_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.txt");
    let file_data = "test data\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    assert!(client.remove_file(p).is_ok());
    finalize_client(client);
}

#[test]
fn should_setstat_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.sh");
    let file_data = "echo 5\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );

    assert!(
        client
            .setstat(
                p,
                Metadata {
                    accessed: Some(SystemTime::UNIX_EPOCH),
                    created: None,
                    file_type: FileType::File,
                    gid: Some(1000),
                    mode: Some(UnixPex::from(0o755)),
                    modified: Some(SystemTime::UNIX_EPOCH),
                    size: 7,
                    symlink: None,
                    uid: Some(1000),
                }
            )
            .is_ok()
    );
    let entry = client.stat(p).ok().unwrap();
    let stat = entry.metadata();
    assert_eq!(stat.accessed, Some(SystemTime::UNIX_EPOCH));
    assert_eq!(stat.created, None);
    assert_eq!(stat.gid.unwrap(), 1000);
    assert_eq!(stat.modified, Some(SystemTime::UNIX_EPOCH));
    assert_eq!(stat.mode.unwrap(), UnixPex::from(0o755));
    assert_eq!(stat.size, 7);
    assert_eq!(stat.uid.unwrap(), 1000);

    finalize_client(client);
}

#[test]
fn should_not_setstat_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("bbbbb/cccc/a.sh");
    assert!(
        client
            .setstat(
                p,
                Metadata {
                    accessed: None,
                    created: None,
                    file_type: FileType::File,
                    gid: Some(1),
                    mode: Some(UnixPex::from(0o755)),
                    modified: None,
                    size: 7,
                    symlink: None,
                    uid: Some(1),
                }
            )
            .is_err()
    );
    finalize_client(client);
}

#[test]
fn should_stat_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.sh");
    let file_data = "echo 5\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default().size(7), Box::new(reader))
            .is_ok()
    );
    let entry = client.stat(p).ok().unwrap();
    assert_eq!(entry.name(), "a.sh");
    let mut expected_path = client.pwd().ok().unwrap();
    expected_path.push("a.sh");
    assert_eq!(entry.path(), expected_path.as_path());
    let meta = entry.metadata();
    assert_eq!(meta.mode.unwrap(), UnixPex::from(0o644));
    assert_eq!(meta.size, 7);
    finalize_client(client);
}

#[test]
fn should_not_stat_file() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.sh");
    assert!(client.stat(p).is_err());
    finalize_client(client);
}

#[test]
fn should_make_symlink() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.sh");
    let file_data = "echo 5\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    let symlink = Path::new("b.sh");
    assert!(client.symlink(symlink, p).is_ok());
    assert!(client.remove_file(symlink).is_ok());
    finalize_client(client);
}

#[test]
fn should_not_make_symlink() {
    crate::mock::logger();
    let TestCtx {
        mut client,
        container: _container,
    } = setup_client();
    // Create file
    let p = Path::new("a.sh");
    let file_data = "echo 5\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(p, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    let symlink = Path::new("b.sh");
    let file_data = "echo 5\n";
    let reader = Cursor::new(file_data.as_bytes());
    assert!(
        client
            .create_file(symlink, &Metadata::default(), Box::new(reader))
            .is_ok()
    );
    assert!(client.symlink(symlink, p).is_err());
    assert!(client.remove_file(symlink).is_ok());
    assert!(client.symlink(symlink, Path::new("c.sh")).is_err());
    finalize_client(client);
}

#[test]
fn should_return_not_connected_error() {
    crate::mock::logger();
    let mut client = SftpFs::libssh2(SshOpts::new("127.0.0.1"));
    assert!(client.change_dir(Path::new("/tmp")).is_err());
    assert!(
        client
            .copy(Path::new("/nowhere"), PathBuf::from("/culonia").as_path())
            .is_err()
    );
    assert!(client.exec("echo 5").is_err());
    assert!(client.disconnect().is_err());
    assert!(client.symlink(Path::new("/a"), Path::new("/b")).is_err());
    assert!(client.list_dir(Path::new("/tmp")).is_err());
    assert!(
        client
            .create_dir(Path::new("/tmp"), UnixPex::from(0o755))
            .is_err()
    );
    assert!(client.pwd().is_err());
    assert!(client.remove_dir_all(Path::new("/nowhere")).is_err());
    assert!(
        client
            .mov(Path::new("/nowhere"), Path::new("/culonia"))
            .is_err()
    );
    assert!(client.stat(Path::new("/tmp")).is_err());
    assert!(
        client
            .setstat(Path::new("/tmp"), Metadata::default())
            .is_err()
    );
    assert!(client.open(Path::new("/tmp/pippo.txt")).is_err());
    assert!(
        client
            .create(Path::new("/tmp/pippo.txt"), &Metadata::default())
            .is_err()
    );
    assert!(
        client
            .append(Path::new("/tmp/pippo.txt"), &Metadata::default())
            .is_err()
    );
}

fn is_send<T: Send>(_send: T) {}

fn is_sync<T: Sync>(_sync: T) {}

#[test]
fn test_should_be_sync() {
    let client = SftpFs::libssh2(
        SshOpts::new("sftp").key_storage(Box::new(ssh_mock::MockSshKeyStorage::default())),
    );

    is_sync(client);
}

#[test]
fn test_should_be_send() {
    let client = SftpFs::libssh2(
        SshOpts::new("sftp").key_storage(Box::new(ssh_mock::MockSshKeyStorage::default())),
    );
    is_send(client);
}

// -- test utils

struct TestCtx {
    client: SftpFs<super::LibSsh2Session>,
    #[allow(dead_code)]
    container: OpensshServer,
}

fn setup_client() -> TestCtx {
    let container = OpensshServer::start();
    let port = container.port();

    use crate::SshAgentIdentity;

    let config_file = ssh_mock::create_ssh_config(port);
    let mut client = SftpFs::libssh2(
        SshOpts::new("sftp")
            .key_storage(Box::new(ssh_mock::MockSshKeyStorage::default()))
            .config_file(config_file.path(), ParseRule::ALLOW_UNKNOWN_FIELDS)
            .ssh_agent_identity(Some(SshAgentIdentity::All)),
    );
    assert!(client.connect().is_ok());
    // Create wrkdir
    let tempdir = PathBuf::from(generate_tempdir());
    assert!(
        client
            .create_dir(tempdir.as_path(), UnixPex::from(0o775))
            .is_ok()
    );
    // Change directory
    assert!(client.change_dir(tempdir.as_path()).is_ok());

    TestCtx { client, container }
}

fn finalize_client(mut client: SftpFs<super::LibSsh2Session>) {
    // Get working directory
    let wrkdir = client.pwd().ok().unwrap();
    // Remove directory
    assert!(client.remove_dir_all(wrkdir.as_path()).is_ok());
    assert!(client.disconnect().is_ok());
}

fn generate_tempdir() -> String {
    use rand::distr::Alphanumeric;
    use rand::{Rng, rng};
    let mut rng = rng();
    let name: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(8)
        .collect();
    format!("/tmp/temp_{name}")
}
