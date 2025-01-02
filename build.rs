use embed_resource::CompilationResult;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let authors = env!("CARGO_PKG_AUTHORS");

    let rc_content = format!(
        r#"
1 VERSIONINFO
FILEVERSION {major},{minor},{patch},0
PRODUCTVERSION {major},{minor},{patch},0
FILEFLAGSMASK 0x3fL
FILEFLAGS 0x0L
FILEOS 0x4L
FILETYPE 0x1L
FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName", "{authors}\0"
            VALUE "FileDescription", "{description}\0"
            VALUE "FileVersion", "{version}\0"
            VALUE "InternalName", "{name}.exe\0"
            VALUE "OriginalFilename", "{name}.exe\0"
            VALUE "ProductName", "{name}\0"
            VALUE "ProductVersion", "{version}\0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x0409, 1200
    END
END
"#,
        major = version.split('.').nth(0).unwrap_or("0"),
        minor = version.split('.').nth(1).unwrap_or("0"),
        patch = version.split('.').nth(2).unwrap_or("0"),
        authors = authors,
        description = description,
        version = version,
        name = name,
    );

    let mut rc_file = File::create("app.rc").expect("Failed to create app.rc file");
    rc_file
        .write_all(rc_content.as_bytes())
        .expect("Failed to write to app.rc file");

    let result = embed_resource::compile("app.rc", embed_resource::NONE);
    match result {
        CompilationResult::Ok | CompilationResult::NotWindows => {
            println!("Resource compiled successfully");
        }
        CompilationResult::Failed(..) | CompilationResult::NotAttempted(..) => {
            panic!("Failed to compile resource");
        }
    }
}
