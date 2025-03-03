use zed_extension_api::{self as zed, LanguageServerId, Result};

pub struct Tact {}

impl zed::Extension for Tact {
    fn new() -> Self
    where
        Self: Sized,
    {
        return Tact {};
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: worktree.which("node").ok_or("please install node")?,
            args: ["--no-lazy", "--inspect=6009"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            env: vec![],
        })
    }
}

zed::register_extension!(Tact);
