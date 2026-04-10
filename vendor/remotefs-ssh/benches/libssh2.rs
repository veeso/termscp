use std::borrow::Cow;
use std::io::{Sink, Write as _};
use std::path::PathBuf;
use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use remotefs::RemoteFs as _;
use remotefs::fs::{Metadata, UnixPex};
use remotefs_ssh::{LibSsh2Session, ScpFs, SftpFs, SshAgentIdentity, SshKeyStorage, SshOpts};
use ssh2_config::ParseRule;
use tempfile::NamedTempFile;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::{Container, Image};

const P: &str = "/tmp/large_file";
const WRITE_SIZE: u64 = 2 * 1024 * 1024; // 2MB

fn benchmark_scp_read(c: &mut Criterion) {
    c.bench_function("scp_read", |b| {
        b.iter_batched(
            || BenchmarkCtx::new(),
            |mut ctx| {
                let reader = Sink::default();

                let sz = ctx
                    .scp
                    .open_file(&PathBuf::from(P), Box::new(reader))
                    .expect("Failed to open file for reading");
                assert_eq!(sz, WRITE_SIZE, "File size mismatch");
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

/*
fn benchmark_scp_write(c: &mut Criterion) {
    c.bench_function("scp_write", |b| {
        b.iter_batched(
            || BenchmarkCtx::new(),
            |mut ctx| {
                let reader = repeat(0x01);

                let sz = ctx
                    .scp
                    .create_file(
                        &PathBuf::from(P),
                        &Metadata::default().size(WRITE_SIZE),
                        Box::new(reader),
                    )
                    .expect("Failed to create file for writing");
                assert_eq!(sz, WRITE_SIZE, "File size mismatch");
            },
            criterion::BatchSize::SmallInput,
        );
    });
}
*/

fn benchmark_sftp_read(c: &mut Criterion) {
    c.bench_function("sftp_read", |b| {
        b.iter_batched(
            || BenchmarkCtx::new(),
            |mut ctx| {
                let reader = Sink::default();

                let sz = ctx
                    .sftp
                    .open_file(&PathBuf::from(P), Box::new(reader))
                    .expect("Failed to open file for reading");
                assert_eq!(sz, WRITE_SIZE, "File size mismatch");
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

/*
fn benchmark_sftp_write(c: &mut Criterion) {
    c.bench_function("sftp_write", |b| {
        b.iter_batched(
            || BenchmarkCtx::new(),
            |mut ctx| {
                let reader = repeat(0x01);

                let sz = ctx
                    .sftp
                    .create_file(
                        &PathBuf::from(P),
                        &Metadata::default().size(WRITE_SIZE),
                        Box::new(reader),
                    )
                    .expect("Failed to create file for writing");
                assert_eq!(sz, WRITE_SIZE, "File size mismatch");
            },
            criterion::BatchSize::SmallInput,
        );
    });
}
*/

struct BenchmarkCtx {
    _container: OpensshServer,
    scp: ScpFs<LibSsh2Session>,
    sftp: SftpFs<LibSsh2Session>,
}

impl BenchmarkCtx {
    pub fn new() -> Self {
        let container = OpensshServer::start();
        let port = container.port();

        let config_file = create_ssh_config(port);
        let scp_client = {
            let mut client = ScpFs::libssh2(
                SshOpts::new("scp")
                    .key_storage(Box::new(MockSshKeyStorage::default()))
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

            // write a file to transfer of 2GB.
            let file_to_transfer = PathBuf::from(P);
            // open file
            let mut writer = client
                .create(&file_to_transfer, &Metadata::default().size(WRITE_SIZE))
                .unwrap();
            let mut written = 0;
            let buf = [0; 1024 * 1024];
            loop {
                let to_write = buf.len().min(WRITE_SIZE as usize - written);
                if to_write == 0 {
                    break;
                }
                writer.write_all(&buf[..to_write]).unwrap();
                written += to_write;
            }

            client
        };
        let sftp_client = {
            let mut client = SftpFs::libssh2(
                SshOpts::new("sftp")
                    .key_storage(Box::new(MockSshKeyStorage::default()))
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

            // write a file to transfer of 2GB.
            let file_to_transfer = PathBuf::from(P);
            // open file
            let mut writer = client
                .create(&file_to_transfer, &Metadata::default().size(WRITE_SIZE))
                .unwrap();
            let mut written = 0;
            let buf = [0; 1024 * 1024];
            loop {
                let to_write = buf.len().min(WRITE_SIZE as usize - written);
                if to_write == 0 {
                    break;
                }
                writer.write_all(&buf[..to_write]).unwrap();
                written += to_write;
            }

            client
        };

        Self {
            _container: container,
            scp: scp_client,
            sftp: sftp_client,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct OpensshServerImage;

impl Image for OpensshServerImage {
    fn name(&self) -> &str {
        "ghcr.io/linuxserver/openssh-server"
    }

    fn tag(&self) -> &str {
        "8.6_p1-r3-ls70"
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("done.")]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[ContainerPort::Tcp(2222)]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        vec![
            ("PUID", "1000"),
            ("PGID", "1000"),
            ("TZ", "Europe/London"),
            ("SUDO_ACCESS", "false"),
            ("PASSWORD_ACCESS", "true"),
            (
                "PUBLIC_KEY",
                "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDErJhQxEI0+VvhlXVUyh+vMCm7aXfCA/g633AG8ezD/5EylwchtAr2JCoBWnxn4zV8nI9dMqOgm0jO4IsXpKOjQojv+0VOH7I+cDlBg0tk4hFlvyyS6YviDAfDDln3jYUM+5QNDfQLaZlH2WvcJ3mkDxLVlI9MBX1BAeSmChLxwAvxALp2ncImNQLzDO9eHcig3dtMrEKkzXQowRW5Y7eUzg2+vvVq4H2DOjWwUndvB5sJkhEfTUVE7ID8ZdGJo60kUb/02dZYj+IbkAnMCsqktk0cg/4XFX82hEfRYFeb1arkysFisPU1DOb6QielL/axeTebVplaouYcXY0pFdJt root@8c50fd4c345a",
            ),
            ("USER_PASSWORD", "password"),
            ("USER_NAME", "sftp"),
        ]
    }
}

pub struct OpensshServer {
    container: Container<OpensshServerImage>,
}

impl OpensshServer {
    pub fn start() -> Self {
        use testcontainers::runners::SyncRunner;
        let container = OpensshServerImage
            .start()
            .expect("Failed to start container");

        Self { container }
    }

    pub fn port(&self) -> u16 {
        std::thread::sleep(Duration::from_secs(5));
        self.container
            .get_host_port_ipv6(2222)
            .expect("Failed to get port")
    }
}

/// Create ssh config file
pub fn create_ssh_config(port: u16) -> NamedTempFile {
    let mut temp = NamedTempFile::new().expect("Failed to create tempfile");
    let config = format!(
        r##"
# ssh config
Compression yes
ConnectionAttempts  3
ConnectTimeout      60
Ciphers             aes128-ctr,aes192-ctr,aes256-ctr
KexAlgorithms       diffie-hellman-group-exchange-sha256
MACs                hmac-sha2-512,hmac-sha2-256,hmac-ripemd160
# Hosts
Host sftp
    HostName    127.0.0.1
    Port        {port}
    User        sftp
Host scp
    HostName    127.0.0.1
    Port        {port}
    User        sftp
"##
    );
    temp.write_all(config.as_bytes()).unwrap();
    temp
}

/// Mock ssh key storage
pub struct MockSshKeyStorage {
    key: NamedTempFile,
}

impl Default for MockSshKeyStorage {
    fn default() -> Self {
        let mut key = NamedTempFile::new().expect("Failed to create tempfile");
        assert!(
            writeln!(
                key,
                r"-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAABFwAAAAdzc2gtcn
NhAAAAAwEAAQAAAQEAxKyYUMRCNPlb4ZV1VMofrzApu2l3wgP4Ot9wBvHsw/+RMpcHIbQK
9iQqAVp8Z+M1fJyPXTKjoJtIzuCLF6Sjo0KI7/tFTh+yPnA5QYNLZOIRZb8skumL4gwHww
5Z942FDPuUDQ30C2mZR9lr3Cd5pA8S1ZSPTAV9QQHkpgoS8cAL8QC6dp3CJjUC8wzvXh3I
oN3bTKxCpM10KMEVuWO3lM4Nvr71auB9gzo1sFJ3bwebCZIRH01FROyA/GXRiaOtJFG/9N
nWWI/iG5AJzArKpLZNHIP+FxV/NoRH0WBXm9Wq5MrBYrD1NQzm+kInpS/2sXk3m1aZWqLm
HF2NKRXSbQAAA8iI+KSniPikpwAAAAdzc2gtcnNhAAABAQDErJhQxEI0+VvhlXVUyh+vMC
m7aXfCA/g633AG8ezD/5EylwchtAr2JCoBWnxn4zV8nI9dMqOgm0jO4IsXpKOjQojv+0VO
H7I+cDlBg0tk4hFlvyyS6YviDAfDDln3jYUM+5QNDfQLaZlH2WvcJ3mkDxLVlI9MBX1BAe
SmChLxwAvxALp2ncImNQLzDO9eHcig3dtMrEKkzXQowRW5Y7eUzg2+vvVq4H2DOjWwUndv
B5sJkhEfTUVE7ID8ZdGJo60kUb/02dZYj+IbkAnMCsqktk0cg/4XFX82hEfRYFeb1arkys
FisPU1DOb6QielL/axeTebVplaouYcXY0pFdJtAAAAAwEAAQAAAP8u3PFuTVV5SfGazwIm
MgNaux82iOsAT/HWFWecQAkqqrruUw5f+YajH/riV61NE9aq2qNOkcJrgpTWtqpt980GGd
SHWlgpRWQzfIooEiDk6Pk8RVFZsEykkDlJQSIu2onZjhi5A5ojHgZoGGabDsztSqoyOjPq
6WPvGYRiDAR3leBMyp1WufBCJqAsC4L8CjPJSmnZhc5a0zXkC9Syz74Fa08tdM7bGhtvP1
GmzuYxkgxHH2IFeoumUSBHRiTZayGuRUDel6jgEiUMxenaDKXe7FpYzMm9tQZA10Mm4LhK
5rP9nd2/KRTFRnfZMnKvtIRC9vtlSLBe14qw+4ZCl60AAACAf1kghlO3+HIWplOmk/lCL0
w75Zz+RdvueL9UuoyNN1QrUEY420LsixgWSeRPby+Rb/hW+XSAZJQHowQ8acFJhU85So7f
4O4wcDuE4f6hpsW9tTfkCEUdLCQJ7EKLCrod6jIV7hvI6rvXiVucRpeAzdOaq4uzj2cwDd
tOdYVsnmQAAACBAOVxBsvO/Sr3rZUbNtA6KewZh/09HNGoKNaCeiD7vaSn2UJbbPRByF/o
Oo5zv8ee8r3882NnmG808XfSn7pPZAzbbTmOaJt0fmyZhivCghSNzV6njW3o0PdnC0fGZQ
ruVXgkd7RJFbsIiD4dDcF4VCjwWHfTK21EOgJUA5pN6TNvAAAAgQDbcJWRx8Uyhkj2+srb
3n2Rt6CR7kEl9cw17ItFjMn+pO81/5U2aGw0iLlX7E06TAMQC+dyW/WaxQRey8RRdtbJ1e
TNKCN34QCWkyuYRHGhcNc0quEDayPw5QWGXlP4BzjfRUcPxY9cCXLe5wDLYsX33HwOAc59
RorU9FCmS/654wAAABFyb290QDhjNTBmZDRjMzQ1YQECAw==
-----END OPENSSH PRIVATE KEY-----"
            )
            .is_ok()
        );
        Self { key }
    }
}

impl SshKeyStorage for MockSshKeyStorage {
    fn resolve(&self, host: &str, username: &str) -> Option<std::path::PathBuf> {
        match (host, username) {
            ("sftp", "sftp") => Some(self.key.path().to_path_buf()),
            ("scp", "sftp") => Some(self.key.path().to_path_buf()),
            _ => None,
        }
    }
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

fn configure_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(std::time::Duration::from_secs(100)) // measure time
        .warm_up_time(std::time::Duration::from_secs(15))
        .sample_size(10) // samples
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = benchmark_scp_read,
    //benchmark_scp_write,
    benchmark_sftp_read,
    //benchmark_sftp_write
);
criterion_main!(benches);
