mod product_cli {
    pub(super) fn run_product() {
        main();
    }

    include!("main.rs");
}

fn main() {
    if ores_clis_core::self_update::self_update_requested() {
        ores_clis_core::self_update::run_self_update_cli(
            ores_clis_core::self_update::SelfUpdateConfig::new(
                "pony-expres",
                "poex-desktop-cli",
                "poex-desktop-cli",
                env!("CARGO_PKG_VERSION"),
            ),
        );
    }

    product_cli::run_product();
}
