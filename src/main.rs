use vetor::Vetor;

// Usando o speedy2d para criar a tela e a animação.

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Pendulo", (800, 400)).unwrap();

    let win = MyWindowHandler
    {
        pendulo_var: Pendulo::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win);
}

struct MyWindowHandler 
{
    pendulo_var: Pendulo,
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.pendulo_var.update();
        self.pendulo_var.draw(graphics);

        helper.request_redraw();
    }
}

// Estruturando o pendulo.

struct Pendulo 
{
    // Origem do pendulo.

    origem: Vetor,

    // Origem da bola do pendulo.

    posicao: Vetor,

    angulo: f32,

    velocidade_angular: f32,
    aceleracao_angular: f32,

    raio: f32,
    gravidade: f32,
}

// Implementando métodos e funções na struct 'Pendulo'

impl Pendulo
{
    fn new(x: f32, y: f32, raio: f32) -> Pendulo
    {
        Pendulo
        {
            origem: Vetor::new(x,y),

            posicao: Vetor::new(0.0, 0.0),

            angulo: 1.0,

            velocidade_angular: 0.0,

            aceleracao_angular: 0.0,
            raio: raio,
            gravidade: 1.5,
        }
    }

    fn update(&mut self)
    {
        self.aceleracao_angular = -1.0 * self.gravidade * self.angulo.sin() / self.raio;

        self.velocidade_angular += self.aceleracao_angular;

        self.angulo += self.velocidade_angular;

        self.posicao
            .set(self.raio * self.angulo.sin(), self.raio * self.angulo.cos());

        self.posicao.add(&self.origem);
    }

    fn draw(&self, graphics: &mut Graphics2D)
    {
        graphics.draw_line(
            (self.origem.x, self.origem.y),
            (self.posicao.x, self.posicao.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.posicao.x, self.posicao.y), 30.0, Color::RED);
    }
}

// Criando um módulo. Dentro dele, só teremos acesso ao que for público.

mod vetor 
{
    // O vetor seguinte tem duas dimensões.

    pub struct Vetor
    {
        pub x: f32,
        pub y: f32,
    }

    // Implementando métodos e funções na struct 'Vetor'


    impl Vetor
    {
        // Cria a estrutura Vetor.

        pub fn new(x:f32, y:f32) -> Vetor
        {
            Vetor
            {
                x,
                y,
            }
        }

        // Isso é um método! Por causa do parâmetro 'self', que pega os valores da struct Vetor

        pub fn add(&mut self, outro: &Vetor ) -> &mut Vetor
        {
            self.x += outro.x;
            self.y += outro.y;
            self // retorno
        }

        pub fn set(&mut self, x:f32, y:f32) -> &Vetor
        {
            self.x = x;
            self.y = y;
            self
        }

        // O "pub" deixa essas funções e métodos públicos.
    }
}