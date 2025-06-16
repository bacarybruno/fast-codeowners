use codeowners_rs::{parse, parse_file, RuleSet};
use napi::Result;
use std::path::PathBuf;

use napi_derive::napi;

#[napi]
pub struct Codeowners {
  ruleset: RuleSet,
  excluded_owners: Vec<String>,
}

#[napi]
impl Codeowners {
  #[napi(constructor)]
  pub fn new(excluded_owners: Option<Vec<String>>) -> Result<Self> {
    let ruleset = Self::create_ruleset()?;
    
    Ok(Codeowners { ruleset, excluded_owners: excluded_owners.unwrap_or_default() })
  }

  #[napi(factory)]
  pub fn from_content(content: String, excluded_owners: Option<Vec<String>>) -> Result<Self> {
    let ruleset = Self::create_ruleset_from_content(content)?;
    Ok(Codeowners { ruleset, excluded_owners: excluded_owners.unwrap_or_default() })
  }

  #[napi]
  pub fn get_owners(&self, path: String) -> Vec<String> {
    self.ruleset.owners(&path)
      .unwrap_or_default()
      .iter()
      .map(|owner| owner.value.clone())
      .filter(|owner| !self.excluded_owners.contains(owner))
      .collect()
  }

  fn codeowners_path() -> Option<PathBuf> {
    const DEFAULT_PATHS: &'static [&'static str] = &[".github/CODEOWNERS", ".gitlab/CODEOWNERS", "docs/CODEOWNERS", "CODEOWNERS"];
  
    DEFAULT_PATHS
            .iter()
            .map(PathBuf::from)
            .find(|p| p.exists())
  }
  
  fn create_ruleset() -> Result<RuleSet> {
    let codeowners_path = Self::codeowners_path()
      .ok_or_else(|| napi::Error::new(napi::Status::GenericFailure, "No CODEOWNERS file found"))?;
    let ruleset = parse_file(&codeowners_path)
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("Failed to parse CODEOWNERS: {}", e)))?
      .into_ruleset();
    Ok(ruleset)
  }
  
  fn create_ruleset_from_content(content: String) -> Result<RuleSet> {
    let ruleset = parse(&content).into_ruleset();
    Ok(ruleset)
  }
}
