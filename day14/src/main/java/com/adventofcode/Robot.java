package com.adventofcode;

import static com.raylib.Colors.RAYWHITE;
import static com.raylib.Raylib.DrawRectangle;

public class Robot {
    public static final int WIDTH = 101;
    public static final int HEIGHT = 103;

    public int px;
    public int py;
    public final int vx;
    public final int vy;

    public Robot(final int px, final int py, final int vx, final int vy) {
        this.px = px;
        this.py = py;
        this.vx = vx;
        this.vy = vy;
    }

    public void stepForwards() {
        step(true);
    }

    public void stepBackwards() {
        step(false);
    }

    private void step(final boolean forwards) {
        final int nx = (forwards) ? px + vx : px - vx;
        final int ny = (forwards) ? py + vy : py - vy;

        if (nx < 0) {
            this.px = WIDTH + nx;
        } else if (nx >= WIDTH) {
            this.px = nx - WIDTH;
        } else {
            this.px = nx;
        }

        if (ny < 0) {
            this.py = HEIGHT + ny;
        } else if (ny >= HEIGHT) {
            this.py = ny - HEIGHT;
        } else {
            this.py = ny;
        }
    }

    public Quadrant getQuadrant() {
        final int midX = Robot.WIDTH / 2;
        final int midY = Robot.HEIGHT / 2;

        if (this.px < midX && this.py < midY) {
            return Quadrant.TOP_LEFT;
        } else if (this.px > midX && this.py < midY) {
            return Quadrant.TOP_RIGHT;
        } else if (this.px < midX && this.py > midY) {
            return Quadrant.BOTTOM_LEFT;
        } else if (this.px > midX && this.py > midY) {
            return Quadrant.BOTTOM_RIGHT;
        } else {
            return Quadrant.CENTRAL;
        }
    }

    public void draw(final int width, final int height) {
        DrawRectangle(this.px * width, this.py * height, width, height, RAYWHITE);
    }
}