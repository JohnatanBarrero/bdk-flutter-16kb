fn main() {
      let target = std::env::var("TARGET").unwrap_or_default();
      if target.contains("android") {
          println!("cargo:rustc-link-arg=-z");
          println!("cargo:rustc-link-arg=max-page-size=16384");
      }
  }
