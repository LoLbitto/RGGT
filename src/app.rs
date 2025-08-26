use std::error::Error;
use std::num::NonZeroU32;
use std::time::Duration;
use std::time::Instant;

use crate::graphic::renderer::Renderer;
use crate::states::State;
use crate::states::main_menu_state::MainMenuState;

use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent, ElementState};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowAttributes, WindowId};

use glutin::config::{Config, ConfigTemplateBuilder, GetGlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glutin_winit::GlWindow;
use glutin_winit::DisplayBuilder;

enum GlDisplayCreationState {
    /// The display was not build yet.
    Builder(DisplayBuilder),
    /// The display was already created for the application.
    Init,
}

pub struct App {
    template: ConfigTemplateBuilder,
    renderer: Option<Renderer>,

    pub state: Option<AppState>, 
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayCreationState,
    exit_state: Result<(), Box<dyn Error>>,
    
    last_update: Instant,

    pub game_state: Option<Box<dyn State>>
}

impl App {
    pub fn new(template: ConfigTemplateBuilder, display_builder: DisplayBuilder) -> Self {
        Self {
            template,
            gl_display: GlDisplayCreationState::Builder(display_builder),
            exit_state: Ok(()),
            gl_context: None,
            state: None,
            renderer: None,
            last_update: Instant::now(),
            game_state: None
        }
    }

    pub fn update(&mut self) {
        let state = self.game_state.as_mut().unwrap();
        state.update();
        self.renderer.as_mut().unwrap().update_solid(state.get_vertices());

        let (has_text, texts) = state.get_text();

        if has_text {
            unsafe{
                self.renderer.as_mut().unwrap().draw_text(texts.unwrap());
            }
        }

        let (has_texture, mut textures, vertices_textura, tex_map) = state.get_textures();
        
        if has_texture {       

            // Só queria expressar aqui, a minha RAIVA por borrows, que palhaçada, eu sei o que eu
            // to fazendo, me explica por que caramba eu não posso acessar uma borrom mutavel e
            // imutal ao mesmo tempo sendo que uma não tem relação com a outra??????????

            self.renderer.as_mut().unwrap().update_texture(vertices_textura.as_ref().unwrap(), &mut textures.as_mut().unwrap(), tex_map.unwrap());
            //println!("entrou aqui");
        }
    }

    pub fn change_state(&mut self, state: Box<dyn State>) {
        self.renderer.as_mut().unwrap().clear_textures();
        self.game_state.replace(state);
    }
}

pub struct AppState {
    gl_surface: Surface<WindowSurface>,
    pub window: Window,
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
        assert!(self.state.replace(AppState { gl_surface, window}).is_none());
        
        let state = MainMenuState::new(self) as Box<dyn State>;

        self.game_state.replace(state);
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
        
        let state = self.game_state.as_mut().unwrap();
        
        match event {
            WindowEvent::CloseRequested => {
                println!("Fechando");
                event_loop.exit();
            },

            WindowEvent::KeyboardInput { event, ..} => {
                state.manage_keyboard_input(event);
            },

            WindowEvent::CursorMoved { position, ..} => {
                state.manage_mouse_movement(position);
            },

            WindowEvent::MouseInput {button, ..} => {
                state.manage_mouse_input(button);
            }

            WindowEvent::Resized(size) => {
                self.renderer.as_mut().unwrap().resize(size.width as i32, size.height as i32);
                state.manage_window_resize(size);
            },

            WindowEvent::RedrawRequested => { 
                self.renderer.as_ref().unwrap().draw();
            }
            _ => (),
        }
 
        if (Instant::now() - self.last_update >= Duration::from_secs_f64(0.016)){
            self.update();
            self.last_update = Instant::now();
        } else {
            self.state.as_mut().unwrap().window.request_redraw();
        }

        //event_loop.set_control_flow(ControlFlow::Poll);
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
            window.request_redraw();
            //println!("teste");
            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }
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

pub fn window_attributes() -> WindowAttributes {
    Window::default_attributes()
        //.with_transparent(true)
        .with_title("Pirâmide poggers")
}
