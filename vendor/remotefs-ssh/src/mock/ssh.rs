//! ## Ssh mock
//!
//! Contains mock for SSH protocol

use std::io::Write;

use tempfile::NamedTempFile;

use crate::SshKeyStorage;

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

// -- config file

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
