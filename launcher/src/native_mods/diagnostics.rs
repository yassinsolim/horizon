//! Compiler guidance for declared internals.
pub(super) fn with_internals_hint(error: String) -> String {
    if error.contains("internals")
        && ["E0432", "E0433", "E0603"]
            .iter()
            .any(|code| error.contains(code))
    {
        format!("{error}\n{}", crate::i18n::tr("Game internals must be declared: run cargo horizon-mod internals sync in the mod's folder, then build again."))
    } else {
        error
    }
}

/// What to do when Cargo couldn't give a crate's file its 1970 time (#16702): the Cargo home is on
/// FAT32 or exFAT, where it stays only when it couldn't move (`rust_toolchain::cargo_home`). Cargo
/// itself names neither the drive nor a way out ("The parameter is incorrect", os error 87).
pub(super) fn with_file_times_hint(error: String) -> String {
    if error.contains("failed to set mtime") && error.contains("(os error 87)") {
        format!("{error}\n{}", crate::i18n::tr("The drive the launcher's build folder is on (FAT32 or exFAT, as on many portable drives) can't store the 1970 file times some crates carry. Move the launcher's data to an NTFS drive in Settings > Data folder, then build again."))
    } else {
        error
    }
}

#[cfg(test)]
mod tests {
    /// Cargo's words on a FAT32 or exFAT Cargo home, in a Windows set to Chinese (the report in
    /// #16702): the hint says what to do; another failure is left as it was.
    #[test]
    fn a_crate_time_the_drive_cant_store_says_what_to_do() {
        let cargo = "error: failed to unpack package `itoa v0.4.8`\n…\nCaused by:\n  failed to set mtime for `E:\\Games\\Horizon-data\\launcher\\cargo\\registry\\src\\index.crates.io-1949cf8c6b5b557f\\itoa-0.4.8\\.cargo_vcs_info.json`\n\nCaused by:\n  参数错误。 (os error 87)";
        let hinted = super::with_file_times_hint(cargo.to_owned());
        assert!(hinted.starts_with(cargo));
        assert!(hinted.contains("NTFS"), "{hinted}");
        let other = "error: failed to set mtime for `x`: Access is denied. (os error 5)";
        assert_eq!(super::with_file_times_hint(other.to_owned()), other);
    }
}
