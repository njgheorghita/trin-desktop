use regex::Regex;
use serde::{Deserialize, Serialize};

// all of the node stats that are displayed in the trin desktop app
#[derive(Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeStats {
    pub cpu: f32,
    pub node_history_log: NodeHistoryLog,
}

// the scraped stats from trin's "trin_history: reports~ data:" log line
// in the future, we should probably just add a jsonrpc endpoint that returns this data
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct NodeHistoryLog {
    pub radius: f32,
    pub content_current: f32,
    pub content_total: f32,
    pub count: u32,
    pub disk_usage: f32,
    pub offers_in: u32,
    pub offers_out: u32,
    pub accepts_in: u32,
    pub accepts_out: u32,
    pub validations_in: u32,
    pub validations_out: u32,
}

impl NodeHistoryLog {
    pub fn parse_log_line(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let re = Regex::new(
            r"radius=(\d+\.?\d*)%.*?content=(\d+\.?\d*)/(\d+\.?\d*)mb.*?#=(\d+).*?disk=(\d+\.?\d*)mb.*?offers=(\d+)/(\d+).*?accepts=(\d+)/(\d+).*?validations=(\d+)/(\d+)",
        )?;

        let captures = re
            .captures(line)
            .ok_or("Failed to match log line pattern")?;

        Ok(Self {
            radius: captures[1].parse()?,
            content_current: captures[2].parse()?,
            content_total: captures[3].parse()?,
            count: captures[4].parse()?,
            disk_usage: captures[5].parse()?,
            offers_in: captures[6].parse()?,
            offers_out: captures[7].parse()?,
            accepts_in: captures[8].parse()?,
            accepts_out: captures[9].parse()?,
            validations_in: captures[10].parse()?,
            validations_out: captures[11].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[2024-10-31][19:13:41][trin_desktop_lib][INFO] Child process stdout: 2024-10-31T19:13:41.425824Z  INFO trin_history: reports~ data: radius=15% content=116.7/120mb #=13763 disk=267.1mb; msgs: offers=0/0, accepts=0/0, validations=0/0", NodeHistoryLog {
        radius: 15.0,
        content_current: 116.7,
        content_total: 120.0,
        count: 13763,
        disk_usage: 267.1,
        offers_in: 0,
        offers_out: 0,
        accepts_in: 0,
        accepts_out: 0,
        validations_in: 0,
        validations_out: 0,
    })]
    #[case("[2024-10-31][19:57:52][trin_desktop_lib][INFO] Child process stdout: 2024-10-31T19:57:52.530059Z  INFO trin_history: reports~ data: radius=8.8% content=117.7/120mb #=11719 disk=267.1mb; msgs: offers=26700/27338, accepts=11305/11305, validations=6050/6050", NodeHistoryLog {
        radius: 8.8,
        content_current: 117.7,
        content_total: 120.0,
        count: 11719,
        disk_usage: 267.1,
        offers_in: 26700,
        offers_out: 27338,
        accepts_in: 11305,
        accepts_out: 11305,
        validations_in: 6050,
        validations_out: 6050,
    })]
    fn test_parse_log_lines(#[case] log: &str, #[case] expected: NodeHistoryLog) {
        let result = NodeHistoryLog::parse_log_line(log).unwrap();
        assert_eq!(result.radius, expected.radius);
        assert_eq!(result.content_current, expected.content_current);
        assert_eq!(result.content_total, expected.content_total);
        assert_eq!(result.count, expected.count);
        assert_eq!(result.disk_usage, expected.disk_usage);
        assert_eq!(result.offers_in, expected.offers_in);
        assert_eq!(result.offers_out, expected.offers_out);
        assert_eq!(result.accepts_in, expected.accepts_in);
        assert_eq!(result.accepts_out, expected.accepts_out);
        assert_eq!(result.validations_in, expected.validations_in);
        assert_eq!(result.validations_out, expected.validations_out);
    }
}
