use ggez::graphics::{Camera3dBundle, Canvas3d, DrawParam3d, Drawable3d, Mesh3d, Mesh3dBuilder};
use std::{env, path};

use ggez::input::keyboard::KeyCode;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

struct PosMesh {
    mesh: Mesh3d,
    pos: mint::Vector3<f32>,
}

impl Drawable3d for PosMesh {
    fn draw(
        &self,
        gfx: &mut impl ggez::context::HasMut<graphics::GraphicsContext>,
        canvas: &mut Canvas3d,
        param: impl Into<DrawParam3d>,
    ) {
        let param = param.into();
        canvas.draw(gfx, &self.mesh, param.position(self.pos));
    }
}

impl PosMesh {
    fn new(mesh: Mesh3d, position: impl Into<mint::Vector3<f32>>) -> Self {
        Self {
            mesh,
            pos: position.into(),
        }
    }
}

struct MainState {
    camera: Camera3dBundle,
    meshes: Vec<PosMesh>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut camera = Camera3dBundle::default();
        camera.camera.yaw = 90.0;
        let plane = PosMesh::new(
            Mesh3dBuilder::new()
                .plane(Vec2::splat(25.0), false)
                .build(ctx),
            Vec3::new(50.0, -5.0, 0.0),
        );
        let cube = PosMesh::new(
            Mesh3dBuilder::new().cube(Vec3::splat(10.0)).build(ctx),
            Vec3::new(-50.0, -5.0, 0.0),
        );
        let pyramid = PosMesh::new(
            Mesh3dBuilder::new()
                .pyramid(Vec2::splat(25.0), 50.0, true)
                .build(ctx),
            Vec3::new(0.0, -5.0, 0.0),
        );

        Ok(MainState {
            camera,
            meshes: vec![plane, cube, pyramid],
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard.clone();
        let (yaw_sin, yaw_cos) = self.camera.camera.yaw.sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();

        if k_ctx.is_key_pressed(KeyCode::Space) {
            self.camera.camera.position.y += 1.0;
        }
        if k_ctx.is_key_pressed(KeyCode::C) {
            self.camera.camera.position.y -= 1.0;
        }
        if k_ctx.is_key_pressed(KeyCode::W) {
            self.camera.camera.position += forward;
        }
        if k_ctx.is_key_pressed(KeyCode::S) {
            self.camera.camera.position -= forward;
        }
        if k_ctx.is_key_pressed(KeyCode::D) {
            self.camera.camera.position += right;
        }
        if k_ctx.is_key_pressed(KeyCode::A) {
            self.camera.camera.position -= right;
        }
        if k_ctx.is_key_pressed(KeyCode::Right) {
            self.camera.camera.yaw += 1.0_f32.to_radians();
        }
        if k_ctx.is_key_pressed(KeyCode::Left) {
            self.camera.camera.yaw -= 1.0_f32.to_radians();
        }
        if k_ctx.is_key_pressed(KeyCode::Up) {
            self.camera.camera.pitch += 1.0_f32.to_radians();
        }
        if k_ctx.is_key_pressed(KeyCode::Down) {
            self.camera.camera.pitch -= 1.0_f32.to_radians();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas3d = Canvas3d::from_frame(ctx, &mut self.camera, Color::BLACK);
        for mesh in self.meshes.iter() {
            canvas3d.draw(ctx, mesh, DrawParam3d::default());
        }
        canvas3d.finish(ctx)?;
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        // Do ggez drawing
        let dest_point1 = Vec2::new(10.0, 210.0);
        let dest_point2 = Vec2::new(10.0, 250.0);
        canvas.draw(
            &graphics::Text::new("You can mix 3d and 2d drawing;"),
            dest_point1,
        );
        canvas.draw(
            &graphics::Text::new(
                "
                WASD: Move
                Arrow Keys: Look
                C/Space: Up and Down
                ",
            ),
            dest_point2,
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("cube", "ggez")
        .window_mode(ggez::conf::WindowMode::default().resizable(true))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}
