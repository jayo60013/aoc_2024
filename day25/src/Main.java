import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;

public class Main {
    public record Input(List<List<Integer>> locks, List<List<Integer>> keys) {
    }

    public static final String FULL_ROW = "#####";
    public static final int LOCK_HEIGHT = 7;

    public static void main(String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: java Day24 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final String contents = Files.readString(Path.of(fileName));

        final Input input = parseInput(contents);

        final int part1Answer = part1(input);
        System.out.printf("Part 1: %d\n", part1Answer);
    }

    public static int part1(final Input input) {
        int result = 0;
        for (final List<Integer> lock : input.locks) {
            for (final List<Integer> key : input.keys) {
                final boolean match = IntStream
                    .range(0, 5)
                    .allMatch(i -> lock.get(i) + key.get(i) <= LOCK_HEIGHT);
                if (match) {
                    result++;
                }
            }
        }
        return result;
    }

    public static Input parseInput(final String input) {
        final List<List<Integer>> locks = new ArrayList<>();
        final List<List<Integer>> keys = new ArrayList<>();

        for (final String part : input.split("\n\n")) {
            final String[] lines = part.split("\n");

            final List<Integer> schematic = getProngs(lines);
            if (FULL_ROW.equals(lines[0])) {
                locks.add(schematic);
            } else {
                keys.add(schematic);
            }
        }
        return new Input(locks, keys);
    }

    public static List<Integer> getProngs(final String[] lines) {
        final int[] prongs = new int[5];

        for (final String line : lines) {
            for (int i = 0; i < line.length(); i++) {
                if (line.charAt(i) == '#') {
                    prongs[i]++;
                }
            }
        }
        return Arrays.stream(prongs).boxed().toList();
    }
}