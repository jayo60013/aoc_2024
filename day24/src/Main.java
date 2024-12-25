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

        part2(wires);
    }

    public static long part1(final Map<String, String> input) {
        final Map<String, String> wires = new HashMap<>(input);
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

    public static void part2(final Map<String, String> wires) {
        final String zs = wires.keySet().stream().filter(key->key.startsWith("z")).sorted(String::compareTo).collect(Collectors.joining(" -> "));
        final String xs = zs.replaceAll("z", "x");
        final String ys = zs.replaceAll("z", "y");

        final String ands = wires.keySet().stream().filter(key -> wires.get(key).contains("AND")).collect(Collectors.joining("; "));
        final String ors = wires.keySet().stream().filter(key -> wires.get(key).contains("OR")).collect(Collectors.joining("; "));
        final String xors = wires.keySet().stream().filter(key -> wires.get(key).contains("XOR")).collect(Collectors.joining("; "));

        // Put this into https://dreampuf.github.io/GraphvizOnline/?engine=dot#digraph%20G%20%7B%0A%0A%7D
        System.out.printf("""
            digraph G {
                subgraph {
                    node [style=filled,color=lightgreen]
                    %s
                }
                subgraph {
                    node [style=filled,color=gray]
                    %s
                }
                subgraph {
                    node [style=filled,color=gray]
                    %s
                }
                subgraph {
                    node [style=filled,color=pink]
                    %s
                }
                subgraph {
                    node [style=filled,color=yellow]
                    %s
                }
                subgraph {
                    node [style=filled,color=lightblue]
                    %s
                }
            """, zs, xs, ys, ands, ors, xors);
        for (final String key : wires.keySet()) {
            final String[] parts = wires.get(key).split(" ");
            if (parts.length != 3) continue;

            final String left = parts[0];
            final String right = parts[2];
            System.out.printf("     %s -> %s; %s -> %s;\n", left, key, right, key);
        }
        System.out.printf("}\n");
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