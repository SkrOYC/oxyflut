#![deny(missing_docs, unsafe_code)]
#![allow(dead_code)]

//! Qualification contract for the safe Oxyflut Rust surface.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

/// Identifies one application runtime generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RuntimeId(u64);

/// Identifies one live view generation within an application runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ViewId(u64);

/// Identifies one stable component within its owning runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComponentKey(u64);

/// Identifies one live semantics node generation within a view.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticsNodeId(u64);

/// Identifies one owned asset or graphics-resource generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResourceId(u64);

/// Identifies one asynchronous asset request generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssetRequestId(u64);

/// Identifies one immutable text-layout generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextLayoutId(u64);

/// Identifies one virtual viewport generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ViewportId(u64);

/// Identifies one gesture sequence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GestureId(u64);

/// Identifies one focus node generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FocusNodeId(u64);

/// Identifies one asynchronous operating-system request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlatformRequestId(u64);

/// Reports an expected failure through the safe application surface.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OxyError {
    /// The supplied owner is missing, stale, or belongs to another runtime.
    StaleOwner,
    /// A value, range, index, or state transition is invalid.
    InvalidInput,
    /// A bounded queue, cache, or allocation limit was reached.
    ResourceLimit,
    /// The operating environment doesn't provide the requested capability.
    Unsupported,
    /// The operation was canceled before publication.
    Cancelled,
    /// The selected substrate rejected or failed the operation.
    SubstrateFailure,
    /// Recovery exhausted its deadline, attempt count, or memory limit.
    RecoveryExhausted,
}

impl Display for OxyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::StaleOwner => "the owner is stale",
            Self::InvalidInput => "the input is invalid",
            Self::ResourceLimit => "a resource limit was reached",
            Self::Unsupported => "the capability is unsupported",
            Self::Cancelled => "the operation was cancelled",
            Self::SubstrateFailure => "the substrate operation failed",
            Self::RecoveryExhausted => "surface recovery was exhausted",
        })
    }
}

impl Error for OxyError {}

/// Stores a two-dimensional point in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
}

/// Stores a two-dimensional size in logical pixels.
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

/// Owns one deterministic headless frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadlessFrame {
    /// Exact raster layout.
    pub descriptor: RasterDescriptor,
    /// Pixel bytes whose length equals `descriptor.row_bytes * descriptor.height`.
    pub pixels: Vec<u8>,
}

/// Stores an axis-aligned rectangle in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Rectangle origin.
    pub origin: Point,
    /// Rectangle size.
    pub size: Size,
}

/// Stores minimum and maximum layout extents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
    /// Minimum permitted size.
    pub minimum: Size,
    /// Maximum permitted size.
    pub maximum: Size,
}

/// Stores normalized red, green, blue, and alpha components.
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

/// Stores a column-major four-by-four transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform(pub [f32; 16]);

/// Selects how a path is filled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FillRule {
    /// Fill using the nonzero winding rule.
    NonZero,
    /// Fill using the even-odd rule.
    EvenOdd,
}

/// Describes one validated vector path.
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    /// Encoded path verbs from the public path builder.
    pub verbs: Vec<u8>,
    /// Points consumed by the encoded verbs.
    pub points: Vec<Point>,
    /// Path fill rule.
    pub fill_rule: FillRule,
}

/// Describes one gradient stop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    /// Normalized location from zero through one.
    pub offset: f32,
    /// Stop color.
    pub color: Color,
}

/// Describes a safe paint source.
#[derive(Clone, Debug, PartialEq)]
pub enum PaintSource {
    /// One solid color.
    Solid(Color),
    /// A linear gradient with validated, ordered stops.
    LinearGradient {
        /// Gradient start point.
        start: Point,
        /// Gradient end point.
        end: Point,
        /// Ordered gradient stops.
        stops: Vec<GradientStop>,
    },
}

/// Selects a bounded image effect.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageEffect {
    /// A Gaussian blur with finite nonnegative sigmas.
    Blur {
        /// Horizontal blur sigma.
        sigma_x: f32,
        /// Vertical blur sigma.
        sigma_y: f32,
    },
    /// A color matrix in row-major order.
    ColorMatrix([f32; 20]),
}

/// Describes safe drawing state.
#[derive(Clone, Debug, PartialEq)]
pub struct Paint {
    /// Paint source.
    pub source: PaintSource,
    /// Optional bounded image effect.
    pub effect: Option<ImageEffect>,
    /// Normalized opacity.
    pub opacity: f32,
}

/// Selects logical text direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
    /// Text and directional layout proceed left to right.
    LeftToRight,
    /// Text and directional layout proceed right to left.
    RightToLeft,
}

/// Selects the unit carried by a text index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextIndex {
    /// A checked UTF-8 byte offset.
    Utf8Bytes(u32),
    /// A checked UTF-16 code-unit offset.
    Utf16Units(u32),
    /// A checked grapheme-boundary ordinal.
    Grapheme(u32),
    /// A logical text position within one immutable layout generation.
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
    /// Logical positions within one immutable layout generation.
    Logical,
}

/// Describes a half-open text range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextRange {
    /// Inclusive range start.
    pub start: TextIndex,
    /// Exclusive range end.
    pub end: TextIndex,
}

/// Describes view metrics owned by one view generation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewMetrics {
    /// Logical view size.
    pub logical_size: Size,
    /// Physical pixels per logical pixel.
    pub device_pixel_ratio: f32,
}

/// Describes one display-synchronized frame instant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameInstant {
    /// Monotonic timestamp in nanoseconds.
    pub monotonic_ns: u64,
    /// Target interval until the next opportunity.
    pub interval: Duration,
}

/// Describes one completed layout operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutResult {
    /// Resulting component size.
    pub size: Size,
    /// Number of participating-node visits made by this policy.
    pub node_visits: u32,
}

/// Describes one text selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Selection {
    /// Selected range.
    pub range: TextRange,
    /// True when the logical caret is at the range start.
    pub caret_at_start: bool,
}

/// Selects pointer contact state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointerPhase {
    /// A contact or pointer button began.
    Down,
    /// An active pointer moved.
    Move,
    /// An active pointer ended normally.
    Up,
    /// An active pointer was canceled.
    Cancel,
}

/// Describes one normalized pointer or touch event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerEvent {
    /// Device-scoped pointer identity.
    pub pointer: u64,
    /// Event phase.
    pub phase: PointerPhase,
    /// View-local position.
    pub position: Point,
    /// Monotonic event timestamp.
    pub monotonic_ns: u64,
}

/// Reports one bounds-pruned hit-test result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HitTestResult {
    /// Deepest eligible component.
    pub target: ComponentKey,
    /// Number of component bounds visited.
    pub visited_nodes: u32,
}

/// Selects a normalized keyboard action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyAction {
    /// A key became pressed.
    Down,
    /// A key was released.
    Up,
}

/// Describes one normalized keyboard event without text content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyEvent {
    /// Physical-key identity.
    pub physical_key: u32,
    /// Logical-key identity.
    pub logical_key: u32,
    /// Press or release action.
    pub action: KeyAction,
    /// Normalized modifier mask.
    pub modifiers: u32,
}

/// Selects focus traversal direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusDirection {
    /// Move to the next traversal node.
    Next,
    /// Move to the previous traversal node.
    Previous,
    /// Move spatially left.
    Left,
    /// Move spatially right.
    Right,
    /// Move spatially up.
    Up,
    /// Move spatially down.
    Down,
}

/// Selects the source of a scroll update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScrollSource {
    /// Discrete mouse-wheel input.
    Wheel,
    /// Precision pointer or trackpad input.
    Precision,
    /// Direct touch input.
    Touch,
    /// Momentum after an input sequence.
    Momentum,
    /// Accessibility or programmatic movement.
    Programmatic,
}

/// Describes one scroll delta.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollDelta {
    /// Horizontal delta.
    pub x: f32,
    /// Vertical delta.
    pub y: f32,
    /// Source behavior family.
    pub source: ScrollSource,
}

/// Describes one realized virtual range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealizedRange {
    /// First realized item, inclusive.
    pub start: u64,
    /// End of the realized item range, exclusive.
    pub end: u64,
    /// Total collection size.
    pub total: u64,
}

/// Reports one asynchronous operation state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PollState {
    /// Work is still pending.
    Pending,
    /// Work completed and published its output.
    Ready,
}

/// Describes validated decoded pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedImage {
    /// Pixel width.
    pub width: u32,
    /// Pixel height.
    pub height: u32,
    /// Bytes per row.
    pub row_bytes: u32,
    /// Closed pixel-format identifier.
    pub format: u32,
}

/// Selects caret affinity at a bidirectional boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextAffinity {
    /// Prefer the upstream visual position.
    Upstream,
    /// Prefer the downstream visual position.
    Downstream,
}

/// Describes one immutable styled run.
#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    /// Checked range in the source text.
    pub range: TextRange,
    /// Font family requested for this run.
    pub font_family: String,
    /// Font size in logical pixels.
    pub font_size: f32,
    /// Locale tag used for shaping.
    pub locale: String,
    /// Foreground color.
    pub color: Color,
}

/// Reports text hit-test geometry and affinity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextHit {
    /// Checked logical text position.
    pub index: TextIndex,
    /// Visual affinity at the position.
    pub affinity: TextAffinity,
    /// True when the position is a grapheme boundary.
    pub grapheme_boundary: bool,
    /// True when the position is a word boundary.
    pub word_boundary: bool,
}

/// Selects a rich-text editing command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditCommand {
    /// Delete the preceding grapheme.
    DeletePreviousGrapheme,
    /// Delete the following grapheme.
    DeleteNextGrapheme,
    /// Delete the preceding word.
    DeletePreviousWord,
    /// Delete the following word.
    DeleteNextWord,
    /// Extend selection by one grapheme.
    ExtendSelectionGrapheme,
    /// Extend selection by one word.
    ExtendSelectionWord,
    /// Select all content.
    SelectAll,
}

/// Describes one attributed input method editor text segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImeTextSegment {
    /// Segment range in checked logical indices.
    pub range: TextRange,
    /// Closed platform-independent attribute mask.
    pub attributes: u64,
}

/// Selects a normalized input method editor action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImeAction {
    /// Inserts or replaces text.
    Insert,
    /// Sets or updates marked text.
    SetMarked,
    /// Clears marked state without committing text.
    Unmark,
    /// Commits the active composition.
    Commit,
    /// Cancels the active composition.
    Cancel,
    /// Deletes surrounding text.
    DeleteSurrounding,
    /// Transfers input method editor focus.
    FocusTransfer,
    /// Reports a platform-specific lock or layout transition.
    PlatformTransition,
}

/// Selects a bidirectional input method editor query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImeQueryKind {
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
    /// Requests input-context and sensitive-field state.
    InputContext,
}

/// Describes one checked input method editor query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImeQuery {
    /// Query generation acknowledged exactly once.
    pub generation: u64,
    /// Query kind.
    pub kind: ImeQueryKind,
    /// Optional checked query range.
    pub range: Option<TextRange>,
    /// Optional query point; ignored for non-point queries.
    pub point: Point,
    /// Maximum logical text units permitted in the response.
    pub maximum_units: u32,
}

/// Owns one bounded input method editor query result.
#[derive(Clone, Debug, PartialEq)]
pub struct ImeQueryResult {
    /// Checked range represented by the returned text, if any.
    pub text_range: Option<TextRange>,
    /// Returned text, bounded by the query.
    pub text: String,
    /// Returned attributed segments.
    pub segments: Vec<ImeTextSegment>,
    /// Current selection.
    pub selection: Selection,
    /// Current marked range, if any.
    pub marked: Option<TextRange>,
    /// Candidate or queried text rectangle.
    pub text_rect: Rect,
    /// Checked character index returned for a point query, if any.
    pub character_index: Option<TextIndex>,
    /// Closed input-context identifier.
    pub input_context: u32,
    /// True when the active field contains sensitive content.
    pub sensitive_field: bool,
}

/// Describes one input method editor transaction.
#[derive(Clone, Debug, PartialEq)]
pub struct ImeTransaction {
    /// Monotonic transaction generation.
    pub generation: u64,
    /// Replacement range, if the platform supplied one.
    pub replacement: Option<TextRange>,
    /// Marked or committed text.
    pub text: String,
    /// Selection after applying the transaction.
    pub selection: Selection,
    /// Marked range after applying the transaction, if any.
    pub marked: Option<TextRange>,
    /// Normalized action represented by the transaction.
    pub action: ImeAction,
    /// Closed input-context identifier.
    pub input_context: u32,
    /// True when the active field contains sensitive content.
    pub sensitive_field: bool,
    /// Attributed composition segments.
    pub segments: Vec<ImeTextSegment>,
    /// Candidate-window rectangle in view-local logical pixels.
    pub candidate_rect: Rect,
}

/// Selects an accessibility action result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticsActionResult {
    /// The target accepted the action.
    Accepted,
    /// The target generation was stale.
    StaleTarget,
    /// The action isn't supported by the target.
    Unsupported,
    /// The payload failed validation.
    InvalidPayload,
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

/// Describes one typed semantics relation to a live node generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticsRelation {
    /// Relation kind.
    pub kind: SemanticsRelationKind,
    /// Related node generation.
    pub target: SemanticsNodeId,
}

/// Describes one typed semantics node.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticsNode {
    /// Node identity.
    pub id: SemanticsNodeId,
    /// Closed role identifier.
    pub role: u32,
    /// Closed state and capability mask.
    pub states: u64,
    /// Closed action mask.
    pub actions: u64,
    /// Human-visible value; diagnostics must not copy it.
    pub value: String,
    /// Human-visible label; diagnostics must not copy it.
    pub label: String,
    /// Computed accessible name; diagnostics must not copy it.
    pub accessible_name: String,
    /// Distinct accessible description; diagnostics must not copy it.
    pub description: String,
    /// Human-visible hint; diagnostics must not copy it.
    pub hint: String,
    /// Platform help or full-description text; diagnostics must not copy it.
    pub help: String,
    /// Human-visible tooltip; diagnostics must not copy it.
    pub tooltip: String,
    /// Attributed accessible text runs; diagnostics must not copy their text.
    pub attributed_text: Vec<ImeTextSegment>,
    /// Stable application-provided identifier.
    pub identifier: String,
    /// View-local bounds.
    pub bounds: Rect,
    /// Node transform.
    pub transform: Transform,
    /// Traversal-order children.
    pub traversal_children: Vec<SemanticsNodeId>,
    /// Hit-test-order children.
    pub hit_test_children: Vec<SemanticsNodeId>,
    /// Role-applicable typed relations.
    pub relations: Vec<SemanticsRelation>,
    /// Current text selection, if applicable.
    pub selection: Option<Selection>,
    /// Immutable text layout that owns every selection and attributed-text index, if applicable.
    pub text_layout: Option<TextLayoutId>,
    /// Scroll position and extent, if applicable.
    pub scroll: Option<(f32, f32, f32)>,
    /// BCP-47 language tag.
    pub language: String,
    /// Logical text direction.
    pub direction: TextDirection,
    /// Heading level, or zero when the node isn't a heading.
    pub heading_level: u32,
    /// True for an accessibility live region.
    pub live_region: bool,
    /// True when this node owns input focus.
    pub input_focus: bool,
    /// True when this node owns accessibility focus.
    pub accessibility_focus: bool,
    /// True when hidden from accessibility.
    pub hidden: bool,
    /// True when interaction is disabled.
    pub disabled: bool,
    /// True when private text must be redacted.
    pub secure_field: bool,
}

// Contract invariant: A semantics node with selection or attributed text carries `text_layout`. Replacing that layout creates a new semantics-node generation before publishing ranges or routing indexed actions.

/// Creates and owns the application runtime and its views.
pub trait ApplicationRuntime {
    /// Creates a view with independent mutable state.
    fn create_view(&mut self, metrics: ViewMetrics) -> Result<ViewId, OxyError>;

    /// Creates a view that renders without an interactive display connection.
    ///
    /// The device-pixel ratio must be finite and positive. Each physical extent must equal its logical extent multiplied by the ratio and rounded to the nearest integer, with half values rounded away from zero.
    fn create_headless_view(&mut self, metrics: HeadlessMetrics) -> Result<ViewId, OxyError>;

    /// Tears down a view and rejects later work for its generation.
    fn destroy_view(&mut self, view: ViewId) -> Result<(), OxyError>;

    /// Applies state changes atomically and publishes dependent work after commit.
    fn batch<R>(
        &mut self,
        update: impl FnOnce(&mut Self) -> Result<R, OxyError>,
    ) -> Result<R, OxyError>
    where
        Self: Sized;

    /// Requests one coalesced update for a live view.
    fn invalidate(&mut self, view: ViewId) -> Result<(), OxyError>;

    /// Processes one interactive or harness-controlled frame instant.
    fn begin_frame(&mut self, view: ViewId, instant: FrameInstant) -> Result<(), OxyError>;

    /// Processes one harness-controlled frame and returns tightly packed RGBA8888 pixels with premultiplied alpha in sRGB.
    fn render_headless(
        &mut self,
        view: ViewId,
        instant: FrameInstant,
    ) -> Result<HeadlessFrame, OxyError>;
}

/// Exposes a mutable reactive value with dependency tracking.
pub trait Signal<T> {
    /// Returns the current value and records a dependency read.
    fn get(&self) -> T
    where
        T: Clone;

    /// Replaces the value and marks dependent work dirty.
    fn set(&self, value: T) -> Result<(), OxyError>;
}

/// Exposes a cached value derived from reactive dependencies.
pub trait Memo<T> {
    /// Returns the cached value or recomputes it after invalidation.
    fn get(&self) -> Result<T, OxyError>
    where
        T: Clone;
}

/// Owns a lifecycle-bound reactive side effect.
pub trait EffectHandle {
    /// Cancels pending effect work and removes subscriptions.
    fn cancel(&mut self);
}

/// Reconciles keyed component collections while preserving owned state.
pub trait ComponentTree {
    /// Reconciles one ordered keyed child set and rejects duplicate keys.
    fn reconcile_keyed(
        &mut self,
        parent: ComponentKey,
        ordered_children: &[ComponentKey],
    ) -> Result<(), OxyError>;

    /// Removes a subtree after subscriptions, effects, focus, scroll, and render state are released.
    fn remove(&mut self, component: ComponentKey) -> Result<(), OxyError>;
}

/// Defines safe custom component layout.
pub trait LayoutPolicy {
    /// Resolves constraints and reports the measured node-visit count.
    fn layout(&mut self, constraints: Constraints) -> Result<LayoutResult, OxyError>;
}

/// Owns one virtualized viewport and platform-appropriate scroll state.
pub trait VirtualViewport {
    /// Returns the viewport generation.
    fn id(&self) -> ViewportId;

    /// Updates the visible extent and returns the required realized range.
    fn realize(&mut self, visible: Rect, cache_extent: f32) -> Result<RealizedRange, OxyError>;

    /// Applies wheel, precision, touch, momentum, or programmatic scrolling.
    fn scroll(&mut self, delta: ScrollDelta) -> Result<Point, OxyError>;

    /// Cancels momentum and releases gesture ownership.
    fn stop(&mut self) -> Result<(), OxyError>;
}

/// Performs bounds-pruned hit testing and deterministic gesture arbitration.
pub trait InteractionRouter {
    /// Hit-tests pointer or touch input without visiting bounds-ineligible subtrees.
    fn hit_test(&self, view: ViewId, event: PointerEvent) -> Result<HitTestResult, OxyError>;

    /// Opens one gesture sequence from the ordered hit path.
    fn begin_gesture(
        &mut self,
        view: ViewId,
        event: PointerEvent,
        hit: HitTestResult,
    ) -> Result<GestureId, OxyError>;

    /// Advances arbitration for an active sequence.
    fn update_gesture(&mut self, gesture: GestureId, event: PointerEvent) -> Result<(), OxyError>;

    /// Resolves or cancels an active sequence exactly once.
    fn end_gesture(&mut self, gesture: GestureId, event: PointerEvent) -> Result<(), OxyError>;
}

/// Owns focus scopes, traversal, keyboard routing, and indicators.
pub trait FocusManager {
    /// Registers a focusable component in one scope.
    fn register(
        &mut self,
        view: ViewId,
        component: ComponentKey,
        scope: Option<FocusNodeId>,
    ) -> Result<FocusNodeId, OxyError>;

    /// Removes a focus node and repairs focus deterministically.
    fn unregister(&mut self, node: FocusNodeId) -> Result<(), OxyError>;

    /// Requests input focus for a live node.
    fn request_focus(&mut self, node: FocusNodeId) -> Result<(), OxyError>;

    /// Traverses within the active scope.
    fn traverse(
        &mut self,
        view: ViewId,
        direction: FocusDirection,
    ) -> Result<Option<FocusNodeId>, OxyError>;

    /// Routes one keyboard event through the focused hierarchy.
    fn route_key(&mut self, view: ViewId, event: KeyEvent) -> Result<bool, OxyError>;

    /// Returns the focus-indicator bounds for rendering and tests.
    fn indicator(&self, view: ViewId) -> Result<Option<Rect>, OxyError>;
}

/// Records safe drawing commands into an immutable picture.
pub trait Canvas {
    /// Saves the current transform and clip state.
    fn save(&mut self);

    /// Restores the most recently saved state.
    fn restore(&mut self) -> Result<(), OxyError>;

    /// Concatenates a general transform.
    fn transform(&mut self, transform: Transform) -> Result<(), OxyError>;

    /// Adds a rectangle clip.
    fn clip_rect(&mut self, bounds: Rect) -> Result<(), OxyError>;

    /// Adds a validated path clip.
    fn clip_path(&mut self, path: &Path) -> Result<(), OxyError>;

    /// Draws a filled rectangle.
    fn draw_rect(&mut self, bounds: Rect, paint: &Paint) -> Result<(), OxyError>;

    /// Draws an oval.
    fn draw_oval(&mut self, bounds: Rect, paint: &Paint) -> Result<(), OxyError>;

    /// Draws a validated path.
    fn draw_path(&mut self, path: &Path, paint: &Paint) -> Result<(), OxyError>;

    /// Draws a decoded or realized image resource.
    fn draw_image(
        &mut self,
        image: ResourceId,
        destination: Rect,
        paint: &Paint,
    ) -> Result<(), OxyError>;

    /// Draws a realized texture into the destination rectangle.
    fn draw_texture(&mut self, texture: ResourceId, destination: Rect) -> Result<(), OxyError>;

    /// Draws a reusable immutable picture under a transform.
    fn draw_picture(&mut self, picture: ResourceId, transform: Transform) -> Result<(), OxyError>;

    /// Draws an immutable text layout at a logical origin.
    fn draw_text_layout(&mut self, layout: TextLayoutId, origin: Point) -> Result<(), OxyError>;

    /// Begins a retained compositing layer for opacity, clips, or backdrop effects.
    fn begin_layer(&mut self, bounds: Rect, paint: &Paint) -> Result<(), OxyError>;

    /// Ends the active retained compositing layer.
    fn end_layer(&mut self) -> Result<(), OxyError>;

    /// Finishes recording and returns the owned picture resource.
    fn finish(self) -> Result<ResourceId, OxyError>
    where
        Self: Sized;
}

/// Loads and cancels asynchronous application assets.
pub trait AssetLoader {
    /// Starts a bounded asynchronous asset load.
    fn load(&mut self, owner: RuntimeId, key: &str) -> Result<AssetRequestId, OxyError>;

    /// Polls a load and writes owned bytes when ready.
    fn poll_load(
        &mut self,
        request: AssetRequestId,
        output: &mut Vec<u8>,
    ) -> Result<PollState, OxyError>;

    /// Starts a cancelable image decode from validated encoded bytes.
    fn decode_image(
        &mut self,
        owner: RuntimeId,
        encoded: &[u8],
    ) -> Result<AssetRequestId, OxyError>;

    /// Polls a decode and writes pixels only after complete validation.
    fn poll_decode(
        &mut self,
        request: AssetRequestId,
        descriptor: &mut Option<DecodedImage>,
        pixels: &mut Vec<u8>,
    ) -> Result<PollState, OxyError>;

    /// Realizes validated decoded pixels as an owned graphics resource.
    fn realize(
        &mut self,
        owner: RuntimeId,
        descriptor: DecodedImage,
        pixels: &[u8],
    ) -> Result<ResourceId, OxyError>;

    /// Inserts a reusable immutable resource under the declared memory cap.
    fn cache(&mut self, key: &str, resource: ResourceId, bytes: u64) -> Result<(), OxyError>;

    /// Returns a cached resource and updates its bounded recency state.
    fn cached(&mut self, key: &str) -> Result<Option<ResourceId>, OxyError>;

    /// Releases a resource after every submitted use is acknowledged.
    fn release(&mut self, resource: ResourceId) -> Result<(), OxyError>;

    /// Cancels an unpublished request.
    fn cancel(&mut self, request: AssetRequestId) -> Result<(), OxyError>;
}

/// Shapes styled text and returns immutable layout generations.
pub trait TextEngine {
    /// Registers a runtime font from validated bytes.
    fn register_font(&mut self, owner: RuntimeId, bytes: &[u8]) -> Result<ResourceId, OxyError>;

    /// Creates one styled bidirectional text layout.
    fn layout_text(
        &mut self,
        owner: RuntimeId,
        text: &str,
        direction: TextDirection,
        maximum_width: f32,
    ) -> Result<TextLayoutId, OxyError>;

    /// Creates one styled bidirectional text layout from checked runs.
    fn layout_styled_text(
        &mut self,
        owner: RuntimeId,
        text: &str,
        runs: &[TextRun],
        direction: TextDirection,
        maximum_width: f32,
    ) -> Result<TextLayoutId, OxyError>;

    /// Returns checked caret geometry for one layout generation.
    fn caret_rect(
        &self,
        layout: TextLayoutId,
        index: TextIndex,
        affinity: TextAffinity,
    ) -> Result<Rect, OxyError>;

    /// Returns checked selection rectangles for one layout generation.
    fn selection_rects(
        &self,
        layout: TextLayoutId,
        range: TextRange,
        output: &mut [Rect],
    ) -> Result<usize, OxyError>;

    /// Hit-tests a point and returns index, affinity, and boundary metadata.
    fn hit_test(&self, layout: TextLayoutId, point: Point) -> Result<TextHit, OxyError>;

    /// Converts between explicitly tagged index units after boundary validation.
    fn convert_index(
        &self,
        layout: TextLayoutId,
        index: TextIndex,
        target_unit: TextIndexUnit,
    ) -> Result<TextIndex, OxyError>;
}

/// Owns rich-text content, selection, history, and input method editor state.
pub trait EditableText {
    /// Replaces a checked range and updates selection atomically.
    fn replace(&mut self, range: TextRange, text: &str) -> Result<(), OxyError>;

    /// Updates the active selection.
    fn select(&mut self, selection: Selection) -> Result<(), OxyError>;

    /// Applies a grapheme, word, or selection command.
    fn command(&mut self, command: EditCommand) -> Result<(), OxyError>;

    /// Applies one checked input method editor composition transaction atomically.
    fn apply_ime(&mut self, transaction: ImeTransaction) -> Result<(), OxyError>;

    /// Commits the active input method editor composition.
    fn commit_ime(&mut self, generation: u64) -> Result<(), OxyError>;

    /// Cancels the active input method editor composition without corrupting selection.
    fn cancel_ime(&mut self, generation: u64) -> Result<(), OxyError>;

    /// Returns the candidate-window rectangle for a checked text position.
    fn candidate_rect(&self, index: TextIndex, affinity: TextAffinity) -> Result<Rect, OxyError>;

    /// Answers one bounded input method editor query without exposing content to diagnostics.
    fn query_ime(&self, query: ImeQuery) -> Result<ImeQueryResult, OxyError>;

    /// Reverts the most recent committed edit.
    fn undo(&mut self) -> Result<(), OxyError>;

    /// Reapplies the most recently reverted edit.
    fn redo(&mut self) -> Result<(), OxyError>;
}

/// Applies incremental semantics updates and routes accessibility actions.
pub trait SemanticsBridge {
    /// Publishes one incremental update for a live view.
    fn update(
        &mut self,
        view: ViewId,
        nodes: &[SemanticsNode],
        deleted: &[SemanticsNodeId],
    ) -> Result<(), OxyError>;

    /// Routes an action to a live semantics node.
    fn perform_action(
        &mut self,
        view: ViewId,
        node: SemanticsNodeId,
        action: u32,
        payload: &[u8],
    ) -> Result<SemanticsActionResult, OxyError>;
}

/// Owns clipboard transactions for one view without diagnostic content capture.
pub trait Clipboard {
    /// Starts a bounded clipboard read for the requested MIME types.
    fn read(&mut self, view: ViewId, mime_types: &[&str]) -> Result<PlatformRequestId, OxyError>;

    /// Starts a bounded clipboard write.
    fn write(
        &mut self,
        view: ViewId,
        mime_type: &str,
        content: &[u8],
    ) -> Result<PlatformRequestId, OxyError>;

    /// Polls a clipboard transaction and writes owned content only on success.
    fn poll(
        &mut self,
        request: PlatformRequestId,
        mime_type: &mut String,
        content: &mut Vec<u8>,
    ) -> Result<PollState, OxyError>;

    /// Cancels an unpublished clipboard transaction.
    fn cancel(&mut self, request: PlatformRequestId) -> Result<(), OxyError>;
}

/// Propagates locale, direction, and platform preference changes per view.
pub trait LocaleManager {
    /// Applies a canonical BCP-47 locale list and direction to one view.
    fn update(
        &mut self,
        view: ViewId,
        locales: &[&str],
        direction: TextDirection,
    ) -> Result<(), OxyError>;
}

/// Controls bounded presentation-resource recovery without replacing framework state.
pub trait RecoveryController {
    /// Begins or advances recovery for a platform-observed fault.
    fn recover(
        &mut self,
        view: ViewId,
        fault: u32,
        attempt: u8,
        deadline_ns: u64,
        transient_memory_cap_bytes: u64,
    ) -> Result<(), OxyError>;

    /// Acknowledges release of resources superseded by recovery.
    fn acknowledge_release(
        &mut self,
        view: ViewId,
        released_bytes: u64,
        monotonic_ns: u64,
    ) -> Result<(), OxyError>;
}

/// Invokes operating-system services through an owned view context.
pub trait PlatformServices {
    /// Requests a user-visible dialog and returns an owned request identifier.
    fn request_dialog(
        &mut self,
        view: ViewId,
        request: &[u8],
    ) -> Result<PlatformRequestId, OxyError>;

    /// Sends one bounded platform message.
    fn send_message(
        &mut self,
        view: ViewId,
        channel: &str,
        bytes: &[u8],
    ) -> Result<PlatformRequestId, OxyError>;

    /// Polls an owned service request and writes the result bytes when ready.
    fn poll(
        &mut self,
        request: PlatformRequestId,
        output: &mut Vec<u8>,
    ) -> Result<PollState, OxyError>;

    /// Cancels an unpublished operating-system request.
    fn cancel(&mut self, request: PlatformRequestId) -> Result<(), OxyError>;
}

/// Selects one built-in machine-local destination admitted by explicit user policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalDiagnosticDestination {
    /// A machine-local file selected by the user through the host.
    UserSelectedMachineLocalFile,
    /// An application-local file enabled by the user.
    UserEnabledApplicationLocalFile,
    /// An in-process bounded buffer enabled by the user.
    UserEnabledMemoryBuffer,
}

/// Admits only a closed machine-local diagnostic destination.
pub trait LocalDiagnosticSinkAdmission {
    /// Concrete admitted sink type.
    type Sink: LocalDiagnosticSink;

    /// Opens one user-controlled destination with a nonzero bounded queue.
    ///
    /// The host verifies that file destinations are machine-local before this call. Remote, undeclared, or unverifiable destinations fail before a sink is created.
    fn open_local(
        &mut self,
        destination: LocalDiagnosticDestination,
        max_queued_records: std::num::NonZeroU32,
    ) -> Result<Self::Sink, OxyError>;
}

/// Receives privacy-classified local diagnostic records without blocking producers.
pub trait LocalDiagnosticSink {
    /// Attempts to copy one schema-valid record into the bounded queue without waiting for destination I/O.
    ///
    /// `Ok` acknowledges queue ownership only. A full, unavailable, or failed sink returns promptly and increments the drop counter without exposing the record elsewhere.
    fn try_emit(&self, record: &[u8]) -> Result<(), OxyError>;

    /// Returns the number of records dropped since creation.
    fn dropped_records(&self) -> u64;
}
