use {
    AddRectangle,
    BackEnd,
    Borrowed,
    Draw,
    Field,
    ImageSize,
    ImageRectangleColorContext,
    Value,
};
use triangulation::{
    rect_tri_list_xy_f32,
    rect_tri_list_rgba_f32,
    rect_tri_list_uv_f32,
};
use internal::{
    CanColor,
    CanSourceRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasSourceRectangle,
    HasTransform,
    HasViewTransform,
    SourceRectangle,
    Matrix2d,
    Scalar,
};

/// An image rectangle context.
pub struct ImageColorContext<'a, 'b, I> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current image.
    pub image: Field<'a, &'b I>,
    /// Current source rectangle.
    pub source_rect: Field<'a, SourceRectangle>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a, 'b, I> 
Clone 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn clone(&self) -> ImageColorContext<'static, 'b, I> {
        ImageColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            image: Value(*self.image.get()),
            source_rect: Value(*self.source_rect.get()),
            color: Value(*self.color.get()),
        }
    }
}

impl<'a, 'b, I> 
HasTransform<'a, Matrix2d> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b, I> 
CanTransform<'a, ImageColorContext<'a, 'b, I>, Matrix2d> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> ImageColorContext<'a, 'b, I> {
        ImageColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, I> 
HasViewTransform<'a, Matrix2d> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a, 'b, I> 
CanViewTransform<'a, ImageColorContext<'a, 'b, I>, Matrix2d>
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> ImageColorContext<'a, 'b, I> {
        ImageColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, I> 
HasColor<'a, Color> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a, 'b, I> 
CanColor<'a, ImageColorContext<'a, 'b, I>, Color> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> ImageColorContext<'a, 'b, I> {
        ImageColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
        }
    }
}

impl<'a, 'b, I> 
HasSourceRectangle<'a, SourceRectangle> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn get_source_rectangle(&'a self) -> &'a SourceRectangle {
        self.source_rect.get()
    }
}

impl<'a, 'b, I> 
CanSourceRectangle<'a, ImageColorContext<'a, 'b, I>, SourceRectangle> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn source_rectangle(
        &'a self, 
        source_rect: SourceRectangle
    ) -> ImageColorContext<'a, 'b, I> {
        ImageColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            image: Borrowed(self.image.get()),
            source_rect: Value(source_rect),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, I> 
AddRectangle<'a, ImageRectangleColorContext<'a, 'b, I>> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn rect(
        &'a self, 
        x: Scalar, 
        y: Scalar, 
        w: Scalar, 
        h: Scalar
    ) -> ImageRectangleColorContext<'a, 'b, I> {
        ImageRectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value([x, y, w, h]),
            image: Borrowed(self.image.get()),
            source_rect: Borrowed(self.source_rect.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a, 'b, B: BackEnd<I>, I: ImageSize> 
Draw<'a, B, I> 
for ImageColorContext<'a, 'b, I> {
    #[inline(always)]
    fn draw(&'a self, back_end: &mut B) {
        if back_end.supports_single_texture()
        && back_end.supports_tri_list_xy_f32_rgba_f32_uv_f32() {
            let color = self.color.get();
            let &texture = self.image.get();
            let source_rect = self.source_rect.get();
            let rect = [
                0.0, 
                0.0, 
                source_rect[2] as f64, 
                source_rect[3] as f64
            ];
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely
            // opaque or if the texture has alpha channel.
            let needs_alpha = color[3] != 1.0 
                || back_end.has_texture_alpha(texture);
            if needs_alpha { back_end.enable_alpha_blend(); }
            back_end.enable_single_texture(texture);
            back_end.tri_list_xy_f32_rgba_f32_uv_f32(
                rect_tri_list_xy_f32(*self.transform.get(), rect),
                rect_tri_list_rgba_f32(*color),
                rect_tri_list_uv_f32(texture, *source_rect)
            );
            back_end.disable_single_texture();
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

