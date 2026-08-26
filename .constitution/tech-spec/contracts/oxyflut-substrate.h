#ifndef OXYFLUT_SUBSTRATE_H_
#define OXYFLUT_SUBSTRATE_H_

#include <stddef.h>
#include <stdint.h>

#if defined(_WIN32)
#define OXY_EXPORT __declspec(dllexport)
#define OXY_CALL __cdecl
#elif defined(__GNUC__) || defined(__clang__)
#define OXY_EXPORT __attribute__((visibility("default")))
#define OXY_CALL
#else
#define OXY_EXPORT
#define OXY_CALL
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define OXY_SUBSTRATE_ABI_VERSION 3u

/* Unless a field comment states otherwise, every pointer passed to or returned from this ABI is nonnull. An array pointer is null if and only if its count is zero. Every out pointer names writable storage and is cleared before a fallible call. All opaque handles belong to the creating substrate and can be used only on the execution domain declared for that operation. */

typedef uint32_t OxyStatus;
enum {
  OXY_STATUS_OK = 0u,
  OXY_STATUS_INVALID_ARGUMENT = 1u,
  OXY_STATUS_INCOMPATIBLE_ABI = 2u,
  OXY_STATUS_STALE_OWNER = 3u,
  OXY_STATUS_RESOURCE_LIMIT = 4u,
  OXY_STATUS_UNSUPPORTED = 5u,
  OXY_STATUS_SUBSTRATE_FAILURE = 6u,
  OXY_STATUS_CANCELLED = 7u,
  OXY_STATUS_DEADLINE_EXCEEDED = 8u
};

typedef uint32_t OxyPlatformEventKind;
enum {
  OXY_PLATFORM_EVENT_POINTER = 1u,
  OXY_PLATFORM_EVENT_KEY = 2u,
  OXY_PLATFORM_EVENT_IME = 3u,
  OXY_PLATFORM_EVENT_LIFECYCLE = 4u,
  OXY_PLATFORM_EVENT_IME_REQUEST = 5u
};

typedef uint32_t OxyPlatformServiceKind;
enum {
  OXY_PLATFORM_SERVICE_DIALOG = 1u,
  OXY_PLATFORM_SERVICE_CLIPBOARD_READ = 2u,
  OXY_PLATFORM_SERVICE_CLIPBOARD_WRITE = 3u,
  OXY_PLATFORM_SERVICE_MESSAGE = 4u
};

typedef struct OxySubstrate OxySubstrate;
typedef struct OxyView OxyView;
typedef struct OxySceneBuilder OxySceneBuilder;
typedef struct OxyScene OxyScene;
typedef struct OxyTexture OxyTexture;
typedef struct OxyColorSource OxyColorSource;
typedef struct OxyColorFilter OxyColorFilter;
typedef struct OxyImageFilter OxyImageFilter;
typedef struct OxyParagraphBuilder OxyParagraphBuilder;
typedef struct OxyParagraph OxyParagraph;

/* Borrowed bytes remain valid only for the containing call or callback. data is null if and only if length is zero. */
typedef struct OxyBorrowedBytes {
  const uint8_t* data;
  uint64_t length;
} OxyBorrowedBytes;

typedef struct OxyOwnedBytes {
  uint8_t* data;
  uint64_t length;
  void (OXY_CALL *release)(void* user_data, uint8_t* data, uint64_t length);
  void* user_data;
} OxyOwnedBytes;

/* For a nonempty OxyOwnedBytes value, release is nonnull and the receiver calls it exactly once. Empty values have null data, zero length, and null release. */

typedef struct OxyPoint {
  float x;
  float y;
} OxyPoint;

typedef struct OxyRect {
  float x;
  float y;
  float width;
  float height;
} OxyRect;

typedef struct OxyColor {
  float red;
  float green;
  float blue;
  float alpha;
} OxyColor;

typedef struct OxyTransform {
  float column_major[16];
} OxyTransform;

typedef struct OxyPathData {
  uint32_t struct_size;
  uint32_t abi_version;
  const uint8_t* verbs;
  uint64_t verb_count;
  const OxyPoint* points;
  uint64_t point_count;
  uint32_t fill_type;
} OxyPathData;

typedef struct OxyPaint {
  uint32_t struct_size;
  uint32_t abi_version;
  OxyColor color;
  float stroke_width;
  float miter_limit;
  uint32_t draw_style;
  uint32_t stroke_cap;
  uint32_t stroke_join;
  uint32_t blend_mode;
  OxyColorSource* color_source; /* Nullable; null selects the inline solid color. */
  OxyColorFilter* color_filter; /* Nullable; null applies no color filter. */
  OxyImageFilter* image_filter; /* Nullable; null applies no image filter. */
} OxyPaint;

/* At most one of color_filter and image_filter can be nonnull. */

typedef uint32_t OxyPixelFormat;
enum {
  OXY_PIXEL_FORMAT_RGBA8888 = 1u
};

typedef uint32_t OxyAlphaType;
enum {
  OXY_ALPHA_TYPE_PREMULTIPLIED = 1u
};

typedef uint32_t OxyColorSpace;
enum {
  OXY_COLOR_SPACE_SRGB = 1u
};

typedef struct OxyRasterDescriptor {
  uint32_t struct_size;
  uint32_t abi_version;
  uint32_t width;
  uint32_t height;
  uint32_t row_bytes;
  OxyPixelFormat pixel_format;
  OxyAlphaType alpha_type;
  OxyColorSpace color_space;
} OxyRasterDescriptor;

typedef struct OxyTextStyle {
  uint32_t struct_size;
  uint32_t abi_version;
  OxyColor color;
  float font_size;
  uint32_t font_weight;
  uint32_t font_style;
  OxyBorrowedBytes font_family_utf8;
  OxyBorrowedBytes locale_utf8;
} OxyTextStyle;

typedef struct OxyTextBox {
  OxyRect rect;
  uint32_t direction;
} OxyTextBox;

typedef struct OxySemanticsNode {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t node_generation;
  uint64_t flags;
  uint64_t actions;
  uint32_t role;
  uint32_t live_region;
  OxyRect bounds;
  OxyTransform transform;
  OxyBorrowedBytes label_utf8;
  OxyBorrowedBytes value_utf8;
  OxyBorrowedBytes hint_utf8;
  OxyBorrowedBytes tooltip_utf8;
  OxyBorrowedBytes identifier_utf8;
  OxyBorrowedBytes language_bcp47_utf8;
  const uint64_t* traversal_children;
  uint64_t traversal_child_count;
  const uint64_t* hit_test_children;
  uint64_t hit_test_child_count;
  const uint64_t* labelled_by;
  uint64_t labelled_by_count;
  const uint64_t* described_by;
  uint64_t described_by_count;
  int64_t text_selection_base_utf16;
  int64_t text_selection_extent_utf16;
  double scroll_position;
  double scroll_minimum;
  double scroll_maximum;
  uint32_t has_scroll;
  uint32_t heading_level;
  uint32_t text_direction;
  uint32_t has_input_focus;
  uint32_t has_accessibility_focus;
  uint32_t is_hidden;
  uint32_t is_disabled;
  uint32_t is_secure_field;
} OxySemanticsNode;

/* When has_scroll is zero, scroll_position, scroll_minimum, and scroll_maximum must all be zero. */

typedef struct OxySemanticsUpdate {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t view_generation;
  const OxySemanticsNode* nodes;
  uint64_t node_count;
  const uint64_t* deleted_node_generations;
  uint64_t deleted_node_count;
} OxySemanticsUpdate;

typedef struct OxyViewMetrics {
  uint32_t struct_size;
  uint32_t abi_version;
  double logical_width;
  double logical_height;
  double device_pixel_ratio;
  uint64_t display_id;
} OxyViewMetrics;

typedef struct OxyFrameOpportunity {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t view_generation;
  uint64_t display_epoch;
  uint64_t monotonic_time_ns;
  uint64_t target_time_ns;
  uint64_t interval_ns;
} OxyFrameOpportunity;

typedef struct OxyPointerEvent {
  uint64_t pointer_generation;
  uint64_t monotonic_time_ns;
  double x;
  double y;
  double delta_x;
  double delta_y;
  uint64_t buttons;
  uint64_t modifiers;
  uint32_t phase;
  uint32_t device_kind;
} OxyPointerEvent;

typedef struct OxyKeyEvent {
  uint64_t monotonic_time_ns;
  uint64_t modifiers;
  uint32_t physical_key;
  uint32_t logical_key;
  uint32_t action;
  uint32_t repeat;
} OxyKeyEvent;

typedef struct OxyImeTextSegment {
  int64_t start;
  int64_t end;
  uint64_t attributes;
} OxyImeTextSegment;

typedef struct OxyImeEvent {
  uint64_t transaction_generation;
  uint32_t kind;
  uint32_t native_index_unit;
  int64_t replacement_start;
  int64_t replacement_end;
  int64_t selection_start;
  int64_t selection_end;
  int64_t marked_start;
  int64_t marked_end;
  OxyRect candidate_rect;
  uint32_t action;
  uint32_t input_context;
  uint32_t sensitive_field;
  uint32_t reserved;
  const OxyImeTextSegment* segments;
  uint64_t segment_count;
  OxyBorrowedBytes text_utf8;
} OxyImeEvent;

typedef struct OxyImeRequest {
  uint64_t request_generation;
  uint32_t kind;
  uint32_t native_index_unit;
  int64_t range_start;
  int64_t range_end;
  OxyPoint point;
  uint32_t maximum_units;
  uint32_t reserved;
} OxyImeRequest;

typedef struct OxyImeResponse {
  uint32_t struct_size;
  uint32_t abi_version;
  int64_t text_range_start;
  int64_t text_range_end;
  int64_t selection_start;
  int64_t selection_end;
  int64_t marked_start;
  int64_t marked_end;
  OxyRect text_rect;
  int64_t character_index;
  uint32_t input_context;
  uint32_t sensitive_field;
  OxyBorrowedBytes text_utf8;
  const OxyImeTextSegment* segments;
  uint64_t segment_count;
} OxyImeResponse;

/* Optional IME ranges and character indices use -1 for both range endpoints or for the index. Other negative values are invalid. Text and segment payloads are borrowed for the call or callback. */

typedef struct OxyLifecycleEvent {
  uint32_t kind;
  uint32_t reserved;
  uint64_t display_id;
  OxyViewMetrics metrics;
} OxyLifecycleEvent;

typedef union OxyPlatformEventPayload {
  OxyPointerEvent pointer;
  OxyKeyEvent key;
  OxyImeEvent ime;
  OxyImeRequest ime_request;
  OxyLifecycleEvent lifecycle;
} OxyPlatformEventPayload;

typedef struct OxyPlatformEvent {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t view_generation;
  OxyPlatformEventKind kind;
  uint32_t reserved;
  OxyPlatformEventPayload payload;
} OxyPlatformEvent;

typedef struct OxyPlatformServiceRequest {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t request_generation;
  uint64_t view_generation;
  OxyPlatformServiceKind service_kind;
  uint32_t payload_version;
  OxyBorrowedBytes payload;
} OxyPlatformServiceRequest;

typedef struct OxySceneSubmission {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t view_generation;
  uint64_t frame_generation;
  uint64_t target_time_ns;
  OxyScene* scene;
} OxySceneSubmission;

/* submit_scene retains submission.scene before returning OXY_STATUS_OK and releases that retained reference only after on_presentation or terminal view teardown. The submission structure itself is borrowed for the call. */

typedef struct OxyRecoveryRequest {
  uint32_t struct_size;
  uint32_t abi_version;
  uint64_t view_generation;
  uint32_t fault_kind;
  uint32_t attempt;
  uint64_t deadline_ns;
  uint64_t transient_memory_cap_bytes;
} OxyRecoveryRequest;

typedef struct OxySubstrateCallbacks {
  uint32_t struct_size;
  uint32_t abi_version;
  void* user_data;
  void (OXY_CALL *on_wakeup)(void* user_data);
  void (OXY_CALL *on_frame_opportunity)(void* user_data,
                               const OxyFrameOpportunity* opportunity);
  void (OXY_CALL *on_presentation)(void* user_data,
                          uint64_t view_generation,
                          uint64_t frame_generation,
                          uint64_t presentation_time_ns,
                          OxyStatus status);
  void (OXY_CALL *on_platform_event)(void* user_data,
                                     const OxyPlatformEvent* event);
  void (OXY_CALL *on_platform_response)(void* user_data,
                                        uint64_t request_generation,
                                        OxyStatus status,
                                        OxyBorrowedBytes payload);
  void (OXY_CALL *on_semantics_action)(void* user_data,
                              uint64_t request_generation,
                              uint64_t view_generation,
                              uint64_t node_generation,
                              uint32_t action,
                              OxyBorrowedBytes payload);
  void (OXY_CALL *on_log)(void* user_data,
                 uint32_t level,
                 OxyBorrowedBytes content_free_message);
} OxySubstrateCallbacks;

typedef struct OxySubstrateConfig {
  uint32_t struct_size;
  uint32_t abi_version;
  OxySubstrateCallbacks callbacks;
  uint64_t maximum_buffer_bytes;
  uint64_t maximum_texture_bytes;
  uint32_t flags;
} OxySubstrateConfig;

/* create copies the callback table through its declared struct_size. Every callback pointer is required for Phase 3A. Callback payloads are callback-scoped. user_data must remain valid until begin_shutdown has disabled callbacks and drain returns OXY_STATUS_OK. Callbacks can arrive on the declared host or graphics runner, never reenter another callback for the same substrate, and must only enqueue normalized work. on_wakeup asks the host event loop to call pump_platform_tasks; it must not perform product work inline. */

typedef struct OxySubstrateApi {
  uint32_t struct_size;
  uint32_t abi_version;
  uint32_t (OXY_CALL *get_abi_version)(void);
  OxyStatus (OXY_CALL *create)(const OxySubstrateConfig* config,
                      OxySubstrate** out_substrate);
  void (OXY_CALL *destroy)(OxySubstrate* substrate);
  OxyStatus (OXY_CALL *create_view)(OxySubstrate* substrate,
                           uint64_t view_generation,
                           const OxyViewMetrics* metrics,
                           OxyView** out_view);
  OxyStatus (OXY_CALL *destroy_view)(OxySubstrate* substrate,
                            uint64_t view_generation,
                            OxyView* view);
  OxyStatus (OXY_CALL *update_view_metrics)(OxySubstrate* substrate,
                                   OxyView* view,
                                   const OxyViewMetrics* metrics);
  OxyStatus (OXY_CALL *create_scene_builder)(OxySubstrate* substrate,
                                    OxySceneBuilder** out_builder);
  OxyStatus (OXY_CALL *scene_builder_save)(OxySceneBuilder* builder);
  OxyStatus (OXY_CALL *scene_builder_restore)(OxySceneBuilder* builder);
  OxyStatus (OXY_CALL *scene_builder_transform)(OxySceneBuilder* builder,
                                       const OxyTransform* transform);
  OxyStatus (OXY_CALL *scene_builder_clip_rect)(OxySceneBuilder* builder,
                                       const OxyRect* rect,
                                       uint32_t clip_operation);
  OxyStatus (OXY_CALL *scene_builder_clip_path)(OxySceneBuilder* builder,
                                               const OxyPathData* path,
                                               uint32_t clip_operation);
  OxyStatus (OXY_CALL *scene_builder_draw_rect)(OxySceneBuilder* builder,
                                       const OxyRect* rect,
                                       const OxyPaint* paint);
  OxyStatus (OXY_CALL *scene_builder_draw_path)(OxySceneBuilder* builder,
                                       const OxyPathData* path,
                                       const OxyPaint* paint);
  OxyStatus (OXY_CALL *scene_builder_draw_texture)(OxySceneBuilder* builder,
                                          OxyTexture* texture,
                                          const OxyRect* destination,
                                          const OxyPaint* paint);
  OxyStatus (OXY_CALL *scene_builder_draw_scene)(OxySceneBuilder* builder,
                                        OxyScene* scene,
                                        const OxyTransform* transform);
  OxyStatus (OXY_CALL *scene_builder_begin_layer)(OxySceneBuilder* builder,
                                         const OxyRect* bounds,
                                         OxyImageFilter* backdrop_filter,
                                         const OxyPaint* paint);
  OxyStatus (OXY_CALL *scene_builder_end_layer)(OxySceneBuilder* builder);
  OxyStatus (OXY_CALL *scene_builder_build)(OxySceneBuilder* builder,
                                   OxyScene** out_scene);
  void (OXY_CALL *destroy_scene_builder)(OxySceneBuilder* builder);
  void (OXY_CALL *retain_scene)(OxyScene* scene);
  void (OXY_CALL *release_scene)(OxyScene* scene);
  OxyStatus (OXY_CALL *create_linear_gradient)(OxySubstrate* substrate,
                                      OxyPoint start,
                                      OxyPoint end,
                                      const OxyColor* colors,
                                      const float* stops,
                                      uint64_t stop_count,
                                      OxyColorSource** out_source);
  void (OXY_CALL *release_color_source)(OxyColorSource* source);
  OxyStatus (OXY_CALL *create_color_matrix_filter)(OxySubstrate* substrate,
                                                  const float row_major[20],
                                                  OxyColorFilter** out_filter);
  void (OXY_CALL *release_color_filter)(OxyColorFilter* filter);
  OxyStatus (OXY_CALL *create_blur_filter)(OxySubstrate* substrate,
                                  float sigma_x,
                                  float sigma_y,
                                  uint32_t tile_mode,
                                  OxyImageFilter** out_filter);
  void (OXY_CALL *release_image_filter)(OxyImageFilter* filter);
  OxyStatus (OXY_CALL *register_font)(OxySubstrate* substrate,
                             uint64_t resource_generation,
                             OxyBorrowedBytes font_bytes);
  OxyStatus (OXY_CALL *create_paragraph_builder)(OxySubstrate* substrate,
                                        uint32_t text_direction,
                                        OxyParagraphBuilder** out_builder);
  OxyStatus (OXY_CALL *paragraph_builder_push_style)(OxyParagraphBuilder* builder,
                                            const OxyTextStyle* style);
  OxyStatus (OXY_CALL *paragraph_builder_pop_style)(OxyParagraphBuilder* builder);
  OxyStatus (OXY_CALL *paragraph_builder_add_text)(OxyParagraphBuilder* builder,
                                          OxyBorrowedBytes utf8_text);
  OxyStatus (OXY_CALL *paragraph_builder_build)(OxyParagraphBuilder* builder,
                                       OxyParagraph** out_paragraph);
  void (OXY_CALL *destroy_paragraph_builder)(OxyParagraphBuilder* builder);
  OxyStatus (OXY_CALL *paragraph_layout)(OxyParagraph* paragraph,
                                float maximum_width);
  OxyStatus (OXY_CALL *paragraph_get_caret_rect)(OxyParagraph* paragraph,
                                        uint64_t logical_position,
                                        uint32_t downstream_affinity,
                                        OxyRect* out_rect);
  OxyStatus (OXY_CALL *paragraph_get_range_boxes)(OxyParagraph* paragraph,
                                         uint64_t logical_start,
                                         uint64_t logical_end,
                                         OxyTextBox* output,
                                         uint64_t output_capacity,
                                         uint64_t* out_required);
  OxyStatus (OXY_CALL *scene_builder_draw_paragraph)(OxySceneBuilder* builder,
                                            OxyParagraph* paragraph,
                                            OxyPoint origin);
  void (OXY_CALL *release_paragraph)(OxyParagraph* paragraph);
  OxyStatus (OXY_CALL *submit_scene)(OxySubstrate* substrate,
                            OxyView* view,
                            const OxySceneSubmission* submission);
  /* On success, out_pixels contains exactly out_descriptor->row_bytes * out_descriptor->height bytes. Phase 3A returns tightly packed RGBA8888 with premultiplied alpha in sRGB. */
  OxyStatus (OXY_CALL *render_headless)(OxySubstrate* substrate,
                               OxyScene* scene,
                               uint32_t width,
                               uint32_t height,
                               OxyOwnedBytes* out_pixels,
                               OxyRasterDescriptor* out_descriptor);
  OxyStatus (OXY_CALL *realize_texture)(OxySubstrate* substrate,
                               uint64_t resource_generation,
                               uint32_t width,
                               uint32_t height,
                               uint32_t pixel_format,
                               OxyBorrowedBytes pixels,
                               OxyTexture** out_texture);
  void (OXY_CALL *retain_texture)(OxyTexture* texture);
  void (OXY_CALL *release_texture)(OxyTexture* texture);
  OxyStatus (OXY_CALL *update_semantics)(OxySubstrate* substrate,
                                const OxySemanticsUpdate* update);
  OxyStatus (OXY_CALL *respond_semantics_action)(OxySubstrate* substrate,
                                        uint64_t request_generation,
                                        OxyStatus result);
  OxyStatus (OXY_CALL *request_platform_service)(OxySubstrate* substrate,
                                                 const OxyPlatformServiceRequest* request);
  OxyStatus (OXY_CALL *cancel_platform_service)(OxySubstrate* substrate,
                                                uint64_t request_generation);
  OxyStatus (OXY_CALL *respond_ime_request)(OxySubstrate* substrate,
                                           uint64_t request_generation,
                                           OxyStatus status,
                                           const OxyImeResponse* response);
  OxyStatus (OXY_CALL *recover)(OxySubstrate* substrate,
                       OxyView* view,
                       const OxyRecoveryRequest* request);
  OxyStatus (OXY_CALL *pump_platform_tasks)(OxySubstrate* substrate,
                                   uint64_t monotonic_time_ns);
  OxyStatus (OXY_CALL *begin_shutdown)(OxySubstrate* substrate);
  OxyStatus (OXY_CALL *drain)(OxySubstrate* substrate,
                     uint64_t monotonic_deadline_ns);
} OxySubstrateApi;

/* Destruction order is begin_shutdown, drain, destroy_view for each remaining view, then destroy. destroy is valid only after drain succeeds or returns a terminal timeout status; it releases implementation state but never calls callbacks. Each successfully acquired or retained resource reference is released exactly once on the graphics domain through the adapter queue. */

/* The sole ABI acquisition symbol. The caller zero-initializes out_api, sets its struct_size and abi_version, and passes the same values as arguments. The implementation copies only the mutually supported prefix and returns OXY_STATUS_INCOMPATIBLE_ABI without creating state when negotiation fails. No panic or C++ exception may cross this boundary. */
OXY_EXPORT OxyStatus OXY_CALL OxySubstrateGetApi(
    uint32_t requested_abi_version,
    uint32_t caller_struct_size,
    OxySubstrateApi* out_api);

#ifdef __cplusplus
}
#endif

#endif
