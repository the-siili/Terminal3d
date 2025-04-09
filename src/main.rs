
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

    impl Clone for Vector2 {
        fn clone(&self) -> Vector2 {
            Vector2::new(self.x, self.y)
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
    impl Add for Vector3 {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z +other.z
            }
        }
    }

    impl Clone for Vector3 {
        fn clone(&self) -> Vector3 {
            Vector3::new(self.x, self.y, self.z)
        }
    }

}

mod render {
    use std::io::empty;

    use crate::math::Vector2;
    use crate::math::Vector3;
    use crate::objects::Shape;
    extern crate bresenham;
    use bresenham::Bresenham;


    pub struct Pixel {
        point: Vector2,
        depth_index: f64,
    }
    
    pub struct Viewport {
        pub surface: Vec<Vec<char>>,
        pub w:usize,
        pub h:usize,
        pub default_c:char,
        pub cam: Vector3,
        pub cam_rotation: Vector3,
        pub fov: f64
    }

    impl Viewport{
        pub fn new(width:usize, height:usize, c:char) -> Self {
            Self{
                surface:vec![vec![c; width]; height],
                w:width,
                h:height,
                default_c: c,
                cam:Vector3::new(0.0, 0.0, -100.0),
                cam_rotation:Vector3::new(0.0, 0.0, 0.0),
                fov:90.0
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
            for (x, y) in Bresenham::new((a.x.round() as isize, a.y.round() as isize,), (b.x.round()as isize, b.y.round() as isize,)) {
                self.draw_point(&Vector2::new(x as f64, y as f64), '■');
            }
        }
        pub fn generate_line(&mut self, a: &Vector2, b: &Vector2) -> Vec<Vector2> {
            let mut points : Vec<Vector2> = Vec::<Vector2>::new();
            for (x, y) in Bresenham::new((a.x.round() as isize, a.y.round() as isize,), (b.x.round()as isize, b.y.round() as isize,)) {
                points.push(Vector2::new(x as f64, y as f64));
            }
            points
        }

        pub fn draw_shape(&mut self, vertecies: &Vec<(f64, f64, f64)>, links: Vec<(usize, usize)>){
            for (a, b) in links {
                let mut x1 = vertecies[a].0;
                let mut y1 = vertecies[a].1;
                let mut z1 = vertecies[a].2;

                let mut x2 = vertecies[b].0;
                let mut y2 = vertecies[b].1;
                let mut z2 = vertecies[b].2;

                if z1-self.cam.z <= 0.0 {
                    z1 = self.cam.z + 0.01;
                }
                if z2-self.cam.z <= 0.0{
                    z2 = self.cam.z + 0.01;
                }
                


                self.draw_line(&self.pos_to_screen(&Vector3::new(x1, y1, z1)),
                            &self.pos_to_screen( &Vector3::new(x2, y2, z2)));

            }
        }
        

        pub fn render_shape(&mut self, shape: &Shape){
            let position: Vector3 = self.rotated_point_by_cam_y(shape.position.clone());
            let mut pixels:Vec<Pixel> = Vec::new();
            for face in &shape.faces {
                let mut vertecies: Vec<(f64, f64, f64)> = Vec::new();
                for i in face {
                    let mut new_z = position.z + shape.vertecies[*i].2;
                    if new_z - self.cam.z <= 0.0 {
                        new_z = self.cam.z + 0.01;
                    }
                    vertecies.push((
                        position.x + shape.vertecies[*i].0,
                        position.y + shape.vertecies[*i].1,
                        new_z));
                    
                }


                let mut corner_pixels: Vec<Pixel> = Vec::new();
                for i in vertecies {
                    corner_pixels.push(Pixel{
                        point:self.pos_to_screen(&Vector3::new(i.0, i.1, i.2)),
                        depth_index: i.2
                    });
                }
            }
        }
        fn pos_to_screen(&self, p:&Vector3) -> Vector2 {

            let px: f64 = (p.x - self.cam.x)*self.fov / (p.z-self.cam.z);
            let py: f64 = (p.y - self.cam.y)*self.fov / (p.z-self.cam.z);
        
            Vector2::new(px, py)
        }


        fn rotated_point_by_cam_y(&mut self, point: Vector3) -> Vector3{
            let new_x = self.cam.y + ((point.x-self.cam.y) * self.cam_rotation.y.cos() + (point.z-self.cam.z)*self.cam_rotation.y.sin());
            let new_z = self.cam.z + (point.x-self.cam.y) * -self.cam_rotation.y.sin() + (point.z - self.cam.z)*self.cam_rotation.y.cos();

            Vector3::new(
                new_x,
                point.y,
                new_z
            )
        }

        pub fn clear(&mut self){
            self.surface = vec![vec![self.default_c; self.w]; self.h];
        }

        pub fn render(&self){
            print!("{}[2J", 27 as char);
            for y in &self.surface{
                for x in y{
                    print!(" {x}");
                }
                println!("");
            }
        }
    }
}

mod objects {
    use crate::Vector3;
    pub struct Shape {
        init_vertecies: Vec<(f64, f64, f64)>,
        pub vertecies: Vec<(f64, f64, f64)>,
        pub faces: Vec<Vec<usize>>,
        pub position: Vector3,
        pub rotation: Vector3
    }

    impl Shape {
        pub fn new(in_vertecies: Vec<(f64, f64, f64)>, in_faces: Vec<Vec<usize>>, in_position: Vector3, in_rotation: Vector3) -> Self {
            Self { 
                init_vertecies: in_vertecies.clone(),
                vertecies: in_vertecies.clone(),
                faces: in_faces,
                position:in_position,
                rotation:in_rotation
            }
        }

        pub fn rotate(&mut self, rot:Vector3){
            self.rotate_x(rot.x);
            self.rotate_y(rot.y);
            self.rotate_z(rot.z);
        }

        pub fn rotate_z(&mut self, rot:f64){
            self.rotation.z += rot;
            for (i, (x, y, z)) in self.vertecies.clone().iter().enumerate() {
                let new_x = x * rot.cos() - y*rot.sin();
                let new_y = x * rot.sin() + y*rot.cos();
                self.vertecies[i].0 = new_x;
                self.vertecies[i].1 = new_y;
            }
        }
        pub fn rotate_y(&mut self, rot:f64){
            self.rotation.y += rot;
            for (i, (x, y, z)) in self.vertecies.clone().iter().enumerate() {
                let new_x = x * rot.cos() + z*rot.sin();
                let new_z = x * -rot.sin() + z*rot.cos();
                self.vertecies[i].0 = new_x;
                self.vertecies[i].2 = new_z;
            }
        }

        pub fn rotate_x(&mut self, rot:f64){
            self.rotation.x += rot;
            for (i, (x, y, z)) in self.vertecies.clone().iter().enumerate() {
                let new_y = y * rot.cos() - z*rot.sin();
                let new_z = y * rot.sin() + z*rot.cos();
                self.vertecies[i].1 = new_y;
                self.vertecies[i].2 = new_z;
            }
        }

    }

}


use render::Viewport;
use math::Vector3;
use objects::Shape;


use getch_rs::{Getch, Key};

fn main() {

    println!("\x1B[?1049h");
    let mut viewport: Viewport = Viewport::new(50, 50, ' ');

    let vertecies: Vec<(f64, f64, f64)> = vec![
        (-10.0, 10.0, -10.0),
        (10.0, 10.0, -10.0),
        (-10.0, -10.0, -10.0),
        (10.0, -10.0, -10.0),
        (-10.0, 10.0, 10.0),
        (10.0, 10.0, 10.0),
        (-10.0, -10.0, 10.0),
        (10.0, -10.0, 10.0)
    ];

    let faces: Vec<Vec<usize>> = vec![vec![0, 1, 2, 3],
                                        vec![0, 2, 4, 6],
                                        vec![0, 1, 4, 5],
                                        vec![4, 5, 6, 7],
                                        vec![1, 5, 3, 8],
                                        vec![2, 6, 8, 3]];

    let mut cube : Shape = Shape::new(vertecies.clone(), 
                                        faces.clone(), 
                                        Vector3::new(0.0, 0.0, 20.0),
                                        Vector3::new(0.0, 0.0, 0.0));
    viewport.render_shape(&cube);
    viewport.render();
    loop {



        movement(&mut viewport, &mut cube);
        viewport.clear();

        //viewport.draw_shape(&vertecies, links);
        viewport.render_shape(&cube);
        viewport.render();
        //thread::sleep(Duration::from_millis(50));
    }
}


fn movement(viewport: &mut Viewport, cube: &mut Shape) {
    //let mut input = String::new();
    //stdin().read_line(&mut input).expect("Error While Input");
    let g = Getch::new();
    match g.getch() {
        Ok(Key::Char('a')) => {viewport.cam.x -= 1.0},
        Ok(Key::Char('d')) => {viewport.cam.x += 1.0},
        Ok(Key::Char('w')) => {viewport.cam.z += 1.0},
        Ok(Key::Char('s')) => {viewport.cam.z -= 1.0},
        Ok(Key::Char(' ')) => {viewport.cam.y -= 1.0},
        Ok(Key::Char('c')) => {viewport.cam.y += 1.0},
        Ok(Key::Char('l')) => {viewport.fov -= 1.0},
        Ok(Key::Char('k')) => {viewport.fov -= 1.0},
        Ok(Key::Char('x')) => {cube.rotate_x(0.1);},
        Ok(Key::Char('y')) => {cube.rotate_y(0.1);},
        Ok(Key::Char('z')) => {cube.rotate_z(0.1);},
        Ok(Key::Char('X')) => {cube.rotate_x(-0.1);},
        Ok(Key::Char('Y')) => {cube.rotate_y(-0.1);},
        Ok(Key::Char('Z')) => {cube.rotate_z(-0.1);},
        Ok(Key::Char('q')) => {println!("\x1B[?1049l"); panic!("quit");},
        _ => {},
    }
}

