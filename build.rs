use std::{path::Path, process::Command, time::Instant};

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src");

    // Compile Tailwind styles
    if std::env::var_os("CARGO_FEATURE_INLINE_CSS").is_some() {
        let output = std::env::var_os("OUT_DIR").unwrap();
        let output = Path::new(&output).join("styles.css");
        let start = Instant::now();
        let tailwind = Command::new("npx")
            .arg("tailwindcss")
            .arg("-i")
            .arg("src/root.css")
            .arg("-o")
            .arg(output)
            .arg("--minify")
            .output()?;
        if !tailwind.status.success() {
            println!(
                "cargo:warning=Failed to compile styles:\n{}",
                String::from_utf8(tailwind.stderr).unwrap()
            );
        }
        println!(
            "cargo:warning=Compiled styles in {:.2}s",
            Instant::now().duration_since(start).as_secs_f64()
        );
    }

    Ok(())
}
