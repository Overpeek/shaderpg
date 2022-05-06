#![feature(generic_associated_types)]
#![feature(let_else)]

//

use anyhow::{format_err, Result};
use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};
use instant::Instant;
use srs2dge::{label, prelude::*, shader::Layout};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};
use wasm_bindgen::prelude::*;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, Device, PipelineLayoutDescriptor,
    ShaderStages,
};

//

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//

const MAX_CHARS: usize = 2000;
const VERT_SHADER: &str = include_str!("../res/shaderpg.vert.wgsl");

//

struct App {
    target: Target,

    ws: WindowState,
    ul: UpdateLoop,

    // text for error messages
    glyphs: Glyphs,
    text_shader: TextShader,
    text_ubo: UniformBuffer<Mat4>,
    text_vbo: VertexBuffer,
    text_ibo: IndexBuffer,
    text_len: u32,
    text_upload: Option<(Vec<DefaultVertex>, Vec<DefaultIndex>)>,

    main_ubo_data: Ubo,
    main_ubo: UniformBuffer<Ubo>,
    main_shader: Option<CustomShader>,

    timer: Instant,
    receiver: Receiver<String>,
}

//

impl App {
    async fn init(
        target: &EventLoopTarget,
        canvas_div_id: &str,
        receiver: Receiver<String>,
    ) -> Self {
        let engine = Engine::new();
        let target = engine
            .new_target_element_id(Arc::new(Window::new(target).unwrap()), canvas_div_id)
            .await;
        // let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let ul = UpdateLoop::new(UpdateRate::PerSecond(1));

        let mut glyphs = Glyphs::new(&target, Rect::new(512, 512));
        glyphs
            .add_font_bytes(include_bytes!("../res/fira/font.ttf"))
            .unwrap();
        let text_shader = TextShader::new(&target);
        let text_ubo = UniformBuffer::new(&target, 1);
        let text_vbo = VertexBuffer::new(&target, MAX_CHARS * 4);
        let text_ibo = IndexBuffer::new(&target, MAX_CHARS * 6);
        let text_len = 0;
        let text_upload = None;

        let main_ubo_data = Ubo {
            aspect: 0.0,
            time: 0.0,
            cursor: Vec2::new(0.0, 0.0),
        };
        let main_ubo = UniformBuffer::new_single(&target, main_ubo_data);
        let main_shader = None;

        let timer = Instant::now();

        Self {
            target,

            ws,
            ul,

            glyphs,
            text_shader,
            text_ubo,
            text_vbo,
            text_ibo,
            text_len,
            text_upload,

            main_ubo_data,
            main_ubo,
            main_shader,

            timer,
            receiver,
        }
    }

    fn recompile(&mut self, source: String) {
        self.main_shader = match CustomShader::new(&self.target, source) {
            Ok(shader) => {
                self.text_len = 0;
                Some(shader)
            }
            Err(err) => {
                let (v, i) = vbo::text(
                    &self.target,
                    &FString::from_iter([format!("{err}").default()]),
                    &mut self.glyphs,
                    18.0,
                    5.0,
                    5.0,
                );
                assert_eq!(i.len() / 5, v.len() / 4);
                self.text_len = (i.len() / 5) as u32;
                self.text_upload = Some((v, i));
                self.main_shader.take()
            }
        };
    }

    fn opt_recompile(&mut self) {
        if let Ok(source) = self.receiver.try_recv() {
            self.recompile(source);
        }
        /* // read the shader source
        let mut file = File::open(PATH)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;

        let Ok(x) = fs::metadata(PATH) else {
            log::debug!("cannot read metadata, update manually");
            return
        };

        let Ok(time) = x.modified() else {
            log::debug!("cannot read metadata, update manually");
            return
        };

        if time > self.last_mod {
            log::debug!("update detected, recompiling");
            self.last_mod = time;
            self.recompile();
        } */
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }

        self.main_ubo_data.cursor.x = self.ws.cursor_pos.x as f32 / self.ws.size.width as f32;
        self.main_ubo_data.cursor.y =
            1.0 - (self.ws.cursor_pos.y as f32 / self.ws.size.height as f32);

        if !self.ws.cursor_in {
            self.main_ubo_data.cursor = Vec2::new(-1.0, -1.0);
        }
    }

    fn draw(&mut self) {
        let mut recompile = false;
        self.ul.update(|| recompile = true);
        if recompile {
            self.opt_recompile();
        }

        let mut frame = self.target.get_frame();

        self.main_ubo_data.aspect = self.ws.aspect;
        self.main_ubo_data.time = self.timer.elapsed().as_secs_f32();

        self.main_ubo
            .upload(&mut self.target, &mut frame, &[self.main_ubo_data]);
        self.text_ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_rh(
                0.0,
                self.ws.size.width as f32,
                self.ws.size.height as f32,
                0.0,
                -100.0,
                100.0,
            )],
        );
        if let Some((v, i)) = self.text_upload.take() {
            self.text_ibo.upload(&mut self.target, &mut frame, &i);
            self.text_vbo.upload(&mut self.target, &mut frame, &v);
        }

        let main_bind_group = self
            .main_shader
            .as_ref()
            .map(|s| s.bind_group(&self.main_ubo));
        let text_bind_group = self.text_shader.bind_group((&self.text_ubo, &self.glyphs));

        let mut render_pass = frame.main_render_pass();

        if let Some(main_shader) = self.main_shader.as_ref() {
            render_pass = render_pass
                .bind_group(main_bind_group.as_ref().unwrap())
                .bind_shader(main_shader)
                .draw(0..4, 0..1)
                .done();
        }

        render_pass = render_pass
            .bind_ibo(&self.text_ibo)
            .bind_vbo(&self.text_vbo)
            .bind_group(&text_bind_group)
            .bind_shader(&self.text_shader)
            .draw_indexed(0..self.text_len * 5, 0, 0..1)
            .done();
        drop(render_pass);

        self.target.finish_frame(frame);
    }
}

//

async fn async_run(canvas_div_id: String, receiver: Receiver<String>) {
    let target = EventLoop::new();
    let app = App::init(&target, &canvas_div_id, receiver).await;
    target.runnable(app);
}

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run(canvas_div_id: String) -> ShaderSender {
    // init_log();

    console_log::init_with_level(log::Level::Info).unwrap();

    let (sender, receiver) = channel();

    as_async(async_run(canvas_div_id, receiver));

    ShaderSender { sender }
}

#[wasm_bindgen]
pub struct ShaderSender {
    sender: Sender<String>,
}

#[wasm_bindgen]
impl ShaderSender {
    pub fn send_shader(&self, source: String) -> Result<(), String> {
        self.sender.send(source).map_err(|err| err.to_string())
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn main() {
//     init_log();
//     as_async(async_run());
// }

// -------------
// CUSTOM SHADER
// -------------

struct CustomShader {
    inner: Shader<(), ()>,
    layout: BindGroupLayout,
    device: Arc<Device>,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Ubo {
    aspect: f32,
    time: f32,
    cursor: Vec2,
}

impl CustomShader {
    pub fn new(target: &Target, source: String) -> Result<Self> {
        // compile modules
        let vert = ShaderModule::new_wgsl_source(target, Cow::Borrowed(VERT_SHADER))
            .expect("Failed to compile the built in vertex module");
        let frag = ShaderModule::new_wgsl_source(target, Cow::Owned(source))
            .map_err(|err| format_err!("{err}"))?;

        // shader layout
        let device = target.get_device();
        let layout = Self::bind_group_layout(&device);

        // shader itself
        let inner = Shader::builder()
            .with_vertex(&vert, "main")
            .with_fragment(&frag, "main")
            .with_format(target.get_format())
            .with_baked_layout(PipelineLayoutDescriptor {
                label: label!(),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            })
            .with_label(label!())
            .build(target);

        Ok(Self {
            inner,
            layout,
            device,
        })
    }
}

// TODO: Automatic
impl Layout for CustomShader {
    type Bindings<'a> = &'a UniformBuffer<Ubo>;

    fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: label!(),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    fn bind_group(&self, bindings: Self::Bindings<'_>) -> wgpu::BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: bindings.get_buffer().as_entire_binding(),
            }],
        })
    }
}

impl Deref for CustomShader {
    type Target = Shader<(), ()>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for CustomShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
