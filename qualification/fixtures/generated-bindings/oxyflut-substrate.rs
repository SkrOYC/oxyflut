pub const OXY_SUBSTRATE_ABI_VERSION: u32 = 10;
pub type OxyStatus = u32;
pub const OXY_STATUS_OK: _bindgen_ty_1 = 0;
pub const OXY_STATUS_INVALID_ARGUMENT: _bindgen_ty_1 = 1;
pub const OXY_STATUS_INCOMPATIBLE_ABI: _bindgen_ty_1 = 2;
pub const OXY_STATUS_STALE_OWNER: _bindgen_ty_1 = 3;
pub const OXY_STATUS_RESOURCE_LIMIT: _bindgen_ty_1 = 4;
pub const OXY_STATUS_UNSUPPORTED: _bindgen_ty_1 = 5;
pub const OXY_STATUS_SUBSTRATE_FAILURE: _bindgen_ty_1 = 6;
pub const OXY_STATUS_CANCELLED: _bindgen_ty_1 = 7;
pub const OXY_STATUS_DEADLINE_EXCEEDED: _bindgen_ty_1 = 8;
pub type _bindgen_ty_1 = core::ffi::c_uint;
pub type OxyPlatformEventKind = u32;
pub const OXY_PLATFORM_EVENT_POINTER: _bindgen_ty_2 = 1;
pub const OXY_PLATFORM_EVENT_KEY: _bindgen_ty_2 = 2;
pub const OXY_PLATFORM_EVENT_IME: _bindgen_ty_2 = 3;
pub const OXY_PLATFORM_EVENT_LIFECYCLE: _bindgen_ty_2 = 4;
pub const OXY_PLATFORM_EVENT_IME_REQUEST: _bindgen_ty_2 = 5;
pub type _bindgen_ty_2 = core::ffi::c_uint;
pub type OxyPlatformServiceKind = u32;
pub const OXY_PLATFORM_SERVICE_DIALOG: _bindgen_ty_3 = 1;
pub const OXY_PLATFORM_SERVICE_CLIPBOARD_READ: _bindgen_ty_3 = 2;
pub const OXY_PLATFORM_SERVICE_CLIPBOARD_WRITE: _bindgen_ty_3 = 3;
pub const OXY_PLATFORM_SERVICE_MESSAGE: _bindgen_ty_3 = 4;
pub type _bindgen_ty_3 = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySubstrate {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyView {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySceneBuilder {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyScene {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyTexture {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyColorSource {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyColorFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyImageFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyParagraphBuilder {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyParagraph {
    _unused: [u8; 0],
}
pub type OxyNativeTextIndexUnit = u32;
pub const OXY_NATIVE_TEXT_INDEX_UTF8_BYTES: _bindgen_ty_4 = 1;
pub const OXY_NATIVE_TEXT_INDEX_UTF16_UNITS: _bindgen_ty_4 = 2;
pub const OXY_NATIVE_TEXT_INDEX_UNICODE_SCALARS: _bindgen_ty_4 = 3;
pub type _bindgen_ty_4 = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyBorrowedBytes {
    pub data: *const u8,
    pub length: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyOwnedBytes {
    pub data: *mut u8,
    pub length: u64,
    pub release: ::core::option::Option<
        unsafe extern "C" fn(user_data: *mut core::ffi::c_void, data: *mut u8, length: u64),
    >,
    pub user_data: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyPoint {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyTransform {
    pub column_major: [f32; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyPathData {
    pub struct_size: u32,
    pub abi_version: u32,
    pub verbs: *const u8,
    pub verb_count: u64,
    pub points: *const OxyPoint,
    pub point_count: u64,
    pub fill_type: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyPaint {
    pub struct_size: u32,
    pub abi_version: u32,
    pub color: OxyColor,
    pub stroke_width: f32,
    pub miter_limit: f32,
    pub draw_style: u32,
    pub stroke_cap: u32,
    pub stroke_join: u32,
    pub blend_mode: u32,
    pub color_source: *mut OxyColorSource,
    pub color_filter: *mut OxyColorFilter,
    pub image_filter: *mut OxyImageFilter,
}
pub type OxyPixelFormat = u32;
pub const OXY_PIXEL_FORMAT_RGBA8888: _bindgen_ty_5 = 1;
pub type _bindgen_ty_5 = core::ffi::c_uint;
pub type OxyAlphaType = u32;
pub const OXY_ALPHA_TYPE_PREMULTIPLIED: _bindgen_ty_6 = 1;
pub type _bindgen_ty_6 = core::ffi::c_uint;
pub type OxyColorSpace = u32;
pub const OXY_COLOR_SPACE_SRGB: _bindgen_ty_7 = 1;
pub type _bindgen_ty_7 = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyRasterDescriptor {
    pub struct_size: u32,
    pub abi_version: u32,
    pub width: u32,
    pub height: u32,
    pub row_bytes: u32,
    pub pixel_format: OxyPixelFormat,
    pub alpha_type: OxyAlphaType,
    pub color_space: OxyColorSpace,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyHeadlessMetrics {
    pub struct_size: u32,
    pub abi_version: u32,
    pub logical_width: f64,
    pub logical_height: f64,
    pub physical_width: u32,
    pub physical_height: u32,
    pub device_pixel_ratio: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyTextStyle {
    pub struct_size: u32,
    pub abi_version: u32,
    pub color: OxyColor,
    pub font_size: f32,
    pub font_weight: u32,
    pub font_style: u32,
    pub font_family_utf8: OxyBorrowedBytes,
    pub locale_utf8: OxyBorrowedBytes,
}
pub type OxyTextIndexUnit = u32;
pub const OXY_TEXT_INDEX_UTF8_BYTES: _bindgen_ty_8 = 1;
pub const OXY_TEXT_INDEX_UTF16_UNITS: _bindgen_ty_8 = 2;
pub const OXY_TEXT_INDEX_GRAPHEME: _bindgen_ty_8 = 3;
pub const OXY_TEXT_INDEX_LOGICAL: _bindgen_ty_8 = 4;
pub type _bindgen_ty_8 = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyTextIndex {
    pub unit: OxyTextIndexUnit,
    pub reserved: u32,
    pub offset: u64,
}
pub type OxyTextAffinity = u32;
pub const OXY_TEXT_AFFINITY_UPSTREAM: _bindgen_ty_9 = 1;
pub const OXY_TEXT_AFFINITY_DOWNSTREAM: _bindgen_ty_9 = 2;
pub type _bindgen_ty_9 = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyTextHit {
    pub index: OxyTextIndex,
    pub affinity: OxyTextAffinity,
    pub grapheme_boundary: u32,
    pub inside: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyTextBox {
    pub rect: OxyRect,
    pub direction: u32,
}
pub type OxySemanticsRelationKind = u32;
pub const OXY_SEMANTICS_RELATION_LABELLED_BY: _bindgen_ty_10 = 1;
pub const OXY_SEMANTICS_RELATION_LABELS: _bindgen_ty_10 = 2;
pub const OXY_SEMANTICS_RELATION_DESCRIBED_BY: _bindgen_ty_10 = 3;
pub const OXY_SEMANTICS_RELATION_DESCRIBES: _bindgen_ty_10 = 4;
pub const OXY_SEMANTICS_RELATION_CONTROLS: _bindgen_ty_10 = 5;
pub const OXY_SEMANTICS_RELATION_CONTROLLED_BY: _bindgen_ty_10 = 6;
pub const OXY_SEMANTICS_RELATION_FLOWS_TO: _bindgen_ty_10 = 7;
pub const OXY_SEMANTICS_RELATION_FLOWS_FROM: _bindgen_ty_10 = 8;
pub const OXY_SEMANTICS_RELATION_MEMBER_OF: _bindgen_ty_10 = 9;
pub const OXY_SEMANTICS_RELATION_OWNS: _bindgen_ty_10 = 10;
pub const OXY_SEMANTICS_RELATION_ERROR_MESSAGE: _bindgen_ty_10 = 11;
pub const OXY_SEMANTICS_RELATION_DETAILS: _bindgen_ty_10 = 12;
pub const OXY_SEMANTICS_RELATION_DETAILS_FOR: _bindgen_ty_10 = 13;
pub const OXY_SEMANTICS_RELATION_ACTIVE_DESCENDANT: _bindgen_ty_10 = 14;
pub type _bindgen_ty_10 = core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySemanticsRelation {
    pub kind: OxySemanticsRelationKind,
    pub reserved: u32,
    pub target_generation: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySemanticsNode {
    pub struct_size: u32,
    pub abi_version: u32,
    pub node_generation: u64,
    pub flags: u64,
    pub actions: u64,
    pub role: u32,
    pub live_region: u32,
    pub bounds: OxyRect,
    pub transform: OxyTransform,
    pub label_utf8: OxyBorrowedBytes,
    pub accessible_name_utf8: OxyBorrowedBytes,
    pub description_utf8: OxyBorrowedBytes,
    pub value_utf8: OxyBorrowedBytes,
    pub hint_utf8: OxyBorrowedBytes,
    pub help_utf8: OxyBorrowedBytes,
    pub tooltip_utf8: OxyBorrowedBytes,
    pub attributed_text_index_unit: OxyNativeTextIndexUnit,
    pub attributed_text_reserved: u32,
    pub attributed_text_segments: *const OxyImeTextSegment,
    pub attributed_text_segment_count: u64,
    pub identifier_utf8: OxyBorrowedBytes,
    pub language_bcp47_utf8: OxyBorrowedBytes,
    pub traversal_children: *const u64,
    pub traversal_child_count: u64,
    pub hit_test_children: *const u64,
    pub hit_test_child_count: u64,
    pub relations: *const OxySemanticsRelation,
    pub relation_count: u64,
    pub text_selection_base_utf16: i64,
    pub text_selection_extent_utf16: i64,
    pub has_text_selection: u32,
    pub text_selection_reserved: u32,
    pub text_layout_generation: u64,
    pub has_text_layout: u32,
    pub text_layout_reserved: u32,
    pub scroll_position: f64,
    pub scroll_minimum: f64,
    pub scroll_maximum: f64,
    pub has_scroll: u32,
    pub heading_level: u32,
    pub text_direction: u32,
    pub has_input_focus: u32,
    pub has_accessibility_focus: u32,
    pub is_hidden: u32,
    pub is_disabled: u32,
    pub is_secure_field: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySemanticsUpdate {
    pub struct_size: u32,
    pub abi_version: u32,
    pub view_generation: u64,
    pub nodes: *const OxySemanticsNode,
    pub node_count: u64,
    pub deleted_node_generations: *const u64,
    pub deleted_node_count: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyViewMetrics {
    pub struct_size: u32,
    pub abi_version: u32,
    pub logical_width: f64,
    pub logical_height: f64,
    pub device_pixel_ratio: f64,
    pub display_id: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyFrameOpportunity {
    pub struct_size: u32,
    pub abi_version: u32,
    pub view_generation: u64,
    pub display_epoch: u64,
    pub monotonic_time_ns: u64,
    pub target_time_ns: u64,
    pub interval_ns: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyPointerEvent {
    pub pointer_generation: u64,
    pub monotonic_time_ns: u64,
    pub x: f64,
    pub y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
    pub buttons: u64,
    pub modifiers: u64,
    pub phase: u32,
    pub device_kind: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyKeyEvent {
    pub monotonic_time_ns: u64,
    pub modifiers: u64,
    pub physical_key: u32,
    pub logical_key: u32,
    pub action: u32,
    pub repeat: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyImeTextSegment {
    pub start: i64,
    pub end: i64,
    pub attributes: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyImeEvent {
    pub transaction_generation: u64,
    pub kind: u32,
    pub native_index_unit: OxyNativeTextIndexUnit,
    pub replacement_start: i64,
    pub replacement_end: i64,
    pub selection_start: i64,
    pub selection_end: i64,
    pub marked_start: i64,
    pub marked_end: i64,
    pub candidate_rect: OxyRect,
    pub action: u32,
    pub input_context: u32,
    pub sensitive_field: u32,
    pub reserved: u32,
    pub segments: *const OxyImeTextSegment,
    pub segment_count: u64,
    pub text_utf8: OxyBorrowedBytes,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyImeRequest {
    pub request_generation: u64,
    pub kind: u32,
    pub native_index_unit: OxyNativeTextIndexUnit,
    pub range_start: i64,
    pub range_end: i64,
    pub point: OxyPoint,
    pub maximum_units: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyImeResponse {
    pub struct_size: u32,
    pub abi_version: u32,
    pub text_range_start: i64,
    pub text_range_end: i64,
    pub selection_start: i64,
    pub selection_end: i64,
    pub marked_start: i64,
    pub marked_end: i64,
    pub text_rect: OxyRect,
    pub character_index: i64,
    pub input_context: u32,
    pub sensitive_field: u32,
    pub text_utf8: OxyBorrowedBytes,
    pub segments: *const OxyImeTextSegment,
    pub segment_count: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyLifecycleEvent {
    pub kind: u32,
    pub reserved: u32,
    pub display_id: u64,
    pub metrics: OxyViewMetrics,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union OxyPlatformEventPayload {
    pub pointer: OxyPointerEvent,
    pub key: OxyKeyEvent,
    pub ime: OxyImeEvent,
    pub ime_request: OxyImeRequest,
    pub lifecycle: OxyLifecycleEvent,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OxyPlatformEvent {
    pub struct_size: u32,
    pub abi_version: u32,
    pub view_generation: u64,
    pub kind: OxyPlatformEventKind,
    pub reserved: u32,
    pub payload: OxyPlatformEventPayload,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyPlatformServiceRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub request_generation: u64,
    pub view_generation: u64,
    pub service_kind: OxyPlatformServiceKind,
    pub payload_version: u32,
    pub payload: OxyBorrowedBytes,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySceneSubmission {
    pub struct_size: u32,
    pub abi_version: u32,
    pub view_generation: u64,
    pub frame_generation: u64,
    pub target_time_ns: u64,
    pub scene: *mut OxyScene,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxyRecoveryRequest {
    pub struct_size: u32,
    pub abi_version: u32,
    pub view_generation: u64,
    pub fault_kind: u32,
    pub attempt: u32,
    pub deadline_ns: u64,
    pub transient_memory_cap_bytes: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySubstrateCallbacks {
    pub struct_size: u32,
    pub abi_version: u32,
    pub user_data: *mut core::ffi::c_void,
    pub on_wakeup: ::core::option::Option<unsafe extern "C" fn(user_data: *mut core::ffi::c_void)>,
    pub on_frame_opportunity: ::core::option::Option<
        unsafe extern "C" fn(
            user_data: *mut core::ffi::c_void,
            opportunity: *const OxyFrameOpportunity,
        ),
    >,
    pub on_presentation: ::core::option::Option<
        unsafe extern "C" fn(
            user_data: *mut core::ffi::c_void,
            view_generation: u64,
            frame_generation: u64,
            presentation_time_ns: u64,
            status: OxyStatus,
        ),
    >,
    pub on_platform_event: ::core::option::Option<
        unsafe extern "C" fn(user_data: *mut core::ffi::c_void, event: *const OxyPlatformEvent),
    >,
    pub on_platform_response: ::core::option::Option<
        unsafe extern "C" fn(
            user_data: *mut core::ffi::c_void,
            request_generation: u64,
            status: OxyStatus,
            payload: OxyBorrowedBytes,
        ),
    >,
    pub on_semantics_action: ::core::option::Option<
        unsafe extern "C" fn(
            user_data: *mut core::ffi::c_void,
            request_generation: u64,
            view_generation: u64,
            node_generation: u64,
            action: u32,
            payload: OxyBorrowedBytes,
        ),
    >,
    pub on_log: ::core::option::Option<
        unsafe extern "C" fn(
            user_data: *mut core::ffi::c_void,
            level: u32,
            content_free_message: OxyBorrowedBytes,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySubstrateConfig {
    pub struct_size: u32,
    pub abi_version: u32,
    pub callbacks: OxySubstrateCallbacks,
    pub maximum_buffer_bytes: u64,
    pub maximum_texture_bytes: u64,
    pub flags: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OxySubstrateApi {
    pub struct_size: u32,
    pub abi_version: u32,
    pub get_abi_version: ::core::option::Option<unsafe extern "C" fn() -> u32>,
    pub create: ::core::option::Option<
        unsafe extern "C" fn(
            config: *const OxySubstrateConfig,
            out_substrate: *mut *mut OxySubstrate,
        ) -> OxyStatus,
    >,
    pub destroy: ::core::option::Option<unsafe extern "C" fn(substrate: *mut OxySubstrate)>,
    pub create_view: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            view_generation: u64,
            metrics: *const OxyViewMetrics,
            out_view: *mut *mut OxyView,
        ) -> OxyStatus,
    >,
    pub destroy_view: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            view_generation: u64,
            view: *mut OxyView,
        ) -> OxyStatus,
    >,
    pub update_view_metrics: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            view: *mut OxyView,
            metrics: *const OxyViewMetrics,
        ) -> OxyStatus,
    >,
    pub create_scene_builder: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            out_builder: *mut *mut OxySceneBuilder,
        ) -> OxyStatus,
    >,
    pub scene_builder_save:
        ::core::option::Option<unsafe extern "C" fn(builder: *mut OxySceneBuilder) -> OxyStatus>,
    pub scene_builder_restore:
        ::core::option::Option<unsafe extern "C" fn(builder: *mut OxySceneBuilder) -> OxyStatus>,
    pub scene_builder_transform: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            transform: *const OxyTransform,
        ) -> OxyStatus,
    >,
    pub scene_builder_clip_rect: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            rect: *const OxyRect,
            clip_operation: u32,
        ) -> OxyStatus,
    >,
    pub scene_builder_clip_path: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            path: *const OxyPathData,
            clip_operation: u32,
        ) -> OxyStatus,
    >,
    pub scene_builder_draw_rect: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            rect: *const OxyRect,
            paint: *const OxyPaint,
        ) -> OxyStatus,
    >,
    pub scene_builder_draw_path: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            path: *const OxyPathData,
            paint: *const OxyPaint,
        ) -> OxyStatus,
    >,
    pub scene_builder_draw_texture: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            texture: *mut OxyTexture,
            destination: *const OxyRect,
            paint: *const OxyPaint,
        ) -> OxyStatus,
    >,
    pub scene_builder_draw_scene: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            scene: *mut OxyScene,
            transform: *const OxyTransform,
        ) -> OxyStatus,
    >,
    pub scene_builder_begin_layer: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            bounds: *const OxyRect,
            backdrop_filter: *mut OxyImageFilter,
            paint: *const OxyPaint,
        ) -> OxyStatus,
    >,
    pub scene_builder_end_layer:
        ::core::option::Option<unsafe extern "C" fn(builder: *mut OxySceneBuilder) -> OxyStatus>,
    pub scene_builder_build: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            out_scene: *mut *mut OxyScene,
        ) -> OxyStatus,
    >,
    pub destroy_scene_builder:
        ::core::option::Option<unsafe extern "C" fn(builder: *mut OxySceneBuilder)>,
    pub retain_scene: ::core::option::Option<unsafe extern "C" fn(scene: *mut OxyScene)>,
    pub release_scene: ::core::option::Option<unsafe extern "C" fn(scene: *mut OxyScene)>,
    pub create_linear_gradient: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            start: OxyPoint,
            end: OxyPoint,
            colors: *const OxyColor,
            stops: *const f32,
            stop_count: u64,
            out_source: *mut *mut OxyColorSource,
        ) -> OxyStatus,
    >,
    pub release_color_source:
        ::core::option::Option<unsafe extern "C" fn(source: *mut OxyColorSource)>,
    pub create_color_matrix_filter: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            row_major: *const f32,
            out_filter: *mut *mut OxyColorFilter,
        ) -> OxyStatus,
    >,
    pub release_color_filter:
        ::core::option::Option<unsafe extern "C" fn(filter: *mut OxyColorFilter)>,
    pub create_blur_filter: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            sigma_x: f32,
            sigma_y: f32,
            tile_mode: u32,
            out_filter: *mut *mut OxyImageFilter,
        ) -> OxyStatus,
    >,
    pub release_image_filter:
        ::core::option::Option<unsafe extern "C" fn(filter: *mut OxyImageFilter)>,
    pub register_font: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            resource_generation: u64,
            font_bytes: OxyBorrowedBytes,
        ) -> OxyStatus,
    >,
    pub create_paragraph_builder: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            text_direction: u32,
            out_builder: *mut *mut OxyParagraphBuilder,
        ) -> OxyStatus,
    >,
    pub paragraph_builder_push_style: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxyParagraphBuilder,
            style: *const OxyTextStyle,
        ) -> OxyStatus,
    >,
    pub paragraph_builder_pop_style: ::core::option::Option<
        unsafe extern "C" fn(builder: *mut OxyParagraphBuilder) -> OxyStatus,
    >,
    pub paragraph_builder_add_text: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxyParagraphBuilder,
            utf8_text: OxyBorrowedBytes,
        ) -> OxyStatus,
    >,
    pub paragraph_builder_build: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxyParagraphBuilder,
            out_paragraph: *mut *mut OxyParagraph,
        ) -> OxyStatus,
    >,
    pub destroy_paragraph_builder:
        ::core::option::Option<unsafe extern "C" fn(builder: *mut OxyParagraphBuilder)>,
    pub paragraph_layout: ::core::option::Option<
        unsafe extern "C" fn(paragraph: *mut OxyParagraph, maximum_width: f32) -> OxyStatus,
    >,
    pub paragraph_get_caret_rect: ::core::option::Option<
        unsafe extern "C" fn(
            paragraph: *mut OxyParagraph,
            position: OxyTextIndex,
            affinity: OxyTextAffinity,
            out_rect: *mut OxyRect,
        ) -> OxyStatus,
    >,
    pub paragraph_get_range_boxes: ::core::option::Option<
        unsafe extern "C" fn(
            paragraph: *mut OxyParagraph,
            start: OxyTextIndex,
            end: OxyTextIndex,
            output: *mut OxyTextBox,
            output_capacity: u64,
            out_required: *mut u64,
        ) -> OxyStatus,
    >,
    pub paragraph_hit_test: ::core::option::Option<
        unsafe extern "C" fn(
            paragraph: *mut OxyParagraph,
            point: OxyPoint,
            out_hit: *mut OxyTextHit,
        ) -> OxyStatus,
    >,
    pub paragraph_convert_index: ::core::option::Option<
        unsafe extern "C" fn(
            paragraph: *mut OxyParagraph,
            index: OxyTextIndex,
            target_unit: OxyTextIndexUnit,
            out_index: *mut OxyTextIndex,
        ) -> OxyStatus,
    >,
    pub scene_builder_draw_paragraph: ::core::option::Option<
        unsafe extern "C" fn(
            builder: *mut OxySceneBuilder,
            paragraph: *mut OxyParagraph,
            origin: OxyPoint,
        ) -> OxyStatus,
    >,
    pub release_paragraph:
        ::core::option::Option<unsafe extern "C" fn(paragraph: *mut OxyParagraph)>,
    pub submit_scene: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            view: *mut OxyView,
            submission: *const OxySceneSubmission,
        ) -> OxyStatus,
    >,
    pub render_headless: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            scene: *mut OxyScene,
            metrics: *const OxyHeadlessMetrics,
            out_pixels: *mut OxyOwnedBytes,
            out_descriptor: *mut OxyRasterDescriptor,
        ) -> OxyStatus,
    >,
    pub realize_texture: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            resource_generation: u64,
            width: u32,
            height: u32,
            pixel_format: u32,
            pixels: OxyBorrowedBytes,
            out_texture: *mut *mut OxyTexture,
        ) -> OxyStatus,
    >,
    pub retain_texture: ::core::option::Option<unsafe extern "C" fn(texture: *mut OxyTexture)>,
    pub release_texture: ::core::option::Option<unsafe extern "C" fn(texture: *mut OxyTexture)>,
    pub update_semantics: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            update: *const OxySemanticsUpdate,
        ) -> OxyStatus,
    >,
    pub respond_semantics_action: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            request_generation: u64,
            result: OxyStatus,
        ) -> OxyStatus,
    >,
    pub request_platform_service: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            request: *const OxyPlatformServiceRequest,
        ) -> OxyStatus,
    >,
    pub cancel_platform_service: ::core::option::Option<
        unsafe extern "C" fn(substrate: *mut OxySubstrate, request_generation: u64) -> OxyStatus,
    >,
    pub respond_ime_request: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            request_generation: u64,
            status: OxyStatus,
            response: *const OxyImeResponse,
        ) -> OxyStatus,
    >,
    pub recover: ::core::option::Option<
        unsafe extern "C" fn(
            substrate: *mut OxySubstrate,
            view: *mut OxyView,
            request: *const OxyRecoveryRequest,
        ) -> OxyStatus,
    >,
    pub pump_platform_tasks: ::core::option::Option<
        unsafe extern "C" fn(substrate: *mut OxySubstrate, monotonic_time_ns: u64) -> OxyStatus,
    >,
    pub begin_shutdown:
        ::core::option::Option<unsafe extern "C" fn(substrate: *mut OxySubstrate) -> OxyStatus>,
    pub drain: ::core::option::Option<
        unsafe extern "C" fn(substrate: *mut OxySubstrate, monotonic_deadline_ns: u64) -> OxyStatus,
    >,
}
unsafe extern "C" {
    pub fn OxySubstrateGetApi(
        requested_abi_version: u32,
        caller_struct_size: u32,
        out_api: *mut OxySubstrateApi,
    ) -> OxyStatus;
}
