//! Config for cosmic-panel
#[cfg(feature = "wayland-rs")]
mod container_config;
mod panel_config;

#[cfg(feature = "wayland-rs")]
pub use container_config::*;
pub use panel_config::*;

#[cfg(feature = "schema")]
#[test]
fn gen_schema() {
    fn gen(appid: &str, version: i32) {
        let string = configurator_schema::gen_schema::<CosmicPanelConfig>()
            .format(configurator_schema::ConfigFormat::CosmicRon)
            .source_home_path(&format!(".config/cosmic/{}/v{}", appid, version))
            .source_paths(&[&format!("/usr/share/cosmic/{}/v{}", appid, version)])
            .call()
            .unwrap();
        std::fs::write(&format!("../../schemas/{}.json", appid), &string).unwrap();
    }

    let appid = "com.system76.CosmicPanel.Dock";
    let version = 1;

    gen(appid, version);
    let appid = "com.system76.CosmicPanel.Panel";

    gen(appid, version);
}
