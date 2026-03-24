struct App {
    title: &'static str,
}

impl App {
    fn new() -> Self {
        Self { title: "OxIde" }
    }

    fn startup_banner(&self) -> String {
        format!(
            "{title}\n\
             bootstrap console application\n\n\
             status: Rust application scaffold is in place.\n\
             next: FrankenTui shell, editor surface, and OxVba workflow commands.\n",
            title = self.title
        )
    }
}

fn main() {
    // Keep the bootstrap binary std-only until the shell and editor beads land.
    let app = App::new();
    print!("{}", app.startup_banner());
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn startup_banner_describes_bootstrap_state() -> Result<(), String> {
        let banner = App::new().startup_banner();

        for needle in [
            "OxIde",
            "bootstrap console application",
            "FrankenTui shell",
            "OxVba",
        ] {
            if !banner.contains(needle) {
                return Err(format!("startup banner is missing {needle:?}"));
            }
        }

        Ok(())
    }
}
