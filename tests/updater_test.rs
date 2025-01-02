use app_updater::{start_update, UpdateInfo};

#[test]
fn test_start_update() {
    let info = start_update(&UpdateInfo {
        download_path: r#"C:\Users\ms\AppData\Local\Temp\tmp-29532-nKxrP7HxCIMu"#.to_string(),
        resource_path: r#"G:\personal\ra3-toolkits-vue\resources"#.to_string(),
        electron_exe: r#"G:\personal\ra3-toolkits-vue\node_modules\electron\dist\electron.exe"#
            .to_string(),
        pid: 29532,
    });
    if let Err(e) = info {
        eprintln!("Error: {:?}", e);
    }
}
