use crate::graphics::*;

#[derive(Clone)]
pub enum Segment2D {
    LineTo(Point2D),
    MoveTo(Point2D),
    QuadraticTo(Point2D, Point2D),
    CubicTo(Point2D, Point2D, Point2D),
}

#[derive(Clone)]
pub struct GradientStop {
    pub offset: f64,
    pub color: RGBA8,
}

#[derive(Clone)]
pub struct LinearGradient {
    pub stops: Vec<GradientStop>,
    pub start: Point2D,
    pub end: Point2D,
}

#[derive(Clone)]
pub struct Shadow2D {
    pub color: RGBA8,
    pub offset: Distance2D,
    pub blur: f64,
}

impl Shadow2D {
    pub fn new(color: RGBA8) -> Self {
        Shadow2D {
            color,
            offset: Distance2D::default(),
            blur: 0.,
        }
    }
    pub fn blur(mut self, amount: f64) -> Self {
        self.blur = amount;
        self
    }
    pub fn offset(mut self, distance: Distance2D) -> Self {
        self.offset = distance;
        self
    }
}

#[derive(Clone)]
pub struct RadialGradient {
    pub stops: Vec<GradientStop>,
    pub start: Point2D,
    pub start_radius: f64,
    pub end: Point2D,
    pub end_radius: f64,
}

#[derive(Clone)]
pub enum VectorTexture<T>
where
    T: ImageRepresentation,
{
    Solid(RGBA8),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Image(Box<T>),
}

#[derive(Clone)]
pub struct Stroke<T>
where
    T: ImageRepresentation,
{
    pub content: VectorTexture<T>,
    pub width: f32,
    pub cap: StrokeCapType,
    pub join: StrokeJoinType,
}

impl<T> Default for Stroke<T>
where
    T: ImageRepresentation,
{
    fn default() -> Self {
        Stroke {
            content: RGBA8::black().into(),
            cap: StrokeCapType::Butt,
            join: StrokeJoinType::Miter,
            width: 1.,
        }
    }
}

#[derive(Clone)]
pub enum StrokeCapType {
    Butt,
    Round,
}

#[derive(Clone)]
pub enum StrokeJoinType {
    Bevel,
    Round,
    Miter,
}

#[derive(Clone)]
pub struct Fill<T>
where
    T: ImageRepresentation,
{
    pub content: VectorTexture<T>,
}

#[derive(Clone)]
pub struct Path<T>
where
    T: ImageRepresentation,
{
    pub orientation: Transform2D,
    pub segments: Vec<Segment2D>,
    pub stroke: Option<Stroke<T>>,
    pub fill: Option<Fill<T>>,
    pub shadow: Option<Shadow2D>,
    pub closed: bool,
}

#[derive(Default)]
pub struct GeometryBuilder {
    segments: Vec<Segment2D>,
}

impl GeometryBuilder {
    pub fn new() -> Self {
        GeometryBuilder::default()
    }
    pub fn line_to(mut self, to: Point2D) -> Self {
        self.segments.push(Segment2D::LineTo(to));
        self
    }
    pub fn quadratic_to(mut self, to: Point2D, handle: Point2D) -> Self {
        self.segments
            .push(Segment2D::QuadraticTo(to, handle));
        self
    }
    pub fn bezier_to(mut self, to: Point2D, handle_1: Point2D, handle_2: Point2D) -> Self {
        self.segments
            .push(Segment2D::CubicTo(to, handle_1, handle_2));
        self
    }
    pub fn done<T>(self) -> Builder<T>
    where
        T: ImageRepresentation,
    {
        Builder::new(self.segments)
    }
}

pub struct GeometryPrimitive {}

impl GeometryPrimitive {
    pub fn rectangle<T>(width: f64, height: f64) -> Builder<T>
    where
        T: ImageRepresentation,
    {
        Builder::new(vec![
            Segment2D::LineTo(Point2D::new(width, 0.)),
            Segment2D::LineTo(Point2D::new(width, height)),
            Segment2D::LineTo(Point2D::new(0., height)),
        ])
    }
    pub fn rounded_rectangle<T>(width: f64, height: f64, radius: f64) -> Builder<T>
    where
        T: ImageRepresentation,
    {
        Builder::new(vec![
            Segment2D::MoveTo(Point2D::new(radius, 0.)),
            Segment2D::LineTo(Point2D::new(width - radius, 0.)),
            Segment2D::CubicTo(
                Point2D::new(width, radius),
                Point2D::new(
                    width - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    0.,
                ),
                Point2D::new(
                    width,
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            ),
            Segment2D::LineTo(Point2D::new(width, height - radius)),
            Segment2D::CubicTo(
                Point2D::new(width - radius, height),
                Point2D::new(
                    width,
                    height - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
                Point2D::new(
                    width - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    height,
                ),
            ),
            Segment2D::LineTo(Point2D::new(radius, height)),
            Segment2D::CubicTo(
                Point2D::new(0., height - radius),
                Point2D::new(
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    height,
                ),
                Point2D::new(
                    0.,
                    height - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            ),
            Segment2D::LineTo(Point2D::new(0., radius)),
            Segment2D::CubicTo(
                Point2D::new(radius, 0.),
                Point2D::new(0., radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)),
                Point2D::new(radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.),
            ),
        ])
    }
    pub fn square<T>(side_length: f64) -> Builder<T>
    where
        T: ImageRepresentation,
    {
        GeometryPrimitive::rectangle(side_length, side_length)
    }
    pub fn rounded_square<T>(side_length: f64, radius: f64) -> Builder<T>
    where
        T: ImageRepresentation,
    {
        GeometryPrimitive::rounded_rectangle(side_length, side_length, radius)
    }
}

pub struct Builder<T>
where
    T: ImageRepresentation,
{
    closed: bool,
    geometry: Vec<Segment2D>,
    fill: Option<Fill<T>>,
    stroke: Option<Stroke<T>>,
    shadow: Option<Shadow2D>,
}

impl<T> Builder<T>
where
    T: ImageRepresentation,
{
    pub fn new(geometry: Vec<Segment2D>) -> Self {
        Builder {
            closed: false,
            geometry,
            fill: None,
            shadow: None,
            stroke: None,
        }
    }
    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }
    pub fn fill(mut self, fill: Fill<T>) -> Self
    where
        T: ImageRepresentation,
    {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, stroke: Stroke<T>) -> Self
    where
        T: ImageRepresentation,
    {
        self.stroke = Some(stroke);
        self
    }
    pub fn shadow(mut self, shadow: Shadow2D) -> Self
    where
        T: ImageRepresentation,
    {
        self.shadow = Some(shadow);
        self
    }
    pub fn finalize(self) -> Path<T>
    where
        T: ImageRepresentation,
    {
        Path {
            closed: self.closed,
            segments: self.geometry,
            orientation: Transform2D::default(),
            fill: self.fill,
            shadow: self.shadow,
            stroke: self.stroke,
        }
    }
}

pub struct StrokeBuilder<T>
where
    T: ImageRepresentation,
{
    stroke: Stroke<T>,
}

impl<T> StrokeBuilder<T>
where
    T: ImageRepresentation,
{
    pub fn new(content: VectorTexture<T>, width: f32) -> Self {
        let mut builder = StrokeBuilder {
            stroke: Stroke::default(),
        };
        builder.stroke.content = content;
        builder.stroke.width = width;
        builder
    }
    pub fn cap_round(mut self) -> Self {
        self.stroke.cap = StrokeCapType::Round;
        self
    }
    pub fn join_bevel(mut self) -> Self {
        self.stroke.join = StrokeJoinType::Bevel;
        self
    }
    pub fn join_round(mut self) -> Self {
        self.stroke.join = StrokeJoinType::Round;
        self
    }
    pub fn finalize(self) -> Stroke<T> {
        self.stroke
    }
}
