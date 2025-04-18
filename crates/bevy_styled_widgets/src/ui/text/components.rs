use bevy::{prelude::*, text::FontWeight};

use super::TextBuilder;

// Text variants that match common typographic roles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum TextVariant {
    Header1, // Largest heading
    Header2, // Second level heading
    Header3, // Third level heading
    Body,    // Standard body text
    Caption, // Small text for captions/labels
    Small,   // Even smaller text
    Quote,   // For blockquotes or testimonials
    Code,    // Text for code snippets
    Label,   // Form labels or section headers
    Error,   // Error messages
    Success, // Success messages
    Warning, // Warning messages
    Info,    // Informational messages
    Custom,  // For custom styling
}

impl Default for TextVariant {
    fn default() -> Self {
        TextVariant::Body
    }
}

#[derive(Component)]
pub struct StyledText {
    pub variant: TextVariant,
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub alignment: Option<JustifyText>,
    pub line_height: Option<f32>,
    pub max_width: Option<f32>,
    pub selectable: bool,
}

impl StyledText {
    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }

    pub fn new<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder().content(content).build()
    }

    pub fn header1<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Header1)
            .content(content)
            .build()
    }

    pub fn header2<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Header2)
            .content(content)
            .build()
    }

    pub fn header3<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Header3)
            .content(content)
            .build()
    }

    pub fn body<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Body)
            .content(content)
            .build()
    }

    pub fn caption<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Caption)
            .content(content)
            .build()
    }

    pub fn small<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Small)
            .content(content)
            .build()
    }

    pub fn error<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Error)
            .content(content)
            .build()
    }

    pub fn success<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Success)
            .content(content)
            .build()
    }

    pub fn warning<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Warning)
            .content(content)
            .build()
    }

    pub fn info<S: Into<String>>(content: S) -> impl Bundle {
        Self::builder()
            .variant(TextVariant::Info)
            .content(content)
            .build()
    }
}
