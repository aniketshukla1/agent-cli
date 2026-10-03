use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub metadata: SkillMetadata,
    pub instructions: String,
    pub dir_path: PathBuf,
    pub scripts: Vec<PathBuf>,
}

impl Skill {
    pub fn parse_skill_file(skill_file: &Path) -> Result<Self> {
        let content = fs::read_to_string(skill_file)
            .with_context(|| format!("Failed to read {}", skill_file.display()))?;

        let (frontmatter, instructions) = if content.starts_with("---") {
            let mut parts = content[3..].splitn(2, "---");
            let fm = parts.next().unwrap_or("");
            let body = parts.next().unwrap_or("").trim();
            (fm, body)
        } else {
            ("", content.as_str())
        };

        let metadata: SkillMetadata = if !frontmatter.is_empty() {
            serde_yaml::from_str(frontmatter)
                .unwrap_or_else(|_| SkillMetadata {
                    name: skill_file
                        .parent()
                        .and_then(|p| p.file_name())
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unnamed_skill".to_string()),
                    description: "No description provided".to_string(),
                    version: None,
                    author: None,
                })
        } else {
            SkillMetadata {
                name: skill_file
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed_skill".to_string()),
                description: "Skill with instructions".to_string(),
                version: None,
                author: None,
            }
        };

        let dir_path = skill_file.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
        let mut scripts = Vec::new();
        let scripts_dir = dir_path.join("scripts");
        if scripts_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(scripts_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_file() {
                        scripts.push(entry.path());
                    }
                }
            }
        }

        Ok(Self {
            metadata,
            instructions: instructions.to_string(),
            dir_path,
            scripts,
        })
    }
}

pub struct SkillsManager {
    pub skills: Vec<Skill>,
}

impl SkillsManager {
    pub fn load_all() -> Self {
        let mut skills = Vec::new();
        let search_dirs = vec![
            PathBuf::from(".agent/skills"),
            PathBuf::from("skills"),
            if let Some(proj_dirs) = directories::ProjectDirs::from("com", "agent", "agent-cli") {
                proj_dirs.config_dir().join("skills")
            } else {
                PathBuf::from(".config/agent-cli/skills")
            },
        ];

        for base in search_dirs {
            if !base.is_dir() {
                continue;
            }
            if let Ok(entries) = fs::read_dir(base) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let skill_file = path.join("SKILL.md");
                        if skill_file.is_file() {
                            if let Ok(skill) = Skill::parse_skill_file(&skill_file) {
                                skills.push(skill);
                            }
                        }
                    }
                }
            }
        }

        Self { skills }
    }

    pub fn find(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.metadata.name.eq_ignore_ascii_case(name))
    }

    pub fn list(&self) -> &[Skill] {
        &self.skills
    }
}
