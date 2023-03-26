use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

fn reumask(umask: u32, path: &Path, file_type: &fs::FileType, file_permissions: &fs::Permissions) {
    /*
     * TODO:
     *   - add support for sticky, suid and sgid preserving.
     */

    let current_permissions = file_permissions.mode() & 0o777;

    let mode = 0o7777 ^ umask;

    let mut permissions = fs::Permissions::from_mode(mode);

    if file_type.is_file() {
        // Add executable bit to owner, group and others if the owner mode 
        // had it and if the new umask allows for it. so files with 700
        // mode on umask 022 will turn into 755 instead of 644.
        let file_mode = if current_permissions & 0o100 != 0 {
            mode & 0o777
        } else {
            mode & 0o666
        };

        permissions.set_mode(file_mode);
    } else if file_type.is_dir() {
        permissions.set_mode(mode & 0o777);
    }

    let new_permissions = permissions.mode() & 0o777;

    if current_permissions != new_permissions {
        println!(
            "[{:o} -> {:o}] {}",
            current_permissions, new_permissions, path.display()
        );
        fs::set_permissions(path, permissions).unwrap();
    }
}

fn list_entries(path: &Path) -> Vec<(PathBuf, fs::FileType, fs::Permissions)> {
    let mut entries = Vec::new();
    let metadata = fs::symlink_metadata(path).unwrap();
    
    // Pass around FileType struct for faster lookup.
    let file_type = metadata.file_type();

    if !file_type.is_symlink() {
        entries.push((
            path.to_path_buf(),
            file_type,
            metadata.permissions(),
        ));

        if file_type.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                entries.extend(list_entries(&entry_path));
            }
        }
    }
    entries
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <umask> <path>", args[0]);
        return;
    }

    let umask = u32::from_str_radix(&args[1], 8).unwrap();
    let path = Path::new(&args[2]);
    let entries = list_entries(&path);

    for (entry_path, entry_type, entry_permissions) in entries {
        reumask(umask, &entry_path, &entry_type, &entry_permissions);
    }
}
