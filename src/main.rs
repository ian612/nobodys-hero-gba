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

use agb::{
    include_aseprite,
    display::object::{Graphics, Tag}
};

// Import the sprites in to this static. This holds the sprite
// and palette data in a way that is manageable by agb.
static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

// We define some easy ways of referencing the sprites
static PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
static PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");
static BALL: &Tag = GRAPHICS.tags().get("Ball");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object = gba.display.object.get_managed();

    // Create an object with the ball sprite
    let mut ball = object.object_sprite(BALL.sprite(0));

    // Input controller
    use agb::input::Button;
    let mut input = agb::input::ButtonController::new();

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

        agb::display::busy_wait_for_vblank();
        object.commit();

        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
    }
}
