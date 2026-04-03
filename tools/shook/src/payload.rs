use serde::{Deserialize, Deserializer};
use strum::{EnumString, VariantNames};
#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub repository: String,
    pub sender: String,
    pub workflow_job: WorkflowJob,
}
#[derive(Debug, Deserialize)]
pub struct WorkflowJob {
    pub workflow_name: String,
    pub job_id: u32,
    pub name: String,
    #[serde(deserialize_with = "parse_labels")]
    pub labels: RunnerSpec,
}
#[derive(Debug)]
struct RunnerSpec {
    pub os: Os,
    pub arch: Arch,
    pub cpu_mhz: usize,
    pub memory_mb: usize,
}

#[derive(Debug, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
enum Os {
    Linux,
}

#[derive(Debug, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
enum Arch {
    X64,
    Arm64,
}
fn parse_labels<'de, D>(deserializer: D) -> Result<RunnerSpec, D::Error>
where
    D: Deserializer<'de>,
{
    let labels: Vec<String> = Vec::deserialize(deserializer)?;

    // We expect a fixed number of labels in the array (4 in this case)
    if labels.len() != 4 {
        return Err(serde::de::Error::invalid_length(
            labels.len(),
            &"4 labels expected",
        ));
    }

    let os = labels[1]
        .parse::<Os>()
        .map_err(|_| serde::de::Error::unknown_variant(labels[1].as_str(), &Os::VARIANTS))?;

    let arch = labels[2]
        .parse::<Arch>()
        .map_err(|_| serde::de::Error::unknown_variant(labels[2].as_str(), &Arch::VARIANTS))?;

    let cpu_mhz: usize = labels[3].parse().map_err(|_| {
        serde::de::Error::custom(format!("Invalid value for cpu_mhz: {}", labels[3]))
    })?;

    // Assume memory is fixed for this example, adjust as needed
    let memory_mb = 1024;

    Ok(RunnerSpec {
        os,
        arch,
        cpu_mhz,
        memory_mb,
    })
}
