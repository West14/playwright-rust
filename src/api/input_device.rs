use crate::imp::{
    core::*,
    page::{MouseClickArgs, Page as PageImpl},
    prelude::*,
    utils::MouseButton
};

pub struct Keyboard {
    inner: Weak<PageImpl>
}

pub struct Mouse {
    inner: Weak<PageImpl>
}

pub struct TouchScreen {
    inner: Weak<PageImpl>
}

impl Keyboard {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn down(&self, key: &str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_down(key).await
    }

    pub async fn up(&self, key: &str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_up(key).await
    }

    pub async fn input_text(&self, text: &str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_input_text(text).await
    }

    pub async fn r#type(&self, text: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_type(text, delay).await
    }

    pub async fn press(&self, key: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_press(key, delay).await
    }
}

impl Mouse {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn r#move(&self, x: f64, y: f64, steps: Option<i32>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.mouse_move(x, y, steps).await
    }

    pub async fn down(
        &self,
        button: Option<MouseButton>,
        click_count: Option<i32>
    ) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.mouse_down(button, click_count).await
    }

    pub async fn up(
        &self,
        button: Option<MouseButton>,
        click_count: Option<i32>
    ) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.mouse_up(button, click_count).await
    }

    pub fn clicker(&mut self, x: f64, y: f64) -> Clicker { Clicker::new(self.inner.clone(), x, y) }

    pub fn dblclicker(&mut self, x: f64, y: f64) -> DblClicker {
        DblClicker::new(self.inner.clone(), x, y)
    }
}

impl TouchScreen {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn tap<'a>(&self, x: f64, y: f64) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.screen_tap(x, y).await
    }
}

macro_rules! clicker {
    ($t: ident, $f: ident, $mf: ident) => {
        pub struct $t {
            inner: Weak<PageImpl>,
            args: MouseClickArgs
        }

        impl $t {
            pub(crate) fn new(inner: Weak<PageImpl>, x:f64, y:f64) -> Self {
                let args = MouseClickArgs::new(x,y);
                Self { inner, args }
            }

            pub async fn $f(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$mf(args).await?;
                Ok(())
            }

            optional_setter!(
                delay, f64;
                button, MouseButton;
                click_count, i32);
        }
    }
}

clicker!(Clicker, click, mouse_click);
clicker!(DblClicker, dblclick, mouse_dblclick);
