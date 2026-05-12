use crate::client::ApiClient;
use crate::config::Config;
use crate::error::CliError;
use crate::project::ProjectConfig;
use crate::types::app::AppAttributes;

pub async fn execute(
    profile: Option<&str>,
    app_id: Option<&str>,
    cli_issuer_id: Option<&str>,
    cli_key_id: Option<&str>,
    cli_key_path: Option<&str>,
) -> Result<(), CliError> {
    let effective_profile = profile
        .map(String::from)
        .or_else(|| {
            ProjectConfig::load()
                .ok()
                .flatten()
                .and_then(|p| p.profile)
        });

    let config = Config::load(effective_profile.as_deref())?;
    let resolved = config.resolve(cli_issuer_id, cli_key_id, cli_key_path);
    let (issuer_id, key_id, key_path) = resolved.require_auth()?;
    let client = ApiClient::new(issuer_id, key_id, key_path);

    let resolved_app_id = if let Some(id) = app_id {
        id.to_string()
    } else {
        let doc = client.get_list::<AppAttributes>("/apps?limit=200").await?;
        if doc.data.is_empty() {
            return Err(CliError::Config(
                "no apps found in this account".into(),
            ));
        }
        if doc.data.len() == 1 {
            let app = &doc.data[0];
            let name = app
                .attributes
                .as_ref()
                .and_then(|a| a.name.as_deref())
                .unwrap_or("unknown");
            eprintln!("Auto-selected app: {name} ({})", app.id);
            app.id.clone()
        } else {
            eprintln!("Multiple apps found in this account:\n");
            for (i, app) in doc.data.iter().enumerate() {
                let attrs = app.attributes.as_ref();
                let name = attrs.and_then(|a| a.name.as_deref()).unwrap_or("unknown");
                let bundle = attrs
                    .and_then(|a| a.bundle_id.as_deref())
                    .unwrap_or("unknown");
                eprintln!("  [{}] {} ({}) — {}", i + 1, name, bundle, app.id);
            }
            eprintln!("\nRe-run with --app-id <ID> to select one:");
            eprintln!("  apple-cli init --app-id <ID>");
            return Err(CliError::Config(
                "multiple apps found — pass --app-id to select one".into(),
            ));
        }
    };

    let app_doc = client
        .get::<AppAttributes>(&format!("/apps/{resolved_app_id}"))
        .await?;
    let attrs = app_doc.data.attributes.as_ref();
    let app_name = attrs.and_then(|a| a.name.clone());
    let bundle_id = attrs.and_then(|a| a.bundle_id.clone());

    let project_config = ProjectConfig {
        profile: effective_profile.clone(),
        app_id: Some(resolved_app_id.clone()),
        app_name: app_name.clone(),
        bundle_id: bundle_id.clone(),
    };
    project_config.save()?;

    ProjectConfig::ensure_gitignore()?;

    ProjectConfig::generate_claude_skill(
        app_name.as_deref(),
        Some(&resolved_app_id),
    )?;

    let output = serde_json::json!({
        "message": "Project initialized",
        "config_path": ".apple/config.toml",
        "profile": effective_profile.as_deref().unwrap_or("default"),
        "app_id": resolved_app_id,
        "app_name": app_name,
        "bundle_id": bundle_id,
        "gitignore_updated": true,
        "claude_skill_created": ".claude/commands/apple.md",
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
