import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import java.io.File

class Day15Tests {

    @ParameterizedTest
    @CsvSource(
        "smaller_sample.txt, 2028",
        "sample.txt, 10092",
        "input.txt, 1563092"
    )
    fun testPart1(filename: String, expected: Int) {
        val content = File(filename).readText()
        val (obstacleMap, instructions, robotPos) = parseInput(content)

        val actual = part1(obstacleMap, instructions, robotPos)

        assertThat(actual).isEqualTo(expected)
    }
}