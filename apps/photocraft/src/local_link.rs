//! PIXORA's local link: the control channel (`control_server`, `docs/control-protocol.md`) runs
//! by default, so any program on this computer that holds the token, can drive PIXORA.
//!
//! - It listens on 127.0.0.1 only, on the first free port from [`FIRST_PORT`].
//! - Every connection authenticates with the token in `pixora-link.token`.
//! - `link.json` beside it tells clients the port, the token file and how to start PIXORA.
//! - Files: absolute paths anywhere in the user's home folder or on any drive; relative paths are
//!   relative to the home folder.
//!
//! The folder is `%APPDATA%\PIXORA` (macOS `~/Library/Application Support/PIXORA`, Linux
//! `~/.config/pixora`), also in portable mode, so clients always know where to look;
//! `PIXORA_LINK_DIR` overrides it. Off: `PIXORA_LINK=0`, a file named `pixora-link-off` in
//! that folder, or an explicit `--control` launch (which keeps its own settings).

use std::net::TcpListener;
use std::path::{Path, PathBuf};

use photocraft_automation::AuthorizedWorkspace;

/// The first port tried; the next [`PORTS`] - 1 are tried when it is taken.
pub const FIRST_PORT: u16 = 47811;
const PORTS: u16 = 10;
/// A file with this name in the link folder turns the link off.
pub const OFF_MARKER: &str = "pixora-link-off";

/// A bound, ready PIXORA Link.
pub struct Link {
    pub listener: TcpListener,
    pub port: u16,
    pub token: String,
    pub token_file: PathBuf,
    pub dir: PathBuf,
    pub workspace: AuthorizedWorkspace,
}

/// Where `link.json` and the token live.
pub fn folder() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("PIXORA_LINK_DIR").filter(|d| !d.is_empty()) {
        return Some(PathBuf::from(dir));
    }
    crate::app_dirs::platform_dir(&|name| std::env::var_os(name))
}

/// Whether the link should run, given the environment and the link folder.
pub fn enabled(env: Option<&str>, dir: &Path) -> bool {
    let off = env.is_some_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "0" | "off" | "false" | "no"));
    !off && !dir.join(OFF_MARKER).exists()
}

/// The user's home folder.
fn home() -> Option<PathBuf> {
    let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(var).filter(|h| !h.is_empty()).map(PathBuf::from)
}

/// Every root a photo may live under: each drive on Windows, `/` elsewhere.
fn file_roots() -> Vec<PathBuf> {
    if cfg!(windows) {
        (b'C'..=b'Z').map(|d| PathBuf::from(format!("{}:\\", d as char))).filter(|p| p.is_dir()).collect()
    } else {
        vec![PathBuf::from("/")]
    }
}

/// Binds the link, or says why not (logged; PIXORA runs on without it).
pub fn prepare() -> Option<Link> {
    let dir = folder()?;
    if !enabled(std::env::var("PIXORA_LINK").ok().as_deref(), &dir) {
        log::info!("PIXORA Link is off");
        return None;
    }
    if let Err(e) = std::fs::create_dir_all(&dir) {
        log::warn!("PIXORA Link: can't create {}: {e}", dir.display());
        return None;
    }
    let token_file = dir.join("pixora-link.token");
    let token = match photocraft_automation::security::server_token(None, Some(&token_file)) {
        Ok(token) => token,
        Err(e) => {
            log::warn!("PIXORA Link: no token: {e}");
            return None;
        }
    };
    let Some((listener, port)) = (FIRST_PORT..FIRST_PORT.saturating_add(PORTS)).find_map(|p| TcpListener::bind(("127.0.0.1", p)).ok().map(|l| (l, p)))
    else {
        log::warn!("PIXORA Link: ports {FIRST_PORT}-{} are all in use", FIRST_PORT.saturating_add(PORTS - 1));
        return None;
    };
    let home = home()?;
    let roots = file_roots();
    let workspace = match AuthorizedWorkspace::new(Some(&home), Some(&home)) {
        Ok(w) => w.with_extra_roots(&roots, &roots),
        Err(e) => {
            log::warn!("PIXORA Link: {e}");
            return None;
        }
    };
    Some(Link { listener, port, token, token_file, dir, workspace })
}

/// Writes `link.json`: how a client finds and starts PIXORA. Best effort.
pub fn write_info(link: &Link) {
    let exe = std::env::current_exe().ok();
    let info = serde_json::json!({
        "app": "PIXORA",
        "version": photocraft_engine::build_info::VERSION,
        "protocol": "photocraft-control",
        "port": link.port,
        "tokenFile": link.token_file,
        "pid": std::process::id(),
        "running": true,
        "exe": exe,
        "home": home(),
    });
    let path = link.dir.join("link.json");
    let tmp = link.dir.join("link.json.tmp");
    let text = serde_json::to_string_pretty(&info).unwrap_or_default();
    match std::fs::write(&tmp, text).and_then(|()| std::fs::rename(&tmp, &path)) {
        Ok(()) => log::info!("PIXORA Link on 127.0.0.1:{} ({})", link.port, path.display()),
        Err(e) => log::warn!("PIXORA Link: can't write {}: {e}", path.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_unless_switched_off() {
        let dir = std::env::temp_dir().join(format!("pixora-link-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(enabled(None, &dir));
        assert!(enabled(Some("1"), &dir));
        assert!(!enabled(Some("0"), &dir));
        assert!(!enabled(Some(" Off "), &dir));
        std::fs::write(dir.join(OFF_MARKER), b"").unwrap();
        assert!(!enabled(None, &dir));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
