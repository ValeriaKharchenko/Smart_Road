use macroquad::prelude::*;

/*ROAD*/
pub fn road() {

    {
        // center -> up
        //first batch
        draw_line(300.0, 0.0, 300.0, 300.0, 1.0, RED);
        draw_line(330.0, 0.0, 330.0, 300.0, 1.0, GRAY);
        draw_line(360.0, 0.0, 360.0, 300.0, 1.0, GRAY);
        draw_line(390.0, 0.0, 390.0, 300.0, 1.0, RED);
        //second batch
        draw_line(390.0, 0.0, 390.0, 300.0, 1.0, RED);
        draw_line(420.0, 0.0, 420.0, 300.0, 1.0, GRAY);
        draw_line(450.0, 0.0, 450.0, 300.0, 1.0, GRAY);
        draw_line(480.0, 0.0, 480.0, 300.0, 1.0, RED);
    }
    {
        // center -> down
        //first batch
        draw_line(300.0, 480.0, 300.0, screen_height(), 1.0, RED);
        draw_line(330.0, 480.0, 330.0, screen_height(), 1.0, GRAY);
        draw_line(360.0, 480.0, 360.0, screen_height(), 1.0, GRAY);
        draw_line(390.0, 480.0, 390.0, screen_height(), 1.0, RED);
        //second batch
        draw_line(390.0, 480.0, 390.0, screen_height(), 1.0, RED);
        draw_line(420.0, 480.0, 420.0, screen_height(), 1.0, GRAY);
        draw_line(450.0, 480.0, 450.0, screen_height(), 1.0, GRAY);
        draw_line(480.0, 480.0, 480.0, screen_height(), 1.0, RED);
    }
    {
        // center -> left
        //first batch
        draw_line(0.0, 300.0, 300.0, 300.0, 1.0, RED);
        draw_line(0.0, 330.0, 300.0, 330.0, 1.0, GRAY);
        draw_line(0.0, 360.0, 300.0, 360.0, 1.0, GRAY);
        draw_line(0.0, 390.0, 300.0, 390.0, 1.0, RED);
        //second batch
        draw_line(0.0, 390.0, 300.0, 390.0, 1.0, RED);
        draw_line(0.0, 420.0, 300.0, 420.0, 1.0, GRAY);
        draw_line(0.0, 450.0, 300.0, 450.0, 1.0, GRAY);
        draw_line(0.0, 480.0, 300.0, 480.0, 1.0, RED);
    }
    {
        // center -> right
        //first batch
        draw_line(480.0, 300.0, 800.0, 300.0, 1.0, RED);
        draw_line(480.0, 330.0, 800.0, 330.0, 1.0, GRAY);
        draw_line(480.0, 360.0, 800.0, 360.0, 1.0, GRAY);
        draw_line(480.0, 390.0, 800.0, 390.0, 1.0, RED);
        //second batch
        draw_line(480.0, 390.0, 800.0, 390.0, 1.0, RED);
        draw_line(480.0, 420.0, 800.0, 420.0, 1.0, GRAY);
        draw_line(480.0, 450.0, 800.0, 450.0, 1.0, GRAY);
        draw_line(480.0, 480.0, 800.0, 480.0, 1.0, RED);
    }
}