//! Integration tests for SigmaOS-display-hotplug.

use sigmaos_display_hotplug_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
