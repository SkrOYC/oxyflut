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

/// Identifies one immutable text layout generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextLayoutGeneration(pub u64);

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

/// Stores a raster size in physical pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PixelSize {
    /// Width in physical pixels.
    pub width: u32,
    /// Height in physical pixels.
    pub height: u32,
}

/// Describes a headless viewport and its raster mapping.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeadlessMetrics {
    /// Viewport size in logical pixels.
    pub logical_size: Size,
    /// Raster size in physical pixels.
    pub physical_size: PixelSize,
    /// Physical pixels per logical pixel.
    pub device_pixel_ratio: f32,
}

/// Selects the byte layout of a headless raster.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    /// Eight-bit red, green, blue, and alpha channels in that byte order.
    Rgba8888,
}

/// Selects the alpha representation of a headless raster.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlphaType {
    /// Color channels are multiplied by alpha.
    Premultiplied,
}

/// Selects the color space of a headless raster.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorSpace {
    /// Standard red, green, and blue color space.
    Srgb,
}

/// Describes the exact layout of a headless raster.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RasterDescriptor {
    /// Width in physical pixels.
    pub width: u32,
    /// Height in physical pixels.
    pub height: u32,
    /// Bytes from the start of one row to the next row.
    pub row_bytes: u32,
    /// Byte layout of each pixel.
    pub pixel_format: PixelFormat,
    /// Alpha representation.
    pub alpha_type: AlphaType,
    /// Color space.
    pub color_space: ColorSpace,
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

/// Selects the unit carried by a checked text index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextIndex {
    /// Checked UTF-8 byte offset.
    Utf8Bytes(u32),
    /// Checked UTF-16 code-unit offset.
    Utf16Units(u32),
    /// Checked grapheme-boundary ordinal.
    Grapheme(u32),
    /// Logical position within one immutable paragraph.
    Logical(u32),
}

/// Selects a text-index unit without carrying an offset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextIndexUnit {
    /// UTF-8 byte offsets.
    Utf8Bytes,
    /// UTF-16 code-unit offsets.
    Utf16Units,
    /// Grapheme-boundary ordinals.
    Grapheme,
    /// Logical paragraph positions.
    Logical,
}

/// Describes a half-open checked text range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextRange {
    /// Inclusive range start.
    pub start: TextIndex,
    /// Exclusive range end.
    pub end: TextIndex,
}

/// Selects caret affinity at a text boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextAffinity {
    /// Associates the position with preceding visual content.
    Upstream,
    /// Associates the position with following visual content.
    Downstream,
}

/// Reports point-to-text hit-test results.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextHit {
    /// Checked hit index.
    pub index: TextIndex,
    /// Visual affinity.
    pub affinity: TextAffinity,
    /// True when the index is a grapheme boundary.
    pub grapheme_boundary: bool,
    /// True when the point lies within paragraph bounds.
    pub inside: bool,
}

/// Describes one attributed semantics text segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticsTextSegment {
    /// Checked segment range.
    pub range: TextRange,
    /// Closed platform-independent attribute mask.
    pub attributes: u64,
}

/// Selects a candidate-neutral semantics relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticsRelationKind {
    /// The target labels this node.
    LabelledBy,
    /// This node labels the target.
    Labels,
    /// The target describes this node.
    DescribedBy,
    /// This node describes the target.
    Describes,
    /// This node controls the target.
    Controls,
    /// The target controls this node.
    ControlledBy,
    /// Reading order flows from this node to the target.
    FlowsTo,
    /// Reading order flows from the target to this node.
    FlowsFrom,
    /// This node belongs to the target collection.
    MemberOf,
    /// This node owns the target.
    Owns,
    /// The target provides error-message content for this node.
    ErrorMessage,
    /// The target provides extended details for this node.
    Details,
    /// This node provides extended details for the target.
    DetailsFor,
    /// The target is the active descendant of this node.
    ActiveDescendant,
}

/// Describes one typed semantics relation to a node generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticsRelation {
    /// Relation kind.
    pub kind: SemanticsRelationKind,
    /// Related node generation.
    pub target_generation: u64,
}

/// Describes semantics scrolling state in logical units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SemanticsScroll {
    /// Current scroll position.
    pub position: f64,
    /// Minimum scroll position.
    pub minimum: f64,
    /// Maximum scroll position.
    pub maximum: f64,
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
    /// Human-visible value; diagnostics cannot copy it.
    pub value: String,
    /// Human-visible label; diagnostics cannot copy it.
    pub label: String,
    /// Computed accessible name; diagnostics cannot copy it.
    pub accessible_name: String,
    /// Distinct accessible description; diagnostics cannot copy it.
    pub description: String,
    /// Human-visible hint; diagnostics cannot copy it.
    pub hint: String,
    /// Platform help or full-description text; diagnostics cannot copy it.
    pub help: String,
    /// Human-visible tooltip; diagnostics cannot copy it.
    pub tooltip: String,
    /// Attributed accessible text segments.
    pub attributed_text: Vec<SemanticsTextSegment>,
    /// Stable application-provided identifier.
    pub identifier: String,
    /// View-local bounds.
    pub bounds: Rect,
    /// Node transform.
    pub transform: Transform,
    /// Traversal child generations.
    pub traversal_children: Vec<u64>,
    /// Hit-test child generations.
    pub hit_test_children: Vec<u64>,
    /// Role-applicable typed relations.
    pub relations: Vec<SemanticsRelation>,
    /// Checked UTF-16 selection, if applicable.
    pub selection_utf16: Option<(u32, u32)>,
    /// Immutable text layout that owns every selection and attributed-text index, if applicable.
    pub text_layout: Option<TextLayoutGeneration>,
    /// Scroll position and extents, if applicable.
    pub scroll: Option<SemanticsScroll>,
    /// Heading level, or zero when the node isn't a heading.
    pub heading_level: u32,
    /// Closed logical text-direction identifier.
    pub text_direction: u32,
    /// BCP-47 language.
    pub language: String,
    /// True for an accessibility live region.
    pub live_region: bool,
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

// Contract invariant: A semantics node with selection or attributed text carries `text_layout`. Replacing that layout requires a new node generation before publishing ranges or routing indexed actions.

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

/// Stores a checked range in the source platform's declared index unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeTextRange {
    /// Inclusive range start.
    pub start: u32,
    /// Exclusive range end.
    pub end: u32,
}

/// Selects the index unit used by a platform input method editor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeTextIndexUnit {
    /// UTF-8 byte offsets.
    Utf8Bytes,
    /// UTF-16 code-unit offsets.
    Utf16Units,
    /// Unicode scalar-value offsets.
    UnicodeScalars,
}

/// Describes one attributed input method editor text segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImeTextSegment {
    /// Segment range in the declared native index unit.
    pub range: NativeTextRange,
    /// Closed platform-independent attribute mask.
    pub attributes: u64,
}

/// Selects a bidirectional input method editor query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImeRequestKind {
    /// Requests bounded surrounding text and attributes.
    SurroundingText,
    /// Requests text and attributes for one range.
    AttributedText,
    /// Requests the text index at a view-local point.
    CharacterIndexAtPoint,
    /// Requests geometry for one text range.
    TextRect,
    /// Requests selection and marked-range state.
    SelectionState,
    /// Requests the closed input-context identifier and sensitive-field state.
    InputContext,
}

/// Describes one candidate-transported input method editor query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImeRequest {
    /// Request generation acknowledged exactly once.
    pub generation: u64,
    /// Query kind.
    pub kind: ImeRequestKind,
    /// Native index unit expected by the platform.
    pub native_index_unit: NativeTextIndexUnit,
    /// Optional checked query range.
    pub range: Option<NativeTextRange>,
    /// Optional query point; ignored for non-point queries.
    pub point: Point,
    /// Maximum text units permitted in the response.
    pub maximum_units: u32,
}

/// Supplies a bounded response to an input method editor query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImeResponse<'a> {
    /// Range represented by the returned text, if any.
    pub text_range: Option<NativeTextRange>,
    /// Current selection.
    pub selection: NativeTextRange,
    /// Current marked range, if any.
    pub marked: Option<NativeTextRange>,
    /// Candidate or queried text rectangle.
    pub text_rect: Rect,
    /// Character index returned for a point query, if any.
    pub character_index: Option<u32>,
    /// Closed input-context identifier.
    pub input_context: u32,
    /// True when the active field contains sensitive content.
    pub sensitive_field: bool,
    /// Callback-scoped UTF-8 text bounded by the request.
    pub text: &'a str,
    /// Callback-scoped attributed text segments.
    pub segments: &'a [ImeTextSegment],
}

/// Selects the outcome of an input method editor query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImeResponseStatus {
    /// The response payload is valid and complete.
    Ok,
    /// The target query generation is stale.
    Stale,
    /// A requested range or index is invalid.
    InvalidRange,
    /// The query isn't supported by the active platform contract.
    Unsupported,
    /// The query was canceled before publication.
    Cancelled,
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
        /// Pointer movement since the preceding event.
        delta: Point,
        /// Pressed-button mask.
        buttons: u64,
        /// Modifier-key mask.
        modifiers: u64,
        /// Closed pointer-device identifier.
        device_kind: u32,
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
        /// Monotonic event time.
        monotonic_ns: u64,
        /// True when the platform identifies this event as a repeat.
        repeat: bool,
    },
    /// Input method editor transaction with callback-scoped UTF-8 text.
    Ime {
        /// Transaction generation.
        transaction: u64,
        /// Closed transaction-kind identifier.
        kind: u32,
        /// Explicit native index unit.
        native_index_unit: NativeTextIndexUnit,
        /// Replacement range, or no range when the platform supplies negative sentinels.
        replacement: Option<NativeTextRange>,
        /// Selection after applying the transaction.
        selection: NativeTextRange,
        /// Marked range after applying the transaction, if any.
        marked: Option<NativeTextRange>,
        /// Candidate-window rectangle in view-local logical pixels.
        candidate_rect: Rect,
        /// Closed transaction or platform-action identifier.
        action: u32,
        /// Closed input-context identifier.
        input_context: u32,
        /// True when the active field contains sensitive content.
        sensitive_field: bool,
        /// Callback-scoped attributed text segments.
        segments: &'a [ImeTextSegment],
        /// Callback-scoped UTF-8 text.
        text: &'a str,
    },
    /// Bidirectional input method editor state query.
    ImeRequest(ImeRequest),
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

/// Reports the terminal result of one submitted frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PresentationStatus {
    /// The frame reached the presentation mechanism.
    Presented,
    /// The submission or callback payload was invalid.
    InvalidArgument,
    /// The adapter and substrate ABI were incompatible.
    IncompatibleAbi,
    /// The view or frame generation was stale.
    StaleOwner,
    /// A bounded resource limit prevented presentation.
    ResourceLimit,
    /// The presentation mechanism was unsupported.
    Unsupported,
    /// The substrate reported an implementation failure.
    SubstrateFailure,
    /// Presentation was cancelled before completion.
    Cancelled,
    /// Presentation missed its terminal deadline.
    DeadlineExceeded,
}

/// Receives normalized asynchronous substrate events.
pub trait SubstrateEvents<E: Error + Send + Sync + 'static> {
    /// Reports one candidate-transported frame opportunity.
    fn frame_opportunity(&mut self, opportunity: FrameOpportunity) -> Result<(), E>;

    /// Reports presentation feedback or a structured terminal failure for one submitted frame.
    ///
    /// `Presented` requires one monotonic presentation timestamp. Every failure requires `None`. The integrated adapter maps the nine declared `OxyStatus` values explicitly and rejects unknown values before invoking this callback.
    fn presented(
        &mut self,
        view: ViewGeneration,
        frame: FrameGeneration,
        presentation_ns: Option<u64>,
        status: PresentationStatus,
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
    /// Candidate-owned immutable paragraph type.
    type Paragraph: Send + Sync + 'static;

    /// Saves transform, clip, and paint state.
    fn save(&mut self) -> Result<(), E>;

    /// Restores the most recently saved state.
    fn restore(&mut self) -> Result<(), E>;

    /// Concatenates a transform.
    fn transform(&mut self, transform: Transform) -> Result<(), E>;

    /// Adds a rectangle clip.
    fn clip_rect(&mut self, bounds: Rect) -> Result<(), E>;

    /// Adds a validated path clip.
    fn clip_path(&mut self, path: &Path) -> Result<(), E>;

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

    /// Draws a reusable immutable scene.
    fn draw_picture(&mut self, picture: &Self::Scene, transform: Transform) -> Result<(), E>;

    /// Draws an immutable laid-out paragraph at a logical origin.
    fn draw_paragraph(&mut self, paragraph: &Self::Paragraph, origin: Point) -> Result<(), E>;

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

    /// Returns caret geometry for a checked position and affinity.
    fn caret_rect(
        &self,
        paragraph: &Self::Paragraph,
        position: TextIndex,
        affinity: TextAffinity,
    ) -> Result<Rect, E>;

    /// Writes selection rectangles and returns the number required.
    fn selection_rects(
        &self,
        paragraph: &Self::Paragraph,
        range: TextRange,
        output: &mut [Rect],
    ) -> Result<usize, E>;

    /// Hit-tests a logical point and returns a checked index and affinity.
    fn hit_test(&self, paragraph: &Self::Paragraph, point: Point) -> Result<TextHit, E>;

    /// Converts a checked index to another unit after boundary validation.
    fn convert_index(
        &self,
        paragraph: &Self::Paragraph,
        index: TextIndex,
        target_unit: TextIndexUnit,
    ) -> Result<TextIndex, E>;
}

/// Implements the physical rendering-substrate boundary for one candidate.
pub trait SubstrateAdapter {
    /// Structured candidate error type.
    type Error: Error + Send + Sync + 'static;
    /// Candidate-owned view mechanism.
    type View: Send + 'static;
    /// Candidate scene builder.
    type Builder: SceneBuilder<Self::Error, Paragraph = <Self::Text as SubstrateText<Self::Error>>::Paragraph>;
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
    ///
    /// The metrics obey the same finite, positive, round-to-nearest consistency rule as the public headless view. The returned descriptor is always tightly packed RGBA8888 with premultiplied alpha in sRGB. The caller provides at least `physical_width * physical_height * 4` bytes, and the successful descriptor's `row_bytes * height` equals the initialized output length.
    fn render_headless(
        &mut self,
        scene: &<Self::Builder as SceneBuilder<Self::Error>>::Scene,
        metrics: HeadlessMetrics,
        output: &mut [u8],
    ) -> Result<RasterDescriptor, Self::Error>;

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

    /// Completes one candidate-transported input method editor query exactly once.
    ///
    /// `Ok` requires a response payload. Every other status requires no payload and terminally acknowledges the request generation.
    fn respond_ime_request(
        &mut self,
        request_generation: u64,
        status: ImeResponseStatus,
        response: Option<ImeResponse<'_>>,
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
