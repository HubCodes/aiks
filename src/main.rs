use minifb::{MouseMode, Window, WindowOptions, ScaleMode, Scale};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder, Point, Transform, StrokeStyle, Color, Gradient, GradientStop, Spread};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use font_kit::loaders::default::Font;

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

fn main() {
    let mut window = Window::new("Raqute", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();
    let font: Font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    loop {
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        let mut pb = PathBuilder::new();
        if let Some(pos) = window.get_mouse_pos(MouseMode::Clamp) {
            //pb.move_to(pos.0, pos.1);
            pb.arc(pos.0 + 50., pos.1 + 50., 10., 0., 0.5 * std::f32::consts::PI);
            pb.line_to(pos.0, pos.1 + 60.);
            pb.arc(pos.0, pos.1 + 50., 10., 0.5 * std::f32::consts::PI, 0.5 * std::f32::consts::PI);
            pb.line_to(pos.0 - 10., pos.1);
            pb.arc(pos.0, pos.1, 10., std::f32::consts::PI, 0.5 * std::f32::consts::PI);
            pb.line_to(pos.0 + 50., pos.1 - 10.);
            pb.arc(pos.0 + 50., pos.1, 10., 3. * std::f32::consts::PI / 2., 0.5 * std::f32::consts::PI);
            pb.line_to(pos.0 + 60., pos.1 + 50.);
            let path = pb.finish();
            let gradient = Source::new_radial_gradient(
                Gradient {
                    stops: vec![
                        GradientStop {
                            position: 0.,
                            color: Color::new(0xff, 0, 0xff, 0),
                        },
                        GradientStop {
                            position: 0.8,
                            color: Color::new(0xff, 0xff, 0xff, 0xff),
                        },
                        GradientStop {
                            position: 1.0,
                            color: Color::new(0xff, 0xff, 0, 0xff),
                        },
                    ],
                },
                Point::new(150., 150.),
                128.,
                Spread::Pad,
            );
            let solid = Source::Solid(SolidSource::from(Color::new(255, 0, 0, 0)));
            //dt.fill(&path, &solid, &DrawOptions::new());
            dt.stroke(&path, &solid, &StrokeStyle::default(), &DrawOptions::new());

            let pos_string = format!("{:?}", pos);
            dt.draw_text(
                &font,
                36.,
                &pos_string,
                Point::new(50., 50.),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                &DrawOptions::new());
            window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
        }
    }
}
