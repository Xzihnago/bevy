//! This module contains basic node bundles used to build UIs

#[cfg(feature = "bevy_text")]
use crate::widget::TextFlags;
use crate::{
    widget::{Button, UiImageSize},
    BackgroundColor, BorderColor, BorderRadius, ContentSize, FocusPolicy, Interaction, Node,
    ScrollPosition, Style, UiImage, UiMaterial, ZIndex,
};
use bevy_asset::Handle;
#[cfg(feature = "bevy_text")]
use bevy_color::Color;
use bevy_ecs::bundle::Bundle;
use bevy_render::view::{InheritedVisibility, ViewVisibility, Visibility};
#[cfg(feature = "bevy_text")]
use bevy_text::{
    BreakLineOn, CosmicBuffer, JustifyText, Text, TextLayoutInfo, TextSection, TextStyle,
};
use bevy_transform::prelude::{GlobalTransform, Transform};

/// The basic UI node.
///
/// Contains the [`Node`] component and other components required to make a container.
///
/// See [`node_bundles`](crate::node_bundles) for more specialized bundles like [`TextBundle`].
#[derive(Bundle, Clone, Debug, Default)]
pub struct NodeBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The scroll position of the node,
    pub scroll_position: ScrollPosition,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

/// A UI node that is an image
///
/// # Extra behaviours
///
/// You may add one or both of the following components to enable additional behaviours:
/// - [`ImageScaleMode`](bevy_sprite::ImageScaleMode) to enable either slicing or tiling of the texture
/// - [`TextureAtlas`](bevy_sprite::TextureAtlas) to draw a specific section of the texture
#[derive(Bundle, Debug, Default)]
pub struct ImageBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The calculated size based on the given image
    pub calculated_size: ContentSize,
    /// The image of the node.
    ///
    /// To tint the image, change the `color` field of this component.
    pub image: UiImage,
    /// The color of the background that will fill the containing node.
    pub background_color: BackgroundColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// The size of the image in pixels
    ///
    /// This component is set automatically
    pub image_size: UiImageSize,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ImageBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

#[cfg(feature = "bevy_text")]
/// A UI node that is text
///
/// The positioning of this node is controlled by the UI layout system. If you need manual control,
/// use [`Text2dBundle`](bevy_text::Text2dBundle).
#[derive(Bundle, Debug, Default)]
pub struct TextBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// Contains the text of the node
    pub text: Text,
    /// Cached cosmic buffer for layout
    pub buffer: CosmicBuffer,
    /// Text layout information
    pub text_layout_info: TextLayoutInfo,
    /// Text system flags
    pub text_flags: TextFlags,
    /// The calculated size based on the given image
    pub calculated_size: ContentSize,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `TextBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,
}

#[cfg(feature = "bevy_text")]
impl TextBundle {
    /// Create a [`TextBundle`] from a single section.
    ///
    /// See [`Text::from_section`] for usage.
    pub fn from_section(value: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: Text::from_section(value, style),
            ..Default::default()
        }
    }

    /// Create a [`TextBundle`] from a list of sections.
    ///
    /// See [`Text::from_sections`] for usage.
    pub fn from_sections(sections: impl IntoIterator<Item = TextSection>) -> Self {
        Self {
            text: Text::from_sections(sections),
            ..Default::default()
        }
    }

    /// Returns this [`TextBundle`] with a new [`JustifyText`] on [`Text`].
    pub const fn with_text_justify(mut self, justify: JustifyText) -> Self {
        self.text.justify = justify;
        self
    }

    /// Returns this [`TextBundle`] with a new [`Style`].
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Returns this [`TextBundle`] with a new [`BackgroundColor`].
    pub const fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = BackgroundColor(color);
        self
    }

    /// Returns this [`TextBundle`] with soft wrapping disabled.
    /// Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, will still occur.
    pub const fn with_no_wrap(mut self) -> Self {
        self.text.linebreak_behavior = BreakLineOn::NoWrap;
        self
    }
}

#[cfg(feature = "bevy_text")]
impl<I> From<I> for TextBundle
where
    I: Into<TextSection>,
{
    fn from(value: I) -> Self {
        Self::from_sections(vec![value.into()])
    }
}

/// A UI node that is a button
///
/// # Extra behaviours
///
/// You may add one or both of the following components to enable additional behaviours:
/// - [`ImageScaleMode`](bevy_sprite::ImageScaleMode) to enable either slicing or tiling of the texture
/// - [`TextureAtlas`](bevy_sprite::TextureAtlas) to draw a specific section of the texture
#[derive(Bundle, Clone, Debug)]
pub struct ButtonBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub button: Button,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// The image of the node
    pub image: UiImage,
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Default for ButtonBundle {
    fn default() -> Self {
        Self {
            node: Default::default(),
            button: Default::default(),
            style: Default::default(),
            interaction: Default::default(),
            focus_policy: FocusPolicy::Block,
            border_color: Default::default(),
            border_radius: Default::default(),
            image: Default::default(),
            background_color: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

/// A UI node that is rendered using a [`UiMaterial`]
///
/// Adding a `BackgroundColor` component to an entity with this bundle will ignore the custom
/// material and use the background color instead.
#[derive(Bundle, Clone, Debug)]
pub struct MaterialNodeBundle<M: UiMaterial> {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The [`UiMaterial`] used to render the node.
    pub material: Handle<M>,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl<M: UiMaterial> Default for MaterialNodeBundle<M> {
    fn default() -> Self {
        Self {
            node: Default::default(),
            style: Default::default(),
            material: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}
