use show_image::{ImageView, ImageInfo, create_window, glam::{vec2, Vec2}};
use rand::Rng;

#[show_image::main]
fn main() {
    let mut window = create_window("image", Default::default()).unwrap();
    // get a random number of particles
    let mut rng = rand::thread_rng();
    //for 1000 iterations
    for _ in 0..100 {
        let mut particles = Vec::new();
        for _ in 0..rng.gen_range(100..200) {
            //create a new particle as a vector of 2 floats
            particles.push(vec2(0.0, 0.0));
        }
        //give the particles X and Y coordinates
        for particle in &mut particles {
            particle.x = rng.gen_range(0.0..1.0);
            particle.y = rng.gen_range(0.0..1.0);
        }
        display_image( particles, &mut window);
        
    }
}

fn display_image( particles: Vec<Vec2>, window: &mut show_image::WindowProxy)  {
    let width = 1920;
    let height = 1080;
    let channels = 3;
    let mut pixel_data = vec![0; width * height * channels];
    for pixel in pixel_data.chunks_mut(3) {
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
    }
    for particle in particles {
        let x = particle.x;
        let y = particle.y;
        let pos_x = (x * width as f32) as usize;
        let pos_y = (y * height as f32) as usize;
        let index = (pos_y * width + pos_x) * channels;
        pixel_data[index] = 255;
        pixel_data[index + 1] = 255;
        pixel_data[index + 2] = 255;
    }

    let pixel_data = pixel_data.into_boxed_slice();
    let image = ImageView::new(ImageInfo::rgb8(1920, 1080), &pixel_data);

    window.set_image("image-001", image).unwrap();
    // return the window
    //window.wait_until_destroyed().unwrap();
}