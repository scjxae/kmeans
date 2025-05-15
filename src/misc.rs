pub use plotters::prelude::*;
use nalgebra::{DVector, Matrix2};
use plotters::{backend::BitMapBackend, coord::Shift, drawing::DrawingArea};

pub const MAKE_GIF : bool       = false;
pub const MARGIN   : u32        = 50;
pub const WIDTH    : u32        = 1000;
pub const HEIGHT   : u32        = 1000;

#[derive(Debug, Clone, Copy)]
pub struct Point{pub x: f64,
                 pub y: f64}

impl Default for Point {
    fn default() -> Self {
        Self{x: 0.0, y: 0.0}
    }
}

impl From<Point> for (f64, f64) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self{x, y}
    }
}

type DAARG<'a> = DrawingArea<BitMapBackend<'a>, Shift>;
pub struct MinMax { pub xmin: f64, pub xmax: f64, pub ymin: f64, pub ymax: f64 }
impl       MinMax { pub fn x(&self) -> (f64, f64) {(self.xmin, self.xmax)} }
impl       MinMax { pub fn y(&self) -> (f64, f64) {(self.ymin, self.ymax)} }

#[inline(always)]
pub fn mahald(x: &Point, y: &Point, inv_cov: &Matrix2<f64>) -> f64 {
    let diff = DVector::from_vec(vec![x.x - y.x, x.y - y.y]);
    let dist = diff.transpose() * inv_cov * diff;
    dist[(0, 0)].sqrt()
}

#[inline(always)]
// musze przyjac argument macierzy nawet jak go nie wykorzystuje
// aby sygnatura funkcji zgadzala sie dla zmiennej d w funkcji kmeans
pub fn eud(x: &Point, y: &Point, _dummy: &Matrix2<f64>) -> f64 {
    let a = x.x - y.x;
    let b = x.y - y.y;
    f64::sqrt(a*a + b*b)
}

#[inline(always)]
pub fn normalize(m: &f64, range: &(f64, f64)) -> f64 {
    (m - range.0) / (range.1 - range.0)
}


fn _make_points() -> Vec<Point> {
    let mut points =  Vec::<Point>::new();
    for _ in 0..1000 {      
        points.push(makerand_range(Point{x: -10., y: 10.},Point{x: -10., y: 10.}));
    }
    points
}

pub fn makerand_range(x: Point, y: Point) -> Point {
    (rand::random_range(x.x..x.y),
     rand::random_range(y.x..y.y)).into()
}

pub fn get_dem_groups(p: &Vec<Vec<i32>>) -> Vec<usize> {
    
    let groups: Vec<usize> = p
        .iter() // przechodze przez wszystkie elementy
        .map(|x| x.iter() // przeksztalcam kazdy element
             .position(|y| *y == 1) // na index gdzie jest 1
             .expect("ups")) // a jak nie ma to upsik
        .collect(); // zbieram nowe elementy do wektora

    groups
}

pub fn get_min_max(points: &Vec<Point>) -> MinMax {
    let foo = {
        let (mut xmin, mut xmax) = (f64::MAX, f64::MIN);
        let (mut ymin, mut ymax) = (f64::MAX, f64::MIN);
        for p in points {
            if p.x < xmin {xmin = p.x};
            if p.x > xmax {xmax = p.x};
            if p.y < ymin {ymin = p.y};
            if p.y > ymax {ymax = p.y};
        }
        MinMax { xmin, xmax, ymin, ymax }
    };
    foo
}

pub const fn generate_colors() -> [(RGBAColor, bool, u32); 14] {

    use plotters::style::colors::{CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW, BLUE, full_palette::*};

    const fn a(c: RGBColor) -> (RGBAColor, bool, u32) {
        (RGBAColor(c.0, c.1, c.2, 1f64),true, 3)
    }
    [
        a(RED     ),
        a(BLUE    ),
        a(YELLOW  ),
        a(CYAN    ),
        a(PURPLE  ),
        a(BROWN   ),
        a(PINK    ),
        a(WHITE   ),
        a(GREEN   ),
        a(MAGENTA ),
        a(ORANGE  ),
        a(TEAL    ),
        a(BLUEGREY),
        a(GREY    )
    ]
}

pub fn make_plot_area(x: &Vec<Point>, drawing_area: &DAARG, k: i32, m: bool, foo: &MinMax, label: &String) {

    //niestety ten jebany textstyle nie moze byc wziety jako referencja
    //tylko trzeba zajebac ownership i wkurwic programiste
    fn make_font(da: &DAARG, size: i32) -> TextStyle<'static> { TextStyle::from(("Iosevka Curly", size).into_text_style(da)).color(&WHITE) }
    
    let light_style = ShapeStyle {
        color: WHITE.to_rgba().mix(0.0625),
        filled: true,
        stroke_width: 1,
    };

    let bold_style = ShapeStyle {
        color: WHITE.to_rgba().mix(0.125),
        ..light_style
    };
    
    let text_style = make_font(drawing_area, 22);

    let title =
        format!("N: {}    K: {}    min_x:  {:.2}  min_y:  {:.2}  max_x:  {:.2}  max_y:{:.2}  mahalanobis: {}",
                x.len(), k, foo.xmin, foo.ymin, foo.xmax, foo.ymax, m);
    
    drawing_area.titled(&title, &text_style).ok();
    let mut chart = ChartBuilder::on(drawing_area)
        .margin(MARGIN)
        .set_label_area_size(LabelAreaPosition::Bottom, MARGIN)
        .set_label_area_size(LabelAreaPosition::Left, MARGIN)
        .build_cartesian_2d(foo.xmin..foo.xmax, foo.ymin..foo.ymax)
        .unwrap();

    let text_style = make_font(drawing_area, 20);
    
    chart
        .configure_mesh()
        .light_line_style(light_style)
        .bold_line_style(bold_style)
        .axis_style(&WHITE)
        .label_style(text_style)
        .draw()
        .unwrap();

    let text_style = make_font(drawing_area, 24);
    
    let text_size = drawing_area.estimate_text_size(&label, &text_style).unwrap();
    let x0 : i32 = ((WIDTH / 2) - (text_size.0 / 2)) as i32;
    let y  : i32 = (HEIGHT - MARGIN) as i32;
        drawing_area.draw_text(label.as_str(), &text_style, (x0, y)).unwrap();

}


#[inline(always)]
fn get_point_to_plot(x: &Point, foo: &MinMax) -> (i32, i32) {
    let m = MARGIN as f64;
    let x1 = normalize(&x.x, &foo.x()) * (WIDTH  as f64 - m * 3.05);
    let y1 = normalize(&x.y, &foo.y()) * (HEIGHT as f64 - m * 3.1);
    (x1 as i32 + (m as i32 + 1) * 2, y1 as i32 + m as i32 + 4)
}

//nie zesraj sie
#[allow(dead_code)]
fn make_points() -> Vec<Point> {
    let mut points =  Vec::<Point>::new();
    for _ in 0..1000 {      
        points.push(makerand_range(Point{x: -10., y: 10.},Point{x: -10., y: 10.}));
    }
    points
}

pub fn plot_gif(buffer: &Vec<Point>, cidx: &Vec<usize>, buf: &DAARG, text: &Vec<String>) {

    if !MAKE_GIF {return;}
    
    let colors = generate_colors();
    let csize = colors.len();
    let foo = get_min_max(buffer);
    let p: Vec<(i32, i32)> = buffer.iter().map(|x| get_point_to_plot(x, &foo)).collect();
    for (x, y) in p.iter().zip(cidx) {
        let el = Circle::<(i32, i32), i32>::new(
            *x,
            5,
            // o ja pierdole zjeb nie zaimplementowal into() dla shapestyle
            unsafe { std::mem::transmute::<(RGBAColor, bool, u32), ShapeStyle>(colors[*y % csize]) });
        buf.draw(&el).ok();
    };

    //trail and error poki dziala
    let text_style =
        TextStyle::from(("Iosevka Curly", 24).into_text_style(buf)).color(&WHITE);
    let mi = MARGIN as i32;
    let mf = MARGIN as f64;
    let textw = text.iter().max_by_key(|x| x.len()).unwrap();
    let textf  = 24 / 2;
    let w2 = buf.estimate_text_size(textw.as_str(), &text_style).unwrap().0;
    let w2 = mi * 2 + w2 as i32 + textf as i32;
    let h2 = mf * text.len() as f64;
    let width  = (mi * 2, w2 as i32);
    let height = (mi    , h2 as i32);
    
    buf.draw(&Rectangle::new(
        [(width.0, height.0),
         (width.1, height.1)], BLACK.filled())
    ).ok();

    for (i, t) in text.iter().enumerate() {
        buf.draw_text(t, &text_style, (mi * 2 + 10, mi + 25 * i as i32)).unwrap();
    }
    
    buf.present().ok();
}
