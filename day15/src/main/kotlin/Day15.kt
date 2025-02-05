import java.io.File

fun main(args: Array<String>) {
    val content = File(args[0]).readText()
    val (obstacleMap, instructions, robotPos) = parseInput(content)

    val part1Answer = part1(obstacleMap, instructions, robotPos)
    println("Part 1: $part1Answer")
}

fun part1(initialObstacleMap: Map<Vec2, Obstacle>, instructions: List<Instruction>, startPos: Vec2): Int {
    assert(startPos != Vec2(-1, -1))
    val obstacleMap = initialObstacleMap.toMutableMap()
    var currPos = startPos

    instructions.forEach { instr ->
        val nextPos = currPos.add(instr.dir)
        val obstacle = obstacleMap[nextPos]

        when (obstacle) {
            null -> currPos = nextPos
            Obstacle.BOX -> {
                var lookAhead = nextPos
                while (obstacleMap[lookAhead] == Obstacle.BOX) {
                    lookAhead = lookAhead.add(instr.dir)
                }

                if (obstacleMap[lookAhead] == null) {
                    obstacleMap[lookAhead] = Obstacle.BOX
                    obstacleMap.remove(nextPos)
                    currPos = nextPos
                }
            }

            else -> {
                //do nothing
            }
        }
    }
    return calcAnswer(obstacleMap)
}

fun calcAnswer(map: Map<Vec2, Obstacle>): Int {
    return map.entries
        .filter { it.value == Obstacle.BOX }
        .sumOf { it.key.y * 100 + it.key.x }
}

fun parseInput(input: String): Triple<Map<Vec2, Obstacle>, List<Instruction>, Vec2> {
    val (map, instructions) = input.split("\n\n", limit = 2)

    val obstacleMap = mutableMapOf<Vec2, Obstacle>()
    var robotPos = Vec2(-1, -1)

    map.lines().forEachIndexed { y, line ->
        line.forEachIndexed { x, ch ->
            Obstacle.fromSymbol(ch)?.let { obstacle ->
                if (obstacle == Obstacle.ROBOT) {
                    robotPos = Vec2(x, y)
                } else {
                    obstacleMap[Vec2(x, y)] = obstacle
                }
            }
        }
    }
    val instructionList = instructions.mapNotNull(Instruction::fromSymbol)
    return Triple(obstacleMap, instructionList, robotPos)
}

enum class Obstacle(val symbol: Char) {
    WALL('#'),
    BOX('O'),
    ROBOT('@');

    companion object {
        private val symbolMap = entries.associateBy(Obstacle::symbol)
        fun fromSymbol(symbol: Char): Obstacle? = symbolMap[symbol]
    }
}

enum class Instruction(val symbol: Char, val dir: Vec2) {
    LEFT('<', Vec2(-1, 0)),
    RIGHT('>', Vec2(1, 0)),
    UP('^', Vec2(0, -1)),
    DOWN('v', Vec2(0, 1));

    companion object {
        private val symbolMap = entries.associateBy(Instruction::symbol)
        fun fromSymbol(symbol: Char): Instruction? = symbolMap[symbol]
    }
}

data class Vec2(val x: Int, val y: Int) {
    fun add(other: Vec2): Vec2 = Vec2(x + other.x, y + other.y)
}