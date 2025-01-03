package com.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import static com.raylib.Colors.BLACK;
import static com.raylib.Raylib.*;


public class Main {
    public static final Pattern ROBOT_PATTERN = Pattern.compile("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)");
    public static final int WINDOW_WIDTH = Robot.WIDTH * 8;
    public static final int TOOLBAR_HEIGHT = 75;
    public static final int WINDOW_HEIGHT = Robot.HEIGHT * 6 + TOOLBAR_HEIGHT;


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
        final long part2Answer = part2(robots);
        System.out.printf("Part 2: %d\n", part2Answer);

        InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "Day14");
        final int cellWidth = WINDOW_WIDTH / Robot.WIDTH;
        final int cellHeight = (WINDOW_HEIGHT - TOOLBAR_HEIGHT) / Robot.HEIGHT;
        SetTargetFPS(60);

        boolean isPlay = true;

        int stepCount = 0;
        while (!WindowShouldClose()) {
            if (IsKeyPressed(KEY_SPACE)) isPlay = !isPlay;
            if (IsKeyPressed(KEY_UP)) {
                skip5StepsForwards(robots);
                stepCount += 5;
            } else if (IsKeyPressed(KEY_DOWN)) {
                skip5StepsBackwards(robots);
                stepCount -= 5;
            } else if (IsKeyPressed(KEY_LEFT)) {
                stepRobotsBackwards(robots);
                stepCount--;
            } else if (IsKeyPressed(KEY_RIGHT)) {
                stepRobotsForwards(robots);
                stepCount++;
            }

            BeginDrawing();
            ClearBackground(BLACK);
            DrawText(String.format("Steps: %d", stepCount), 5, WINDOW_HEIGHT - TOOLBAR_HEIGHT, 24, RAYWHITE);
            for (final Robot robot : robots) {
                robot.draw(cellWidth, cellHeight);
            }
            EndDrawing();
        }
        CloseWindow();
    }

    public static void skip5StepsForwards(final List<Robot> robots) {
        IntStream.range(0, 5).forEach(i -> stepRobotsForwards(robots));
    }

    public static void skip5StepsBackwards(final List<Robot> robots) {
        IntStream.range(0, 5).forEach(i -> stepRobotsBackwards(robots));
    }

    public static void stepRobotsForwards(final List<Robot> robots) {
        robots.forEach(Robot::stepForwards);
    }

    public static void stepRobotsBackwards(final List<Robot> robots) {
        robots.forEach(Robot::stepBackwards);
    }

    public static long part1(final List<Robot> input) {
        final List<Robot> robots = List.copyOf(input);
        IntStream.range(0, 100).forEach(i -> robots.forEach(Robot::stepForwards));

        return getSafetyScore(robots);
    }

    public static int part2(final List<Robot> input) {
        final List<Robot> robots = List.copyOf(input);
        final Map<Integer, Long> safetyScores = new HashMap<>();
        IntStream.range(1, Robot.WIDTH * Robot.HEIGHT)
            .forEach(i -> {
                robots.forEach(Robot::stepForwards);
                safetyScores.put(i, getSafetyScore(robots));
            });

        return safetyScores.entrySet()
            .stream()
            .min(Map.Entry.comparingByValue())
            .map(Map.Entry::getKey)
            .orElseThrow();
    }

    public static long getSafetyScore(final List<Robot> robots) {
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