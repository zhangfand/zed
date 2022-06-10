use anyhow::Result;
use std::net::SocketAddr;
use std::{fmt::Write, time::Duration};
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct StatsD {
    socket: UdpSocket,
    server_address_string: String,
    buffer: String,
    server_address: Option<SocketAddr>,
}

pub trait Metric {
    fn format<'a>(&self, buffer: &mut String, tags: impl IntoIterator<Item = (&'a str, &'a str)>)
    where
        Self: Sized,
    {
        buffer.clear();
        self.format_prefix(buffer);
        for (i, (key, value)) in tags.into_iter().enumerate() {
            if i == 0 {
                buffer.push_str("|#");
            } else {
                buffer.push_str(",");
            }
            buffer.push_str(key);
            buffer.push_str(":");
            buffer.push_str(value);
        }
    }

    fn format_prefix(&self, output: &mut String);
}

pub struct Count<'a> {
    pub name: &'a str,
    pub count: usize,
}

pub struct Guage<'a> {
    pub name: &'a str,
    pub value: f64,
}

pub struct Timer<'a> {
    pub name: &'a str,
    pub duration: Duration,
}

impl StatsD {
    pub async fn new(bind_address: impl ToSocketAddrs, server_address: String) -> Result<Self> {
        let mut this = Self {
            socket: UdpSocket::bind(bind_address).await?,
            server_address_string: server_address.to_string(),
            server_address: None,
            buffer: String::new(),
        };
        this.resolve_server().await?;
        Ok(this)
    }

    pub async fn send<'a, M, T>(&mut self, metric: M, tags: T) -> Result<()>
    where
        M: Metric,
        T: IntoIterator<Item = (&'a str, &'a str)>,
    {
        metric.format(&mut self.buffer, tags);
        if self.server_address.is_none() {
            self.resolve_server().await?;
        }
        if let Some(server_address) = &self.server_address {
            self.socket
                .send_to(self.buffer.as_bytes(), server_address)
                .await?;
        }
        Ok(())
    }

    async fn resolve_server(&mut self) -> Result<()> {
        self.server_address = tokio::net::lookup_host(&self.server_address_string)
            .await
            .ok()
            .and_then(|mut s| s.next());
        Ok(())
    }
}

impl<'a> Metric for Count<'a> {
    fn format_prefix(&self, output: &mut String) {
        output
            .write_fmt(format_args!("{}:{}|c", self.name, self.count))
            .unwrap();
    }
}

impl<'a> Metric for Guage<'a> {
    fn format_prefix(&self, output: &mut String) {
        output
            .write_fmt(format_args!("{}:{}|g", self.name, self.value))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_metrics() {
        let mut datagram = String::new();

        Count {
            name: "page.views",
            count: 1,
        }
        .format(&mut datagram, []);
        assert_eq!(datagram, "page.views:1|c");

        Count {
            name: "users.online",
            count: 10,
        }
        .format(&mut datagram, [("country", "china")]);
        assert_eq!(datagram, "users.online:10|c|#country:china");

        Count {
            name: "users.online",
            count: 10,
        }
        .format(&mut datagram, [("country", "china"), ("status", "idle")]);
        assert_eq!(datagram, "users.online:10|c|#country:china,status:idle");

        Guage {
            name: "fuel.level",
            value: 0.5,
        }
        .format(&mut datagram, []);
        assert_eq!(datagram, "fuel.level:0.5|g");
    }
}
