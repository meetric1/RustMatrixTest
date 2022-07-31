use sfml::graphics::RenderWindow;
use sfml::graphics::*;
use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Vector4;
use cgmath::*;
use sfml::system::Vector2f;
use sfml::window::Event;
use sfml::window::Key;
use sfml::window::mouse::Button;
use std::f32::consts::PI;
use std::time::Instant;
use std::time::Duration;

static FPS : f32 = 128.0;
static GRAVITY: Vector3<f32> = Vector3::new(0.0, 1.0 as f32 * 9.8, 0.0);

include!("Object.rs");
include!("all_collision.rs");

fn generate_matrix_fixed(c0r0: f32, c1r0: f32, c2r0: f32, c3r0: f32,
    c0r1: f32, c1r1: f32, c2r1: f32, c3r1: f32,
    c0r2: f32, c1r2: f32, c2r2: f32, c3r2: f32,
    c0r3: f32, c1r3: f32, c2r3: f32, c3r3: f32) -> Matrix4<f32> {
    Matrix4::new(
        c0r0, c0r1, c0r2, c0r3,
        c1r0, c1r1, c1r2, c1r3,
        c2r0, c2r1, c2r2, c2r3,
        c3r0, c3r1, c3r2, c3r3
    )
}

fn set_pos(matrix: &mut Matrix4<f32>, pos: Vector3<f32> ) {
    matrix[3][0] = pos.x;
    matrix[3][1] = pos.y;
    matrix[3][2] = pos.z;
}

fn get_pos(matrix: &Matrix4<f32>) -> Vector3<f32> {
    return Vector3::new(matrix[3][0], matrix[3][1], matrix[3][2]);
}

fn main() {
    // initialize window stuff
    let inv_fps = 1.0 / FPS as f32;
    let mut delta_time : f32 = 0.0;
    let mut window = RenderWindow::new((800, 600), "matrix", sfml::window::Style::CLOSE, &Default::default());
    window.set_mouse_cursor_visible(false);
    let font : sfml::SfBox<Font> = Font::from_file("C:/Windows/Fonts/Arial.ttf").unwrap();
    let mut fps_counter = Text::new("FPS: ", &font, 20);
    fps_counter.set_position(Vector2f::new(10.0, 10.0));
    fps_counter.set_fill_color(Color::WHITE);

    // our objects
    let mut objects: Vec<Object> = Vec::new();
    objects.push(Object::new(Vector3::new(0.1, 0.1, 0.1), 1.0, 0.5));

    // main camera matrix
    let projection_matrix = generate_matrix_fixed(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.003, 0.0
    );

    // camera offset matrix
    let mut position_matrix = generate_matrix_fixed(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    // mouse stuff
    let mut mouse_angle = Vector2::new(0.0 as f32, 0.0);

    // main loop
    loop {
        let t = Instant::now();
        
        window.clear(Color::BLACK);

        // get mouse delta
        if window.has_focus() {
            let mouse_pos = Vector2::new(800.0 - window.mouse_position().x as f32, window.mouse_position().y as f32) - Vector2::new(window.size().x as f32, window.size().y as f32) * 0.5;
            window.set_mouse_position(sfml::system::Vector2i::from(((window.size().x / 2) as i32, (window.size().y / 2) as i32)));
            mouse_angle += mouse_pos * 0.005;
            mouse_angle.y = mouse_angle.y.max(-PI * 0.5).min(PI * 0.5);
        }

        // apply mouse delta to new eye angles
        let mut eye_matrix = Matrix4::from_angle_y(Rad(mouse_angle.x));  // yaw rotation
        eye_matrix = Matrix4::from_angle_x(Rad(mouse_angle.y)) * eye_matrix;  // pitch rotation

        // most expensive thing lmao
        calc_all_physics(&mut objects);

        // sort by distance
        objects.sort_by(|a, b| {(eye_matrix * position_matrix * b.get_pos_matrix()).z.partial_cmp(&(eye_matrix * position_matrix * a.get_pos_matrix()).z).unwrap()});
        
        // render all objects
        for object in &mut objects {
            object.calc_physics(delta_time * 2.0);
            //println!("{}, {}, {}", object.pos.x, object.pos.y, object.pos.z);

            // the order you multiply matrices is important!
            let mut new_point = projection_matrix * eye_matrix * position_matrix * object.get_pos_matrix();
            new_point /= new_point.w;

            // if behind camera, don't render
            if new_point.z < 0.0 { 
                continue; 
            }

            let offset_x = window.size().x as f32 * 0.5;
            let offset_y = window.size().y as f32 * 0.5;

            // draw object
            let visual_radius = new_point.z * object.radius;  // not actual radius, just the radius of what is seen on screen
            object.render_object.set_radius(visual_radius);
            object.render_object.set_fill_color(Color::rgb(((object.pos.x + 10.0) * 12.5) as u8, ((object.pos.y + 10.0) * 12.5) as u8, ((object.pos.z + 10.0) * 12.5) as u8));
            object.render_object.set_origin(Vector2f::new(visual_radius, visual_radius));
            object.render_object.set_position(Vector2f::new(new_point.x + offset_x, new_point.y + offset_y));
            object.render_object.set_outline_thickness(1.0);
            object.render_object.set_outline_color(Color::BLACK);
            window.draw(&object.render_object);

            let mypos = get_pos(&position_matrix);
            let dist = 1000.0 / ((object.pos + mypos).magnitude2());
            if dist > 50.0 {
                object.force = (object.pos + mypos) * dist;
            }
        }

        match window.poll_event() {
            Some(Event::Closed) => {    // kill loop on close event
                return;
            }
            Some(Event::MouseButtonPressed{button, ..}) => {
                if button == Button::LEFT {
                    for z in -2 .. 3 {
                        for y in -2 .. 3 {
                            for x in -2 .. 3 {
                                objects.push(Object::new(Vector3::new(x as f32, y as f32, z as f32), 1.0, 0.5));
                            }
                        }
                    }
                }

                if button == Button::RIGHT {
                    objects = Vec::new();
                }
            }
            _ => {}     // ignore other events
        }



        // WASD movement
        let move_speed = 5000.0 * delta_time;
        if Key::is_pressed(Key::W) {
            let eye_forward = Vector3::new(-eye_matrix[0][2], -eye_matrix[1][2], -eye_matrix[2][2]);
            let new_pos = get_pos(&position_matrix) + eye_forward * inv_fps * move_speed;
            set_pos(&mut position_matrix, new_pos);
        }

        if Key::is_pressed(Key::A) {
            let eye_forward = Vector3::new(-eye_matrix[0][0], -eye_matrix[1][0], -eye_matrix[2][0]);
            let new_pos = get_pos(&position_matrix) - eye_forward * inv_fps * move_speed;
            set_pos(&mut position_matrix, new_pos);
        }

        if Key::is_pressed(Key::S) {
            let eye_forward = Vector3::new(-eye_matrix[0][2], -eye_matrix[1][2], -eye_matrix[2][2]);
            let new_pos = get_pos(&position_matrix) - eye_forward * inv_fps * move_speed;
            set_pos(&mut position_matrix, new_pos);
        }

        if Key::is_pressed(Key::D) {
            let eye_forward = Vector3::new(-eye_matrix[0][0], -eye_matrix[1][0], -eye_matrix[2][0]);
            let new_pos = get_pos(&position_matrix) + eye_forward * inv_fps * move_speed;
            set_pos(&mut position_matrix, new_pos);
        }

        // fps counter
        let sleep_time = inv_fps as f64 - t.elapsed().as_secs_f64();
        if sleep_time > 0.0 {spin_sleep::sleep(Duration::from_secs_f64(sleep_time))}
        delta_time = t.elapsed().as_secs_f32();
        fps_counter.set_string(&format!("FPS: {}", 1.0 / delta_time));
        window.draw(&fps_counter);

        // draw window
        window.display();
    }
}