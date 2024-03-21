use std::collections::HashMap;
use base64::prelude::*;
use ahqstore_types::{AHQStoreApplication, AppRepo, DownloadUrl, InstallType, InstallerFormat, InstallerOptions, InstallerOptionsWin32};
use serde_json::to_string;

fn main() {
    let app = AHQStoreApplication {
        appDisplayName: "Firefox Web Browser".into(),
        appId: "nmCvSrMD3BMR9E2V0Opl6edlfU6Q3S5".into(),
        appShortcutName: "".into(),
        authorId: "1".into(),
        description: format!(r#"Mozilla Firefox is a free, open-source browser that allows users to access the web on Windows, Mac, Linux, Android, and iOS. It was first released in November 2004 and is developed by the Mozilla Foundation and Mozilla Corporation. Firefox is customizable with themes, plug-ins, and add-ons, and has a large collection of add-ons available. 
Redistributed by AHQ Store (ahqsecret@gmail.com)"#),
        displayImages: vec![],
        downloadUrls: {
            let mut val = HashMap::new();
            val.insert(1, DownloadUrl {
                installerType: InstallerFormat::WindowsInstallerMsi,
                url: "https://download.mozilla.org/?product=firefox-msi-latest-ssl&os=win64&lang=en-US".into()
            });
            
            val
        },
        icon: BASE64_STANDARD.encode(
            std::fs::read("./firefox.png").unwrap()
        ),
        install: InstallerOptions {
            installType: InstallType::Computer,
            linux: None,
            win32: Some(InstallerOptionsWin32 {
                assetId: 1,
                deps: None,
                exec: None,
                installerArgs: None
            })
        },
        repo: AppRepo {
            author: "ahqstore".into(),
            repo: "redistributed_apps".into()
        },
        site: Some("https://www.mozilla.org/en-US/firefox/".into()),
        source: Some("Mozilla Org".into()),
        version: "latest".into()
    };

    std::fs::write("./firefox.json", to_string(&app).unwrap()).unwrap();
}
