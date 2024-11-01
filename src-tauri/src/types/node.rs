use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeStats {
    pub cpu: f32,
    pub radius: u32,
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

#[derive(Debug, Default)]
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
            r"radius=(\d+)%\s+content=(\d+\.?\d*)/(\d+\.?\d*)mb\s+#=(\d+)\s+disk=(\d+\.?\d*)mb.*offers=(\d+)/(\d+),\s+accepts=(\d+)/(\d+),\s+validations=(\d+)/(\d+)",
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
