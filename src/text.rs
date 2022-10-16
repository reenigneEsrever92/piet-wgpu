use piet::kurbo::*;
use piet::*;

#[derive(Clone)]
pub struct WgpuText;

impl piet::Text for WgpuText {
    type TextLayoutBuilder = WgpuTextLayoutBuilder;

    type TextLayout = WgpuTextLayout;

    fn font_family(&mut self, family_name: &str) -> Option<FontFamily> {
        todo!()
    }

    fn load_font(&mut self, data: &[u8]) -> Result<FontFamily, Error> {
        todo!()
    }

    fn new_text_layout(&mut self, text: impl TextStorage) -> Self::TextLayoutBuilder {
        todo!()
    }
}

pub struct WgpuTextLayoutBuilder;

impl piet::TextLayoutBuilder for WgpuTextLayoutBuilder {
    type Out = WgpuTextLayout;

    fn max_width(self, width: f64) -> Self {
        todo!()
    }

    fn alignment(self, alignment: TextAlignment) -> Self {
        todo!()
    }

    fn default_attribute(self, attribute: impl Into<TextAttribute>) -> Self {
        todo!()
    }

    fn range_attribute(
        self,
        range: impl std::ops::RangeBounds<usize>,
        attribute: impl Into<TextAttribute>,
    ) -> Self {
        todo!()
    }

    fn build(self) -> Result<Self::Out, Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct WgpuTextLayout;

impl TextLayout for WgpuTextLayout {
    fn size(&self) -> Size {
        todo!()
    }

    fn trailing_whitespace_width(&self) -> f64 {
        todo!()
    }

    fn image_bounds(&self) -> kurbo::Rect {
        todo!()
    }

    fn text(&self) -> &str {
        todo!()
    }

    fn line_text(&self, line_number: usize) -> Option<&str> {
        todo!()
    }

    fn line_metric(&self, line_number: usize) -> Option<LineMetric> {
        todo!()
    }

    fn line_count(&self) -> usize {
        todo!()
    }

    fn hit_test_point(&self, point: kurbo::Point) -> HitTestPoint {
        todo!()
    }

    fn hit_test_text_position(&self, idx: usize) -> HitTestPosition {
        todo!()
    }
}
