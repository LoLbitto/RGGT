use std::error::Error;
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::Deref;

mod renderer;
use crate::renderer::Renderer;

use gl::types::GLfloat;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes};

use glutin::config::{Config, ConfigTemplateBuilder, GetGlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glutin_winit::{DisplayBuilder, GlWindow};

use winit::event_loop::EventLoop;
//use std::ops::ControlFlow;
use winit::event_loop::ControlFlow;
use winit::window::WindowId;

fn main() {
    
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);

    let template =
        ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let mut app = App::new(template, display_builder);
    event_loop.run_app(&mut app);
}

enum GlDisplayCreationState {
    /// The display was not build yet.
    Builder(DisplayBuilder),
    /// The display was already created for the application.
    Init,
}

struct App {
    template: ConfigTemplateBuilder,
    renderer: Option<Renderer>,

    state: Option<AppState>, 
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayCreationState,
    exit_state: Result<(), Box<dyn Error>>,
}

impl App {
    fn new(template: ConfigTemplateBuilder, display_builder: DisplayBuilder) -> Self {
        Self {
            template,
            gl_display: GlDisplayCreationState::Builder(display_builder),
            exit_state: Ok(()),
            gl_context: None,
            state: None,
            renderer: None,
        }
    }
}

struct AppState {
    gl_surface: Surface<WindowSurface>,
    window: Window
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

impl ApplicationHandler for App {

    fn resumed (&mut self, event_loop: &ActiveEventLoop) {
        let (window, gl_config) = match &self.gl_display {
            
            // se tiver no builder, significa que é novo e foi acabado de ser criado
            // nesse caso, é preciso gerar todas as variáveis e objetos necessários
            GlDisplayCreationState::Builder(display_builder) => {
                // display_builder vai ser passado na main
                let (window, gl_config) = match display_builder.clone().build(
                    event_loop,
                    self.template.clone(),
                    gl_config_picker,
                ) {
                    Ok((window, gl_config)) => (window.unwrap(), gl_config),
                    Err(err) => {
                        self.exit_state = Err(err);
                        event_loop.exit();
                        return;
                    },
                };

                println!("Pega a configuração com {} exemplos", gl_config.num_samples());

                // marca o display como iniciado
                self.gl_display = GlDisplayCreationState::Init;

                // cria contexto do opengl
                // (Olhar a fundo como funciona isso depois)
                self.gl_context = Some(create_gl_context(&window, &gl_config).treat_as_possibly_current());
                
                // retorna a janela e a config do opengl
                (window, gl_config)
            },
            GlDisplayCreationState::Init => {
                println!("Recreating window in `resumed`");
                
                // pega a config dnv
                let gl_config = self.gl_context.as_ref().unwrap().config();

                // n faço ideia oq são essas linha, dps olho mlr
                match glutin_winit::finalize_window(event_loop, window_attributes(), &gl_config) {
                    Ok(window) => (window, gl_config),
                    Err(err) => {
                        self.exit_state = Err(err.into());
                        event_loop.exit();
                        return;
                    },
                }  
            },
        };

        // constrói atributos de superfície da janela
        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");

        // constrói a superfície em sí
        let gl_surface =
            unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        // pega o contexto e deixa-o como o contexto atual
        // isso é necesário para colocar shaders e buffers
        let gl_context = self.gl_context.as_ref().unwrap();
        gl_context.make_current(&gl_surface).unwrap();

        // cria um renderizador para o display
        self.renderer.get_or_insert_with(|| Renderer::new(&gl_config.display()));

        // tenta colocar vsync
        // se o "set_swap_interval" retornar um erro, vai ser retornado "true" pois a atribuição
        // deu certo
        if let Err(res) = gl_surface
            .set_swap_interval(gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {res:?}");
        }
        
        // coloca um novo AppState com a superficie e a janela
        // o "replace" retorna o valor antigo, e esse valor tem q ser vazio, caso contrario ele
        // retornará "false" e o programa soltará um erro
        assert!(self.state.replace(AppState { gl_surface, window }).is_none());
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // This event is only raised on Android, where the backing NativeWindow for a GL
        // Surface can appear and disappear at any moment.
        println!("Android window removed");

        // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
        // the window back to the system.
        self.state = None;

        // Make context not current.
        self.gl_context = Some(
            self.gl_context.take().unwrap().make_not_current().unwrap().treat_as_possibly_current(),
        );
    }



    fn window_event (&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Fechando");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: Key::Named(NamedKey::ArrowUp), .. },
                ..
            } => self.renderer.as_mut().unwrap().update(1.0),

             WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: Key::Named(NamedKey::ArrowDown), .. },
                ..
            } => self.renderer.as_mut().unwrap().update(-1.0),

            WindowEvent::RedrawRequested => { 
                self.renderer.as_ref().unwrap().draw();
                self.state.as_ref().unwrap().window.request_redraw();
            }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // NOTE: BS da nvidia, vou deixar pq n faz diferença
        let _gl_display = self.gl_context.take().unwrap().display();

        // limpa janela
        self.state = None;
        #[cfg(egl_backend)]
        #[allow(irrefutable_let_patterns)]
        if let glutin::display::Display::Egl(display) = _gl_display {
            unsafe {
                display.terminate();
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(AppState { gl_surface, window }) = self.state.as_ref() {
            let gl_context = self.gl_context.as_ref().unwrap();
            let renderer = self.renderer.as_ref().unwrap();
            renderer.draw();
            window.request_redraw();

            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display.create_context(gl_config, &context_attributes).unwrap_or_else(|_| {
            gl_display.create_context(gl_config, &fallback_context_attributes).unwrap_or_else(
                |_| {
                    gl_display
                        .create_context(gl_config, &legacy_context_attributes)
                        .expect("failed to create context")
                },
            )
        })
    }
}

fn window_attributes() -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title("Glutin triangle gradient example (press Escape to exit)")
}
