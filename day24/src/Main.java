import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;

public class Main {
    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day24 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));
        final Map<String, String> wires = parseInput(contents);

        final long part1Answer = part1(wires);
        System.out.printf("Part 1: %d\n", part1Answer);
    }

    public static long part1(final Map<String, String> wires) {
        boolean isComputed = false;
        while (!isComputed) {
            for (final String key : wires.keySet()) {
                if (wires.get(key).length() == 1) {
                    continue;
                }

                final String[] parts = wires.get(key).split(" ");
                final String lhs = wires.get(parts[0]);
                final String gate = parts[1];
                final String rhs = wires.get(parts[2]);

                if (lhs.length() != 1 || rhs.length() != 1) {
                    continue;
                }

                final boolean lhsBool = lhs.equals("1");
                final boolean rhsBool = rhs.equals("1");

                final boolean result;
                switch (gate) {
                    case "AND" -> result = lhsBool && rhsBool;
                    case "XOR" -> result = lhsBool ^ rhsBool;
                    case "OR" -> result = lhsBool || rhsBool;
                    default -> throw new IllegalStateException("unreachable");
                }

                wires.put(key, result ? "1" : "0");
            }
            isComputed = wires
                .values()
                .stream()
                .allMatch(value -> !value.startsWith("z") && value.length() == 1);
        }

        final String binaryValue = wires
            .keySet()
            .stream()
            .filter(key -> key.startsWith("z"))
            .sorted(Comparator.reverseOrder())
            .map(wires::get)
            .collect(Collectors.joining());
        return Long.parseLong(binaryValue, 2);
    }

    public static Map<String, String> parseInput(final String input) {
        final String[] parts = input.split("\n\n");

        final Map<String, String> values = new HashMap<>();

        for (final String line : parts[0].split("\n")) {
            final String[] lineParts = line.split(": ");
            values.put(lineParts[0], lineParts[1]);
        }

        for (final String line : parts[1].split("\n")) {
            final String[] lineParts = line.split(" -> ");
            values.put(lineParts[1], lineParts[0]);
        }

        return values;
    }
}