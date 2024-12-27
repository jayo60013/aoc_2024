import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.time.Instant;
import java.util.*;

public class Main {
    public record Input(Map<Point, Integer> grid, List<Point> startingPoints, int width, int height) {
    }

    public record Point(int x, int y) {
    }

    public static final Point[] DIRECTIONS = new Point[]{
        new Point(-1, 0), new Point(1, 0),
        new Point(0, -1), new Point(0, 1)
    };

    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day10 <filename.txt>");
            return;
        }
        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));

        final Instant start = Instant.now();
        final Input input = parseInput(contents);

        final int part1Answer = part1(input);
        final int part2Answer = part2(input);
        final Instant end = Instant.now();

        System.out.printf("Part 1: %d\n", part1Answer);
        System.out.printf("Part 2: %d\n", part2Answer);
        System.out.printf("Both parts took: %dms\n", Duration.between(start, end).toMillis());
    }

    public static int part1(final Input input) {
        int result = 0;
        for (final Point startPoint : input.startingPoints) {
            final Queue<Point> queue = new ArrayDeque<>();
            final Set<Point> visited = new HashSet<>();
            queue.add(startPoint);

            while (!queue.isEmpty()) {
                final Point cp = queue.poll();

                if (!visited.add(cp)) continue;

                if (input.grid.get(cp) == 9) {
                    result++;
                }
                queue.addAll(getAllNeighbours(cp, input));
            }
        }
        return result;
    }

    public static int part2(final Input input) {
        int result = 0;
        for (final Point startPoint : input.startingPoints) {
            final Queue<Point> queue = new ArrayDeque<>();
            queue.add(startPoint);

            while (!queue.isEmpty()) {
                final Point cp = queue.poll();

                if (input.grid.get(cp) == 9) {
                    result++;
                }
                queue.addAll(getAllNeighbours(cp, input));
            }
        }
        return result;
    }

    public static List<Point> getAllNeighbours(final Point p, Input input) {
        final int level = input.grid.get(p);
        final List<Point> neighbours = new ArrayList<>();

        for (final Point dir : DIRECTIONS) {
            final Point np = new Point(p.x + dir.x, p.y + dir.y);

            if (np.x < 0 || np.x >= input.width || np.y < 0 || np.y >= input.height) continue;

            final int newLevel = input.grid.get(np);
            if (level + 1 != newLevel) continue;

            neighbours.add(np);
        }
        return neighbours;
    }

    public static Input parseInput(final String input) {
        final Map<Point, Integer> grid = new HashMap<>();
        final List<Point> startingPoints = new ArrayList<>();

        final String[] lines = input.split("\n");
        final int height = lines.length;
        final int width = lines[0].length();

        for (int y = 0; y < lines.length; y++) {
            for (int x = 0; x < lines[y].length(); x++) {
                final int level = lines[y].charAt(x) - '0';
                final Point p = new Point(x, y);
                grid.put(p, level);

                if (level == 0) startingPoints.add(p);
            }
        }
        return new Input(grid, startingPoints, width, height);
    }
}