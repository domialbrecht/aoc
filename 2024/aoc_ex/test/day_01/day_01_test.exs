defmodule ElixirAoc.Day01Test do
  use ExUnit.Case

  test "day01 part1" do
    assert ElixirAoc.Day01.part1("3   4
4   3
2   5
1   3
3   9
3   3") == 11
  end

  test "day01 part1 is not 12" do
    assert ElixirAoc.Day01.part1("3   4
4   3
2   5
1   3
3   9
3   3") != 12
  end

  test "day01 part2" do
    assert ElixirAoc.Day01.part2("3   4
4   3
2   5
1   3
3   9
3   3") == 31
  end
end
