// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

// Imports
use agb::{
    display::object::{Graphics, Object, OamManaged, Tag},
    include_aseprite,
    input::Button
};

// Define Objects and types
struct Paddle<'obj> {
    start: Object<'obj>,
    mid: Object<'obj>,
    end: Object<'obj>,
}

impl<'obj> Paddle<'obj> {
    fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut paddle_start = object.object_sprite(PADDLE_END.sprite(0));
        let mut paddle_mid = object.object_sprite(PADDLE_MID.sprite(0));
        let mut paddle_end = object.object_sprite(PADDLE_END.sprite(0));

        paddle_start.show();
        paddle_mid.show();
        paddle_end.set_vflip(true).show();

        let mut paddle = Self {
            start: paddle_start,
            mid: paddle_mid,
            end: paddle_end,
        };

        paddle.set_position(start_x, start_y);

        paddle
    }

    fn set_position(&mut self, x: i32, y: i32) {
        // new! use of the `set_position` method. This is a helper feature using
        // agb's vector types. For now we can just use it to avoid adding them
        // separately
        self.start.set_position((x, y));
        self.mid.set_position((x, y + 16));
        self.end.set_position((x, y + 32));
    }
}

struct Dave<'obj> {
    down_idle: Object<'obj>
}

impl<'obj> Dave<'obj> {
    fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32) -> Self {
        let mut down_idle = object.object_sprite(DAVE_DOWN.sprite(0));

        down_idle.show();

        let mut dave = Self {
            down_idle: down_idle
        };

        dave.down_idle.set_position((start_x, start_y));

        dave
    }
}

// Import the sprites in to this static. This holds the sprite
// and palette data in a way that is manageable by agb.
static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
static DAVE_SPRITES: &Graphics = include_aseprite!("gfx/dave.aseprite");

// We define some easy ways of referencing the sprites
static PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
static PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");
static DAVE_DOWN: &Tag = DAVE_SPRITES.tags().get("Dave Down");
static BALL: &Tag = GRAPHICS.tags().get("Ball");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object = gba.display.object.get_managed();

    // Input controller
    let mut input = agb::input::ButtonController::new();

    // Create an object with the ball sprite
    let mut ball = object.object_sprite(BALL.sprite(0));

    // Create some paddles
    // let mut paddle_a = Paddle::new(&object, 8, 8); // the left paddle
    let mut paddle_b = Paddle::new(&object, 240 - 16 - 8, 8); // the right paddle
    paddle_b.start.set_hflip(true);
    paddle_b.mid.set_hflip(true);
    paddle_b.end.set_hflip(true);

    // Place this at some point on the screen, (50, 50) for example
    ball.set_x(50).set_y(50).show();

    // Now commit the object controller so this change is reflected on the screen.
    // This isn't how we will do this in the final version of the code, but will do
    // for this example.
    let mut ball_x = 50;
    let mut ball_y = 50;

    // now we initialise the x and y velocities to 0 rather than 1
    let mut x_velocity = 0;
    let mut y_velocity = 0;

    // Dave positions
    let mut dave_x: i32 = 120;
    let mut dave_y: i32 = 80;

    // Dave velocities
    let mut dave_vel_x: i32 = 0;
    let mut dave_vel_y: i32 = 0;

    // Make our buddy Dave
    let mut dave = object.object_sprite(DAVE_DOWN.sprite(0));
    dave.set_x(dave_x as u16).set_y(dave_y as u16).show();

    // Main loop
    loop {
        ball_x = (ball_x + x_velocity).clamp(0, agb::display::WIDTH - 16);
        ball_y = (ball_y + y_velocity).clamp(0, agb::display::HEIGHT - 16);

        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad
        // buttons are being pressed
        x_velocity = input.x_tri() as i32;
        y_velocity = input.y_tri() as i32;

        if input.is_pressed(Button::A) {
            // the A button is pressed
            x_velocity = x_velocity*2;
            y_velocity = y_velocity*2;
        }

        ball.set_x(ball_x as u16).set_y(ball_y as u16);

        // Dave controls
        dave_x = (dave_x + dave_vel_x).clamp(0, agb::display::WIDTH - 16);
        dave_y = (dave_y + dave_vel_y).clamp(-8, agb::display::HEIGHT - 32);

        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad
        // buttons are being pressed
        dave_vel_x = input.x_tri() as i32;
        dave_vel_y = input.y_tri() as i32;

        if input.is_pressed(Button::A) {
            // the A button is pressed
            dave_vel_x = dave_vel_x*2;
            dave_vel_y = dave_vel_y*2;
        }

        dave.set_x(dave_x as u16).set_y(dave_y as u16);

        agb::display::busy_wait_for_vblank();
        object.commit();

        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
    }
}
