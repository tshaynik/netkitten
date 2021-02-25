use structopt::StructOpt;

use anyhow::{anyhow, Context, Result};
use std::str::FromStr;

#[derive(StructOpt, Debug, Clone)]
pub struct ScanOpts {
    host: String,
    port_range: PortRange,
}

#[derive(Debug, Clone)]
struct PortRange {
    low_port: u16,
    high_port: Option<u16>,
}

/// Valid port ranges are two port numbers between 0 and 65536
/// separated with a "-" or a single port number.
impl FromStr for PortRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ports = s.split("-");

        let low_port = ports
            .next()
            .ok_or_else(|| anyhow!("empty string provided for port range"))
            .and_then(|port| {
                u16::from_str_radix(port, 10)
                    .with_context(|| format!("failed to parse {} as a valid port number.", port))
            })?;

        let high_port = if let Some(port) = ports.next() {
            Some(
                u16::from_str_radix(port, 10)
                    .with_context(|| format!("failed to parse {} as a valid port number.", port))?,
            )
        } else {
            None
        };

        if let Some(_) = ports.next() {
            return Err(anyhow!("port range must contain at most one hyphen"));
        };

        if let Some(high) = high_port {
            if high <= low_port {
                return Err(anyhow!(
                    "the second port number must be larger than the first"
                ));
            }
        };

        Ok(PortRange {
            low_port,
            high_port,
        })
    }
}

pub fn scan(opts: &ScanOpts) -> Result<()> {
    println!("{:?}", opts.port_range);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_valid_port_range_single() -> Result<()> {
        let port_range = PortRange::from_str("8888")?;

        assert_eq!(port_range.low_port, 8888);
        assert_eq!(port_range.high_port, None);

        Ok(())
    }

    #[test]
    fn parse_valid_port_range() -> Result<()> {
        let port_range = PortRange::from_str("123-43012")?;

        assert_eq!(port_range.low_port, 123);
        assert_eq!(port_range.high_port, Some(43012));

        Ok(())
    }

    #[test]
    fn parse_invalid_port_range() {
        assert!(PortRange::from_str("-43012").is_err());
        assert!(PortRange::from_str("123-43012-").is_err());
        assert!(PortRange::from_str("12343012").is_err());
        assert!(PortRange::from_str("776-12").is_err());
    }
}
