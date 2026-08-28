use std::collections::BTreeMap;
use std::io::Cursor;

use oxyflut_qualification::environment::{InventoryValue, MissingReason};

use super::{
    COMMAND_OUTPUT_LIMIT, WAYLAND_PROTOCOL_INTERFACES, capture_response, read_bounded,
    read_bounded_prefix, wayland_protocol_version,
};

#[test]
fn bounded_source_failures_remain_explicit_for_live_response_callers() {
    let mut failures = BTreeMap::new();
    let captured = capture_response(
        read_bounded(Cursor::new(b"12345"), 4).map(Some),
        "wayland_info",
        &mut failures,
    );

    assert!(captured.is_none());
    assert_eq!(
        failures.get("wayland_info"),
        Some(&MissingReason::InventoryExceedsBound)
    );
}

#[test]
fn oversized_wayland_response_reports_inventory_exceeds_bound()
-> Result<(), Box<dyn std::error::Error>> {
    let mut response = wayland_protocol_fixture();
    for index in 0..256 {
        response.push_str(&format!(
            "interface: 'unrelated_interface_{index}', version: 1, name: {}\n  detail: bounded fixture data\n",
            index + 32
        ));
    }

    let captured =
        read_bounded_prefix(Cursor::new(response), COMMAND_OUTPUT_LIMIT).map_err(|reason| {
            std::io::Error::other(format!("could not capture protocol prefix: {reason:?}"))
        })?;
    assert!(captured.truncated);
    assert!(matches!(
        wayland_protocol_version(
            Some(&captured.contents),
            MissingReason::ManualCapture,
            captured.truncated,
        ),
        InventoryValue::Missing {
            reason: MissingReason::InventoryExceedsBound
        }
    ));
    Ok(())
}

#[test]
fn fixture_sized_wayland_response_keeps_its_protocol_value()
-> Result<(), Box<dyn std::error::Error>> {
    let captured = read_bounded_prefix(
        Cursor::new(wayland_protocol_fixture()),
        COMMAND_OUTPUT_LIMIT,
    )
    .map_err(|reason| std::io::Error::other(format!("could not capture fixture: {reason:?}")))?;
    assert!(!captured.truncated);
    assert_eq!(
        wayland_protocol_version(
            Some(&captured.contents),
            MissingReason::ManualCapture,
            captured.truncated,
        )
        .observed_value(),
        Some(
            "wayland-wl_compositor-1-wl_shm-2-wl_seat-3-wl_output-4-xdg_wm_base-5-zwp_linux_dmabuf_v1-6-wp_viewporter-7-wp_fractional_scale_manager_v1-8"
        )
    );
    Ok(())
}

fn wayland_protocol_fixture() -> String {
    WAYLAND_PROTOCOL_INTERFACES
        .iter()
        .enumerate()
        .map(|(index, interface)| {
            format!(
                "interface: '{interface}', version: {}, name: {index}\n",
                index + 1
            )
        })
        .collect()
}
