pub mod editor;
pub mod plugin;
pub mod proxy;

vst::plugin_main!(proxy::ProxyPlugin);
