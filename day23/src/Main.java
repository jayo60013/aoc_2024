import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Main {
    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day23 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));
        final Map<String, Set<String>> input = parseInput(contents);

        final long start = System.currentTimeMillis();
        final long part1Ans = part1(input);
        final String part2Ans = part2(input);
        final long duration = System.currentTimeMillis() - start;

        System.out.printf("Part 1: %d\n", part1Ans);
        System.out.printf("Part 2: %s\n", part2Ans);
        System.out.printf("Took: %dms\n", duration);
    }


    public static int part1(final Map<String, Set<String>> input) {
        final Set<List<String>> ans = new HashSet<>();

        for (final String n1 : input.keySet()) {
            final Set<String> edges = input.get(n1);
            if (!n1.startsWith("t")) continue;

            for (final String n2 : edges) {
                final Set<String> intersection = new HashSet<>(input.get(n2));
                intersection.retainAll(edges);

                for (final String n3 : intersection) {
                    final List<String> set = Stream.of(n1, n2, n3)
                        .sorted(String::compareTo)
                        .distinct()
                        .toList();
                    ans.add(set);
                }
            }
        }

        return ans.size();
    }

    public static String part2(final Map<String, Set<String>> input) {
        final Set<String> seen = new HashSet<>();
        final List<String> party = new ArrayList<>();
        List<String> largest = new ArrayList<>();

        for (final String n1 : input.keySet()) {
            if (seen.contains(n1)) continue;

            party.clear();
            party.add(n1);

            for (final String n2 : input.get(n1)) {
                if (input.get(n2).containsAll(party)) {
                    seen.add(n2);
                    party.add(n2);
                }
            }

            if (party.size() > largest.size()) {
                largest = List.copyOf(party);
            }
        }
        return largest
            .stream()
            .sorted(String::compareTo)
            .collect(Collectors.joining(","));
    }

    public static Map<String, Set<String>> parseInput(final String input) {
        final Map<String, Set<String>> map = new HashMap<>();
        input
            .lines()
            .forEach(line -> {
                final String[] parts = line.split("-");
                map.computeIfAbsent(parts[0], k -> new HashSet<>()).add(parts[1]);
                map.computeIfAbsent(parts[1], k -> new HashSet<>()).add(parts[0]);
            });
        return map;
    }
}