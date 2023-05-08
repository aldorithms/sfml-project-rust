use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Texture::from_file};
use sfml::system::Vector2f;
use sfml::window::{Event, Key, Style::CLOSE};
use sfml::SfBox;



fn main() {
    let mut window: RenderWindow = RenderWindow::new((800, 600),"SFML Example", CLOSE,&Default::default());
    window.set_vertical_sync_enabled(true);
    let mut texture: SfBox<Texture> = from_file("assets/sfml-icon-small.png").unwrap();
    let mut sprite: Sprite = Sprite::with_texture(&texture);
    sprite.set_position(Vector2f::new(100., 100.));
    sprite.set_scale(Vector2f::new(0.5, 0.5));
    sprite.set_color(Color::new(255, 255, 255, 200));
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => window.close(),
                    _ => {}
                },
                _ => {}
            }
        }
        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.display();
    }
}
