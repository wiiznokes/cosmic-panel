use cosmic_config::CosmicConfigEntry;
use cosmic_panel_config::CosmicPanelConfig;

fn main() {
    fn inner(appid: &str, version: u64) {
        let string = configurator_schema::SchemaGenerator::new()
            .format(configurator_schema::ConfigFormat::CosmicRon)
            .source_home_path(&format!(".config/cosmic/{}/v{}", appid, version))
            .source_paths(&[format!("/usr/share/cosmic/{}/v{}", appid, version)])
            .write_path(&format!("configurator/configurator/test_configs/{}", appid))
            .generate::<CosmicPanelConfig>()
            .unwrap();
        std::fs::write(
            &format!("../../cosmic_compat/schemas/{}.json", appid),
            &string,
        )
        .unwrap();
    }

    let appid = "com.system76.CosmicPanel.Dock";
    let version = CosmicPanelConfig::VERSION;

    inner(appid, version);
    let appid = "com.system76.CosmicPanel.Panel";

    inner(appid, version);
}
