public class Robot {

    //    public static final int WIDTH = 11;
//    public static final int HEIGHT = 7;
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

    public void step() {
        final int nx = px + vx;
        final int ny = py + vy;

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
}
