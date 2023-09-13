use bracket_lib::prelude::*;

// 定义游戏模式枚举
enum GameMode {
    Menu,     // 菜单模式
    Playing,  // 游戏进行中模式
    End,      // 游戏结束模式
}

const SCREEN_WIDTH: i32 = 80;   // 屏幕宽度
const SCREEN_HEIGHT: i32 = 50;  // 屏幕高度
const FRAME_DURATION: f32 = 75.0;  // 帧间隔时间

// 定义玩家结构体
struct Player {
    x: i32,      // 玩家的 x 坐标
    y: i32,      // 玩家的 y 坐标
    velocity: f32,  // 玩家的垂直速度
}

impl Player {
    // 创建一个新的玩家实例
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    // 在屏幕上绘制玩家
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    // 处理重力和玩家的移动
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    // 使玩家执行跳跃
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

// 定义游戏状态结构体
struct State {
    player: Player,         // 玩家
    mode: GameMode,         // 当前游戏模式
    frame_time: f32,        // 当前帧的时间
    obstacle: Obstacle,     // 障碍物
    score: i32,             // 玩家得分
}

impl State {
    // 创建一个新的游戏状态实例
    fn new() -> Self {
        State {
            player: Player::new(5, 25),  // 创建玩家，初始位置在 (5, 25)
            frame_time: 0.0,             // 初始化帧时间
            mode: GameMode::Menu,        // 初始模式为菜单模式
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),  // 创建障碍物，初始位置在屏幕右侧
            score: 0,                    // 初始得分为 0
        }
    }

    // 处理游戏进行中的状态
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);  // 清空屏幕，背景色设置为 NAVY

        self.frame_time += ctx.frame_time_ms;  // 更新帧时间

        // 根据帧时间更新玩家位置
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // 处理玩家输入，空格键用于使玩家跳跃
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        // 在屏幕上绘制玩家、障碍物和其他UI元素
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to flap");  // 显示跳跃提示
        ctx.print(0, 1, &format!("Score: {}", self.score));  // 显示当前得分

        self.obstacle.render(ctx, self.player.x);  // 绘制障碍物
        if self.player.x > self.obstacle.x {
            // 如果玩家通过了当前障碍物，增加得分并生成新的障碍物
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        // 检查是否发生碰撞，如果玩家飞出屏幕或撞到障碍物，游戏进入游戏结束模式
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    // 重置游戏状态以便重新开始游戏
    fn restart(&mut self) {
        self.player = Player::new(5, 25);  // 重置玩家位置
        self.frame_time = 0.0;            // 重置帧时间
        self.mode = GameMode::Playing;    // 设置游戏模式为进行中
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);  // 重置障碍物位置
        self.score = 0;                   // 重置得分
    }

    // 处理菜单模式
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();  // 清空屏幕

        // 显示欢迎消息和选项
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // 检查玩家输入来选择菜单选项
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),  // P键选择重新开始游戏
                VirtualKeyCode::Q => ctx.quitting = true,  // Q键选择退出游戏
                _ => {}
            }
        }
    }

    // 处理游戏结束模式
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();  // 清空屏幕

        // 显示游戏结束消息和玩家得分
        ctx.print_centered(5, "You are Dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Again Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // 检查玩家输入来选择菜单选项
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),  // P键选择重新开始游戏
                VirtualKeyCode::Q => ctx.quitting = true,  // Q键选择退出游戏
                _ => {}
            }
        }
    }
}

// 实现 GameState trait 来处理游戏状态的更新和绘制
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),   // 菜单模式
            GameMode::Playing => self.play(ctx),     // 游戏进行中模式
            GameMode::End => self.dead(ctx),         // 游戏结束模式
        }
    }
}

// 定义障碍物结构体
struct Obstacle {
    x: i32,         // 障碍物的 x 坐标
    gap_y: i32,     // 障碍物的间隙位置
    size: i32,      // 障碍物的大小
}

impl Obstacle {
    // 创建一个新的障碍物实例
    fn new(x: i32, size: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40).clamp(0, SCREEN_HEIGHT),  // 随机生成间隙位置
            size: i32::max(2, 20 - size),  // 随机生成障碍物大小
        }
    }

    // 在屏幕上绘制障碍物
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;  // 障碍物相对于玩家的屏幕位置

        let half_size = self.size / 2;

        // 绘制障碍物的上半部分
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // 绘制障碍物的下半部分
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    // 检查玩家是否碰撞到障碍物
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let obstacle_x = self.x;
        let obstacle_top = self.gap_y - half_size;
        let obstacle_bottom = self.gap_y + half_size;

        player.x == obstacle_x && (player.y < obstacle_top || player.y > obstacle_bottom)
    }
}

// 主函数，创建游戏窗口并运行游戏循环
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}