import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Main {

    public static final Pattern ROBOT_PATTERN = Pattern.compile("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)");

    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day14 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));
        final List<Robot> robots = parseInput(contents);
        final long part1Answer = part1(robots);
        System.out.printf("Part 1: %d\n", part1Answer);
    }

    public static long part1(final List<Robot> robots) {
        IntStream.range(0, 100).forEach(i -> robots.forEach(Robot::step));

        final Map<Quadrant, Long> counts = robots
            .stream()
            .map(Robot::getQuadrant)
            .filter(quad -> quad != Quadrant.CENTRAL)
            .collect(Collectors.groupingBy(e -> e, Collectors.counting()));

        return counts
            .keySet()
            .stream()
            .mapToLong(key -> counts.getOrDefault(key, 0L))
            .reduce((acc, val) -> acc * val)
            .orElseThrow();
    }

    public static List<Robot> parseInput(final String input) {
        return input
            .lines()
            .map(Main::parseRobot)
            .toList();
    }

    public static Robot parseRobot(final String line) {
        final Matcher m = ROBOT_PATTERN.matcher(line);
        if (!m.find()) {
            throw new IllegalArgumentException(String.format("Invalid line: %s", line));
        }
        return new Robot(
            Integer.parseInt(m.group(1)), Integer.parseInt(m.group(2)),
            Integer.parseInt(m.group(3)), Integer.parseInt(m.group(4))
        );
    }
}