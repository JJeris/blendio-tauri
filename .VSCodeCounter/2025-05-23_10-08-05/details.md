# Details

Date : 2025-05-23 10:08:05

Directory c:\\bakalaura_darba\\blendio-tauri

Total : 62 files,  4621 codes, 584 comments, 296 blanks, all 5501 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [README.md](/README.md) | Markdown | 0 | 0 | 1 | 1 |
| [index.html](/index.html) | HTML | 13 | 0 | 2 | 15 |
| [package.json](/package.json) | JSON | 27 | 0 | 1 | 28 |
| [src-tauri/.env](/src-tauri/.env) | Properties | 1 | 0 | 1 | 2 |
| [src-tauri/Cargo.toml](/src-tauri/Cargo.toml) | TOML | 27 | 4 | 6 | 37 |
| [src-tauri/build.rs](/src-tauri/build.rs) | Rust | 3 | 0 | 1 | 4 |
| [src-tauri/capabilities/create-project-file-popup.json](/src-tauri/capabilities/create-project-file-popup.json) | JSON | 13 | 0 | 1 | 14 |
| [src-tauri/capabilities/default.json](/src-tauri/capabilities/default.json) | JSON | 19 | 0 | 0 | 19 |
| [src-tauri/capabilities/download-popup.json](/src-tauri/capabilities/download-popup.json) | JSON | 13 | 0 | 1 | 14 |
| [src-tauri/capabilities/launch-blender-popup.json](/src-tauri/capabilities/launch-blender-popup.json) | JSON | 13 | 0 | 0 | 13 |
| [src-tauri/capabilities/launch-project-file-popup.json](/src-tauri/capabilities/launch-project-file-popup.json) | JSON | 13 | 0 | 0 | 13 |
| [src-tauri/migrations/20250427133325\_init.down.sql](/src-tauri/migrations/20250427133325_init.down.sql) | SQLite | 5 | 0 | 1 | 6 |
| [src-tauri/migrations/20250427133325\_init.up.sql](/src-tauri/migrations/20250427133325_init.up.sql) | SQLite | 50 | 5 | 5 | 60 |
| [src-tauri/migrations/20250503124439\_add\_index\_to\_project\_file.down.sql](/src-tauri/migrations/20250503124439_add_index_to_project_file.down.sql) | SQLite | 1 | 1 | 1 | 3 |
| [src-tauri/migrations/20250503124439\_add\_index\_to\_project\_file.up.sql](/src-tauri/migrations/20250503124439_add_index_to_project_file.up.sql) | SQLite | 1 | 1 | 0 | 2 |
| [src-tauri/migrations/20250504092442\_add\_index\_to\_python\_scripts.down.sql](/src-tauri/migrations/20250504092442_add_index_to_python_scripts.down.sql) | SQLite | 1 | 1 | 1 | 3 |
| [src-tauri/migrations/20250504092442\_add\_index\_to\_python\_scripts.up.sql](/src-tauri/migrations/20250504092442_add_index_to_python_scripts.up.sql) | SQLite | 1 | 1 | 0 | 2 |
| [src-tauri/migrations/20250504172200\_add\_index\_to\_installed\_blender\_versions.down.sql](/src-tauri/migrations/20250504172200_add_index_to_installed_blender_versions.down.sql) | SQLite | 1 | 1 | 1 | 3 |
| [src-tauri/migrations/20250504172200\_add\_index\_to\_installed\_blender\_versions.up.sql](/src-tauri/migrations/20250504172200_add_index_to_installed_blender_versions.up.sql) | SQLite | 1 | 1 | 0 | 2 |
| [src-tauri/src/blender\_version/commands.rs](/src-tauri/src/blender_version/commands.rs) | Rust | 1,015 | 218 | 15 | 1,248 |
| [src-tauri/src/blender\_version/mod.rs](/src-tauri/src/blender_version/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src-tauri/src/db\_repo/blender\_repo\_path\_repo.rs](/src-tauri/src/db_repo/blender_repo_path_repo.rs) | Rust | 72 | 3 | 7 | 82 |
| [src-tauri/src/db\_repo/installed\_blender\_version\_repo.rs](/src-tauri/src/db_repo/installed_blender_version_repo.rs) | Rust | 82 | 3 | 7 | 92 |
| [src-tauri/src/db\_repo/launch\_argument\_repo.rs](/src-tauri/src/db_repo/launch_argument_repo.rs) | Rust | 75 | 3 | 7 | 85 |
| [src-tauri/src/db\_repo/mod.rs](/src-tauri/src/db_repo/mod.rs) | Rust | 10 | 0 | 2 | 12 |
| [src-tauri/src/db\_repo/project\_fiile\_repo.rs](/src-tauri/src/db_repo/project_fiile_repo.rs) | Rust | 73 | 3 | 7 | 83 |
| [src-tauri/src/db\_repo/python\_script\_repo.rs](/src-tauri/src/db_repo/python_script_repo.rs) | Rust | 69 | 3 | 7 | 79 |
| [src-tauri/src/file\_system\_utility/commands.rs](/src-tauri/src/file_system_utility/commands.rs) | Rust | 235 | 64 | 14 | 313 |
| [src-tauri/src/file\_system\_utility/mod.rs](/src-tauri/src/file_system_utility/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src-tauri/src/launch\_argument/commands.rs](/src-tauri/src/launch_argument/commands.rs) | Rust | 200 | 50 | 5 | 255 |
| [src-tauri/src/launch\_argument/mod.rs](/src-tauri/src/launch_argument/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src-tauri/src/lib.rs](/src-tauri/src/lib.rs) | Rust | 84 | 7 | 12 | 103 |
| [src-tauri/src/main.rs](/src-tauri/src/main.rs) | Rust | 5 | 1 | 2 | 8 |
| [src-tauri/src/models/blender\_repo\_path.rs](/src-tauri/src/models/blender_repo_path.rs) | Rust | 11 | 0 | 2 | 13 |
| [src-tauri/src/models/downloadable\_blender\_version.rs](/src-tauri/src/models/downloadable_blender_version.rs) | Rust | 20 | 0 | 2 | 22 |
| [src-tauri/src/models/installed\_blender\_version.rs](/src-tauri/src/models/installed_blender_version.rs) | Rust | 15 | 1 | 2 | 18 |
| [src-tauri/src/models/launch\_argument.rs](/src-tauri/src/models/launch_argument.rs) | Rust | 13 | 0 | 2 | 15 |
| [src-tauri/src/models/mod.rs](/src-tauri/src/models/mod.rs) | Rust | 12 | 0 | 2 | 14 |
| [src-tauri/src/models/project\_file.rs](/src-tauri/src/models/project_file.rs) | Rust | 13 | 0 | 2 | 15 |
| [src-tauri/src/models/python\_script.rs](/src-tauri/src/models/python_script.rs) | Rust | 10 | 0 | 2 | 12 |
| [src-tauri/src/project\_file/commands.rs](/src-tauri/src/project_file/commands.rs) | Rust | 743 | 172 | 9 | 924 |
| [src-tauri/src/project\_file/consts.rs](/src-tauri/src/project_file/consts.rs) | Rust | 2 | 1 | 1 | 4 |
| [src-tauri/src/project\_file/mod.rs](/src-tauri/src/project_file/mod.rs) | Rust | 4 | 0 | 1 | 5 |
| [src-tauri/src/python\_script/commands.rs](/src-tauri/src/python_script/commands.rs) | Rust | 124 | 30 | 4 | 158 |
| [src-tauri/src/python\_script/mod.rs](/src-tauri/src/python_script/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src-tauri/tauri.conf.json](/src-tauri/tauri.conf.json) | JSON | 38 | 0 | 1 | 39 |
| [src/App.jsx](/src/App.jsx) | JavaScript JSX | 20 | 1 | 2 | 23 |
| [src/components/TitleBar/TitleBar.css](/src/components/TitleBar/TitleBar.css) | PostCSS | 19 | 0 | 3 | 22 |
| [src/components/TitleBar/TitleBar.jsx](/src/components/TitleBar/TitleBar.jsx) | JavaScript JSX | 17 | 0 | 3 | 20 |
| [src/main.jsx](/src/main.jsx) | JavaScript JSX | 9 | 0 | 2 | 11 |
| [src/popup/CreateBlendPopup.jsx](/src/popup/CreateBlendPopup.jsx) | JavaScript JSX | 81 | 0 | 9 | 90 |
| [src/popup/DownloadPopup.jsx](/src/popup/DownloadPopup.jsx) | JavaScript JSX | 68 | 0 | 10 | 78 |
| [src/popup/LaunchBlendPopup.jsx](/src/popup/LaunchBlendPopup.jsx) | JavaScript JSX | 226 | 1 | 21 | 248 |
| [src/popup/LaunchBlenderPopup.jsx](/src/popup/LaunchBlenderPopup.jsx) | JavaScript JSX | 177 | 0 | 19 | 196 |
| [src/router.jsx](/src/router.jsx) | JavaScript JSX | 24 | 0 | 4 | 28 |
| [src/styles/main.css](/src/styles/main.css) | PostCSS | 82 | 0 | 20 | 102 |
| [src/utils/web.js](/src/utils/web.js) | JavaScript | 30 | 0 | 4 | 34 |
| [src/views/BlenderDownload.jsx](/src/views/BlenderDownload.jsx) | JavaScript JSX | 127 | 0 | 9 | 136 |
| [src/views/InstalledBlenderVersions.jsx](/src/views/InstalledBlenderVersions.jsx) | JavaScript JSX | 151 | 1 | 11 | 163 |
| [src/views/ProjectFiles.jsx](/src/views/ProjectFiles.jsx) | JavaScript JSX | 201 | 1 | 16 | 218 |
| [src/views/Settings.jsx](/src/views/Settings.jsx) | JavaScript JSX | 231 | 0 | 17 | 248 |
| [vite.config.js](/vite.config.js) | JavaScript | 22 | 6 | 4 | 32 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)