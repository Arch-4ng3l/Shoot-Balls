extern crate raylib;
use rand::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 800.0;

fn hit(pos1_x: f32, pos1_y: f32, rad1: f32, pos2_x: f32, pos2_y: f32, rad2: f32) -> bool {
    pos1_y - rad1 < pos2_y + rad2
        && pos1_y + rad1 > pos2_y - rad2
        && pos1_x - rad1 < pos2_x + rad2
        && pos1_x + rad1 > pos2_x - rad2
}
#[derive(Clone, Copy)]
struct Projectile {
    position: Vector2,
    speed: f32,
    radius: f32,
}

impl Projectile {
    fn new(player: &Player) -> Self {
        Projectile {
            position: Vector2::new(player.position.x, player.position.y - player.radius),
            speed: 6.0,
            radius: 10.0,
        }
    }
}

#[derive(Clone, Copy)]
struct Enemy {
    position: Vector2,
    radius: f32,
}

impl Enemy {
    fn new(range: f32) -> Self {
        Enemy {
            position: Vector2::new(rand::thread_rng().gen_range(10.0..WIDTH), 10.0),
            radius: rand::thread_rng().gen_range(range..35.0 + range),
        }
    }
}
struct Player {
    score: i32,
    hp: i32,
    position: Vector2,
    color: Color,
    speed: f32,
    radius: f32,
}

struct Button {
    position: Vector2,
    a: i32,
    b: i32,
}
fn main() {
    let mut player = Player {
        score: 0,
        hp: 10,
        position: Vector2::new(WIDTH / 2.0, HEIGHT / 2.0),
        color: Color::GREEN,
        radius: 40.0,
        speed: 6.0,
    };

    let mut projectiles = Vec::new();
    let mut enemys = Vec::new();
    let (mut r1, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .vsync()
        .title("Shoot Balls")
        .build();

    let mut start = false;
    let mut diffic = 1.0; 
    while !r1.window_should_close() {
        while player.hp > 0 && start {
            let max_len = ((player.score / 40 + 10) as f32 * diffic) as usize;
            let enemy_speed = (player.score as f32 / 40.0 + diffic) * diffic;
            let mut range = 20.0 - (player.score / 20) as f32;
            if range < 2.0 {
                range = 2.0
            }
            if (r1.is_key_down(KEY_UP) || r1.is_key_down(KEY_W)) 
                && player.position.y - player.speed - player.radius / 2.0 > 0.0
            {
                player.position.y -= player.speed;
            }
            if (r1.is_key_down(KEY_DOWN) || r1.is_key_down(KEY_S))
                && player.position.y + player.speed + player.radius / 2.0 < HEIGHT
            {
                player.position.y += player.speed;
            }
            if (r1.is_key_down(KEY_RIGHT) || r1.is_key_down(KEY_D))
                && player.position.x + player.speed + player.radius / 2.0 < WIDTH
            {
                player.position.x += player.speed;
            }

            if (r1.is_key_down(KEY_LEFT) || r1.is_key_down(KEY_A))
                && player.position.x - player.speed - player.radius / 2.0 > 0.0
            {
                player.position.x -= player.speed;
            }

            if r1.is_key_pressed(KEY_SPACE) || r1.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON){
                projectiles.push(Projectile::new(&player));
            }

            let mut temp2 = enemys.clone();

            if temp2.len() <= max_len {
                enemys.push(Enemy::new(range));
            }

            let mut d = r1.begin_drawing(&thread);
            temp2 = enemys.clone();

            d.clear_background(Color::WHITE);

            d.draw_circle_v(player.position, player.radius, player.color);
            let mut dead_enemys = Vec::new();
            for x in 0..temp2.len() {
                enemys[x].position.y += enemy_speed;

                if hit(
                    player.position.x,
                    player.position.y,
                    player.radius,
                    enemys[x].position.x,
                    enemys[x].position.y,
                    enemys[x].radius,
                ) {
                    player.hp -= 1;
                    dead_enemys.push(x);
                } else if enemys[x].position.y > HEIGHT {
                    dead_enemys.push(x);
                }

                d.draw_circle_v(enemys[x].position, enemys[x].radius, Color::RED);
            }

            for i in dead_enemys {
                if i < enemys.len() {
                    enemys.remove(i);
                }
            }
            temp2 = enemys.clone();
            let mut i = 0;
            let mut temp = projectiles.clone();
            for mut proj in projectiles {
                proj.position.y -= proj.speed;

                temp[i] = proj;
                if proj.position.y < 0.0 {
                    temp.remove(i);
                } else {
                    i += 1
                }
                let mut i2 = 0;
                for enemy in enemys.clone() {
                    if hit(
                        proj.position.x,
                        proj.position.y,
                        proj.radius,
                        enemy.position.x,
                        enemy.position.y,
                        enemy.radius,
                    ) {
                        if i2 < temp2.len() {
                            temp2.remove(i2);
                            if i > 0 {
                                temp.remove(i - 1);

                                i -= 1;
                            }
                            player.score += 1;
                        }
                    } else {
                        i2 += 1;
                    }
                }

                d.draw_circle_v(proj.position, proj.radius, Color::BLUE);
            }

            d.draw_text(
                &format!("Score: {}", player.score),
                20,
                20,
                20,
                Color::BLACK,
            );
            d.draw_text(
                &format!("HP: {}", player.hp),
                WIDTH as i32 - (measure_text("HP: 10", 20) + 20),
                20,
                20,
                Color::BLACK,
            );
            projectiles = temp;
            enemys = temp2;
        }

        let x = r1.get_mouse_x();

        let y = r1.get_mouse_y();

        if r1.is_key_down(KEY_ESCAPE) {
            break;
        }
        if start {
            let but1 = Button {
                position: Vector2::new(WIDTH / 2.0, HEIGHT / 1.5),
                a: 400,
                b: 70,
            };

            if x > but1.position.x as i32 - but1.a / 2
                && x < but1.position.x as i32 + but1.a / 2
                && y > but1.position.y as i32
                && y < but1.position.y as i32 + but1.b
                && r1.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON)
            {
                player = Player {
                    score: 0,
                    hp: 10,
                    position: Vector2::new(WIDTH / 2.0, HEIGHT / 2.0),
                    color: Color::GREEN,
                    radius: 40.0,
                    speed: 6.0,
                };

                enemys = Vec::new();
                projectiles = Vec::new();
            }

            let mut d = r1.begin_drawing(&thread);
            let mut text_len = measure_text("U DIED", 100);
            d.clear_background(Color::WHITE);
            d.draw_text(
                "U DIED",
                (WIDTH as i32 / 2) - (text_len / 2),
                (HEIGHT / 2.0) as i32,
                100,
                Color::RED,
            );
            text_len = measure_text(&format!("U scored {}", player.score), 100);
            d.draw_text(
                &format!("U scored {}", player.score),
                (WIDTH as i32 / 2) - (text_len / 2),
                HEIGHT as i32 / 4,
                100,
                Color::BLACK,
            );
            d.draw_rectangle_lines(
                but1.position.x as i32 - but1.a / 2,
                but1.position.y as i32,
                but1.a,
                but1.b,
                Color::BLACK,
            );
            text_len = measure_text("Restart", 70);
            d.draw_text(
                "Restart",
                (WIDTH as i32 / 2) - (text_len / 2),
                (HEIGHT / 1.5) as i32,
                70,
                Color::BLACK,
            );
        } else {
            let mut text_len = measure_text("Easy", 80);
            let but2 = Button {
                position: Vector2::new(WIDTH / 2.0 - text_len as f32 / 2.0 - 10.0, HEIGHT / 2.0),
                a: text_len + 20,
                b: 70,
            };
            if x > but2.position.x as i32  - but2.a 
                && x < but2.position.x as i32 + but2.a 
                && y > but2.position.y as i32
                && y < but2.position.y as i32 + but2.b
                && r1.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON)
            {
                start = true;   
            }
            let but3 = Button {
                position: Vector2::new(WIDTH / 2.0 - text_len as f32 / 2.0 - 10.0, HEIGHT / 1.5),
                a: text_len + 20,
                b: 70,
            };
            if x > but3.position.x as i32  - but3.a 
                && x < but3.position.x as i32 + but3.a 
                && y > but3.position.y as i32
                && y < but3.position.y as i32 + but3.b
                && r1.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON)
            {   
                diffic = 1.5; 
                start = true;   
            }
            let mut d = r1.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            d.draw_text(
                "Easy",
                (WIDTH as i32 / 2) - (text_len / 2),
                HEIGHT as i32 / 2,
                80,
                Color::BLACK,
            );

            text_len = measure_text("Hard", 80); 
            d.draw_text("Hard", (WIDTH as i32 / 2) - (text_len /2), (HEIGHT/ 1.5) as i32, 80, Color::BLACK); 

            text_len = measure_text("Shoot The Balls", 90); 

            d.draw_text("Shoot The Balls", (WIDTH as i32/2) - (text_len/2), HEIGHT as i32/4, 90, Color::BLUE); 

            d.draw_rectangle_lines(
                but2.position.x as i32,
                but2.position.y as i32,
                but2.a,
                but2.b,
                Color::BLACK,
            );

            d.draw_rectangle_lines(but3.position.x as i32, but3.position.y as i32, but3.a, but3.b, Color::BLACK); 
        }
    }
}
