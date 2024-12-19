import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Main {
    public record Input(List<String> patterns, List<String> designs) {
    }

    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day19 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));
        final Input input = parseInput(contents);

        final long start = System.currentTimeMillis();
        final long part1Ans = part1(input);
        final long part2Ans = part2(input);
        final long duration = System.currentTimeMillis() - start;

        System.out.printf("Part 1: %d\n", part1Ans);
        System.out.printf("Part 2: %s\n", part2Ans);
        System.out.printf("Took: %dms\n", duration);
    }

    public static long part1(final Input input) {
        return input.designs
                .stream()
                .filter(design -> isDesignable(input.patterns, design))
                .count();
    }

    public static long part2(final Input input) {
        return input.designs
                .stream()
                .mapToLong(design -> numberOfWays(new HashMap<>(), input.patterns, design))
                .sum();
    }

    public static boolean isDesignable(final List<String> patterns, final String design) {
        if (design.isEmpty()) return true;

        for (final String pattern : patterns) {
            if (design.startsWith(pattern)) {
                final boolean isPossible = isDesignable(patterns, design.substring(pattern.length()));
                if (isPossible) return true;
            }
        }

        return false;
    }

    public static long numberOfWays(final Map<String, Long> lookup, final List<String> patterns, final String design) {
        if (design.isEmpty()) return 1;
        if (lookup.containsKey(design)) return lookup.get(design);

        long ways = 0;
        for (final String pattern : patterns) {
            if (design.startsWith(pattern)) {
                ways += numberOfWays(lookup, patterns, design.substring(pattern.length()));
            }
        }

        lookup.put(design, ways);
        return ways;
    }

    public static Input parseInput(final String input) {
        final String[] parts = input.split("\n\n");
        final List<String> patterns = Arrays.stream(parts[0].split(", ")).toList();
        final List<String> designs = parts[1].lines().toList();
        return new Input(patterns, designs);
    }
}
