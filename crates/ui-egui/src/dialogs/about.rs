use super::Dialog;

/// Help ▸ About Atelier Ledger: the app, its contributors and the AI models that helped. The credits
/// are compiled in (`crate::credits`, docs/contributors.md).
pub(super) fn about(ui: &mut egui::Ui, d: &mut Dialog) {
    ui.horizontal(|ui| {
        for tab in ["About", "Contributors", "Models"] {
            if ui.selectable_label(d.tab == tab, tab).clicked() {
                d.tab = tab.to_string();
            }
        }
    });
    ui.separator();
    match d.tab.as_str() {
        "Contributors" => crate::credits::contributors_ui(ui),
        "Models" => crate::credits::models_ui(ui),
        _ => {
            ui.heading("Atelier Ledger");
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.add_space(4.0);
            ui.label("The Nation's spreadsheet. Part of Atelier, built on GridCraft by the ArtCraft team.");
            ui.hyperlink_to("aetherionnnation.com/atelier/ledger", "https://www.aetherionnnation.com/atelier/ledger");
            ui.hyperlink_to("Join the Nation", "https://www.aetherionnnation.com");
            ui.hyperlink_to("Source on GitHub", "https://github.com/Aetherion-Nation-Builders/atelier-ledger");
        }
    }
}
