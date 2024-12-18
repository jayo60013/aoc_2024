import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.stream.Collectors;

public class Main {

    public record Point(int x, int y) {
    }

    public record State(Point pos, int score) implements Comparable<State> {
        @Override
        public int compareTo(State other) {
            return score - other.score;
        }
    }

    public static boolean IS_SAMPLE = false;
    public static int SIZE = IS_SAMPLE ? 6 : 70;
    public static int LIMIT = IS_SAMPLE ? 12 : 1024;
    public static Point[] DIRECTIONS = {
            new Point(-1, 0),
            new Point(1, 0),
            new Point(0, -1),
            new Point(0, 1)
    };

    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day18 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final List<String> contents = Files.readAllLines(Path.of(fileName));
        final Set<Point> bytes = parseInput(contents, LIMIT);
        final int part1Ans = part1(bytes);
        final String part2Ans = part2(contents);
        System.out.printf("Part 1: %d\n", part1Ans);
        System.out.printf("Part 2: %s\n", part2Ans);
    }

    public static int part1(final Set<Point> bytes) {
        final Point start = new Point(0, 0);
        final Point end = new Point(SIZE, SIZE);

        final Set<Point> visited = new HashSet<>();
        final PriorityQueue<State> queue = new PriorityQueue<>();

        visited.add(start);
        queue.add(new State(start, 0));

        while (!queue.isEmpty()) {
            final State curr = queue.poll();

            if (curr.pos.equals(end)) {
                return curr.score;
            }

            for (final Point nbor : getNbors(curr.pos, bytes)) {
                if (!visited.contains(nbor)) {
                    visited.add(nbor);
                    queue.add(new State(nbor, curr.score + 1));
                }
            }
        }
        return -1;
    }

    public static String part2(final List<String> input) {
        for (int i = LIMIT; i < input.size(); i++) {
            final Set<Point> bytes = parseInput(input, i);
            final int steps = part1(bytes);
            if (steps == -1) {
                return input.get(i - 1);
            }
        }
        return "oopsie";
    }

    public static List<Point> getNbors(final Point point, final Set<Point> bytes) {
        final List<Point> nbors = new ArrayList<>();
        for (final Point dir : DIRECTIONS) {
            final Point newPoint = new Point(point.x + dir.x, point.y + dir.y);
            if (!checkInbounds(newPoint)) continue;
            if (bytes.contains(newPoint)) continue;

            nbors.add(newPoint);
        }
        return nbors;
    }

    public static boolean checkInbounds(final Point p) {
        return 0 <= p.x && p.x <= SIZE && 0 <= p.y && p.y <= SIZE;
    }

    public static Set<Point> parseInput(final List<String> input, final int limit) {
        return input
                .stream()
                .limit(limit)
                .map(line -> {
                    final String[] parts = line.split(",");
                    final int x = Integer.parseInt(parts[0]);
                    final int y = Integer.parseInt(parts[1]);
                    return new Point(x, y);
                })
                .collect(Collectors.toSet());
    }
}