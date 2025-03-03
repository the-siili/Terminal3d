
mod math {

    pub struct Vector2 {
        pub x: f64,
        pub y: f64
    }

    impl Vector2 {
        pub fn new(in_x: f64, in_y: f64) -> Self {
            Self { 
                x:in_x, 
                y:in_y
            }
        }

        pub fn distance(&self, p: &Vector2) -> f64 {
            ((p.x - self.x).powf(2.0) + (p.y - self.y).powf(2.0)).sqrt()
        }
    }
    use std::ops::Add;
    impl Add for Vector2 {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }



    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64
    }

    impl Vector3 {
        pub fn new(in_x: f64, in_y: f64, in_z: f64) -> Self {
            Self { 
                x:in_x, 
                y:in_y, 
                z:in_z }
        }

        pub fn distance(&self, p: &Vector3) -> f64 {
            ((p.x - self.x).powf(2.0) + (p.y - self.y).powf(2.0) + (p.z - self.z).powf(2.0)).sqrt()
        }
    }

}

mod render {
    use crate::math::Vector2;
    extern crate bresenham;
    use bresenham::Bresenham;

    pub struct Viewport {
        pub surface: Vec<Vec<char>>,
        pub w:usize,
        pub h:usize,
        pub default_c:char
    }

    impl Viewport{
        pub fn new(width:usize, height:usize, c:char) -> Self {
            Self{
                surface:vec![vec![c; width]; height],
                w:width,
                h:height,
                default_c: c
            }
        }

        pub fn draw_point(&mut self, p: &Vector2, c: char){
            let x = p.x + (self.w/2) as f64;
            let y = p.y  + (self.h/2) as f64;
            if 0.0 <= x  && x < self.w as f64 && 0.0 <= y && y < self.h as f64 {
                self.surface[y as usize][x as usize] = c;
            }
        }

        pub fn draw_line(&mut self, a: &Vector2, b: &Vector2){
            for (x, y) in Bresenham::new((a.x as isize, a.y as isize,), (b.x as isize, b.y as isize,)) {
                self.draw_point(&Vector2::new(x as f64, y as f64), '#');
            }
        }

        pub fn clear(&mut self){
            self.surface = vec![vec![self.default_c; self.w]; self.h];
        }

        pub fn render(&self){
            //print!("{esc}c", esc = 27 as char);
            for y in &self.surface{
                for x in y{
                    print!(" {x}");
                }
                println!("");
            }
        }
    }
}

use render::Viewport;
use math::Vector2;
use math::Vector3;
fn main() {
    let mut viewport: Viewport = Viewport::new(70, 70, '.');

    let cam_pos: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    let fov: i32 = 30;
    let length: f64 = 0.1;

    let a: Vector3 = Vector3::new(-10.0, 10.0, 0.0);
    let b: Vector3 = Vector3::new(10.0, 10.0, 16.0);

    let c: Vector3 = Vector3::new(-10.0, -10.0, 0.0);
    let d: Vector3 = Vector3::new(10.0, -10.0, 16.0);

    let ap: Vector2 = pos_to_screen(length, &cam_pos, &a);
    let bp: Vector2 = pos_to_screen(length, &cam_pos, &b);
    let cp: Vector2 = pos_to_screen(length, &cam_pos, &c);
    let dp: Vector2 = pos_to_screen(length, &cam_pos, &d);
    println!("{} {}", ap.x, ap.y);
    println!("{} {}", bp.x, bp.y);
    println!("{} {}", cp.x, cp.y);
    println!("{} {}", dp.x, dp.y);
    viewport.draw_line(&ap, &bp);
    viewport.draw_line(&ap, &cp);
    viewport.draw_line(&bp, &dp);
    viewport.draw_line(&cp, &dp);


    viewport.render();
}

fn pos_to_screen(length: f64, cam_pos: &Vector3, p:&Vector3) -> Vector2{
    let mut px: f64 = (p.x - cam_pos.x) / (length/(p.z - cam_pos.z).abs()).abs();
    let mut py: f64 = (p.y - cam_pos.y) / (length/(p.z - cam_pos.z).abs()).abs();
    
    Vector2::new(px, py)
}