#![deny(missing_docs, unsafe_code)]
#![allow(dead_code)]

//! Qualification contract shared by both rendering-substrate adapters.

use std::error::Error;

/// Identifies one live view generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ViewGeneration(pub u64);

/// Identifies one submitted frame generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FrameGeneration(pub u64);

/// Identifies one display epoch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DisplayEpoch(pub u64);

/// Identifies one owned resource generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResourceGeneration(pub u64);

/// Stores a point in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
}

/// Stores a size in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    /// Horizontal extent.
    pub width: f32,
    /// Vertical extent.
    pub height: f32,
}

/// Stores an axis-aligned logical rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Rectangle origin.
    pub origin: Point,
    /// Rectangle size.
    pub size: Size,
}

/// Stores normalized color components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Red component.
    pub red: f32,
    /// Green component.
    pub green: f32,
    /// Blue component.
    pub blue: f32,
    /// Alpha component.
    pub alpha: f32,
}

/// Stores a column-major transform matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform(pub [f32; 16]);

/// Describes one candidate-neutral path.
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    /// Encoded closed verb set.
    pub verbs: Vec<u8>,
    /// Points consumed by the verbs.
    pub points: Vec<Point>,
    /// Closed fill-rule identifier.
    pub fill_rule: u32,
}

/// Describes one linear-gradient stop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    /// Normalized location.
    pub offset: f32,
    /// Stop color.
    pub color: Color,
}

/// Selects a candidate-neutral paint source.
#[derive(Clone, Debug, PartialEq)]
pub enum PaintSource {
    /// Solid color.
    Solid(Color),
    /// Linear gradient.
    LinearGradient {
        /// Start point.
        start: Point,
        /// End point.
        end: Point,
        /// Ordered validated stops.
        stops: Vec<GradientStop>,
    },
}

/// Selects one bounded image effect.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageEffect {
    /// Gaussian blur.
    Blur {
        /// Horizontal sigma.
        sigma_x: f32,
        /// Vertical sigma.
        sigma_y: f32,
    },
    /// Color matrix in row-major order.
    ColorMatrix([f32; 20]),
}

/// Describes candidate-neutral drawing state.
#[derive(Clone, Debug, PartialEq)]
pub struct Paint {
    /// Paint source.
    pub source: PaintSource,
    /// Optional bounded effect.
    pub effect: Option<ImageEffect>,
    /// Closed blend-mode identifier.
    pub blend_mode: u32,
}

/// Describes one checked text style run over UTF-8 source bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyleRun {
    /// Inclusive checked UTF-8 byte start.
    pub start_utf8: u32,
    /// Exclusive checked UTF-8 byte end.
    pub end_utf8: u32,
    /// Font family.
    pub font_family: String,
    /// BCP-47 locale.
    pub locale: String,
    /// Font size in logical pixels.
    pub font_size: f32,
    /// Foreground color.
    pub color: Color,
}

/// Describes one complete candidate-neutral semantics node.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticsNode {
    /// Node generation.
    pub generation: u64,
    /// Closed role identifier.
    pub role: u32,
    /// Closed state mask.
    pub states: u64,
    /// Closed action mask.
    pub actions: u64,
    /// View-local bounds.
    pub bounds: Rect,
    /// Node transform.
    pub transform: Transform,
    /// Traversal child generations.
    pub traversal_children: Vec<u64>,
    /// Hit-test child generations.
    pub hit_test_children: Vec<u64>,
    /// Label relation generations.
    pub labelled_by: Vec<u64>,
    /// Description relation generations.
    pub described_by: Vec<u64>,
    /// Checked UTF-16 selection, if applicable.
    pub selection_utf16: Option<(u32, u32)>,
    /// BCP-47 language.
    pub language: String,
    /// True when input-focused.
    pub input_focus: bool,
    /// True when accessibility-focused.
    pub accessibility_focus: bool,
    /// True when hidden.
    pub hidden: bool,
    /// True when disabled.
    pub disabled: bool,
    /// True when private text is redacted.
    pub secure_field: bool,
}

/// Selects a recoverable fault mechanism.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryFault {
    /// Presentation metrics or drawable size changed.
    Resize,
    /// The presentation surface became invalid.
    SurfaceLoss,
    /// The operating environment resumed or changed display topology.
    ResumeOrTopology,
    /// The graphics device reported a recoverable loss.
    DeviceLoss,
}

/// Describes one view creation or metrics update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewMetrics {
    /// Logical view size.
    pub logical_size: Size,
    /// Physical pixels per logical pixel.
    pub device_pixel_ratio: f32,
    /// Stable display identity within the operating-environment session.
    pub display_id: u64,
}

/// Describes one interactive frame opportunity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameOpportunity {
    /// Target view generation.
    pub view: ViewGeneration,
    /// Active display epoch.
    pub display_epoch: DisplayEpoch,
    /// Monotonic callback timestamp in nanoseconds.
    pub monotonic_ns: u64,
    /// Target presentation timestamp in nanoseconds.
    pub target_ns: u64,
    /// Local opportunity interval in nanoseconds.
    pub interval_ns: u64,
}

/// Describes one scene submission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Submission {
    /// Target view generation.
    pub view: ViewGeneration,
    /// Submitted frame generation.
    pub frame: FrameGeneration,
    /// Target presentation timestamp in nanoseconds.
    pub target_ns: u64,
}

/// Describes one bounded recovery command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryRequest {
    /// Target view generation.
    pub view: ViewGeneration,
    /// Fault mechanism to recover.
    pub fault: RecoveryFault,
    /// One-based recovery attempt number.
    pub attempt: u8,
    /// Absolute monotonic deadline in nanoseconds.
    pub deadline_ns: u64,
    /// Maximum transient graphics memory during this recovery.
    pub transient_memory_cap_bytes: u64,
}

/// Selects a physical execution domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionDomain {
    /// Serial host callback intake and normalization.
    HostCallback,
    /// Nonreentrant application and view policy.
    Application,
    /// Cancelable asset and decoding workers.
    Worker,
    /// Graphics-affine submission, recovery, and release.
    Graphics,
}

/// Reports the result of routing one semantics action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticsActionResult {
    /// The live target accepted the action.
    Accepted,
    /// The target generation was stale.
    StaleTarget,
    /// The target doesn't implement the action.
    Unsupported,
    /// The payload failed validation.
    InvalidPayload,
}

/// Describes one normalized candidate-transported platform event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlatformEvent<'a> {
    /// Pointer or touch input.
    Pointer {
        /// Pointer generation.
        pointer: u64,
        /// Closed phase identifier.
        phase: u32,
        /// View-local position.
        position: Point,
        /// Monotonic event time.
        monotonic_ns: u64,
    },
    /// Keyboard input without committed text.
    Key {
        /// Physical-key identifier.
        physical: u32,
        /// Logical-key identifier.
        logical: u32,
        /// Closed action identifier.
        action: u32,
        /// Modifier mask.
        modifiers: u64,
    },
    /// Input method editor transaction with callback-scoped UTF-8 text.
    Ime {
        /// Transaction generation.
        transaction: u64,
        /// Closed transaction-kind identifier.
        kind: u32,
        /// Explicit native index unit.
        native_index_unit: u32,
        /// Callback-scoped UTF-8 text.
        text: &'a str,
    },
    /// View, display, or recovery lifecycle transition.
    Lifecycle {
        /// Closed lifecycle-kind identifier.
        kind: u32,
        /// Current display identity.
        display_id: u64,
        /// Current view metrics.
        metrics: ViewMetrics,
    },
}

/// Selects an integrated platform-service mechanism.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformService {
    /// User-visible file or directory dialog.
    Dialog,
    /// Clipboard read.
    ClipboardRead,
    /// Clipboard write.
    ClipboardWrite,
    /// Bounded platform message.
    Message,
}

/// Receives normalized asynchronous substrate events.
pub trait SubstrateEvents<E: Error + Send + Sync + 'static> {
    /// Reports one candidate-transported frame opportunity.
    fn frame_opportunity(&mut self, opportunity: FrameOpportunity) -> Result<(), E>;

    /// Reports presentation feedback for one submitted frame.
    fn presented(
        &mut self,
        view: ViewGeneration,
        frame: FrameGeneration,
        presentation_ns: u64,
    ) -> Result<(), E>;

    /// Reports a candidate-transported platform event for normalization.
    fn platform_event(&mut self, view: ViewGeneration, event: PlatformEvent<'_>) -> Result<(), E>;

    /// Completes one candidate-transported platform-service request.
    fn platform_response(
        &mut self,
        request_generation: u64,
        status: u32,
        callback_scoped_payload: &[u8],
    ) -> Result<(), E>;

    /// Reports an accessibility action for a live view and semantics node.
    fn semantics_action(
        &mut self,
        request_generation: u64,
        view: ViewGeneration,
        node_generation: u64,
        action: u32,
        callback_scoped_payload: &[u8],
    ) -> Result<(), E>;

    /// Requests that the host event loop pump queued substrate work without running product work inline.
    fn wake_host(&self) -> Result<(), E>;
}

/// Records one immutable candidate scene.
pub trait SceneBuilder<E: Error + Send + Sync + 'static> {
    /// Candidate-owned immutable scene type.
    type Scene: Send + Sync + 'static;
    /// Candidate-owned texture type.
    type Texture: Send + Sync + 'static;
    /// Candidate-owned picture type.
    type Picture: Send + Sync + 'static;

    /// Saves transform, clip, and paint state.
    fn save(&mut self) -> Result<(), E>;

    /// Restores the most recently saved state.
    fn restore(&mut self) -> Result<(), E>;

    /// Concatenates a transform.
    fn transform(&mut self, transform: Transform) -> Result<(), E>;

    /// Adds a rectangle clip.
    fn clip_rect(&mut self, bounds: Rect) -> Result<(), E>;

    /// Draws a rectangle.
    fn draw_rect(&mut self, bounds: Rect, paint: &Paint) -> Result<(), E>;

    /// Draws a validated path.
    fn draw_path(&mut self, path: &Path, paint: &Paint) -> Result<(), E>;

    /// Draws an owned texture.
    fn draw_texture(
        &mut self,
        texture: &Self::Texture,
        destination: Rect,
        paint: &Paint,
    ) -> Result<(), E>;

    /// Draws a reusable picture.
    fn draw_picture(&mut self, picture: &Self::Picture, transform: Transform) -> Result<(), E>;

    /// Begins a retained layer with bounded effect metadata.
    fn begin_layer(&mut self, bounds: Rect, paint: &Paint) -> Result<(), E>;

    /// Ends the active retained layer.
    fn end_layer(&mut self) -> Result<(), E>;

    /// Finishes recording and returns an immutable scene.
    fn build(self) -> Result<Self::Scene, E>;
}

/// Supplies text layout and editing geometry through one candidate implementation.
pub trait SubstrateText<E: Error + Send + Sync + 'static> {
    /// Candidate-owned immutable paragraph type.
    type Paragraph: Send + Sync + 'static;

    /// Registers one runtime font and preserves its bytes for the required lifetime.
    fn register_font(&mut self, bytes: &[u8]) -> Result<ResourceGeneration, E>;

    /// Builds and lays out one styled bidirectional paragraph.
    fn layout_paragraph(
        &mut self,
        style_runs: &[TextStyleRun],
        utf8_text: &str,
        maximum_width: f32,
    ) -> Result<Self::Paragraph, E>;

    /// Returns caret geometry for a checked logical position and affinity.
    fn caret_rect(
        &self,
        paragraph: &Self::Paragraph,
        logical_position: u32,
        downstream_affinity: bool,
    ) -> Result<Rect, E>;

    /// Writes selection rectangles and returns the number required.
    fn selection_rects(
        &self,
        paragraph: &Self::Paragraph,
        logical_start: u32,
        logical_end: u32,
        output: &mut [Rect],
    ) -> Result<usize, E>;
}

/// Implements the physical rendering-substrate boundary for one candidate.
pub trait SubstrateAdapter {
    /// Structured candidate error type.
    type Error: Error + Send + Sync + 'static;
    /// Candidate-owned view mechanism.
    type View: Send + 'static;
    /// Candidate scene builder.
    type Builder: SceneBuilder<Self::Error>;
    /// Candidate text implementation.
    type Text: SubstrateText<Self::Error>;

    /// Installs the owned callback receiver before creating a view.
    ///
    /// The adapter serializes callbacks, never reenters the receiver, and disables callbacks before returning it from shutdown.
    fn install_events(
        &mut self,
        events: Box<dyn SubstrateEvents<Self::Error> + Send>,
    ) -> Result<(), Self::Error>;

    /// Checks source, binary, and contract compatibility before other work.
    fn check_compatibility(&self) -> Result<(), Self::Error>;

    /// Returns the candidate text implementation.
    fn text(&mut self) -> &mut Self::Text;

    /// Creates one view mechanism for a live generation.
    fn create_view(
        &mut self,
        generation: ViewGeneration,
        metrics: ViewMetrics,
    ) -> Result<Self::View, Self::Error>;

    /// Updates metrics for an existing view mechanism.
    fn update_view_metrics(
        &mut self,
        view: &mut Self::View,
        metrics: ViewMetrics,
    ) -> Result<(), Self::Error>;

    /// Creates one scene builder.
    fn create_scene_builder(&mut self) -> Result<Self::Builder, Self::Error>;

    /// Submits one immutable scene to an interactive view.
    ///
    /// A successful return retains candidate ownership through presentation acknowledgement or terminal view teardown; dropping the caller's reference cannot invalidate in-flight work.
    fn submit(
        &mut self,
        view: &mut Self::View,
        scene: &<Self::Builder as SceneBuilder<Self::Error>>::Scene,
        submission: Submission,
    ) -> Result<(), Self::Error>;

    /// Renders one immutable scene without an interactive display connection.
    fn render_headless(
        &mut self,
        scene: &<Self::Builder as SceneBuilder<Self::Error>>::Scene,
        size: Size,
        output: &mut [u8],
    ) -> Result<usize, Self::Error>;

    /// Realizes decoded pixels as an owned graphics resource.
    fn realize_texture(
        &mut self,
        generation: ResourceGeneration,
        size: Size,
        encoded_format: u32,
        pixels: &[u8],
    ) -> Result<<Self::Builder as SceneBuilder<Self::Error>>::Texture, Self::Error>;

    /// Applies one typed semantics update.
    fn update_semantics(
        &mut self,
        view: ViewGeneration,
        nodes: &[SemanticsNode],
        deleted_generations: &[u64],
    ) -> Result<(), Self::Error>;

    /// Acknowledges one candidate-transported semantics action exactly once.
    fn respond_semantics_action(
        &mut self,
        request_generation: u64,
        result: SemanticsActionResult,
    ) -> Result<(), Self::Error>;

    /// Starts one candidate-transported platform service with an owned request generation.
    fn request_platform_service(
        &mut self,
        request_generation: u64,
        view: ViewGeneration,
        service: PlatformService,
        payload: &[u8],
    ) -> Result<(), Self::Error>;

    /// Cancels an unpublished candidate-transported platform service.
    fn cancel_platform_service(&mut self, request_generation: u64) -> Result<(), Self::Error>;

    /// Performs one bounded recovery mechanism.
    fn recover(
        &mut self,
        view: &mut Self::View,
        request: RecoveryRequest,
    ) -> Result<(), Self::Error>;

    /// Pumps queued candidate work on the host domain until no immediately runnable task remains.
    fn pump_host_tasks(&mut self, monotonic_ns: u64) -> Result<(), Self::Error>;

    /// Starts idempotent shutdown, rejects new work, and disables external callback production.
    fn begin_shutdown(&mut self) -> Result<(), Self::Error>;

    /// Drains or rejects late completions before the absolute monotonic deadline.
    fn drain(&mut self, monotonic_deadline_ns: u64) -> Result<(), Self::Error>;

    /// Tears down a view after callbacks and in-flight work are disabled or drained.
    fn destroy_view(&mut self, view: Self::View) -> Result<(), Self::Error>;
}
