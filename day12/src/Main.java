import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;

public class Main {

    public record Point(int x, int y) {
    }

    public record Vec2(float x, float y) {
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
        final int part2Answer = part2(grid);
        System.out.printf("Part 2: %d\n", part2Answer);
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

    /**
     * WHAT THE FUCK???
     *
     * @param grid
     * @return
     */
    public static int part2(final List<List<Character>> grid) {
        final Set<Point> claimed = new HashSet<>();
        final int width = grid.get(0).size();
        final int height = grid.size();
        int total = 0;

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                final Point source = new Point(x, y);
                if (claimed.contains(source)) continue;

                final Set<Point> edges = new HashSet<>();
                int area = 0;
                final Queue<Point> queue = new LinkedList<>();
                final Set<Point> visited = new HashSet<>();
                queue.add(source);
                final char oldChar = grid.get(source.y).get(source.x);

                while (!queue.isEmpty()) {
                    final Point currPoint = queue.poll();

                    if (!isPointInbound(currPoint, width, height)) {
                        edges.add(currPoint);
                        continue;
                    }

                    final char newChar = grid.get(currPoint.y).get(currPoint.x);
                    if (visited.contains(currPoint)) continue;

                    if (oldChar != newChar) {
                        edges.add(currPoint);
                        continue;
                    }
                    area++;
                    claimed.add(currPoint);
                    visited.add(currPoint);

                    final List<Point> neighbors = getNeighbors(currPoint);
                    queue.addAll(neighbors);
                }

                final Map<Vec2, Vec2> edgeFacing = new HashMap<>();
                for (final Point edge : edges) {
                    for (final Point dir : DIRECTIONS) {
                        final Point dp = new Point(edge.x + dir.x, edge.y + dir.y);
                        if (edges.contains(dp)) continue;

                        final Vec2 avg = new Vec2((edge.x + dp.x) / 2.f, (edge.y + dp.y) / 2.f);
                        final Vec2 facing = new Vec2(avg.x - edge.x, avg.y - edge.y);
                        edgeFacing.put(avg, facing);
                    }
                }
                final Set<Vec2> seen = new HashSet<>();
                int sides = 0;
                for (final Vec2 edge : edgeFacing.keySet()) {
                    if (!seen.add(edge)) continue;
                    sides++;
                    final Vec2 direction = edgeFacing.get(edge);
                    if (edge.y % 1 == 0) {
                        //-1 & 1
                        for (int dy = -1; dy < 2; dy += 2) {
                            final float cy = edge.y + dy;
                            Vec2 nextEdge = new Vec2(edge.x, cy);
                            while (Objects.equals(edgeFacing.get(nextEdge), direction)) {
                                seen.add(nextEdge);
                                nextEdge = new Vec2(edge.x, nextEdge.y + dy);
                            }
                        }
                    } else {
                        for (int dx = -1; dx < 2; dx += 2) {
                            final float cx = edge.x + dx;
                            Vec2 nextEdge = new Vec2(cx, edge.y);
                            while (Objects.equals(edgeFacing.get(nextEdge), direction)) {
                                seen.add(nextEdge);
                                nextEdge = new Vec2(nextEdge.x + dx, edge.y);
                            }
                        }
                    }
                }
                System.out.printf("%c: %d * %d = %d\n", oldChar, area, sides, sides * area);
                total += sides * area;
                return total;
            }
        }
        return total;
    }

    public static List<Point> getNeighbors(final Point p) {
        return Arrays.stream(DIRECTIONS)
            .map(d -> new Point(p.x + d.x, p.y + d.y))
            .toList();
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