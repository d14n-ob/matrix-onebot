fn main() {
    #[cfg(feature = "windows-build")]
    {
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "windows" {
            embed_resource::compile("./resource/logo.rc", embed_resource::NONE);
        }
    }
}