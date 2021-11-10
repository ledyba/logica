pub mod gui;
pub mod proxy;
pub mod plugin;

vst::plugin_main!(proxy::ProxyPlugin);
