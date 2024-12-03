defmodule ElixirAoc.Day02Test do
  use ExUnit.Case

  test "day_02 part1" do
    assert ElixirAoc.Day02.part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9") == 2
  end

  test "day_02 part1 too big gap" do
    assert ElixirAoc.Day02.part1("1 2 3 4 5 10") == 0
  end

  test "day_02 part1 too big gap decr" do
    assert ElixirAoc.Day02.part1("10 9 8 7 3") == 0
  end

  test "day_02 part1 large gap" do
    assert ElixirAoc.Day02.part1("55 59 60 61 62 65") == 0
  end

  test "day_02 part1 not cont" do
    assert ElixirAoc.Day02.part1("10 9 8 7 7") == 0
  end

  test "day_02 part1 same start" do
    assert ElixirAoc.Day02.part1("1 1 2 3 4") == 0
  end

  test "day_02 part1 same enc" do
    assert ElixirAoc.Day02.part1("1 2 3 4 4") == 0
  end

  test "day_02 part1 0" do
    assert ElixirAoc.Day02.part1("0 1 2 3 4") == 1
  end

  test "day_02 part1 data" do
    assert ElixirAoc.Day02.part1("72 78 78 79 83
8 15 15 17 19 25
60 66 69 70 74 75 77 79
19 24 25 29 28
28 34 38 40 42 42
24 31 34 35 39 43
11 17 18 21 25 28 34
63 69 76 77 78
28 35 37 43 41
71 78 79 84 85 87 90 90
25 31 32 33 36 41 45
36 41 42 43 48 53
37 36 35 32 29 32
25 23 20 17 16 13 13
33 31 30 27 25 22 20 16
47 45 43 41 39 33
15 12 11 8 11 10
63 60 58 59 58 57 56 57
5 4 3 2 1 4 4
") == 0
  end

  test "day_02 part2" do
    assert ElixirAoc.Day02.part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9") == 4
  end

  test "day_02 part2 first" do
    assert ElixirAoc.Day02.part2("7 6 4 2 1") == 1
  end

  test "day_02 part2 second unsafe" do
    assert ElixirAoc.Day02.part2("1 2 7 8 9") == 0
  end

  test "day_02 part2 third unsafe" do
    assert ElixirAoc.Day02.part2("9 7 6 2 1") == 0
  end

  test "day_02 part2 forth safe" do
    assert ElixirAoc.Day02.part2("1 3 2 4 5") == 1
  end
end
