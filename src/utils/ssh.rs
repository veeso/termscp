use ssh2_config::{ParseRule, SshConfig};

pub fn parse_ssh2_config(path: &str) -> Result<SshConfig, String> {
    use std::fs::File;
    use std::io::BufReader;

    let mut reader = File::open(path)
        .map_err(|e| format!("failed to open {path}: {e}"))
        .map(BufReader::new)?;
    SshConfig::default()
        .parse(&mut reader, ParseRule::ALLOW_UNKNOWN_FIELDS)
        .map_err(|e| format!("Failed to parse ssh2 config: {e}"))
}

#[cfg(test)]
mod test {

    use crate::utils::{ssh::parse_ssh2_config, test_helpers};

    #[test]
    fn should_parse_ssh2_config() {
        let rsa_key = test_helpers::create_sample_file_with_content("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDErJhQxEI0+VvhlXVUyh+vMCm7aXfCA/g633AG8ezD/5EylwchtAr2JCoBWnxn4zV8nI9dMqOgm0jO4IsXpKOjQojv+0VOH7I+cDlBg0tk4hFlvyyS6YviDAfDDln3jYUM+5QNDfQLaZlH2WvcJ3mkDxLVlI9MBX1BAeSmChLxwAvxALp2ncImNQLzDO9eHcig3dtMrEKkzXQowRW5Y7eUzg2+vvVq4H2DOjWwUndvB5sJkhEfTUVE7ID8ZdGJo60kUb/02dZYj+IbkAnMCsqktk0cg/4XFX82hEfRYFeb1arkysFisPU1DOb6QielL/axeTebVplaouYcXY0pFdJt root@8c50fd4c345a");
        let ssh_config_file = test_helpers::create_sample_file_with_content(format!(
            r#"
Host test
        HostName 127.0.0.1
        Port 2222
        User test
        IdentityFile {}
        StrictHostKeyChecking no
        UserKnownHostsFile /dev/null
"#,
            rsa_key.path().display()
        ));

        assert!(parse_ssh2_config(
            ssh_config_file
                .path()
                .to_string_lossy()
                .to_string()
                .as_str()
        )
        .is_ok());
    }
}
