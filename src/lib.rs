use external_contracts::{
    declare_app_extend,
    plugin::{Plugin, PluginContext},
};

#[derive(Debug, Default)]
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }

    fn run(&self, ctx: &PluginContext) {
        println!(
            "Running {}! Data from the context: {:?}",
            self.name(),
            ctx.data_example
        );
    }
}

declare_app_extend!(MyPlugin, MyPlugin::default);
