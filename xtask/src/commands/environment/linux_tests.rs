use std::collections::BTreeMap;
use std::io::Cursor;

use oxyflut_qualification::environment::{InventoryValue, MissingReason};

use super::{
    PROTOCOL_COMMAND_OUTPUT_LIMIT, WAYLAND_PROTOCOL_INTERFACES, capture_response, read_bounded,
    read_bounded_prefix, wayland_protocol_version, x11_protocol_version,
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
    response.push_str(&"x".repeat(PROTOCOL_COMMAND_OUTPUT_LIMIT));

    let captured = read_bounded_prefix(Cursor::new(response), PROTOCOL_COMMAND_OUTPUT_LIMIT)
        .map_err(|reason| {
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
fn oversized_x11_response_reports_inventory_exceeds_bound() -> Result<(), Box<dyn std::error::Error>>
{
    let mut response = "version number: 11.0\n".to_owned();
    response.push_str(&"x".repeat(PROTOCOL_COMMAND_OUTPUT_LIMIT));

    let captured = read_bounded_prefix(Cursor::new(response), PROTOCOL_COMMAND_OUTPUT_LIMIT)
        .map_err(|reason| {
            std::io::Error::other(format!("could not capture protocol prefix: {reason:?}"))
        })?;
    assert!(captured.truncated);
    assert!(matches!(
        x11_protocol_version(
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
fn partial_wayland_interface_observation_is_preserved() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        wayland_protocol_version(
            Some("interface: 'wl_compositor', version: 1, name: 1\n"),
            MissingReason::ManualCapture,
            false,
        )
        .observed_value(),
        Some("wayland-wl_compositor-1")
    );
    Ok(())
}

#[test]
fn fixture_sized_wayland_response_keeps_its_protocol_value()
-> Result<(), Box<dyn std::error::Error>> {
    let captured = read_bounded_prefix(
        Cursor::new(wayland_protocol_fixture()),
        PROTOCOL_COMMAND_OUTPUT_LIMIT,
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
