import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;

public class Day17 {
    public static void main(final String[] args) throws IOException {
        if (args.length != 1) {
            System.out.println("Usage: java Day17 <filename.txt>");
            return;
        }

        final String fileName = args[0];
        final List<String> contents = Files.readAllLines(Path.of(fileName));
        final Device device = parseInput(contents);

        final String part1Answer = part1(device);
        final long part2Answer = part2(device);

        System.out.printf("Part 1: %s\n", part1Answer);
        System.out.printf("Part 2: %d\n", part2Answer);
    }

    public static long combo(final Device device, final int literal_operand) {
        if (literal_operand <= 3) {
            return literal_operand;
        } else if (literal_operand == 4) {
            return device.a;
        } else if (literal_operand == 5) {
            return device.b;
        } else if (literal_operand == 6) {
            return device.c;
        }

        return -1;
    }

    public static String part1(final Device device) {
        final StringBuilder output = new StringBuilder();

        int pc = 0;
        while (pc < device.program.size()) {
            final int opcode = device.program.get(pc);
            final int operand = device.program.get(pc + 1);

            if (opcode == 0) {
                device.a >>= combo(device, operand);
            } else if (opcode == 1) {
                device.b ^= operand;
            } else if (opcode == 2) {
                device.b = combo(device, operand) % 8;
            } else if (opcode == 3) {
                if (device.a != 0) {
                    pc = operand;
                    continue;
                }
            } else if (opcode == 4) {
                device.b ^= device.c;
            } else if (opcode == 5) {
                output.append(combo(device, operand) % 8).append(",");
            } else if (opcode == 6) {
                device.b = device.a >> combo(device, operand);
            } else if (opcode == 7) {
                device.c = device.a >> combo(device, operand);
            }

            pc += 2;
        }
        return output.substring(0, output.length() - 1);
    }

    public static long part2(final Device device) {
        //Increment from 0 until digit produced matches last digit. Times by 8 then repeat
        long a = 0;
        final List<String> targetOutput = device.program
                .stream()
                .map(String::valueOf)
                .toList();
        for (int i = targetOutput.size(); i >= 0; i--) {
            System.out.printf("i: %d, a: %d\n", i, a);
            final String subTargetOutput = String.join(",", targetOutput.subList(i, targetOutput.size()));
            String output = "";
            while (!subTargetOutput.equals(output)) {
                device.a = a;
                output = part1(device);
                a++;
            }
            a >>= 3;
        }
        return a;
    }

    public static Device parseInput(final List<String> input) {
        if (input.size() != 5) {
            return null;
        }

        final int a = Integer.parseInt(input.get(0).split(": ")[1]);
        final int b = Integer.parseInt(input.get(1).split(": ")[1]);
        final int c = Integer.parseInt(input.get(2).split(": ")[1]);

        final List<Integer> program = Arrays.stream(input.get(4)
                        .split(": ")[1]
                        .split(","))
                .map(Integer::parseInt)
                .toList();
        return new Device(a, b, c, program);
    }
}