import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;

public class Main {

    public record Point(int x, int y) {
    }

    public static final Point[] DIRECTIONS = new Point[]{
        new Point(-1, 0), new Point(1, 0),
        new Point(0, -1), new Point(0, 1)
    };

    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day12 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));
        final List<List<Character>> grid = parseInput(contents);
        final int part1Answer = part1(grid);
        System.out.printf("Part 1: %d\n", part1Answer);
    }

    public static int part1(final List<List<Character>> grid) {
        final Set<Point> claimed = new HashSet<>();
        final int width = grid.get(0).size();
        final int height = grid.size();
        int total = 0;

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                final Point source = new Point(x, y);
                if (claimed.contains(source)) continue;

                int perimeter = 0;
                int area = 0;
                final Queue<Point> queue = new LinkedList<>();
                final Set<Point> visited = new HashSet<>();
                queue.add(source);
                final char oldChar = grid.get(source.y).get(source.x);

                while (!queue.isEmpty()) {
                    final Point currPoint = queue.poll();

                    if (!isPointInbound(currPoint, width, height)) {
                        perimeter++;
                        continue;
                    }

                    final char newChar = grid.get(currPoint.y).get(currPoint.x);
                    if (visited.contains(currPoint)) continue;

                    if (oldChar != newChar) {
                        perimeter++;
                        continue;
                    }
                    area++;
                    claimed.add(currPoint);
                    visited.add(currPoint);

                    final List<Point> neighbors = getNeighbors(currPoint);
                    queue.addAll(neighbors);
                }
                total += perimeter * area;
            }
        }
        return total;
    }

    public static List<Point> getNeighbors(final Point p) {
        final List<Point> neighbors = new ArrayList<>();
        for (final Point dir : DIRECTIONS) {
            neighbors.add(new Point(
                p.x + dir.x, p.y + dir.y
            ));
        }
        return neighbors;
    }

    public static boolean isPointInbound(final Point p, final int width, final int height) {
        return 0 <= p.x && p.x < width && 0 <= p.y && p.y < height;
    }

    public static List<List<Character>> parseInput(final String input) {
        return input
            .lines()
            .map(line -> line
                .chars()
                .mapToObj(c -> (char) c)
                .toList()
            )
            .toList();
    }
}