include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(all(target_arch="x86_64", target_vendor="unknown", target_os="linux", target_env="gnu"))]
    fn x86_64_unknown_linux_gnu() {
        assert_eq!(get(), "x86_64-unknown-linux-gnu");
    }
}
