pub mod prelude {
    pub type DrawingContextGl = drawing_impeller::ImpellerContextGl;
    pub type DrawingContextVulkan = drawing_impeller::ImpellerContextVulkan;
    pub type DrawingVulkanSwapChain = drawing_impeller::VulkanSwapchain;
    pub type DrawingSurface = drawing_impeller::ImpellerSurface;
    pub type DrawingTexture = drawing_impeller::ImpellerTexture;
    pub type DrawingDisplayList = drawing_impeller::DisplayList;
    pub type DrawingDisplayListBuilder = drawing_impeller::DisplayListBuilder;
    pub type DrawingPaint = drawing_impeller::Paint;
    pub type DrawingPath = drawing_impeller::Path;
    pub type DrawingPathBuilder = drawing_impeller::PathBuilder;
    pub type DrawingParagraph = drawing_impeller::Paragraph;
    pub type DrawingParagraphBuilder = drawing_impeller::ParagraphBuilder;
    pub type DrawingFonts = drawing_impeller::Fonts;
    pub type DrawingGlyphInfo = drawing_impeller::GlyphInfo;
    pub type DrawingLineMetrics = drawing_impeller::LineMetrics;
    pub type DrawingFragmentProgram = drawing_impeller::ImpellerFragmentProgram;
    pub type DrawingColorSourceFragment = drawing_impeller::ColorSourceFragment;
    pub type DrawingImageFilterFragment = drawing_impeller::ImageFilterFragment;

    pub use drawing_api::prelude::*;

    pub use drawing_api::euclid::rect;
}

pub mod euclid {
    pub use drawing_api::euclid::*;
}

pub use prelude::*;
