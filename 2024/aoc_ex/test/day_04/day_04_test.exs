defmodule ElixirAoc.Day04Test do
  use ExUnit.Case

  test "day_04 part1" do
    assert ElixirAoc.Day04.part1("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX") == 18
  end

  test "day_04 part2" do
    assert ElixirAoc.Day04.part2(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........") == 9
  end
end
