use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Depth {
	Low,
	Medium,
	High,
	Deep,
}

impl Default for Depth {
	fn default() -> Self {
		Self::Low
	}
}

impl Depth {
	pub fn estimated_duration_minutes(&self) -> u32 {
		match self {
			Depth::Low => 30,
			Depth::Medium => 60,
			Depth::High => 120,
			Depth::Deep => 240,
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Detectability {
	Stealth,
	Guided,
	Aggressive,
}

impl Default for Detectability {
	fn default() -> Self {
		Self::Guided
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Risk {
	Conservative,
	Balanced,
	Assertive,
	Minimal,
	Moderate,
	Elevated,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RateLimit {
	pub probes_per_second: u32,
	pub concurrent_targets: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Randomization {
	pub jitter_ms: u64,
	pub ordering: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSetConfig {
	pub name: String,
	pub description: String,
	pub enabled_pipelines: Vec<String>,
	#[serde(default)]
	pub allowed_targets: Vec<String>,
	#[serde(default)]
	pub depth: Depth,
	#[serde(default)]
	pub detectability: Detectability,
	#[serde(default = "default_risk_balanced")]
	pub target_risk: Risk,
	#[serde(default = "default_risk_balanced")]
	pub operator_risk: Risk,
	pub rate_limit: RateLimit,
	pub evidence_retention: String,
	pub randomization: Randomization,
	pub requires_scope: String,
}

fn default_risk_balanced() -> Risk {
	Risk::Balanced
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSet {
	pub name: String,
	pub description: String,
	pub enabled_pipelines: Vec<String>,
	pub allowed_targets: Vec<IpNet>,
	pub depth: Depth,
	pub detectability: Detectability,
	pub target_risk: Risk,
	pub operator_risk: Risk,
	pub rate_limit: RateLimit,
	pub evidence_retention: String,
	pub randomization: Randomization,
	pub requires_scope: String,
}

impl TryFrom<ScanSetConfig> for ScanSet {
	type Error = ScanSetError;

	fn try_from(value: ScanSetConfig) -> std::result::Result<Self, Self::Error> {
		let allowed_targets = value
			.allowed_targets
			.into_iter()
			.map(|entry| {
				let label = entry.clone();
				entry
					.parse::<IpNet>()
					.map_err(|source| ScanSetError::InvalidTarget(label, source))
			})
			.collect::<std::result::Result<Vec<_>, _>>()?;

		Ok(Self {
			name: value.name,
			description: value.description,
			enabled_pipelines: value.enabled_pipelines,
			allowed_targets,
			depth: value.depth,
			detectability: value.detectability,
			target_risk: value.target_risk,
			operator_risk: value.operator_risk,
			rate_limit: value.rate_limit,
			evidence_retention: value.evidence_retention,
			randomization: value.randomization,
			requires_scope: value.requires_scope,
		})
	}
}

#[derive(Debug, Error)]
pub enum ScanSetError {
	#[error("invalid target '{0}': {1}")]
	InvalidTarget(String, #[source] ipnet::AddrParseError),
}

#[allow(dead_code)]
pub struct ScanSetStore {
	sets: HashMap<String, ScanSet>,
}

#[allow(dead_code)]
impl ScanSetStore {
	pub fn new() -> Self {
		Self {
			sets: HashMap::new(),
		}
	}

	pub fn insert(&mut self, set: ScanSet) {
		self.sets.insert(set.name.clone(), set);
	}

	pub fn list(&self) -> Vec<&ScanSet> {
		self.sets.values().collect()
	}

	pub fn get(&self, name: &str) -> Option<&ScanSet> {
		self.sets.get(name)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanSetPreview {
	pub applicable_pipelines: Vec<String>,
	pub estimated_duration_minutes: u32,
	pub depth: Depth,
	pub detectability: Detectability,
	pub target_risk: Risk,
	pub operator_risk: Risk,
	pub violations: Vec<String>,
}

impl ScanSetPreview {
	pub fn from_scan_set(set: &ScanSet) -> Self {
		let mut violations = Vec::new();

		if matches!(set.detectability, Detectability::Aggressive)
			&& matches!(set.target_risk, Risk::Minimal)
		{
			violations.push(String::from(
				"Aggressive detectability with minimal target risk may trigger noise",
			));
		}

		Self {
			applicable_pipelines: set.enabled_pipelines.clone(),
			estimated_duration_minutes: set.depth.estimated_duration_minutes(),
			depth: set.depth.clone(),
			detectability: set.detectability.clone(),
			target_risk: set.target_risk.clone(),
			operator_risk: set.operator_risk.clone(),
			violations,
		}
	}
}

pub fn load_config_from_path(path: impl AsRef<Path>) -> Result<ScanSetConfig> {
	let data = std::fs::read_to_string(path.as_ref()).with_context(|| {
		format!(
			"failed to read scan set config: {}",
			path.as_ref().display()
		)
	})?;
	let config: ScanSetConfig = serde_json::from_str(&data)
		.with_context(|| format!("failed to parse JSON from {}", path.as_ref().display()))?;
	Ok(config)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE_JSON: &str = r#"
    {
        "name": "custom/deep-audit",
        "description": "Deep audit",
        "enabled_pipelines": ["discovery", "memory", "privilege"],
        "allowed_targets": ["203.0.113.0/28"],
        "depth": "deep",
        "detectability": "aggressive",
        "target_risk": "elevated",
        "operator_risk": "assertive",
        "rate_limit": {"probes_per_second": 5, "concurrent_targets": 2},
        "evidence_retention": "30d",
        "randomization": {"jitter_ms": 200, "ordering": "per-scan"},
        "requires_scope": "hela.scans.deep"
    }
    "#;

	#[test]
	fn parse_scan_set_config() {
		let config: ScanSetConfig = serde_json::from_str(EXAMPLE_JSON).expect("json parse");
		let scan_set = ScanSet::try_from(config).expect("config should be valid");
		assert_eq!(scan_set.depth, Depth::Deep);
		assert_eq!(scan_set.detectability, Detectability::Aggressive);
		assert_eq!(scan_set.enabled_pipelines.len(), 3);
		assert_eq!(scan_set.allowed_targets.len(), 1);
	}

	#[test]
	fn preview_reports_violations() {
		let config: ScanSetConfig = serde_json::from_str(EXAMPLE_JSON).expect("json parse");
		let mut scan_set = ScanSet::try_from(config).unwrap();
		scan_set.target_risk = Risk::Minimal;
		let preview = ScanSetPreview::from_scan_set(&scan_set);
		assert_eq!(preview.depth, Depth::Deep);
		assert_eq!(preview.detectability, Detectability::Aggressive);
		assert!(!preview.violations.is_empty());
		assert_eq!(preview.estimated_duration_minutes, 240);
	}

	#[test]
	fn invalid_allowed_target() {
		let invalid_json = r#"
        {
            "name": "bad",
            "description": "Bad target",
            "enabled_pipelines": ["discovery"],
            "allowed_targets": ["not-a-network"],
            "rate_limit": {"probes_per_second": 1, "concurrent_targets": 1},
            "evidence_retention": "7d",
            "randomization": {"jitter_ms": 0, "ordering": "default"},
            "requires_scope": "hela.scans.default"
        }
        "#;
		let config: ScanSetConfig = serde_json::from_str(invalid_json).unwrap();
		let err = ScanSet::try_from(config).unwrap_err();
		match err {
			ScanSetError::InvalidTarget(value, _) => assert_eq!(value, "not-a-network"),
		}
	}
}
