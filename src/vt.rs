use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub const USAGE: &str = "usage: ox-vt replay <file.vt>";

#[derive(Debug)]
pub enum OxVtError {
    MissingCommand,
    UnknownCommand(String),
    MissingReplayPath,
    ExtraReplayArgs(Vec<String>),
    PathNotFound(PathBuf),
    PathIsNotFile(PathBuf),
    Io {
        action: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    Stdout(io::Error),
}

impl fmt::Display for OxVtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCommand => write!(f, "missing ox-vt command\n{USAGE}"),
            Self::UnknownCommand(command) => {
                write!(f, "unknown ox-vt command: {command}\n{USAGE}")
            }
            Self::MissingReplayPath => write!(f, "ox-vt replay requires a file path\n{USAGE}"),
            Self::ExtraReplayArgs(args) => write!(
                f,
                "ox-vt replay accepts exactly one file path; extra arguments: {}\n{USAGE}",
                args.join(" ")
            ),
            Self::PathNotFound(path) => write!(f, "VT capture not found: {}", path.display()),
            Self::PathIsNotFile(path) => {
                write!(f, "VT capture path is not a file: {}", path.display())
            }
            Self::Io {
                action,
                path,
                source,
            } => write!(
                f,
                "failed to {action} VT capture {}: {source}",
                path.display()
            ),
            Self::Stdout(source) => write!(f, "failed to write ox-vt output: {source}"),
        }
    }
}

impl std::error::Error for OxVtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } | Self::Stdout(source) => Some(source),
            _ => None,
        }
    }
}

pub fn run_cli<I, S, W>(args: I, output: &mut W) -> Result<(), OxVtError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
    W: Write,
{
    let mut args = args.into_iter().map(Into::into);
    let Some(command) = args.next() else {
        return Err(OxVtError::MissingCommand);
    };

    match command.as_str() {
        "-h" | "--help" => {
            writeln!(output, "{USAGE}").map_err(OxVtError::Stdout)?;
            Ok(())
        }
        "replay" => {
            let Some(path) = args.next() else {
                return Err(OxVtError::MissingReplayPath);
            };
            let extra_args: Vec<String> = args.collect();
            if !extra_args.is_empty() {
                return Err(OxVtError::ExtraReplayArgs(extra_args));
            }

            replay_file(Path::new(&path), output)
        }
        other => Err(OxVtError::UnknownCommand(other.to_string())),
    }
}

pub fn replay_file<W>(path: &Path, output: &mut W) -> Result<(), OxVtError>
where
    W: Write,
{
    let metadata = fs::metadata(path).map_err(|source| {
        if source.kind() == io::ErrorKind::NotFound {
            OxVtError::PathNotFound(path.to_path_buf())
        } else {
            OxVtError::Io {
                action: "inspect",
                path: path.to_path_buf(),
                source,
            }
        }
    })?;

    if !metadata.is_file() {
        return Err(OxVtError::PathIsNotFile(path.to_path_buf()));
    }

    let mut input = fs::File::open(path).map_err(|source| OxVtError::Io {
        action: "open",
        path: path.to_path_buf(),
        source,
    })?;

    replay_stream(&mut input, output).map_err(|source| OxVtError::Io {
        action: "stream",
        path: path.to_path_buf(),
        source,
    })?;
    Ok(())
}

pub fn replay_stream<R, W>(input: &mut R, output: &mut W) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    io::copy(input, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_stream_is_byte_exact() {
        let fixture = b"\x1b[2J\x1b[HFire Horse\x1b[0m\n";
        let mut input = &fixture[..];
        let mut output = Vec::new();

        replay_stream(&mut input, &mut output).expect("stream VT fixture");

        assert_eq!(output, fixture);
    }

    #[test]
    fn replay_rejects_missing_path() {
        let mut output = Vec::new();
        let error = replay_file(
            Path::new("target/__missing_ox_vt_replay_fixture__.vt"),
            &mut output,
        )
        .expect_err("missing path should fail");

        assert!(matches!(error, OxVtError::PathNotFound(_)));
        assert!(output.is_empty());
    }

    #[test]
    fn replay_rejects_directory_path() {
        let mut output = Vec::new();
        let error =
            replay_file(Path::new("."), &mut output).expect_err("directory path should fail");

        assert!(matches!(error, OxVtError::PathIsNotFile(_)));
        assert!(output.is_empty());
    }

    #[test]
    fn cli_replays_existing_golden() {
        let mut output = Vec::new();

        run_cli(
            ["replay", "tests/wtd/goldens/W038/uxlab_once_smoke.vt"],
            &mut output,
        )
        .expect("replay W038 smoke golden");

        assert!(output.starts_with(b"\x1b]2;"));
        assert!(contains_bytes(&output, b"suite: lab-smoke"));
    }

    #[test]
    fn cli_rejects_unknown_command() {
        let mut output = Vec::new();
        let error = run_cli(["unknown"], &mut output).expect_err("unknown command should fail");

        assert!(matches!(error, OxVtError::UnknownCommand(ref command) if command == "unknown"));
        assert!(error.to_string().contains(USAGE));
        assert!(output.is_empty());
    }

    fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
        haystack
            .windows(needle.len())
            .any(|candidate| candidate == needle)
    }
}
