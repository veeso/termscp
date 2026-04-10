use std::borrow::Cow;
use std::time::Duration;

use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::{Container, Image};

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
