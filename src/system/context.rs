extern crate glfw;

use glfw::Glfw;
use system::window::IonicWindow;

pub struct IonicContext
{
    glfw_context: Glfw
}

impl IonicContext
{
    pub fn new() -> Self
    {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        return IonicContext
            {
                glfw_context: glfw
            }
    }

    pub fn get_glfw(&self) -> Glfw
    {
        self.glfw_context
    }

    pub fn create_window(&self, title: &str, width: u32, height: u32, windowMode: glfw::WindowMode) -> IonicWindow
    {
        let mut window = self.glfw_context.create_window(width, height, title, windowMode).expect("Faild to create GLFW window.").0;
        IonicWindow::new(window, title)
    }
}